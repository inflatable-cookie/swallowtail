use crate::output::{OutputState, failure};
use std::future::Future;
use std::pin::Pin;
use std::process::{Child, Command, ExitStatus, Stdio};
use std::sync::mpsc::Receiver;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Waker};
use std::thread;
use std::time::{Duration, Instant};
use swallowtail_runtime::{ProcessExit, RuntimeFailure};

/// Bound for joining the output reader threads after the child exits.
///
/// A descendant of the child can inherit the output pipes and hold them open
/// past the child's own exit, so EOF (and therefore a clean reader finish)
/// may arrive arbitrarily late. The supervisor must not wait forever for the
/// readers; output capture is bounded and abandoned when this bound elapses.
pub(crate) const READER_JOIN_BOUND: Duration = Duration::from_secs(2);
/// Poll-interval for bounded reader joins.
const JOIN_POLL_INTERVAL: Duration = Duration::from_millis(10);
/// Grace period between a close-stdin request and process-tree termination.
pub(crate) const GRACEFUL_STOP_BOUND: Duration = Duration::from_secs(1);

/// Consecutive failed kill attempts before a requested force stop is
/// reported as failed. Each attempt is one supervision tick, so this is a
/// one-second give-up bound for a child that survives SIGKILL.
const MAX_FORCE_KILL_ATTEMPTS: u32 = 100;

/// Outcome of joining one output reader thread under a bound.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum JoinOutcome {
    /// The reader thread finished normally.
    Completed,
    /// The reader thread was still running when the bound elapsed; its
    /// handle is dropped and the thread is detached until the pipe closes.
    TimedOut,
    /// The reader thread panicked.
    Panicked,
}

/// Joins one output reader thread, giving up after `bound` has elapsed.
///
/// Returns `Completed` only when the thread actually finished. A `TimedOut`
/// result drops the handle and detaches the blocked thread; it finishes by
/// itself if the pipe ever closes.
pub(crate) fn join_with_bound(handle: thread::JoinHandle<()>, bound: Duration) -> JoinOutcome {
    let deadline = Instant::now()
        .checked_add(bound)
        .expect("reader join deadline is representable");
    loop {
        if handle.is_finished() {
            return match handle.join() {
                Ok(()) => JoinOutcome::Completed,
                Err(_) => JoinOutcome::Panicked,
            };
        }
        let now = Instant::now();
        if now >= deadline {
            return JoinOutcome::TimedOut;
        }
        thread::sleep((deadline - now).min(JOIN_POLL_INTERVAL));
    }
}

pub(crate) enum ChildCommand {
    RequestStop,
    ForceStop,
}

#[derive(Default)]
struct ExitInner {
    result: Option<Result<ProcessExit, RuntimeFailure>>,
    waiters: Vec<Waker>,
}

#[derive(Default)]
pub(crate) struct ExitState {
    inner: Mutex<ExitInner>,
}

impl ExitState {
    pub(crate) fn complete(&self, result: Result<ProcessExit, RuntimeFailure>) {
        let mut inner = self.inner.lock().expect("local process exit lock poisoned");
        if inner.result.is_none() {
            inner.result = Some(result);
        }
        for waiter in inner.waiters.drain(..) {
            waiter.wake();
        }
    }

    pub(crate) fn future(self: &Arc<Self>) -> ExitFuture {
        ExitFuture {
            state: Arc::clone(self),
        }
    }

    pub(crate) fn is_complete(&self) -> bool {
        self.inner
            .lock()
            .expect("local process exit lock poisoned")
            .result
            .is_some()
    }
}

pub(crate) struct ExitFuture {
    state: Arc<ExitState>,
}

impl Future for ExitFuture {
    type Output = Result<ProcessExit, RuntimeFailure>;

    fn poll(self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<Self::Output> {
        let mut inner = self
            .state
            .inner
            .lock()
            .expect("local process exit lock poisoned");
        if let Some(result) = &inner.result {
            Poll::Ready(result.clone())
        } else {
            if !inner
                .waiters
                .iter()
                .any(|waiter| waiter.will_wake(context.waker()))
            {
                inner.waiters.push(context.waker().clone());
            }
            Poll::Pending
        }
    }
}

pub(crate) fn supervise_child(
    child: &mut Child,
    commands: Receiver<ChildCommand>,
    stdout_reader: thread::JoinHandle<()>,
    stderr_reader: thread::JoinHandle<()>,
    output: &OutputState,
    exit: &ExitState,
) {
    let process_group_id = child.id();
    let mut graceful_requested = false;
    let mut graceful_signal_sent = false;
    let mut graceful_deadline = None;
    let mut force_deadline = None;
    let mut force_seen = false;
    let mut failed_kills = 0_u32;
    let status = loop {
        match commands.recv_timeout(Duration::from_millis(10)) {
            Ok(ChildCommand::RequestStop) => {
                if !graceful_requested {
                    graceful_requested = true;
                    graceful_deadline = Instant::now().checked_add(GRACEFUL_STOP_BOUND);
                }
            }
            Ok(ChildCommand::ForceStop) => force_seen = true,
            Err(_) => {}
        }
        match child.try_wait() {
            Ok(Some(status)) => break Ok(status),
            Ok(None) => {}
            Err(_) => {
                break Err(failure(
                    "swallowtail.local_process.wait_failed",
                    "Local process exit could not be observed",
                ));
            }
        }
        if graceful_requested
            && !graceful_signal_sent
            && graceful_deadline.is_some_and(|deadline| Instant::now() >= deadline)
        {
            graceful_signal_sent = true;
            force_deadline = Instant::now().checked_add(GRACEFUL_STOP_BOUND);
            let _ = terminate_process_tree(process_group_id, false);
        } else if graceful_signal_sent
            && !force_seen
            && force_deadline.is_some_and(|deadline| Instant::now() >= deadline)
        {
            force_seen = true;
        }
        if force_seen {
            // The natural exit above wins over a stop that raced it: killing
            // is attempted only after the child is still listed. SIGKILL
            // cannot be blocked or ignored, so a failed kill on our own child
            // means it already exited and the next try_wait reaps it. The
            // process-group request also covers descendants that inherited
            // the launcher's process group.
            if !terminate_process_tree(process_group_id, true) {
                failed_kills = failed_kills.saturating_add(1);
                if failed_kills >= MAX_FORCE_KILL_ATTEMPTS {
                    break Err(failure(
                        "swallowtail.local_process.force_stop_failed",
                        "Local process could not be force-stopped",
                    ));
                }
            } else {
                failed_kills = 0;
            }
        }
    };

    // Reap the root first, then close any inherited pipe holders that escaped
    // the root's lifetime. The process group id is the launcher's pid and is
    // retained for this final bounded cleanup request.
    let _ = terminate_process_tree(process_group_id, true);

    // Join the readers under a bound. A descendant holding the output pipes
    // can delay EOF past the child's own exit; waiting forever would stall
    // wait() and read_output() consumers and leak the supervisor thread.
    let stdout_outcome = join_with_bound(stdout_reader, READER_JOIN_BOUND);
    let stderr_outcome = join_with_bound(stderr_reader, READER_JOIN_BOUND);

    if matches!(stdout_outcome, JoinOutcome::TimedOut)
        || matches!(stderr_outcome, JoinOutcome::TimedOut)
    {
        // Output capture is abandoned: the blocked reader thread is detached
        // and finishes if the pipe ever closes, and the stream closes so
        // read_output() drains to terminal truth instead of stalling.
        output.close_abandoned();
    }

    if matches!(stdout_outcome, JoinOutcome::Panicked)
        || matches!(stderr_outcome, JoinOutcome::Panicked)
    {
        exit.complete(Err(failure(
            "swallowtail.local_process.reader_panicked",
            "Local process output supervision failed",
        )));
        return;
    }
    exit.complete(status.map(exit_record));
}

fn terminate_process_tree(process_group_id: u32, force: bool) -> bool {
    #[cfg(unix)]
    {
        let signal = if force { "-KILL" } else { "-TERM" };
        let group = format!("-{process_group_id}");
        Command::new("/bin/kill")
            .arg(signal)
            .arg(group)
            .env_clear()
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .map(|status| status.success())
            .unwrap_or(false)
    }

    #[cfg(windows)]
    {
        let mut command = Command::new(r"C:\Windows\System32\taskkill.exe");
        command
            .args(["/PID", &process_group_id.to_string(), "/T"])
            .env_clear()
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null());
        if force {
            command.arg("/F");
        }
        command
            .status()
            .map(|status| status.success())
            .unwrap_or(false)
    }

    #[cfg(not(any(unix, windows)))]
    {
        let _ = (process_group_id, force);
        false
    }
}

fn exit_record(status: ExitStatus) -> ProcessExit {
    ProcessExit::new(status.success(), status.code())
}

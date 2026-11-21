use crate::output::{OutputState, failure};
use crate::process_exit::{ChildCommand, ExitState};
use std::collections::VecDeque;
use std::future::Future;
use std::io::Write;
use std::pin::Pin;
use std::process::{Child, ChildStderr, ChildStdin, ChildStdout};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Waker};
use std::thread;
#[cfg(unix)]
use std::time::Duration;
use swallowtail_runtime::{
    BoxFuture, ProcessExit, ProcessHandle, ProcessInputChunk, ProcessOutputChunk, RuntimeFailure,
};

mod supervision;

type BootstrapAdmission<'a> =
    dyn Fn(&mut dyn FnMut() -> std::io::Result<usize>) -> std::io::Result<usize> + Sync + 'a;

pub(crate) struct LocalProcessParts {
    pub(crate) child: Child,
    pub(crate) group_owner: Option<Child>,
    pub(crate) stdin: ChildStdin,
    pub(crate) stdout: ChildStdout,
    pub(crate) stderr: ChildStderr,
}

struct WriteReplyState {
    outcome: Option<Result<(), RuntimeFailure>>,
    waker: Option<Waker>,
}

struct WriteReply {
    state: Mutex<WriteReplyState>,
    cancelled: AtomicBool,
}

impl WriteReply {
    fn new() -> Arc<Self> {
        Arc::new(Self {
            state: Mutex::new(WriteReplyState {
                outcome: None,
                waker: None,
            }),
            cancelled: AtomicBool::new(false),
        })
    }

    fn complete(&self, result: Result<(), RuntimeFailure>) {
        let waker = {
            let mut state = self
                .state
                .lock()
                .expect("local process stdin reply lock poisoned");
            if state.outcome.is_some() {
                return;
            }
            state.outcome = Some(result);
            state.waker.take()
        };
        if let Some(waker) = waker {
            waker.wake();
        }
    }

    fn is_cancelled(&self) -> bool {
        self.cancelled.load(Ordering::SeqCst)
    }
}

struct WriteRequest {
    bytes: Vec<u8>,
    reply: Arc<WriteReply>,
}

enum StdinPipeState {
    Raw(ChildStdin),
    Bootstrapping,
    ActorRunning,
    Closed,
}

struct StdinInbox {
    pipe: StdinPipeState,
    close_requested: bool,
    queue: VecDeque<WriteRequest>,
    actor_waker: Option<Waker>,
    close_done_waiters: Vec<(u64, Waker)>,
    next_waiter_id: u64,
    written_bytes: usize,
}

pub(super) struct StdinCoordinator {
    inbox: Mutex<StdinInbox>,
    stdin_limit: usize,
}

impl StdinCoordinator {
    pub(super) fn new(stdin: ChildStdin, stdin_limit: usize) -> Arc<Self> {
        Arc::new(Self {
            inbox: Mutex::new(StdinInbox {
                pipe: StdinPipeState::Raw(stdin),
                close_requested: false,
                queue: VecDeque::new(),
                actor_waker: None,
                close_done_waiters: Vec::new(),
                next_waiter_id: 1,
                written_bytes: 0,
            }),
            stdin_limit,
        })
    }

    fn wake_actor(&self) {
        let waker = self
            .inbox
            .lock()
            .expect("local process stdin lock poisoned")
            .actor_waker
            .take();
        if let Some(waker) = waker {
            waker.wake();
        }
    }

    fn finish_actor_close(&self) {
        let (pending, waiters) = {
            let mut inbox = self
                .inbox
                .lock()
                .expect("local process stdin lock poisoned");
            inbox.pipe = StdinPipeState::Closed;
            inbox.actor_waker = None;
            let pending = std::mem::take(&mut inbox.queue);
            let waiters = std::mem::take(&mut inbox.close_done_waiters);
            (pending, waiters)
        };
        for request in pending {
            request.reply.complete(Err(stdin_closed_failure()));
        }
        for (_, waker) in waiters {
            waker.wake();
        }
    }

    fn close_nonblocking(&self) {
        let (dropped, actor_waker, waiters) = {
            let mut inbox = self
                .inbox
                .lock()
                .expect("local process stdin lock poisoned");
            inbox.close_requested = true;
            match std::mem::replace(&mut inbox.pipe, StdinPipeState::Closed) {
                StdinPipeState::Raw(stdin) => {
                    let waiters = std::mem::take(&mut inbox.close_done_waiters);
                    (Some(stdin), None, waiters)
                }
                StdinPipeState::ActorRunning => {
                    inbox.pipe = StdinPipeState::ActorRunning;
                    (None, inbox.actor_waker.take(), Vec::new())
                }
                StdinPipeState::Bootstrapping => {
                    inbox.pipe = StdinPipeState::Bootstrapping;
                    (None, None, Vec::new())
                }
                StdinPipeState::Closed => {
                    let waiters = std::mem::take(&mut inbox.close_done_waiters);
                    (None, None, waiters)
                }
            }
        };
        drop(dropped);
        if let Some(waker) = actor_waker {
            waker.wake();
        }
        for (_, waker) in waiters {
            waker.wake();
        }
    }

    fn enqueue_write(self: &Arc<Self>, bytes: Vec<u8>) -> Result<Arc<WriteReply>, RuntimeFailure> {
        let reply = WriteReply::new();
        let (spawn_stdin, actor_waker) = {
            let mut inbox = self
                .inbox
                .lock()
                .expect("local process stdin lock poisoned");
            if inbox.close_requested || matches!(inbox.pipe, StdinPipeState::Closed) {
                return Err(stdin_closed_failure());
            }
            if matches!(inbox.pipe, StdinPipeState::Bootstrapping) {
                return Err(stdin_closed_failure());
            }
            let spawn_stdin = match std::mem::replace(&mut inbox.pipe, StdinPipeState::ActorRunning)
            {
                StdinPipeState::Raw(stdin) => Some(stdin),
                StdinPipeState::ActorRunning => None,
                StdinPipeState::Bootstrapping | StdinPipeState::Closed => unreachable!(),
            };
            inbox.queue.push_back(WriteRequest {
                bytes,
                reply: Arc::clone(&reply),
            });
            (spawn_stdin, inbox.actor_waker.take())
        };
        if let Some(stdin) = spawn_stdin {
            let coordinator = Arc::clone(self);
            if thread::Builder::new()
                .name("swallowtail-stdin".to_owned())
                .spawn(move || run_stdin_actor(coordinator, stdin))
                .is_err()
            {
                self.finish_actor_close();
                return Err(stdin_write_failed_failure());
            }
        } else if let Some(waker) = actor_waker {
            waker.wake();
        }
        Ok(reply)
    }

    fn wait_closed_done(self: &Arc<Self>) -> CloseDoneFuture {
        CloseDoneFuture {
            coordinator: Arc::clone(self),
            waiter_id: None,
        }
    }
}

struct WriteFuture {
    coordinator: Arc<StdinCoordinator>,
    reply: Arc<WriteReply>,
}

impl Future for WriteFuture {
    type Output = Result<(), RuntimeFailure>;

    fn poll(self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<Self::Output> {
        let mut state = self
            .reply
            .state
            .lock()
            .expect("local process stdin reply lock poisoned");
        if let Some(outcome) = state.outcome.take() {
            state.waker = None;
            Poll::Ready(outcome)
        } else {
            state.waker = Some(context.waker().clone());
            Poll::Pending
        }
    }
}

impl Drop for WriteFuture {
    fn drop(&mut self) {
        self.reply.cancelled.store(true, Ordering::SeqCst);
        self.reply
            .state
            .lock()
            .expect("local process stdin reply lock poisoned")
            .waker = None;
        self.coordinator.wake_actor();
    }
}

struct CloseDoneFuture {
    coordinator: Arc<StdinCoordinator>,
    waiter_id: Option<u64>,
}

impl Future for CloseDoneFuture {
    type Output = ();

    fn poll(self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.get_mut();
        let mut inbox = this
            .coordinator
            .inbox
            .lock()
            .expect("local process stdin lock poisoned");
        if matches!(inbox.pipe, StdinPipeState::Closed) {
            if let Some(id) = this.waiter_id.take() {
                inbox
                    .close_done_waiters
                    .retain(|(existing, _)| *existing != id);
            }
            return Poll::Ready(());
        }
        let id = match this.waiter_id {
            Some(id) => id,
            None => {
                let id = inbox.next_waiter_id;
                inbox.next_waiter_id = inbox.next_waiter_id.wrapping_add(1);
                this.waiter_id = Some(id);
                id
            }
        };
        if let Some((_, slot)) = inbox
            .close_done_waiters
            .iter_mut()
            .find(|(existing, _)| *existing == id)
        {
            *slot = context.waker().clone();
        } else {
            inbox.close_done_waiters.push((id, context.waker().clone()));
        }
        Poll::Pending
    }
}

impl Drop for CloseDoneFuture {
    fn drop(&mut self) {
        if let Some(id) = self.waiter_id.take() {
            self.coordinator
                .inbox
                .lock()
                .expect("local process stdin lock poisoned")
                .close_done_waiters
                .retain(|(existing, _)| *existing != id);
        }
    }
}

#[cfg(unix)]
fn run_stdin_actor(coordinator: Arc<StdinCoordinator>, stdin: ChildStdin) {
    futures_lite::future::block_on(async move {
        let Ok(async_stdin) = async_io::Async::new(stdin) else {
            coordinator.finish_actor_close();
            return;
        };
        loop {
            let next = std::future::poll_fn(|context| {
                let mut inbox = coordinator
                    .inbox
                    .lock()
                    .expect("local process stdin lock poisoned");
                if inbox.close_requested {
                    return Poll::Ready(None);
                }
                while let Some(request) = inbox.queue.pop_front() {
                    if !request.reply.is_cancelled() {
                        return Poll::Ready(Some(request));
                    }
                }
                inbox.actor_waker = Some(context.waker().clone());
                Poll::Pending
            })
            .await;
            let Some(request) = next else {
                break;
            };
            let within_limit = {
                let inbox = coordinator
                    .inbox
                    .lock()
                    .expect("local process stdin lock poisoned");
                request.bytes.len() <= coordinator.stdin_limit.saturating_sub(inbox.written_bytes)
            };
            if !within_limit {
                request.reply.complete(Err(failure(
                    "swallowtail.local_process.stdin_limit_exceeded",
                    "Local process input exceeded its host-approved limit",
                )));
                continue;
            }
            let write_all = async {
                let mut offset = 0;
                while offset < request.bytes.len() {
                    let count = async_stdin
                        .write_with(|stream| (&*stream).write(&request.bytes[offset..]))
                        .await
                        .map_err(|_| stdin_write_failed_failure())?;
                    if count == 0 {
                        return Err(stdin_write_failed_failure());
                    }
                    // Charge bytes immediately as they are accepted by the kernel
                    // pipe so cancelling a partial write never grants extra quota.
                    coordinator
                        .inbox
                        .lock()
                        .expect("local process stdin lock poisoned")
                        .written_bytes += count;
                    offset += count;
                }
                Ok(())
            };
            let interrupt = std::future::poll_fn(|context| {
                let mut inbox = coordinator
                    .inbox
                    .lock()
                    .expect("local process stdin lock poisoned");
                if inbox.close_requested {
                    return Poll::Ready(true);
                }
                if request.reply.is_cancelled() {
                    return Poll::Ready(false);
                }
                inbox.actor_waker = Some(context.waker().clone());
                Poll::Pending
            });
            enum WriteOutcome {
                Finished(Result<(), RuntimeFailure>),
                Closed,
                Cancelled,
            }
            let outcome = futures_lite::future::race(
                async { WriteOutcome::Finished(write_all.await) },
                async {
                    if interrupt.await {
                        WriteOutcome::Closed
                    } else {
                        WriteOutcome::Cancelled
                    }
                },
            )
            .await;
            match outcome {
                WriteOutcome::Finished(result) => request.reply.complete(result),
                WriteOutcome::Closed => {
                    request.reply.complete(Err(stdin_closed_failure()));
                    break;
                }
                WriteOutcome::Cancelled => {}
            }
        }
        // Drop the Async<ChildStdin> before marking Closed so EOF is guaranteed
        // on the child pipe before close_stdin().await returns.
        drop(async_stdin);
        coordinator.finish_actor_close();
    });
}

#[cfg(not(unix))]
fn run_stdin_actor(coordinator: Arc<StdinCoordinator>, mut stdin: ChildStdin) {
    loop {
        let next = {
            let mut inbox = coordinator
                .inbox
                .lock()
                .expect("local process stdin lock poisoned");
            if inbox.close_requested {
                break;
            }
            inbox.queue.pop_front()
        };
        let Some(request) = next else {
            thread::sleep(std::time::Duration::from_millis(5));
            continue;
        };
        if request.reply.is_cancelled() {
            continue;
        }
        let within_limit = {
            let inbox = coordinator
                .inbox
                .lock()
                .expect("local process stdin lock poisoned");
            request.bytes.len() <= coordinator.stdin_limit.saturating_sub(inbox.written_bytes)
        };
        if !within_limit {
            request.reply.complete(Err(failure(
                "swallowtail.local_process.stdin_limit_exceeded",
                "Local process input exceeded its host-approved limit",
            )));
            continue;
        }
        let mut offset = 0;
        let mut failed = false;
        while offset < request.bytes.len() {
            match stdin.write(&request.bytes[offset..]) {
                Ok(0) | Err(_) => {
                    failed = true;
                    break;
                }
                Ok(count) => {
                    coordinator
                        .inbox
                        .lock()
                        .expect("local process stdin lock poisoned")
                        .written_bytes += count;
                    offset += count;
                }
            }
        }
        if failed || stdin.flush().is_err() {
            request.reply.complete(Err(stdin_write_failed_failure()));
        } else {
            request.reply.complete(Ok(()));
        }
    }
    drop(stdin);
    coordinator.finish_actor_close();
}

struct SharedChild {
    stdin: Arc<StdinCoordinator>,
    output: Arc<OutputState>,
    exit: Arc<ExitState>,
    commands: Sender<ChildCommand>,
    stop_requested: AtomicBool,
    force_requested: AtomicBool,
}

#[cfg(unix)]
struct BootstrapRestoreGuard<'a> {
    coordinator: &'a StdinCoordinator,
    stdin: Option<ChildStdin>,
}

#[cfg(unix)]
impl Drop for BootstrapRestoreGuard<'_> {
    fn drop(&mut self) {
        let stdin = self.stdin.take();
        let (dropped, waiters) = {
            let mut inbox = self
                .coordinator
                .inbox
                .lock()
                .expect("local process stdin lock poisoned");
            if !inbox.close_requested
                && matches!(inbox.pipe, StdinPipeState::Bootstrapping)
                && let Some(stdin) = stdin
            {
                inbox.pipe = StdinPipeState::Raw(stdin);
                (None, Vec::new())
            } else {
                inbox.pipe = StdinPipeState::Closed;
                (stdin, std::mem::take(&mut inbox.close_done_waiters))
            }
        };
        drop(dropped);
        for (_, waker) in waiters {
            waker.wake();
        }
    }
}

pub(crate) struct LocalProcessHandle {
    shared: Arc<SharedChild>,
}

impl LocalProcessHandle {
    #[cfg(unix)]
    pub(crate) async fn bootstrap(
        &self,
        bytes: &[u8],
        timeout: Duration,
    ) -> Result<(), RuntimeFailure> {
        self.bootstrap_with_admission(bytes, timeout, &|write| write())
            .await
    }

    #[cfg(unix)]
    pub(crate) async fn bootstrap_with_admission(
        &self,
        bytes: &[u8],
        timeout: Duration,
        admit: &BootstrapAdmission<'_>,
    ) -> Result<(), RuntimeFailure> {
        use async_io::{Async, Timer};
        use nix::fcntl::{FcntlArg, OFlag, fcntl};

        let stdin = {
            let mut inbox = self
                .shared
                .stdin
                .inbox
                .lock()
                .expect("local process stdin lock poisoned");
            if inbox.close_requested {
                return Err(stdin_closed_failure());
            }
            match std::mem::replace(&mut inbox.pipe, StdinPipeState::Bootstrapping) {
                StdinPipeState::Raw(stdin) => stdin,
                previous => {
                    inbox.pipe = previous;
                    return Err(stdin_closed_failure());
                }
            }
        };
        let mut restore_guard = BootstrapRestoreGuard {
            coordinator: &self.shared.stdin,
            stdin: None,
        };
        let original = fcntl(&stdin, FcntlArg::F_GETFL).map_err(|_| {
            failure(
                "swallowtail.local_process.bootstrap_unavailable",
                "Bootstrap input could not be configured",
            )
        })?;
        let stdin = Async::new(stdin).map_err(|_| {
            failure(
                "swallowtail.local_process.bootstrap_unavailable",
                "Bootstrap input could not be configured",
            )
        })?;
        let write = async {
            let mut offset = 0;
            // The final byte is the caller's admission point. A pending write
            // has not sent it; a successful final write resolves this write
            // future in the same poll. Later errors still stop the child.
            let frame_end = bytes.len().saturating_sub(1);
            while offset < frame_end {
                let count = stdin
                    .write_with(|stream| (&*stream).write(&bytes[offset..frame_end]))
                    .await
                    .map_err(|_| {
                        failure(
                            "swallowtail.local_process.bootstrap_write_failed",
                            "Bootstrap input could not be written",
                        )
                    })?;
                if count == 0 {
                    return Err(failure(
                        "swallowtail.local_process.bootstrap_write_failed",
                        "Bootstrap input could not be written",
                    ));
                }
                offset += count;
            }
            if let Some(final_byte) = bytes.last() {
                let count = stdin
                    .write_with(|stream| {
                        admit(&mut || (&*stream).write(std::slice::from_ref(final_byte)))
                    })
                    .await
                    .map_err(|_| {
                        failure(
                            "swallowtail.local_process.bootstrap_write_failed",
                            "Bootstrap input could not be written",
                        )
                    })?;
                if count != 1 {
                    return Err(failure(
                        "swallowtail.local_process.bootstrap_write_failed",
                        "Bootstrap input could not be written",
                    ));
                }
            }
            Ok(())
        };
        futures_lite::future::race(write, async {
            Timer::after(timeout).await;
            Err(failure(
                "swallowtail.local_process.bootstrap_timeout",
                "Bootstrap input deadline expired",
            ))
        })
        .await?;
        let stdin = stdin.into_inner().map_err(|_| {
            failure(
                "swallowtail.local_process.bootstrap_unavailable",
                "Bootstrap input could not be restored",
            )
        })?;
        fcntl(
            &stdin,
            FcntlArg::F_SETFL(OFlag::from_bits_truncate(original)),
        )
        .map_err(|_| {
            failure(
                "swallowtail.local_process.bootstrap_unavailable",
                "Bootstrap input could not be restored",
            )
        })?;
        restore_guard.stdin = Some(stdin);
        drop(restore_guard);
        Ok(())
    }

    async fn write_async(&self, chunk: ProcessInputChunk) -> Result<(), RuntimeFailure> {
        let reply = self.shared.stdin.enqueue_write(chunk.into_bytes())?;
        WriteFuture {
            coordinator: Arc::clone(&self.shared.stdin),
            reply,
        }
        .await
    }

    fn close_input(&self) {
        self.shared.stdin.close_nonblocking();
    }

    fn force(&self) -> Result<(), RuntimeFailure> {
        self.close_input();
        if self.shared.exit.is_complete() {
            return Ok(());
        }
        if self.shared.force_requested.swap(true, Ordering::SeqCst) {
            return Ok(());
        }
        self.shared
            .commands
            .send(ChildCommand::ForceStop)
            .map_err(|_| control_closed_failure())
            .or_else(|error| {
                if self.shared.exit.is_complete() {
                    Ok(())
                } else {
                    Err(error)
                }
            })
    }

    fn request(&self) -> Result<(), RuntimeFailure> {
        self.close_input();
        if self.shared.exit.is_complete() || self.shared.stop_requested.swap(true, Ordering::SeqCst)
        {
            return Ok(());
        }
        self.shared
            .commands
            .send(ChildCommand::RequestStop)
            .map_err(|_| control_closed_failure())
            .or_else(|error| {
                if self.shared.exit.is_complete() {
                    Ok(())
                } else {
                    Err(error)
                }
            })
    }
}

impl ProcessHandle for LocalProcessHandle {
    fn write_stdin(&self, chunk: ProcessInputChunk) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
        Box::pin(self.write_async(chunk))
    }

    fn close_stdin(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
        self.close_input();
        let wait = self.shared.stdin.wait_closed_done();
        Box::pin(async move {
            wait.await;
            Ok(())
        })
    }

    fn read_output(&self) -> BoxFuture<'_, Result<Option<ProcessOutputChunk>, RuntimeFailure>> {
        Box::pin(self.shared.output.read())
    }

    fn request_stop(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
        let result = self.request();
        Box::pin(async move { result })
    }

    fn force_stop(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
        let result = self.force();
        Box::pin(async move { result })
    }

    fn wait(&self) -> BoxFuture<'_, Result<ProcessExit, RuntimeFailure>> {
        let shared = Arc::clone(&self.shared);
        Box::pin(async move { shared.exit.future().await })
    }
}

fn stdin_closed_failure() -> RuntimeFailure {
    failure(
        "swallowtail.local_process.stdin_closed",
        "Local process input is closed",
    )
}

fn stdin_write_failed_failure() -> RuntimeFailure {
    failure(
        "swallowtail.local_process.stdin_write_failed",
        "Local process input could not be written",
    )
}

fn control_closed_failure() -> RuntimeFailure {
    failure(
        "swallowtail.local_process.control_closed",
        "Local process control is no longer available",
    )
}

impl Drop for LocalProcessHandle {
    fn drop(&mut self) {
        if !self.shared.exit.is_complete() {
            let _ = self.force();
        }
    }
}

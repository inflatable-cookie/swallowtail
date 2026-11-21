use crate::output::{OutputState, failure};
use crate::process_exit::{ChildCommand, ExitState};
use std::collections::VecDeque;
use std::future::Future;
use std::io::Write;
use std::pin::Pin;
use std::process::{Child, ChildStderr, ChildStdin, ChildStdout};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::Sender;
use std::sync::{Arc, Condvar, Mutex};
use std::task::{Context, Poll, Waker};
use std::thread;
use std::time::{Duration, Instant};
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
    Closing,
    Closed,
}

/// Maximum number of queued stdin write requests allowed while an active write
/// is in progress.
const MAX_QUEUED_STDIN_REQUESTS: usize = 64;

struct StdinInbox {
    pipe: StdinPipeState,
    close_requested: bool,
    queue: VecDeque<WriteRequest>,
    actor_waker: Option<Waker>,
    actor_handle: Option<thread::JoinHandle<()>>,
    close_done_waiters: Vec<(u64, Waker)>,
    next_waiter_id: u64,
    written_bytes: usize,
    active_remaining_bytes: usize,
    active_write_in_progress: bool,
    queued_bytes: usize,
}

impl StdinInbox {
    fn purge_cancelled_queue(&mut self) {
        let mut retained = VecDeque::with_capacity(self.queue.len());
        let mut retained_bytes = 0usize;
        while let Some(request) = self.queue.pop_front() {
            if !request.reply.is_cancelled() {
                retained_bytes = retained_bytes.saturating_add(request.bytes.len());
                retained.push_back(request);
            }
        }
        self.queue = retained;
        self.queued_bytes = retained_bytes;
    }
}

pub(crate) struct StdinCoordinator {
    inbox: Mutex<StdinInbox>,
    actor_notify: Condvar,
    stdin_limit: usize,
    blocking_close: bool,
    #[cfg(test)]
    before_raw_drop: Mutex<Option<Box<dyn FnOnce() + Send>>>,
}

impl StdinCoordinator {
    pub(super) fn new(stdin: ChildStdin, stdin_limit: usize) -> Arc<Self> {
        Arc::new(Self {
            inbox: Mutex::new(StdinInbox {
                pipe: StdinPipeState::Raw(stdin),
                close_requested: false,
                queue: VecDeque::new(),
                actor_waker: None,
                actor_handle: None,
                close_done_waiters: Vec::new(),
                next_waiter_id: 1,
                written_bytes: 0,
                active_remaining_bytes: 0,
                active_write_in_progress: false,
                queued_bytes: 0,
            }),
            actor_notify: Condvar::new(),
            stdin_limit,
            blocking_close: cfg!(not(unix)),
            #[cfg(test)]
            before_raw_drop: Mutex::new(None),
        })
    }

    #[cfg(any(not(unix), test))]
    fn is_close_requested(&self) -> bool {
        self.inbox
            .lock()
            .expect("local process stdin lock poisoned")
            .close_requested
    }

    fn cancel_queued_and_wake(&self) {
        let waker = {
            let mut inbox = self
                .inbox
                .lock()
                .expect("local process stdin lock poisoned");
            inbox.purge_cancelled_queue();
            inbox.actor_waker.take()
        };
        if let Some(waker) = waker {
            waker.wake();
        }
        self.actor_notify.notify_all();
    }

    fn finish_actor_close(&self) {
        let (pending, waiters) = {
            let mut inbox = self
                .inbox
                .lock()
                .expect("local process stdin lock poisoned");
            inbox.pipe = StdinPipeState::Closed;
            inbox.actor_waker = None;
            inbox.active_remaining_bytes = 0;
            inbox.active_write_in_progress = false;
            inbox.queued_bytes = 0;
            let pending = std::mem::take(&mut inbox.queue);
            let waiters = std::mem::take(&mut inbox.close_done_waiters);
            (pending, waiters)
        };
        self.actor_notify.notify_all();
        for request in pending {
            request.reply.complete(Err(stdin_closed_failure()));
        }
        for (_, waker) in waiters {
            waker.wake();
        }
    }

    pub(crate) fn shutdown_and_join(&self) {
        if let Err((_, Some(pending))) = self.shutdown_and_join_bounded(Duration::from_secs(2)) {
            let _ = pending.join();
        }
    }

    /// Requests stdin closure and joins any stdin actor within `bound`.
    ///
    /// If a blocking writer cannot exit within `bound`, this returns the typed
    /// failure together with the unjoined [`thread::JoinHandle`] so the caller
    /// (the process supervisor) retains direct ownership and joins the actor
    /// after publishing the bounded exit failure, rather than leaving the
    /// handle inside the coordinator.
    pub(crate) fn shutdown_and_join_bounded(
        &self,
        bound: Duration,
    ) -> Result<(), (RuntimeFailure, Option<thread::JoinHandle<()>>)> {
        self.close_nonblocking();
        let handle = self
            .inbox
            .lock()
            .expect("local process stdin lock poisoned")
            .actor_handle
            .take();
        let Some(handle) = handle else {
            return Ok(());
        };
        let deadline = Instant::now()
            .checked_add(bound)
            .expect("stdin join deadline is representable");
        loop {
            if handle.is_finished() {
                return handle
                    .join()
                    .map_err(|_| (stdin_join_failed_failure(), None));
            }
            let now = Instant::now();
            if now >= deadline {
                return Err((stdin_join_failed_failure(), Some(handle)));
            }
            let remaining = (deadline - now).min(Duration::from_millis(10));
            let inbox = self
                .inbox
                .lock()
                .expect("local process stdin lock poisoned");
            if !handle.is_finished() {
                let _ = self
                    .actor_notify
                    .wait_timeout(inbox, remaining)
                    .expect("local process stdin lock poisoned");
            }
        }
    }

    fn finish_raw_drop(&self, stdin: ChildStdin) {
        #[cfg(test)]
        if let Some(hook) = self
            .before_raw_drop
            .lock()
            .expect("before_raw_drop lock poisoned")
            .take()
        {
            hook();
        }
        drop(stdin);
        let waiters = {
            let mut inbox = self
                .inbox
                .lock()
                .expect("local process stdin lock poisoned");
            inbox.pipe = StdinPipeState::Closed;
            std::mem::take(&mut inbox.close_done_waiters)
        };
        self.actor_notify.notify_all();
        for (_, waker) in waiters {
            waker.wake();
        }
    }

    fn close_nonblocking(&self) {
        let (dropped, pending, actor_waker, waiters) = {
            let mut inbox = self
                .inbox
                .lock()
                .expect("local process stdin lock poisoned");
            inbox.close_requested = true;
            inbox.queued_bytes = 0;
            let pending = std::mem::take(&mut inbox.queue);
            match std::mem::replace(&mut inbox.pipe, StdinPipeState::Closing) {
                StdinPipeState::Raw(stdin) => {
                    // Hold Closing until drop(stdin) actually completes so a
                    // concurrent close_stdin() never observes Closed before EOF.
                    (Some(stdin), pending, None, Vec::new())
                }
                StdinPipeState::ActorRunning => {
                    // Keep ActorRunning until the actor actually drops the pipe
                    // descriptor and calls finish_actor_close().
                    inbox.pipe = StdinPipeState::ActorRunning;
                    (None, pending, inbox.actor_waker.take(), Vec::new())
                }
                StdinPipeState::Bootstrapping => {
                    inbox.pipe = StdinPipeState::Bootstrapping;
                    (None, pending, None, Vec::new())
                }
                StdinPipeState::Closing => (None, pending, None, Vec::new()),
                StdinPipeState::Closed => {
                    inbox.pipe = StdinPipeState::Closed;
                    let waiters = std::mem::take(&mut inbox.close_done_waiters);
                    (None, pending, None, waiters)
                }
            }
        };
        if let Some(stdin) = dropped {
            self.finish_raw_drop(stdin);
        }
        for request in pending {
            request.reply.complete(Err(stdin_closed_failure()));
        }
        if let Some(waker) = actor_waker {
            waker.wake();
        }
        self.actor_notify.notify_all();
        for (_, waker) in waiters {
            waker.wake();
        }
    }

    fn enqueue_write(self: &Arc<Self>, bytes: Vec<u8>) -> Result<Arc<WriteReply>, RuntimeFailure> {
        let reply = WriteReply::new();
        let actor_waker = {
            let mut inbox = self
                .inbox
                .lock()
                .expect("local process stdin lock poisoned");
            if inbox.close_requested
                || matches!(inbox.pipe, StdinPipeState::Closing | StdinPipeState::Closed)
            {
                return Err(stdin_closed_failure());
            }
            if matches!(inbox.pipe, StdinPipeState::Bootstrapping) {
                return Err(stdin_closed_failure());
            }
            inbox.purge_cancelled_queue();
            let committed = inbox
                .written_bytes
                .saturating_add(inbox.active_remaining_bytes)
                .saturating_add(inbox.queued_bytes);
            if bytes.len() > self.stdin_limit.saturating_sub(committed)
                || inbox.queue.len() >= MAX_QUEUED_STDIN_REQUESTS
            {
                return Err(failure(
                    "swallowtail.local_process.stdin_limit_exceeded",
                    "Local process input exceeded its host-approved limit",
                ));
            }
            if matches!(inbox.pipe, StdinPipeState::Raw(_)) {
                let StdinPipeState::Raw(stdin) =
                    std::mem::replace(&mut inbox.pipe, StdinPipeState::ActorRunning)
                else {
                    unreachable!();
                };
                // Spawn and install actor_handle while holding inbox so
                // shutdown_and_join() can never observe ActorRunning without a
                // join handle or race ahead of first-actor startup.
                let coordinator = Arc::clone(self);
                match thread::Builder::new()
                    .name("swallowtail-stdin".to_owned())
                    .spawn(move || run_stdin_actor(coordinator, stdin))
                {
                    Ok(handle) => {
                        inbox.actor_handle = Some(handle);
                    }
                    Err(_) => {
                        inbox.pipe = StdinPipeState::Closed;
                        let waiters = std::mem::take(&mut inbox.close_done_waiters);
                        drop(inbox);
                        for (_, waker) in waiters {
                            waker.wake();
                        }
                        return Err(stdin_write_failed_failure());
                    }
                }
            }
            inbox.queued_bytes = inbox.queued_bytes.saturating_add(bytes.len());
            inbox.queue.push_back(WriteRequest {
                bytes,
                reply: Arc::clone(&reply),
            });
            inbox.actor_waker.take()
        };
        if let Some(waker) = actor_waker {
            waker.wake();
        }
        self.actor_notify.notify_one();
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
        self.coordinator.cancel_queued_and_wake();
    }
}

struct CloseDoneFuture {
    coordinator: Arc<StdinCoordinator>,
    waiter_id: Option<u64>,
}

impl Future for CloseDoneFuture {
    type Output = Result<(), RuntimeFailure>;

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
            return Poll::Ready(Ok(()));
        }
        if this.coordinator.blocking_close && inbox.active_write_in_progress {
            if let Some(id) = this.waiter_id.take() {
                inbox
                    .close_done_waiters
                    .retain(|(existing, _)| *existing != id);
            }
            return Poll::Ready(Err(stdin_close_blocked_failure()));
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
                    inbox.queued_bytes = inbox.queued_bytes.saturating_sub(request.bytes.len());
                    if !request.reply.is_cancelled() {
                        inbox.active_remaining_bytes = request.bytes.len();
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
                    let mut inbox = coordinator
                        .inbox
                        .lock()
                        .expect("local process stdin lock poisoned");
                    inbox.written_bytes += count;
                    inbox.active_remaining_bytes =
                        inbox.active_remaining_bytes.saturating_sub(count);
                    drop(inbox);
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
            coordinator
                .inbox
                .lock()
                .expect("local process stdin lock poisoned")
                .active_remaining_bytes = 0;
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
fn run_stdin_actor(coordinator: Arc<StdinCoordinator>, stdin: ChildStdin) {
    run_blocking_stdin_actor(coordinator, stdin);
}

#[cfg(any(not(unix), test))]
fn run_blocking_stdin_actor<W: Write + Send + 'static>(
    coordinator: Arc<StdinCoordinator>,
    mut writer: W,
) {
    loop {
        let next = {
            let mut inbox = coordinator
                .inbox
                .lock()
                .expect("local process stdin lock poisoned");
            loop {
                if inbox.close_requested {
                    break None;
                }
                let mut found = None;
                while let Some(request) = inbox.queue.pop_front() {
                    inbox.queued_bytes = inbox.queued_bytes.saturating_sub(request.bytes.len());
                    if !request.reply.is_cancelled() {
                        inbox.active_remaining_bytes = request.bytes.len();
                        inbox.active_write_in_progress = true;
                        found = Some(request);
                        break;
                    }
                }
                if found.is_some() {
                    break found;
                }
                inbox = coordinator
                    .actor_notify
                    .wait(inbox)
                    .expect("local process stdin lock poisoned");
            }
        };
        let Some(request) = next else {
            break;
        };
        let mut offset = 0;
        let mut failed = false;
        while offset < request.bytes.len() {
            if request.reply.is_cancelled() || coordinator.is_close_requested() {
                break;
            }
            match writer.write(&request.bytes[offset..]) {
                Ok(0) | Err(_) => {
                    failed = true;
                    break;
                }
                Ok(count) => {
                    let mut inbox = coordinator
                        .inbox
                        .lock()
                        .expect("local process stdin lock poisoned");
                    inbox.written_bytes += count;
                    inbox.active_remaining_bytes =
                        inbox.active_remaining_bytes.saturating_sub(count);
                    offset += count;
                }
            }
        }
        let close_requested = {
            let mut inbox = coordinator
                .inbox
                .lock()
                .expect("local process stdin lock poisoned");
            inbox.active_remaining_bytes = 0;
            inbox.active_write_in_progress = false;
            inbox.close_requested
        };
        if close_requested {
            request.reply.complete(Err(stdin_closed_failure()));
            break;
        }
        if request.reply.is_cancelled() {
            continue;
        }
        if failed || writer.flush().is_err() {
            request.reply.complete(Err(stdin_write_failed_failure()));
        } else {
            request.reply.complete(Ok(()));
        }
    }
    // Drop the writer before marking Closed so close_stdin() only reports Ok(())
    // once the underlying descriptor has actually been released.
    drop(writer);
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
        let should_close = {
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
                None
            } else {
                // Keep Closing until the descriptor has actually dropped before
                // publishing Closed and waking close_done_waiters.
                inbox.pipe = StdinPipeState::Closing;
                Some(stdin)
            }
        };
        if let Some(dropped) = should_close {
            drop(dropped);
            let waiters = {
                let mut inbox = self
                    .coordinator
                    .inbox
                    .lock()
                    .expect("local process stdin lock poisoned");
                inbox.pipe = StdinPipeState::Closed;
                std::mem::take(&mut inbox.close_done_waiters)
            };
            self.coordinator.actor_notify.notify_all();
            for (_, waker) in waiters {
                waker.wake();
            }
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
        Box::pin(self.shared.stdin.wait_closed_done())
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

fn stdin_close_blocked_failure() -> RuntimeFailure {
    failure(
        "swallowtail.local_process.stdin_close_blocked",
        "Local process input could not be closed while a blocking write is in progress",
    )
}

fn stdin_join_failed_failure() -> RuntimeFailure {
    failure(
        "swallowtail.local_process.stdin_join_failed",
        "Local process input writer could not be joined",
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
        self.close_input();
        if !self.shared.exit.is_complete() {
            let _ = self.force();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Barrier;

    struct BlockingWriterState {
        entered: Mutex<bool>,
        entered_cv: Condvar,
        release: Mutex<bool>,
        release_cv: Condvar,
        dropped: AtomicBool,
    }

    struct StalledWriter {
        state: Arc<BlockingWriterState>,
    }

    impl Write for StalledWriter {
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            {
                let mut entered = self.state.entered.lock().expect("entered lock");
                *entered = true;
                self.state.entered_cv.notify_all();
            }
            let mut release = self.state.release.lock().expect("release lock");
            while !*release {
                release = self.state.release_cv.wait(release).expect("release wait");
            }
            Ok(buf.len())
        }

        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }

    impl Drop for StalledWriter {
        fn drop(&mut self) {
            self.state.dropped.store(true, Ordering::SeqCst);
        }
    }

    fn blocking_coordinator_with_writer<W: Write + Send + 'static>(
        writer: W,
        stdin_limit: usize,
    ) -> Arc<StdinCoordinator> {
        let coordinator = Arc::new(StdinCoordinator {
            inbox: Mutex::new(StdinInbox {
                pipe: StdinPipeState::ActorRunning,
                close_requested: false,
                queue: VecDeque::new(),
                actor_waker: None,
                actor_handle: None,
                close_done_waiters: Vec::new(),
                next_waiter_id: 1,
                written_bytes: 0,
                active_remaining_bytes: 0,
                active_write_in_progress: false,
                queued_bytes: 0,
            }),
            actor_notify: Condvar::new(),
            stdin_limit,
            blocking_close: true,
            before_raw_drop: Mutex::new(None),
        });
        let worker = Arc::clone(&coordinator);
        let handle = thread::Builder::new()
            .name("swallowtail-stdin-test".to_owned())
            .spawn(move || run_blocking_stdin_actor(worker, writer))
            .expect("test stdin actor spawns");
        coordinator.inbox.lock().expect("stdin lock").actor_handle = Some(handle);
        coordinator
    }

    #[test]
    fn blocking_close_fails_closed_while_writer_is_stalled_and_succeeds_after_drop() {
        let state = Arc::new(BlockingWriterState {
            entered: Mutex::new(false),
            entered_cv: Condvar::new(),
            release: Mutex::new(false),
            release_cv: Condvar::new(),
            dropped: AtomicBool::new(false),
        });
        let coordinator = blocking_coordinator_with_writer(
            StalledWriter {
                state: Arc::clone(&state),
            },
            1024,
        );

        // Start a write that blocks inside StalledWriter::write.
        let reply = coordinator
            .enqueue_write(b"stalled-payload".to_vec())
            .expect("write enqueues");
        {
            let mut entered = state.entered.lock().expect("entered lock");
            while !*entered {
                entered = state.entered_cv.wait(entered).expect("entered wait");
            }
        }

        // Calling close_stdin while the blocking write is in progress must NOT
        // claim success while the writer is still open; it must fail closed with
        // a typed error and leave the pipe state ActorRunning until the writer drops.
        coordinator.close_nonblocking();
        let close_err = futures_lite::future::block_on(coordinator.wait_closed_done())
            .expect_err("close must fail closed while blocking write still owns pipe");
        assert_eq!(
            close_err.diagnostic().code(),
            "swallowtail.local_process.stdin_close_blocked"
        );
        assert!(
            !state.dropped.load(Ordering::SeqCst),
            "writer must still be owned while blocked"
        );
        assert!(
            matches!(
                coordinator.inbox.lock().expect("stdin lock").pipe,
                StdinPipeState::ActorRunning
            ),
            "pipe state must remain ActorRunning until writer is actually dropped"
        );

        // Once the blocking write unblocks, the actor sees close_requested, fails
        // the write reply, drops the writer, and marks the pipe Closed.
        {
            let mut release = state.release.lock().expect("release lock");
            *release = true;
            state.release_cv.notify_all();
        }
        coordinator.shutdown_and_join();
        assert!(
            state.dropped.load(Ordering::SeqCst),
            "writer must be dropped before shutdown_and_join returns"
        );
        futures_lite::future::block_on(coordinator.wait_closed_done())
            .expect("close succeeds once the writer has been dropped");
        let write_outcome = futures_lite::future::block_on(WriteFuture {
            coordinator: Arc::clone(&coordinator),
            reply,
        });
        assert_eq!(
            write_outcome
                .expect_err("closed write fails")
                .diagnostic()
                .code(),
            "swallowtail.local_process.stdin_closed"
        );
    }

    #[test]
    fn blocking_close_succeeds_promptly_when_actor_is_idle() {
        let state = Arc::new(BlockingWriterState {
            entered: Mutex::new(false),
            entered_cv: Condvar::new(),
            release: Mutex::new(true),
            release_cv: Condvar::new(),
            dropped: AtomicBool::new(false),
        });
        let coordinator = blocking_coordinator_with_writer(
            StalledWriter {
                state: Arc::clone(&state),
            },
            1024,
        );

        coordinator.close_nonblocking();
        futures_lite::future::block_on(coordinator.wait_closed_done())
            .expect("idle blocking actor closes and drops descriptor promptly");
        assert!(
            state.dropped.load(Ordering::SeqCst),
            "writer must be dropped before close_stdin returns Ok(())"
        );
        coordinator.shutdown_and_join();
    }

    #[cfg(unix)]
    #[test]
    fn shutdown_and_join_synchronizes_with_concurrent_first_actor_startup() {
        use std::process::{Command, Stdio};

        for _ in 0..8 {
            let mut child = Command::new("/bin/cat")
                .stdin(Stdio::piped())
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .spawn()
                .expect("cat fixture spawns");
            let stdin = child.stdin.take().expect("stdin pipe present");
            let coordinator = StdinCoordinator::new(stdin, 1024);
            let start = Arc::new(Barrier::new(2));

            let writer_coordinator = Arc::clone(&coordinator);
            let writer_start = Arc::clone(&start);
            let writer = thread::spawn(move || {
                writer_start.wait();
                writer_coordinator.enqueue_write(b"race".to_vec())
            });

            start.wait();
            coordinator.shutdown_and_join();
            let write_result = writer.join().expect("writer thread finishes");

            // After shutdown_and_join returns, either enqueue_write lost the race
            // and never spawned an actor, or it won and shutdown_and_join already
            // observed and joined that actor handle.
            let inbox = coordinator.inbox.lock().expect("stdin lock");
            assert!(
                inbox.actor_handle.is_none(),
                "shutdown_and_join must never leave an unjoined actor handle behind"
            );
            assert!(
                matches!(inbox.pipe, StdinPipeState::Closed),
                "pipe must be closed after shutdown_and_join"
            );
            drop(inbox);
            if let Ok(reply) = write_result {
                let _ = futures_lite::future::block_on(WriteFuture {
                    coordinator: Arc::clone(&coordinator),
                    reply,
                });
            }
            let _ = child.kill();
            let _ = child.wait();
        }
    }

    #[cfg(unix)]
    #[test]
    fn supervised_exit_is_bounded_when_blocking_stdin_writer_remains_stalled() {
        use crate::process_exit::{ReaderSupervision, supervise_child};
        use crate::process_reader::{ReaderControl, spawn_reader};
        use std::process::{Command, Stdio};

        let state = Arc::new(BlockingWriterState {
            entered: Mutex::new(false),
            entered_cv: Condvar::new(),
            release: Mutex::new(false),
            release_cv: Condvar::new(),
            dropped: AtomicBool::new(false),
        });
        let coordinator = blocking_coordinator_with_writer(
            StalledWriter {
                state: Arc::clone(&state),
            },
            1024,
        );
        let _reply = coordinator
            .enqueue_write(b"surviving-descendant".to_vec())
            .expect("write enqueues");
        {
            let mut entered = state.entered.lock().expect("entered lock");
            while !*entered {
                entered = state.entered_cv.wait(entered).expect("entered wait");
            }
        }

        // Spawn a root child that exits immediately while the blocking stdin
        // writer remains stalled (mirroring a surviving Windows descendant
        // keeping the pipe read end open).
        let mut child = Command::new("/usr/bin/true")
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("true fixture spawns");
        let stdout = child.stdout.take().expect("stdout pipe");
        let stderr = child.stderr.take().expect("stderr pipe");
        let output = Arc::new(OutputState::default());
        let reader_control = Arc::new(ReaderControl::default());
        let stdout_reader = spawn_reader(
            "swallowtail-test-stdout",
            stdout,
            1024,
            swallowtail_runtime::ProcessOutputStream::Stdout,
            Arc::clone(&output),
            Arc::clone(&reader_control),
        )
        .expect("stdout reader spawns");
        let stderr_reader = spawn_reader(
            "swallowtail-test-stderr",
            stderr,
            1024,
            swallowtail_runtime::ProcessOutputStream::Stderr,
            Arc::clone(&output),
            Arc::clone(&reader_control),
        )
        .expect("stderr reader spawns");
        let exit = Arc::new(ExitState::default());
        let (_cmd_tx, cmd_rx) = std::sync::mpsc::channel();
        let supervisor_coordinator = Arc::clone(&coordinator);
        let supervisor_output = Arc::clone(&output);
        let supervisor_exit = Arc::clone(&exit);
        let mut group_owner = None;
        let supervisor = thread::spawn(move || {
            supervise_child(
                &mut child,
                &mut group_owner,
                cmd_rx,
                ReaderSupervision {
                    stdout: stdout_reader,
                    stderr: stderr_reader,
                    control: reader_control,
                },
                &supervisor_coordinator,
                &supervisor_output,
                &supervisor_exit,
            );
        });

        // Process exit must still be published within READER_JOIN_BOUND with a
        // typed failure instead of hanging on the blocked writer, while the
        // supervisor retains direct ownership of the pending actor handle
        // instead of returning it to the coordinator.
        let exit_err = futures_lite::future::block_on(exit.future())
            .expect_err("stalled blocking stdin writer must publish typed join failure");
        assert_eq!(
            exit_err.diagnostic().code(),
            "swallowtail.local_process.stdin_join_failed"
        );
        assert!(
            !state.dropped.load(Ordering::SeqCst),
            "writer is still blocked when bounded exit publishes"
        );
        assert!(
            coordinator
                .inbox
                .lock()
                .expect("stdin lock")
                .actor_handle
                .is_none(),
            "supervisor must own the pending actor handle rather than leaving it in the coordinator"
        );
        // Simulate caller teardown after wait() returns: drop the caller's
        // handles without calling any private shutdown helper a second time.
        drop(_reply);
        drop(coordinator);
        drop(exit);
        drop(output);
        assert!(
            !supervisor.is_finished(),
            "supervisor must remain alive owning the pending actor handle until it completes"
        );

        // Once the blocked writer eventually unblocks, the supervisor itself
        // joins the actor and exits cleanly.
        {
            let mut release = state.release.lock().expect("release lock");
            *release = true;
            state.release_cv.notify_all();
        }
        supervisor
            .join()
            .expect("supervisor joins pending stdin actor and exits cleanly");
        assert!(
            state.dropped.load(Ordering::SeqCst),
            "blocked writer must be dropped before the supervisor finishes joining the actor"
        );
    }

    #[cfg(unix)]
    #[test]
    fn concurrent_raw_pipe_close_waits_for_descriptor_drop_before_reporting_ok() {
        use std::pin::pin;
        use std::process::{Command, Stdio};

        let mut child = Command::new("/bin/cat")
            .stdin(Stdio::piped())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .expect("cat fixture spawns");
        let stdin = child.stdin.take().expect("stdin pipe present");
        let coordinator = StdinCoordinator::new(stdin, 1024);

        let drop_entered = Arc::new((Mutex::new(false), Condvar::new()));
        let allow_drop = Arc::new((Mutex::new(false), Condvar::new()));
        {
            let entered = Arc::clone(&drop_entered);
            let allow = Arc::clone(&allow_drop);
            *coordinator
                .before_raw_drop
                .lock()
                .expect("before_raw_drop lock") = Some(Box::new(move || {
                {
                    let mut flag = entered.0.lock().expect("entered lock");
                    *flag = true;
                    entered.1.notify_all();
                }
                let mut flag = allow.0.lock().expect("allow lock");
                while !*flag {
                    flag = allow.1.wait(flag).expect("allow wait");
                }
            }));
        }

        let first_coordinator = Arc::clone(&coordinator);
        let first_closer = thread::spawn(move || {
            first_coordinator.close_nonblocking();
            futures_lite::future::block_on(first_coordinator.wait_closed_done())
        });

        // Wait until the first closer has moved the raw pipe into Closing and
        // is paused immediately before drop(stdin).
        {
            let mut flag = drop_entered.0.lock().expect("entered lock");
            while !*flag {
                flag = drop_entered.1.wait(flag).expect("entered wait");
            }
        }

        // A second concurrent close must NOT observe Closed or return Ok(())
        // while the first closer still holds the descriptor.
        coordinator.close_nonblocking();
        let mut second_wait = pin!(coordinator.wait_closed_done());
        let mut context = Context::from_waker(Waker::noop());
        assert!(
            matches!(second_wait.as_mut().poll(&mut context), Poll::Pending),
            "concurrent raw-pipe close must stay Pending until descriptor drop finishes"
        );

        // Release the first closer so it drops stdin and publishes Closed.
        {
            let mut flag = allow_drop.0.lock().expect("allow lock");
            *flag = true;
            allow_drop.1.notify_all();
        }
        first_closer
            .join()
            .expect("first closer thread finishes")
            .expect("first close succeeds after drop");
        futures_lite::future::block_on(second_wait).expect("second close succeeds after drop");

        // Both successful closes strictly imply EOF on the child pipe.
        let status = child.wait().expect("cat exits after stdin EOF");
        assert!(status.success());
    }
}

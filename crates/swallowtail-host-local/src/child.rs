use crate::output::{OutputState, failure};
use crate::process_exit::{ChildCommand, ExitState};
use std::io::Write;
use std::process::{Child, ChildStderr, ChildStdin, ChildStdout};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex};
use std::task::{Poll, Waker};
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

enum StdinChannel {
    Raw(ChildStdin),
    #[cfg(unix)]
    Async(async_io::Async<ChildStdin>),
}

enum StdinSlot {
    Idle(StdinChannel),
    Writing,
    Closed,
}

struct StdinState {
    slot: StdinSlot,
    writer_waiters: Vec<Waker>,
    close_waiters: Vec<Waker>,
}

impl StdinState {
    fn new(stdin: ChildStdin) -> Self {
        Self {
            slot: StdinSlot::Idle(StdinChannel::Raw(stdin)),
            writer_waiters: Vec::new(),
            close_waiters: Vec::new(),
        }
    }
}

struct SharedChild {
    stdin: Mutex<StdinState>,
    stdin_bytes: Mutex<usize>,
    stdin_limit: usize,
    output: Arc<OutputState>,
    exit: Arc<ExitState>,
    commands: Sender<ChildCommand>,
    stop_requested: AtomicBool,
    force_requested: AtomicBool,
}

struct StdinWriteGuard<'a> {
    shared: &'a SharedChild,
    channel: Option<StdinChannel>,
}

impl Drop for StdinWriteGuard<'_> {
    fn drop(&mut self) {
        let channel = self.channel.take();
        let (dropped, writers) = {
            let mut state = self
                .shared
                .stdin
                .lock()
                .expect("local process stdin lock poisoned");
            let dropped = match (&state.slot, channel) {
                (StdinSlot::Writing, Some(channel)) => {
                    state.slot = StdinSlot::Idle(channel);
                    None
                }
                (StdinSlot::Writing, None) => {
                    state.slot = StdinSlot::Closed;
                    None
                }
                (_, channel) => channel,
            };
            let writers = std::mem::take(&mut state.writer_waiters);
            (dropped, writers)
        };
        drop(dropped);
        for waker in writers {
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
            let mut state = self
                .shared
                .stdin
                .lock()
                .expect("local process stdin lock poisoned");
            match std::mem::replace(&mut state.slot, StdinSlot::Writing) {
                StdinSlot::Idle(StdinChannel::Raw(stdin)) => stdin,
                StdinSlot::Idle(StdinChannel::Async(stdin)) => {
                    stdin.into_inner().map_err(|_| {
                        state.slot = StdinSlot::Closed;
                        failure(
                            "swallowtail.local_process.bootstrap_unavailable",
                            "Bootstrap input could not be configured",
                        )
                    })?
                }
                previous => {
                    state.slot = previous;
                    return Err(failure(
                        "swallowtail.local_process.stdin_closed",
                        "Local process input is closed",
                    ));
                }
            }
        };
        let mut restore_guard = StdinWriteGuard {
            shared: &self.shared,
            channel: None,
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
        restore_guard.channel = Some(StdinChannel::Raw(stdin));
        drop(restore_guard);
        Ok(())
    }

    async fn acquire_stdin(&self) -> Result<StdinWriteGuard<'_>, RuntimeFailure> {
        let channel = std::future::poll_fn(|context| {
            let mut state = self
                .shared
                .stdin
                .lock()
                .expect("local process stdin lock poisoned");
            match std::mem::replace(&mut state.slot, StdinSlot::Writing) {
                StdinSlot::Idle(channel) => Poll::Ready(Ok(channel)),
                StdinSlot::Closed => {
                    state.slot = StdinSlot::Closed;
                    Poll::Ready(Err(stdin_closed_failure()))
                }
                StdinSlot::Writing => {
                    state.writer_waiters.push(context.waker().clone());
                    Poll::Pending
                }
            }
        })
        .await?;
        Ok(StdinWriteGuard {
            shared: &self.shared,
            channel: Some(channel),
        })
    }

    async fn wait_stdin_closed(&self) {
        std::future::poll_fn(|context| {
            let mut state = self
                .shared
                .stdin
                .lock()
                .expect("local process stdin lock poisoned");
            if matches!(state.slot, StdinSlot::Closed) {
                Poll::Ready(())
            } else {
                state.close_waiters.push(context.waker().clone());
                Poll::Pending
            }
        })
        .await;
    }

    async fn write_async(&self, chunk: ProcessInputChunk) -> Result<(), RuntimeFailure> {
        let bytes = chunk.into_bytes();
        let mut guard = self.acquire_stdin().await?;
        {
            let written = self
                .shared
                .stdin_bytes
                .lock()
                .expect("local process stdin count lock poisoned");
            if bytes.len() > self.shared.stdin_limit.saturating_sub(*written) {
                return Err(failure(
                    "swallowtail.local_process.stdin_limit_exceeded",
                    "Local process input exceeded its host-approved limit",
                ));
            }
        }
        #[cfg(unix)]
        {
            let channel = guard.channel.take().expect("acquired stdin channel");
            let async_stdin = match channel {
                StdinChannel::Async(async_stdin) => async_stdin,
                StdinChannel::Raw(raw) => async_io::Async::new(raw).map_err(|_| {
                    failure(
                        "swallowtail.local_process.stdin_write_failed",
                        "Local process input could not be written",
                    )
                })?,
            };
            guard.channel = Some(StdinChannel::Async(async_stdin));
            let Some(StdinChannel::Async(stdin)) = guard.channel.as_ref() else {
                unreachable!("stdin channel was just stored as Async");
            };
            let write_all = async {
                let mut offset = 0;
                while offset < bytes.len() {
                    let count = stdin
                        .write_with(|stream| (&*stream).write(&bytes[offset..]))
                        .await
                        .map_err(|_| stdin_write_failed_failure())?;
                    if count == 0 {
                        return Err(stdin_write_failed_failure());
                    }
                    offset += count;
                }
                Ok(())
            };
            let closed = async {
                self.wait_stdin_closed().await;
                Err(stdin_closed_failure())
            };
            futures_lite::future::race(write_all, closed).await?;
        }
        #[cfg(not(unix))]
        {
            let Some(StdinChannel::Raw(stdin)) = guard.channel.as_mut() else {
                unreachable!("non-unix stdin channel is always Raw");
            };
            stdin
                .write_all(&bytes)
                .map_err(|_| stdin_write_failed_failure())?;
            stdin.flush().map_err(|_| stdin_write_failed_failure())?;
        }
        let mut written = self
            .shared
            .stdin_bytes
            .lock()
            .expect("local process stdin count lock poisoned");
        *written += bytes.len();
        Ok(())
    }

    fn close_input(&self) {
        let (dropped, writers, closers) = {
            let mut state = self
                .shared
                .stdin
                .lock()
                .expect("local process stdin lock poisoned");
            let dropped = match std::mem::replace(&mut state.slot, StdinSlot::Closed) {
                StdinSlot::Idle(channel) => Some(channel),
                StdinSlot::Writing | StdinSlot::Closed => None,
            };
            let writers = std::mem::take(&mut state.writer_waiters);
            let closers = std::mem::take(&mut state.close_waiters);
            (dropped, writers, closers)
        };
        drop(dropped);
        for waker in writers {
            waker.wake();
        }
        for waker in closers {
            waker.wake();
        }
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
        Box::pin(async { Ok(()) })
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

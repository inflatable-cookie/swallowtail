use crate::output::{OutputState, failure};
use crate::process_exit::{ChildCommand, ExitState};
use std::io::Write;
use std::process::{Child, ChildStderr, ChildStdin, ChildStdout};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex};
#[cfg(unix)]
use std::time::Duration;
use swallowtail_runtime::{
    BoxFuture, ProcessExit, ProcessHandle, ProcessInputChunk, ProcessOutputChunk, RuntimeFailure,
};

mod supervision;

pub(crate) struct LocalProcessParts {
    pub(crate) child: Child,
    pub(crate) group_owner: Option<Child>,
    pub(crate) stdin: ChildStdin,
    pub(crate) stdout: ChildStdout,
    pub(crate) stderr: ChildStderr,
}

struct SharedChild {
    stdin: Mutex<Option<ChildStdin>>,
    stdin_bytes: Mutex<usize>,
    stdin_limit: usize,
    output: Arc<OutputState>,
    exit: Arc<ExitState>,
    commands: Sender<ChildCommand>,
    stop_requested: AtomicBool,
    force_requested: AtomicBool,
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
        use async_io::{Async, Timer};
        use nix::fcntl::{FcntlArg, OFlag, fcntl};

        let stdin = self
            .shared
            .stdin
            .lock()
            .expect("local process stdin lock poisoned")
            .take()
            .ok_or_else(|| {
                failure(
                    "swallowtail.local_process.stdin_closed",
                    "Local process input is closed",
                )
            })?;
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
            while offset < bytes.len() {
                let count = stdin
                    .write_with(|stream| (&*stream).write(&bytes[offset..]))
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
        *self
            .shared
            .stdin
            .lock()
            .expect("local process stdin lock poisoned") = Some(stdin);
        Ok(())
    }
    fn write(&self, chunk: ProcessInputChunk) -> Result<(), RuntimeFailure> {
        let bytes = chunk.into_bytes();
        let mut written = self
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
        let mut stdin = self
            .shared
            .stdin
            .lock()
            .expect("local process stdin lock poisoned");
        let Some(stdin) = stdin.as_mut() else {
            return Err(failure(
                "swallowtail.local_process.stdin_closed",
                "Local process input is closed",
            ));
        };
        stdin.write_all(&bytes).map_err(|_| {
            failure(
                "swallowtail.local_process.stdin_write_failed",
                "Local process input could not be written",
            )
        })?;
        stdin.flush().map_err(|_| {
            failure(
                "swallowtail.local_process.stdin_write_failed",
                "Local process input could not be written",
            )
        })?;
        *written += bytes.len();
        Ok(())
    }

    fn close_input(&self) {
        self.shared
            .stdin
            .lock()
            .expect("local process stdin lock poisoned")
            .take();
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
        let result = self.write(chunk);
        Box::pin(async move { result })
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

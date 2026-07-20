use crate::output::{OutputState, failure};
use crate::process_exit::{ChildCommand, ExitState, supervise_child};
use crate::process_reader::spawn_reader;
use std::io::Write;
use std::process::{Child, ChildStderr, ChildStdin, ChildStdout};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{self, Sender};
use std::sync::{Arc, Mutex};
use std::thread;
use swallowtail_runtime::{
    BoxFuture, ProcessExit, ProcessHandle, ProcessInputChunk, ProcessOutputChunk,
    ProcessOutputStream, RuntimeFailure,
};

struct SharedChild {
    stdin: Mutex<Option<ChildStdin>>,
    stdin_bytes: Mutex<usize>,
    stdin_limit: usize,
    output: Arc<OutputState>,
    exit: Arc<ExitState>,
    commands: Sender<ChildCommand>,
    force_requested: AtomicBool,
}

pub(crate) struct LocalProcessHandle {
    shared: Arc<SharedChild>,
}

impl LocalProcessHandle {
    pub(crate) fn supervise(
        mut child: Child,
        stdin: ChildStdin,
        stdout: ChildStdout,
        stderr: ChildStderr,
        stdin_limit: usize,
        stdout_limit: usize,
        stderr_limit: usize,
    ) -> Result<Self, RuntimeFailure> {
        let output = Arc::new(OutputState::default());
        let stdout_reader = spawn_reader(
            "swallowtail-stdout",
            stdout,
            stdout_limit,
            ProcessOutputStream::Stdout,
            Arc::clone(&output),
        );
        let stderr_reader = spawn_reader(
            "swallowtail-stderr",
            stderr,
            stderr_limit,
            ProcessOutputStream::Stderr,
            Arc::clone(&output),
        );
        let (stdout_reader, stderr_reader) = match (stdout_reader, stderr_reader) {
            (Ok(stdout_reader), Ok(stderr_reader)) => (stdout_reader, stderr_reader),
            _ => {
                let _ = child.kill();
                let _ = child.wait();
                return Err(failure(
                    "swallowtail.local_process.reader_start_failed",
                    "Local process output supervision could not start",
                ));
            }
        };

        let exit = Arc::new(ExitState::default());
        let (commands, command_receiver) = mpsc::channel();
        let supervisor_exit = Arc::clone(&exit);
        let supervision_slot = Arc::new(Mutex::new(Some((child, stdout_reader, stderr_reader))));
        let supervisor_parts = Arc::clone(&supervision_slot);
        let supervisor = thread::Builder::new()
            .name("swallowtail-process".to_owned())
            .spawn(move || {
                let (mut child, stdout_reader, stderr_reader) = supervisor_parts
                    .lock()
                    .expect("local process supervision lock poisoned")
                    .take()
                    .expect("local process supervision parts must be present");
                supervise_child(
                    &mut child,
                    command_receiver,
                    stdout_reader,
                    stderr_reader,
                    &supervisor_exit,
                );
            });
        if supervisor.is_err() {
            if let Some((mut child, stdout_reader, stderr_reader)) = supervision_slot
                .lock()
                .expect("local process supervision lock poisoned")
                .take()
            {
                let _ = child.kill();
                let _ = child.wait();
                let _ = stdout_reader.join();
                let _ = stderr_reader.join();
            }
            return Err(failure(
                "swallowtail.local_process.supervisor_start_failed",
                "Local process supervision could not start",
            ));
        }

        Ok(Self {
            shared: Arc::new(SharedChild {
                stdin: Mutex::new(Some(stdin)),
                stdin_bytes: Mutex::new(0),
                stdin_limit,
                output,
                exit,
                commands,
                force_requested: AtomicBool::new(false),
            }),
        })
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
        if self.shared.force_requested.swap(true, Ordering::SeqCst) {
            return Ok(());
        }
        self.shared
            .commands
            .send(ChildCommand::ForceStop)
            .map_err(|_| {
                failure(
                    "swallowtail.local_process.control_closed",
                    "Local process control is no longer available",
                )
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
        self.close_input();
        Box::pin(async { Ok(()) })
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

impl Drop for LocalProcessHandle {
    fn drop(&mut self) {
        if !self.shared.exit.is_complete() {
            let _ = self.force();
        }
    }
}

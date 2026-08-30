use super::{OutputState, ProcessState};
use std::collections::VecDeque;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::task::Poll;
use swallowtail_core::SafeDiagnostic;
use swallowtail_runtime::{
    BoxFuture, ProcessExit, ProcessHandle, ProcessInputChunk, ProcessOutputChunk,
    ProcessOutputStream, RuntimeFailure,
};

pub(super) struct FakeProcessHandle {
    pub(super) state: Arc<ProcessState>,
    pub(super) output: Arc<Mutex<OutputState>>,
    pub(super) exit: Arc<Mutex<ProcessExit>>,
    pub(super) hold_open: Arc<AtomicBool>,
    pub(super) fail_stdin: Arc<AtomicBool>,
}

impl ProcessHandle for FakeProcessHandle {
    fn write_stdin(&self, chunk: ProcessInputChunk) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
        if self.fail_stdin.load(Ordering::SeqCst) {
            return Box::pin(async {
                Err(RuntimeFailure::new(SafeDiagnostic::new(
                    "fixture.process.stdin_failed",
                    "Fixture process stdin write failed",
                )))
            });
        }
        self.state
            .stdin
            .lock()
            .expect("stdin lock is available")
            .extend_from_slice(chunk.bytes());
        Box::pin(async { Ok(()) })
    }

    fn close_stdin(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
        self.state.stdin_closed.store(true, Ordering::SeqCst);
        Box::pin(async { Ok(()) })
    }

    fn read_output(&self) -> BoxFuture<'_, Result<Option<ProcessOutputChunk>, RuntimeFailure>> {
        Box::pin(std::future::poll_fn(|context| {
            let mut output = self.output.lock().expect("output lock is available");
            if let Some(chunk) = output.chunks.pop_front() {
                return Poll::Ready(Ok(Some(chunk)));
            }
            if output.closed
                || !self.hold_open.load(Ordering::SeqCst)
                || self.state.force_stopped.load(Ordering::SeqCst)
            {
                return Poll::Ready(Ok(None));
            }
            output.waker = Some(context.waker().clone());
            Poll::Pending
        }))
    }

    fn request_stop(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
        self.state.stdin_closed.store(true, Ordering::SeqCst);
        Box::pin(async { Ok(()) })
    }

    fn force_stop(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
        self.state.force_stopped.store(true, Ordering::SeqCst);
        self.hold_open.store(false, Ordering::SeqCst);
        self.output.lock().expect("output lock is available").wake();
        Box::pin(async { Ok(()) })
    }

    fn wait(&self) -> BoxFuture<'_, Result<ProcessExit, RuntimeFailure>> {
        self.state.waited.store(true, Ordering::SeqCst);
        let exit = *self.exit.lock().expect("exit lock is available");
        Box::pin(async move { Ok(exit) })
    }
}

pub(super) fn stdout_chunks(stdout: &str) -> VecDeque<ProcessOutputChunk> {
    if stdout.is_empty() {
        VecDeque::new()
    } else {
        VecDeque::from([ProcessOutputChunk::new(
            ProcessOutputStream::Stdout,
            stdout.as_bytes().to_vec(),
        )])
    }
}

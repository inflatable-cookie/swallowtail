use super::SharedAgent;
use std::sync::Arc;
use swallowtail_runtime::{
    BoxFuture, ProcessExit, ProcessHandle, ProcessInputChunk, ProcessOutputChunk, RuntimeFailure,
};

pub struct FixtureProcessHandle(Arc<SharedAgent>);

impl ProcessHandle for FixtureProcessHandle {
    fn write_stdin(&self, chunk: ProcessInputChunk) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
        let result = self.0.handle_write(chunk);
        Box::pin(async move { result })
    }

    fn close_stdin(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
        self.stop()
    }

    fn read_output(&self) -> BoxFuture<'_, Result<Option<ProcessOutputChunk>, RuntimeFailure>> {
        Box::pin(async move {
            let mut state = self.0.state.lock().expect("fixture agent lock poisoned");
            while state.output.is_empty() && !state.stopped {
                state = self
                    .0
                    .changed
                    .wait(state)
                    .expect("fixture agent wait lock poisoned");
            }
            Ok(state.output.pop_front())
        })
    }

    fn request_stop(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
        self.stop()
    }

    fn force_stop(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
        self.stop()
    }

    fn wait(&self) -> BoxFuture<'_, Result<ProcessExit, RuntimeFailure>> {
        Box::pin(async { Ok(ProcessExit::new(true, Some(0))) })
    }
}

impl FixtureProcessHandle {
    pub(crate) fn new(agent: Arc<SharedAgent>) -> Self {
        Self(agent)
    }

    fn stop(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
        let mut state = self.0.state.lock().expect("fixture agent lock poisoned");
        state.stopped = true;
        self.0.changed.notify_all();
        Box::pin(async { Ok(()) })
    }
}

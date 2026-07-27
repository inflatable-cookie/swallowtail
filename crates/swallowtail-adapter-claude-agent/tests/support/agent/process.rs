use super::*;

pub(in crate::support) struct FixtureProcessHandle {
    agent: Arc<SharedAgent>,
    cleanup: Arc<Mutex<Vec<&'static str>>>,
}

impl FixtureProcessHandle {
    pub(in crate::support) fn new(
        agent: Arc<SharedAgent>,
        cleanup: Arc<Mutex<Vec<&'static str>>>,
    ) -> Self {
        Self { agent, cleanup }
    }
}

impl ProcessHandle for FixtureProcessHandle {
    fn write_stdin(&self, chunk: ProcessInputChunk) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
        let result = self.agent.handle_write(chunk);
        Box::pin(async move { result })
    }

    fn close_stdin(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
        self.stop()
    }

    fn read_output(&self) -> BoxFuture<'_, Result<Option<ProcessOutputChunk>, RuntimeFailure>> {
        Box::pin(async move {
            let mut state = self
                .agent
                .state
                .lock()
                .expect("fixture agent lock poisoned");
            while state.output.is_empty() && !state.stopped {
                state = self
                    .agent
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
        self.cleanup
            .lock()
            .expect("fixture cleanup lock poisoned")
            .push("process_joined");
        Box::pin(async { Ok(ProcessExit::new(true, Some(0))) })
    }
}

impl FixtureProcessHandle {
    fn stop(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
        let mut state = self
            .agent
            .state
            .lock()
            .expect("fixture agent lock poisoned");
        state.stopped = true;
        self.agent.changed.notify_all();
        Box::pin(async { Ok(()) })
    }
}

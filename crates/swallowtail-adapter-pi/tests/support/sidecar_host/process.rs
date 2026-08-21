use super::super::CleanupEvent;
use super::script::respond;
use super::{Shared, SidecarFixtureHost, SidecarScenario, fixture_failure};
use serde_json::Value;
use std::sync::Arc;
use swallowtail_runtime::{
    BoxFuture, ProcessExit, ProcessHandle, ProcessInputChunk, ProcessOutputChunk, ProcessRequest,
    ProcessService, RuntimeFailure, ScopeId,
};

impl ProcessService for SidecarFixtureHost {
    fn start(
        &self,
        _scope: ScopeId,
        request: ProcessRequest,
    ) -> BoxFuture<'static, Result<Box<dyn ProcessHandle>, RuntimeFailure>> {
        *self
            .shared
            .process_request
            .lock()
            .expect("sidecar fixture process lock poisoned") = Some(request);
        let handle = SidecarFixtureProcess {
            shared: Arc::clone(&self.shared),
            scenario: self.scenario,
            wait_failure: self.process_wait_failure,
        };
        Box::pin(async move { Ok(Box::new(handle) as Box<dyn ProcessHandle>) })
    }
}

struct SidecarFixtureProcess {
    shared: Arc<Shared>,
    scenario: SidecarScenario,
    wait_failure: bool,
}

impl ProcessHandle for SidecarFixtureProcess {
    fn write_stdin(&self, chunk: ProcessInputChunk) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
        let result = (|| {
            let line = chunk
                .bytes()
                .strip_suffix(b"\n")
                .ok_or_else(fixture_failure)?;
            let value: Value = serde_json::from_slice(line).map_err(|_| fixture_failure())?;
            let mut state = self
                .shared
                .process
                .lock()
                .expect("sidecar fixture state lock poisoned");
            state.input.push(value.clone());
            respond(self.scenario, &value, &mut state)?;
            self.shared.changed.notify_all();
            Ok(())
        })();
        Box::pin(async move { result })
    }

    fn close_stdin(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
        self.stop()
    }

    fn read_output(&self) -> BoxFuture<'_, Result<Option<ProcessOutputChunk>, RuntimeFailure>> {
        Box::pin(async move {
            let mut state = self
                .shared
                .process
                .lock()
                .expect("sidecar fixture state lock poisoned");
            while state.output.is_empty() && !state.stopped {
                state = self
                    .shared
                    .changed
                    .wait(state)
                    .expect("sidecar fixture wait lock poisoned");
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
        self.shared
            .cleanup
            .lock()
            .expect("sidecar fixture cleanup lock poisoned")
            .push(CleanupEvent::ProcessWait);
        let wait_failure = self.wait_failure;
        Box::pin(async move {
            if wait_failure {
                Err(fixture_failure())
            } else {
                Ok(ProcessExit::new(true, Some(0)))
            }
        })
    }
}

impl SidecarFixtureProcess {
    fn stop(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
        self.shared
            .process
            .lock()
            .expect("sidecar fixture state lock poisoned")
            .stopped = true;
        self.shared.changed.notify_all();
        Box::pin(async { Ok(()) })
    }
}

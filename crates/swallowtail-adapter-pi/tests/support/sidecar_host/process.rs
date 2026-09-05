use super::super::CleanupEvent;
use super::script::respond;
use super::{Shared, SidecarFixtureHost, SidecarScenario, fixture_failure};
use serde_json::Value;
use std::sync::Arc;
use std::time::Duration;
use swallowtail_runtime::{
    BoxFuture, ProcessExit, ProcessHandle, ProcessInputChunk, ProcessOutputChunk, ProcessRequest,
    ProcessService, RuntimeFailure, ScopeId,
};

/// Large named hang guard for fixture waits that must resolve through
/// explicit test ordering. Expiry is a broken ordering contract, so it fails
/// loudly instead of hanging the run; no passing test relies on this bound.
const HANG_GUARD: Duration = Duration::from_secs(120);

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
            exit_failure: self.process_exit_failure,
        };
        Box::pin(async move { Ok(Box::new(handle) as Box<dyn ProcessHandle>) })
    }
}

struct SidecarFixtureProcess {
    shared: Arc<Shared>,
    scenario: SidecarScenario,
    wait_failure: bool,
    exit_failure: bool,
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
            let state = self
                .shared
                .process
                .lock()
                .expect("sidecar fixture state lock poisoned");
            let (mut state, wait) = self
                .shared
                .changed
                .wait_timeout_while(state, HANG_GUARD, |state| {
                    state.output.is_empty() && !state.stopped
                })
                .expect("sidecar fixture wait lock poisoned");
            assert!(
                !wait.timed_out(),
                "fixture hang guard: sidecar output was never observed within {HANG_GUARD:?}"
            );
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
        let exit_failure = self.exit_failure;
        let hold = matches!(self.scenario, SidecarScenario::Hold);
        let shared = Arc::clone(&self.shared);
        Box::pin(async move {
            if hold {
                let state = shared
                    .process
                    .lock()
                    .expect("sidecar fixture state lock poisoned");
                let (_state, wait) = shared
                    .changed
                    .wait_timeout_while(state, HANG_GUARD, |state| !state.hold_released)
                    .expect("sidecar fixture wait lock poisoned");
                assert!(
                    !wait.timed_out(),
                    "fixture hang guard: held sidecar exit was never released within {HANG_GUARD:?}"
                );
            }
            let result = if wait_failure {
                Err(fixture_failure())
            } else if exit_failure {
                Ok(ProcessExit::new(false, Some(1)))
            } else {
                Ok(ProcessExit::new(true, Some(0)))
            };
            shared
                .process
                .lock()
                .expect("sidecar fixture state lock poisoned")
                .exited = true;
            shared.changed.notify_all();
            result
        })
    }
}

impl SidecarFixtureProcess {
    fn stop(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
        let mut state = self
            .shared
            .process
            .lock()
            .expect("sidecar fixture state lock poisoned");
        if !matches!(self.scenario, SidecarScenario::Hold) || state.hold_released {
            state.stopped = true;
        }
        self.shared.changed.notify_all();
        Box::pin(async { Ok(()) })
    }
}

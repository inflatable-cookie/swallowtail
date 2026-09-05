use super::super::host::{CleanupEvent, SdkFixtureHost, Shared, fixture_failure};
use super::script::respond;
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

impl ProcessService for SdkFixtureHost {
    fn start(
        &self,
        _scope: ScopeId,
        request: ProcessRequest,
    ) -> BoxFuture<'static, Result<Box<dyn ProcessHandle>, RuntimeFailure>> {
        *self
            .shared
            .process_request
            .lock()
            .expect("SDK fixture process lock poisoned") = Some(request);
        let handle = SdkFixtureProcess {
            shared: Arc::clone(&self.shared),
            scenario: self.scenario,
            exit_observable: self.exit_observable,
            attests_empty_owned_tree: self.attests_empty_owned_tree,
            stall: self.stall,
        };
        let stalled = self.stall == Some(super::super::host::Stall::ProcessStart);
        Box::pin(async move {
            if stalled {
                std::future::pending::<()>().await;
            }
            Ok(Box::new(handle) as Box<dyn ProcessHandle>)
        })
    }
}

struct SdkFixtureProcess {
    shared: Arc<Shared>,
    scenario: super::super::host::SdkScenario,
    exit_observable: bool,
    attests_empty_owned_tree: bool,
    stall: Option<super::super::host::Stall>,
}

impl ProcessHandle for SdkFixtureProcess {
    fn write_stdin(&self, chunk: ProcessInputChunk) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
        let stalling_writes = self
            .shared
            .process
            .lock()
            .expect("SDK fixture state lock poisoned")
            .stalling_writes;
        if stalling_writes || self.stall == Some(super::super::host::Stall::ProcessWrite) {
            return Box::pin(async move {
                std::future::pending::<()>().await;
                Ok(())
            });
        }
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
                .expect("SDK fixture state lock poisoned");
            state.input.push(value.clone());
            if !state.holding {
                respond(self.scenario, &value, &mut state)?;
            }
            self.shared.changed.notify_all();
            Ok(())
        })();
        Box::pin(async move { result })
    }

    fn close_stdin(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
        self.record(CleanupEvent::ProcessRequestStop);
        self.stop()
    }

    fn read_output(&self) -> BoxFuture<'_, Result<Option<ProcessOutputChunk>, RuntimeFailure>> {
        let held = self.stall == Some(super::super::host::Stall::PumpRead);
        Box::pin(async move {
            let state = self
                .shared
                .process
                .lock()
                .expect("SDK fixture state lock poisoned");
            let (mut state, wait) = self
                .shared
                .changed
                .wait_timeout_while(state, HANG_GUARD, |state| {
                    if held || state.holding_pump {
                        !state.pump_released
                    } else {
                        state.output.is_empty() && !state.stopped
                    }
                })
                .expect("SDK fixture wait lock poisoned");
            assert!(
                !wait.timed_out(),
                "fixture hang guard: SDK output was never released or produced within {HANG_GUARD:?}"
            );
            if (held || state.holding_pump) && state.pump_released {
                return Ok(None);
            }
            Ok(state.output.pop_front())
        })
    }

    fn request_stop(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
        self.record(CleanupEvent::ProcessRequestStop);
        self.stop()
    }

    fn force_stop(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
        // The request is always recorded before any stall: a stalled
        // termination still proves the request was made.
        self.record(CleanupEvent::ProcessForceStop);
        if self.stall == Some(super::super::host::Stall::ForceStop) {
            return Box::pin(async move {
                std::future::pending::<()>().await;
                Ok(())
            });
        }
        self.stop()
    }

    fn wait(&self) -> BoxFuture<'_, Result<ProcessExit, RuntimeFailure>> {
        self.record(CleanupEvent::ProcessWait);
        let observable = self.exit_observable;
        let attests = self.attests_empty_owned_tree;
        let open_hold = matches!(self.scenario, super::super::host::SdkScenario::OpenHold);
        let shared = Arc::clone(&self.shared);
        Box::pin(async move {
            if open_hold {
                let state = shared
                    .process
                    .lock()
                    .expect("SDK fixture state lock poisoned");
                let (_state, wait) = shared
                    .changed
                    .wait_timeout_while(state, HANG_GUARD, |state| !state.process_hold_released)
                    .expect("SDK fixture wait lock poisoned");
                assert!(
                    !wait.timed_out(),
                    "fixture hang guard: open-hold exit was never released within {HANG_GUARD:?}"
                );
            }
            match (observable, attests) {
                // Only a host with a concrete owned-tree observation may
                // construct the attesting exit.
                (true, true) => Ok(ProcessExit::attesting_empty_owned_tree(true, Some(0))),
                (true, false) => Ok(ProcessExit::new(true, Some(0))),
                (false, _) => Err(fixture_failure()),
            }
        })
    }
}

impl SdkFixtureProcess {
    fn record(&self, event: CleanupEvent) {
        self.shared
            .cleanup
            .lock()
            .expect("SDK fixture cleanup lock poisoned")
            .push(event);
    }

    fn stop(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
        self.shared
            .process
            .lock()
            .expect("SDK fixture state lock poisoned")
            .stopped = true;
        self.shared.changed.notify_all();
        Box::pin(async { Ok(()) })
    }
}

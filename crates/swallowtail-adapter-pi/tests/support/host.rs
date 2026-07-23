use serde_json::Value;
use std::collections::VecDeque;
use std::sync::atomic::AtomicUsize;
use std::sync::{Arc, Condvar, Mutex};
use std::task::Waker;
use swallowtail_core::ExecutionHostId;
use swallowtail_runtime::{
    BoxFuture, HostServices, ProcessExit, ProcessHandle, ProcessInputChunk, ProcessOutputChunk,
    ProcessRequest, ProcessService, RuntimeFailure, ScopeId,
};

use self::script::respond;
use self::task_time::ThreadTaskService;

mod authority;
mod inspection;
mod script;
mod task_time;

#[derive(Clone, Copy)]
pub enum Scenario {
    Complete,
    Disconnect,
    Hold,
    Malformed,
    ProviderFailure,
    ResponseMismatch,
    RetryDrift,
    StateMismatch,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CleanupEvent {
    ProcessWait,
    ResourceRelease,
    CredentialRelease,
}

#[derive(Clone)]
pub struct FixtureHost {
    shared: Arc<Shared>,
    scenario: Scenario,
    process_wait_failure: bool,
}

struct Shared {
    process_request: Mutex<Option<ProcessRequest>>,
    process: Mutex<ProcessState>,
    changed: Condvar,
    credential_acquisitions: AtomicUsize,
    cleanup: Mutex<Vec<CleanupEvent>>,
    time: Mutex<TimeState>,
}

#[derive(Default)]
struct TimeState {
    now: u64,
    fire_through: Option<u64>,
    waiters: Vec<Waker>,
}

#[derive(Default)]
struct ProcessState {
    input: Vec<Value>,
    output: VecDeque<ProcessOutputChunk>,
    stopped: bool,
}

impl FixtureHost {
    pub fn new(scenario: Scenario) -> Self {
        Self {
            shared: Arc::new(Shared {
                process_request: Mutex::new(None),
                process: Mutex::new(ProcessState::default()),
                changed: Condvar::new(),
                credential_acquisitions: AtomicUsize::new(0),
                cleanup: Mutex::new(Vec::new()),
                time: Mutex::new(TimeState::default()),
            }),
            scenario,
            process_wait_failure: false,
        }
    }

    pub fn with_immediate_time(self) -> Self {
        let mut time = self
            .shared
            .time
            .lock()
            .expect("Pi fixture time lock poisoned");
        time.now = 1_000;
        time.fire_through = Some(u64::MAX);
        drop(time);
        self
    }

    pub fn with_process_wait_failure(mut self) -> Self {
        self.process_wait_failure = true;
        self
    }

    pub fn services(&self, host: ExecutionHostId) -> HostServices {
        HostServices::new(host)
            .with_task(Arc::new(ThreadTaskService))
            .with_process(Arc::new(self.clone()))
            .with_credential(Arc::new(self.clone()))
            .with_working_resource(Arc::new(self.clone()))
            .with_time(Arc::new(self.clone()))
    }
}

impl ProcessService for FixtureHost {
    fn start(
        &self,
        _scope: ScopeId,
        request: ProcessRequest,
    ) -> BoxFuture<'static, Result<Box<dyn ProcessHandle>, RuntimeFailure>> {
        *self
            .shared
            .process_request
            .lock()
            .expect("Pi fixture process lock poisoned") = Some(request);
        let handle = FixtureProcess {
            shared: Arc::clone(&self.shared),
            scenario: self.scenario,
            wait_failure: self.process_wait_failure,
        };
        Box::pin(async move { Ok(Box::new(handle) as Box<dyn ProcessHandle>) })
    }
}

struct FixtureProcess {
    shared: Arc<Shared>,
    scenario: Scenario,
    wait_failure: bool,
}

impl ProcessHandle for FixtureProcess {
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
                .expect("Pi fixture state lock poisoned");
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
                .expect("Pi fixture state lock poisoned");
            while state.output.is_empty() && !state.stopped {
                state = self
                    .shared
                    .changed
                    .wait(state)
                    .expect("Pi fixture wait lock poisoned");
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
            .expect("Pi fixture cleanup lock poisoned")
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

impl FixtureProcess {
    fn stop(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
        self.shared
            .process
            .lock()
            .expect("Pi fixture state lock poisoned")
            .stopped = true;
        self.shared.changed.notify_all();
        Box::pin(async { Ok(()) })
    }
}

fn fixture_failure() -> RuntimeFailure {
    RuntimeFailure::new(swallowtail_core::SafeDiagnostic::new(
        "fixture.pi_rpc.failed",
        "Pi RPC fixture failed",
    ))
}

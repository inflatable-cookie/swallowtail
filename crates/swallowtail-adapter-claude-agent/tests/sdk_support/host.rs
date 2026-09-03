//! Provider-free fake Claude Agent SDK sidecar host.
//!
//! No Node runtime, SDK package, native binary, provider session, or login is
//! involved: the fixture answers the private wire directly, so every case is
//! a wire-and-lifecycle proof rather than a provider observation.

use serde_json::Value;
use std::collections::VecDeque;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Condvar, Mutex};
use std::task::Waker;
use swallowtail_core::ExecutionHostId;
use swallowtail_runtime::{HostServices, ProcessOutputChunk, ProcessRequest};
use task_time::ThreadTaskService;

mod authority;
mod scenario;

pub use scenario::{SdkScenario, Stall};
mod process;
mod script;
mod task_time;

/// Ordered host-visible cleanup effects, so close ordering is testable.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CleanupEvent {
    ProcessRequestStop,
    ProcessForceStop,
    ProcessWait,
    ResourceRelease,
    CredentialRelease,
}

#[derive(Clone)]
pub struct SdkFixtureHost {
    shared: Arc<Shared>,
    scenario: SdkScenario,
    exit_observable: bool,
    attests_empty_owned_tree: bool,
    pub(super) stall: Option<Stall>,
}

pub(super) struct Shared {
    pub(super) process_request: Mutex<Option<ProcessRequest>>,
    pub(super) process: Mutex<ProcessState>,
    pub(super) changed: Condvar,
    pub(super) credential_acquisitions: AtomicUsize,
    pub(super) cleanup: Mutex<Vec<CleanupEvent>>,
    pub(super) time: Mutex<TimeState>,
    /// Scopes the fixture host accepted for its own reaping, recorded so tests
    /// can tell ownership transfer from a join.
    pub(super) relinquished: Mutex<Option<Arc<Mutex<Vec<String>>>>>,
}

#[derive(Default)]
pub(super) struct TimeState {
    pub(super) now: u64,
    pub(super) fire_through: Option<u64>,
    pub(super) waiters: Vec<Waker>,
}

#[derive(Default)]
pub(super) struct ProcessState {
    pub(super) input: Vec<Value>,
    pub(super) output: VecDeque<ProcessOutputChunk>,
    pub(super) stopped: bool,
    pub(super) opened: bool,
    pub(super) holding: bool,
    pub(super) stalling_writes: bool,
    pub(super) pump_released: bool,
    pub(super) holding_pump: bool,
}

impl SdkFixtureHost {
    pub fn new(scenario: SdkScenario) -> Self {
        Self {
            shared: Arc::new(Shared {
                process_request: Mutex::new(None),
                process: Mutex::new(ProcessState::default()),
                changed: Condvar::new(),
                credential_acquisitions: AtomicUsize::new(0),
                cleanup: Mutex::new(Vec::new()),
                time: Mutex::new(TimeState {
                    now: 1_000,
                    fire_through: None,
                    waiters: Vec::new(),
                }),
                relinquished: Mutex::new(None),
            }),
            scenario,
            exit_observable: true,
            attests_empty_owned_tree: false,
            stall: None,
        }
    }

    /// Makes one host service hang forever.
    pub fn stalling(mut self, stall: Stall) -> Self {
        self.stall = Some(stall);
        self
    }

    /// Makes the sidecar process unjoinable, so no exit is ever observed.
    pub fn without_observable_exit(mut self) -> Self {
        self.exit_observable = false;
        self
    }

    /// Models a hypothetical execution host whose concrete mechanism attests
    /// that its owned tree is empty. No host in this repository can do this
    /// today; the fixture exists so the only path to `Clean` stays proved.
    pub fn attesting_empty_owned_tree(mut self) -> Self {
        self.attests_empty_owned_tree = true;
        self
    }

    /// Makes every later wire write hang, modelling a process whose stdin
    /// stops draining while the sidecar stays alive.
    pub fn stall_writes(&self) {
        self.shared
            .process
            .lock()
            .expect("SDK fixture state lock poisoned")
            .stalling_writes = true;
    }

    /// Stops answering further commands, modelling a sidecar that goes quiet
    /// while staying alive.
    pub fn hold_responses(&self) {
        self.shared
            .process
            .lock()
            .expect("SDK fixture state lock poisoned")
            .holding = true;
    }

    /// Fires every host deadline as soon as it is awaited.
    pub fn with_immediate_time(self) -> Self {
        let mut time = self
            .shared
            .time
            .lock()
            .expect("SDK fixture time lock poisoned");
        time.fire_through = Some(u64::MAX);
        drop(time);
        self
    }

    /// Services whose task seam is the real local host's, so a guard task runs
    /// behind a handle that owns its worker thread and joins on drop.
    pub fn services_with_local_tasks(&self, host: ExecutionHostId) -> HostServices {
        let task = swallowtail_host_local::LocalScopedTaskService::new(host.clone());
        self.services(host).with_task(Arc::new(task))
    }

    pub fn services(&self, host: ExecutionHostId) -> HostServices {
        let (task_service, relinquished) = ThreadTaskService::new(host.clone());
        *self
            .shared
            .relinquished
            .lock()
            .expect("SDK fixture relinquish lock poisoned") = Some(relinquished);
        HostServices::new(host)
            .with_task(Arc::new(task_service))
            .with_process(Arc::new(self.clone()))
            .with_credential(Arc::new(self.clone()))
            .with_working_resource(Arc::new(self.clone()))
            .with_time(Arc::new(self.clone()))
    }

    pub fn process_environment(&self) -> Vec<String> {
        self.shared
            .process_request
            .lock()
            .expect("SDK fixture process lock poisoned")
            .as_ref()
            .expect("SDK fixture process started")
            .environment()
            .map(|value| value.as_host_value().to_owned())
            .collect()
    }

    /// Scopes this host accepted for reaping. Acceptance is ownership
    /// transfer, never join or cleanup evidence.
    pub fn relinquished_scopes(&self) -> Vec<String> {
        self.shared
            .relinquished
            .lock()
            .expect("SDK fixture relinquish lock poisoned")
            .as_ref()
            .map(|scopes| {
                scopes
                    .lock()
                    .expect("SDK fixture relinquish lock poisoned")
                    .clone()
            })
            .unwrap_or_default()
    }

    /// Holds the pump task open from this point on, so it outlives process
    /// exit until released.
    pub fn hold_pump(&self) {
        self.shared
            .process
            .lock()
            .expect("SDK fixture state lock poisoned")
            .holding_pump = true;
        self.shared.changed.notify_all();
    }

    /// Lets a held pump task finish, so the guard's ordered
    /// cleanup can proceed without any further call from the route.
    pub fn release_pump(&self) {
        self.shared
            .process
            .lock()
            .expect("SDK fixture state lock poisoned")
            .pump_released = true;
        self.shared.changed.notify_all();
    }

    pub fn inputs(&self) -> Vec<Value> {
        self.shared
            .process
            .lock()
            .expect("SDK fixture state lock poisoned")
            .input
            .clone()
    }

    pub fn credential_acquisitions(&self) -> usize {
        self.shared.credential_acquisitions.load(Ordering::SeqCst)
    }

    /// Waits until `event` has been recorded, bounded so a missing effect
    /// fails the test instead of hanging it.
    ///
    /// The guards do their work in host tasks, so an assertion made the instant
    /// the public future returns can race them.
    pub fn wait_for_cleanup(&self, event: CleanupEvent) {
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(10);
        loop {
            if self.cleanup_events().contains(&event) {
                return;
            }
            assert!(
                std::time::Instant::now() < deadline,
                "host cleanup never recorded {event:?}"
            );
            std::thread::sleep(std::time::Duration::from_millis(10));
        }
    }

    pub fn cleanup_events(&self) -> Vec<CleanupEvent> {
        self.shared
            .cleanup
            .lock()
            .expect("SDK fixture cleanup lock poisoned")
            .clone()
    }

    /// Emits one extra record outside any command response, used to prove
    /// unsolicited records fail closed.
    pub fn emit(&self, record: Value) {
        let mut state = self
            .shared
            .process
            .lock()
            .expect("SDK fixture state lock poisoned");
        script::push(&mut state, record);
        self.shared.changed.notify_all();
    }
}

pub(super) fn fixture_failure() -> swallowtail_runtime::RuntimeFailure {
    swallowtail_runtime::RuntimeFailure::new(swallowtail_core::SafeDiagnostic::new(
        "fixture.claude_agent_sdk.failed",
        "Claude Agent SDK sidecar fixture failed",
    ))
}

/// Leased working-resource root the fixture reports back through the wire.
pub const FIXTURE_CWD: &str = "/fixture/claude-agent-sdk-workspace";
/// Selected model the fixture confirms as effective.
pub const FIXTURE_MODEL: &str = "claude-sonnet-5";

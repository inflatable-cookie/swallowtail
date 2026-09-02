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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SdkScenario {
    /// Open, one streamed turn, and a sidecar-joined graceful close.
    Complete,
    /// The sidecar cannot prove its native child exited.
    CloseUnconfirmed,
    /// The sidecar claims a graceful join it never observed.
    CloseGracefulWithoutObservation,
    /// One `canUseTool` admission request during the turn.
    ToolAdmission,
    /// More admission requests than the bounded exchange accepts.
    ToolAdmissionOverflow,
    /// Interrupt reports a receipt the runtime never advertised.
    UnadvertisedInterruptReceipt,
    /// Open reports a non-subscription access profile.
    AccountApiKeySource,
    /// Open reports a delegated cloud provider rather than first party.
    AccountNotFirstParty,
    /// Open leaks an account identity field.
    AccountIdentityLeak,
    /// Open reports a version outside the bound one-point claim.
    IdentityMismatch,
    /// Open reports a cwd other than the leased resource root.
    CwdMismatch,
    /// Open advertises tools beyond the read-only set.
    ToolsWidened,
    /// The stream carries an unqualified event name.
    UnknownEvent,
    /// The stream carries invalid JSON.
    Malformed,
    /// The stream ends mid-record.
    Disconnect,
    /// The sidecar reports a terminal failure.
    TerminalRecord,
    /// A tool ends without ever starting.
    ToolOrderingDrift,
}

#[derive(Clone)]
pub struct SdkFixtureHost {
    shared: Arc<Shared>,
    scenario: SdkScenario,
    exit_observable: bool,
}

pub(super) struct Shared {
    pub(super) process_request: Mutex<Option<ProcessRequest>>,
    pub(super) process: Mutex<ProcessState>,
    pub(super) changed: Condvar,
    pub(super) credential_acquisitions: AtomicUsize,
    pub(super) cleanup: Mutex<Vec<CleanupEvent>>,
    pub(super) time: Mutex<TimeState>,
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
            }),
            scenario,
            exit_observable: true,
        }
    }

    /// Makes the sidecar process unjoinable, so no exit is ever observed.
    pub fn without_observable_exit(mut self) -> Self {
        self.exit_observable = false;
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

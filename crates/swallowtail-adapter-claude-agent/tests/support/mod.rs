use futures_executor::block_on;
use serde_json::{Value, json};
use std::collections::VecDeque;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Condvar, Mutex};
use swallowtail_core::{
    AccessProfile, AccessProfileId, AccessRequirement, AccessStatus, AdapterId, Capability,
    CapabilityConstraint, CapabilityProfile, CapabilityRequirement, ConfiguredInstance,
    ConfiguredInstanceId, CredentialMechanism, CredentialRef, CredentialState, DriverRole,
    EndpointAudience, EndpointAuthorization, EntitlementMetering, EntitlementState,
    ExecutionHostId, ExecutionLayer, HarnessConfigurationPosture, HarnessIsolation,
    HostServiceKind, InstanceOwnership, InstancePolicyId, InstanceRevision, InstanceTargetRef,
    ModelId, ModelRoute, ModelRouteId, ModelRouteRevision, OperationRequirements, OperationShape,
    PreflightContext, PreflightPlan, ProtocolFacadeId, ResourceAccess, ResourceRepresentation,
    RuntimeReadiness, SessionAccessPolicy, SessionProviderStatePolicy, SupportAuthority, preflight,
};
use swallowtail_runtime::{
    BoxFuture, CleanupOutcome, CredentialLease, Deadline, DeadlineObservation, HostServices,
    JoinedTask, MonotonicInstant, ProcessExit, ProcessHandle, ProcessInputChunk,
    ProcessOutputChunk, ProcessOutputStream, ProcessRequest, ProcessService, ResourceLease,
    RuntimeFailure, ScopeId, ScopedTaskService, SecretLease, SessionCleanupRequest, TimeService,
    WorkingResourceIoService, WorkingResourceReadRequest, WorkingResourceRef,
    WorkingResourceService, WorkingResourceText,
};

mod agent;
pub mod selection;
mod services;

use agent::SharedAgent;
pub use agent::{ObservedProcess, Scenario};
pub use services::DeadlineWait;
use services::{FixtureTime, ThreadTaskService};

#[derive(Clone)]
pub struct FixtureHost {
    agent: Arc<SharedAgent>,
    process: Arc<Mutex<Option<ObservedProcess>>>,
    cleanup: Arc<Mutex<Vec<&'static str>>>,
    reads: Arc<AtomicUsize>,
    resource_releases: Arc<AtomicUsize>,
    credential_acquires: Arc<AtomicUsize>,
    credential_releases: Arc<AtomicUsize>,
    immediate_deadline: bool,
    deadline_after_waits: Option<usize>,
    deadline_waits: Option<Vec<DeadlineWait>>,
}

impl FixtureHost {
    pub fn new(scenario: Scenario, version: &str) -> Self {
        Self {
            agent: SharedAgent::new(scenario, version),
            process: Arc::new(Mutex::new(None)),
            cleanup: Arc::new(Mutex::new(Vec::new())),
            reads: Arc::new(AtomicUsize::new(0)),
            resource_releases: Arc::new(AtomicUsize::new(0)),
            credential_acquires: Arc::new(AtomicUsize::new(0)),
            credential_releases: Arc::new(AtomicUsize::new(0)),
            immediate_deadline: false,
            deadline_after_waits: None,
            deadline_waits: None,
        }
    }

    pub fn with_immediate_deadline(mut self) -> Self {
        self.immediate_deadline = true;
        self
    }

    #[allow(dead_code)]
    pub fn with_deadline_after_waits(mut self, waits: usize) -> Self {
        self.deadline_after_waits = Some(waits);
        self
    }

    /// Scripts every deadline wait observation in call order: the first
    /// observation bounds the operation turn, later ones bound joined
    /// session cleanup. An exhausted script fails the wait instead of
    /// guessing, keeping each observation an exact named control.
    pub fn with_deadline_waits(mut self, waits: impl IntoIterator<Item = DeadlineWait>) -> Self {
        self.deadline_waits = Some(waits.into_iter().collect());
        self
    }

    /// Makes the fixture agent hold back the session-close response so a
    /// cleanup genuinely crosses its caller boundary while still pending.
    pub fn with_held_session_close_response(self) -> Self {
        self.agent.hold_close_response();
        self
    }

    /// Releases a previously held session-close response.
    #[allow(dead_code)]
    pub fn release_held_session_close_response(&self) {
        self.agent.release_held_close_response();
    }

    fn fixture_time(&self) -> FixtureTime {
        match &self.deadline_waits {
            Some(waits) => FixtureTime::scripted(waits.clone()),
            None => FixtureTime::new(self.immediate_deadline, self.deadline_after_waits),
        }
    }

    pub fn services(&self, host: ExecutionHostId) -> HostServices {
        HostServices::new(host)
            .with_task(Arc::new(ThreadTaskService))
            .with_time(Arc::new(self.fixture_time()))
            .with_process(Arc::new(self.clone()))
            .with_credential(Arc::new(self.clone()))
            .with_working_resource(Arc::new(self.clone()))
            .with_working_resource_io(Arc::new(self.clone()))
    }

    pub fn cleanup_request(&self) -> SessionCleanupRequest {
        SessionCleanupRequest::new(Deadline::at(MonotonicInstant::from_ticks(10_000)))
    }

    pub fn cleanup_services(&self, host: ExecutionHostId) -> HostServices {
        HostServices::new(host)
            .with_task(Arc::new(ThreadTaskService))
            .with_time(Arc::new(FixtureTime::new(false, None)))
            .with_process(Arc::new(self.clone()))
            .with_credential(Arc::new(self.clone()))
            .with_working_resource(Arc::new(self.clone()))
            .with_working_resource_io(Arc::new(self.clone()))
    }

    #[allow(dead_code)]
    pub fn services_without_credential(&self, host: ExecutionHostId) -> HostServices {
        HostServices::new(host)
            .with_task(Arc::new(ThreadTaskService))
            .with_time(Arc::new(self.fixture_time()))
            .with_process(Arc::new(self.clone()))
            .with_working_resource(Arc::new(self.clone()))
            .with_working_resource_io(Arc::new(self.clone()))
    }

    pub fn observed_process(&self) -> ObservedProcess {
        self.process
            .lock()
            .expect("fixture process lock poisoned")
            .clone()
            .expect("process was observed")
    }

    pub fn reads(&self) -> usize {
        self.reads.load(Ordering::SeqCst)
    }

    pub fn resource_releases(&self) -> usize {
        self.resource_releases.load(Ordering::SeqCst)
    }

    pub fn credential_acquires(&self) -> usize {
        self.credential_acquires.load(Ordering::SeqCst)
    }

    pub fn credential_releases(&self) -> usize {
        self.credential_releases.load(Ordering::SeqCst)
    }

    pub fn writes(&self) -> Vec<Value> {
        self.agent
            .state
            .lock()
            .expect("fixture agent lock poisoned")
            .writes
            .clone()
    }

    #[allow(dead_code)]
    pub fn wait_for_write(&self, method: &str) {
        self.agent.wait_for_method(method);
    }

    #[allow(dead_code)]
    pub fn cleanup_events(&self) -> Vec<&'static str> {
        self.cleanup
            .lock()
            .expect("fixture cleanup lock poisoned")
            .clone()
    }
}

fn fixture_failure() -> RuntimeFailure {
    RuntimeFailure::new(swallowtail_core::SafeDiagnostic::new(
        "fixture.claude_agent_acp.failed",
        "Claude Agent ACP fixture failed",
    ))
}

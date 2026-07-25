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
    RuntimeFailure, ScopeId, ScopedTaskService, SecretLease, TimeService, WorkingResourceIoService,
    WorkingResourceReadRequest, WorkingResourceRef, WorkingResourceService, WorkingResourceText,
};

mod agent;
pub mod selection;
mod services;

use agent::SharedAgent;
pub use agent::{ObservedProcess, Scenario};
use services::{FixtureTime, ThreadTaskService};

#[derive(Clone)]
pub struct FixtureHost {
    agent: Arc<SharedAgent>,
    process: Arc<Mutex<Option<ObservedProcess>>>,
    reads: Arc<AtomicUsize>,
    resource_releases: Arc<AtomicUsize>,
    credential_acquires: Arc<AtomicUsize>,
    credential_releases: Arc<AtomicUsize>,
    immediate_deadline: bool,
}

impl FixtureHost {
    pub fn new(scenario: Scenario, version: &str) -> Self {
        Self {
            agent: SharedAgent::new(scenario, version),
            process: Arc::new(Mutex::new(None)),
            reads: Arc::new(AtomicUsize::new(0)),
            resource_releases: Arc::new(AtomicUsize::new(0)),
            credential_acquires: Arc::new(AtomicUsize::new(0)),
            credential_releases: Arc::new(AtomicUsize::new(0)),
            immediate_deadline: false,
        }
    }

    pub fn with_immediate_deadline(mut self) -> Self {
        self.immediate_deadline = true;
        self
    }

    pub fn services(&self, host: ExecutionHostId) -> HostServices {
        HostServices::new(host)
            .with_task(Arc::new(ThreadTaskService))
            .with_time(Arc::new(FixtureTime::new(self.immediate_deadline)))
            .with_process(Arc::new(self.clone()))
            .with_credential(Arc::new(self.clone()))
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
}

fn fixture_failure() -> RuntimeFailure {
    RuntimeFailure::new(swallowtail_core::SafeDiagnostic::new(
        "fixture.claude_agent_acp.failed",
        "Claude Agent ACP fixture failed",
    ))
}

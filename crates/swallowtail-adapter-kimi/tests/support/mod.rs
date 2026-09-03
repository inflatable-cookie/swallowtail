#![allow(dead_code, unused_imports)]

use futures_executor::block_on;
use serde_json::Value;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;
use swallowtail_core::{
    AccessProfile, AccessProfileId, AccessRequirement, AccessStatus, AdapterId, Capability,
    CapabilityConstraint, CapabilityProfile, CapabilityRequirement, ConfiguredInstance,
    ConfiguredInstanceId, CredentialMechanism, CredentialRef, CredentialState, DriverRole,
    EndpointAudience, EndpointAuthorization, EntitlementMetering, EntitlementState,
    ExecutionHostId, ExecutionLayer, HostServiceKind, InstanceOwnership, InstancePolicyId,
    InstanceRevision, InstanceTargetRef, ModelId, ModelRoute, ModelRouteId, ModelRouteRevision,
    OperationRequirements, OperationShape, PreflightContext, PreflightPlan, ProtocolFacadeId,
    ResourceAccess, ResourceRepresentation, RuntimeReadiness, SessionAccessPolicy,
    SessionProviderStatePolicy, SupportAuthority, preflight,
};
use swallowtail_runtime::{
    BoxFuture, CleanupOutcome, CredentialLease, CredentialService, Deadline, DeadlineObservation,
    DelegatedCredential, HostServices, InteractiveSessionHandle, JoinedTask, MonotonicInstant,
    ProcessExit, ProcessHandle, ProcessInputChunk, ProcessOutputChunk, ProcessOutputStream,
    ProcessRequest, ProcessService, ResourceLease, RuntimeFailure, ScopeId, ScopedTaskService,
    SessionCleanupRequest, TimeService, WorkingResourceIoService, WorkingResourceReadRequest,
    WorkingResourceRef, WorkingResourceService, WorkingResourceText, WorkingResourceWriteRequest,
};

mod selection;
pub use selection::{
    plan_reasoning_selection, plan_selection, reasoning_selection, selection,
    try_version_selection, version_selection,
};
mod agent;
pub use agent::Scenario;
use agent::{AgentState, SharedAgent};

pub fn close_session(
    session: Box<dyn InteractiveSessionHandle>,
    services: HostServices,
) -> BoxFuture<'static, CleanupOutcome> {
    session.close(
        SessionCleanupRequest::new(Deadline::at(MonotonicInstant::from_ticks(10_000))),
        services,
    )
}

struct FixtureTime;

impl TimeService for FixtureTime {
    fn now(&self) -> MonotonicInstant {
        MonotonicInstant::from_ticks(0)
    }

    fn wait_until(&self, _deadline: Deadline) -> BoxFuture<'static, DeadlineObservation> {
        Box::pin(std::future::pending())
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CleanupEvent {
    ProcessWait,
    ResourceRelease,
    CredentialRelease,
}

include!("host.rs");

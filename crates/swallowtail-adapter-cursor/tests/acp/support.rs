use futures_executor::block_on;
use futures_util::StreamExt;
use serde_json::{Value, json};
use std::collections::VecDeque;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Condvar, Mutex};
use std::thread::JoinHandle;
use swallowtail_adapter_cursor::{
    CursorAcpDriver, cursor_acp_descriptor, cursor_agent_release_binding,
    cursor_subscription_access_profile,
};
use swallowtail_core::{
    AccessProfileId, AccessRequirement, AccessStatus, Capability, CapabilityConstraint,
    CapabilityProfile, CapabilityRequirement, ConfiguredInstance, ConfiguredInstanceId,
    CredentialState, DriverRole, EndpointAuthorization, EntitlementState, ExecutionHostId,
    ExecutionLayer, ExtensionNamespace, HarnessConfigurationPosture, HarnessIsolation,
    InstanceOwnership, InstancePolicyId, InstanceRevision, InstanceTargetRef,
    NamedCapabilityConstraint, OperationRequirements, OperationShape, PreflightContext,
    ProtocolFacadeId, ResourceAccess, ResourceRepresentation, RuntimeReadiness,
    SessionAccessPolicy, SessionProviderStatePolicy, SupportAuthority, preflight,
};
use swallowtail_runtime::{
    ActivityKind, ActivityLifecyclePhase, ActivityObservation, BoxFuture, CleanupOutcome, Deadline,
    DeadlineObservation, EnvironmentRef, HostServices, InteractiveSessionDriver,
    InteractiveSessionHandle, JoinedTask, MonotonicInstant, OpenSessionRequest, OperationContent,
    ProcessExit, ProcessHandle, ProcessInputChunk, ProcessOutputChunk, ProcessOutputStream,
    ProcessRequest, ProcessService, RequestId, ResourceLease, ResumeSessionRequest,
    RuntimeEventKind, RuntimeFailure, RuntimeTurnId, ScopeId, ScopedTaskService,
    SessionCleanupRequest, SessionPlanAgreement, TerminalStatus, TimeService, TurnRequest,
    WorkingResourceIoService, WorkingResourceReadRequest, WorkingResourceRef,
    WorkingResourceService, WorkingResourceText, WorkingResourceWriteRequest,
};

fn close_session(
    session: Box<dyn InteractiveSessionHandle>,
    services: HostServices,
) -> BoxFuture<'static, CleanupOutcome> {
    session.close(
        SessionCleanupRequest::new(Deadline::at(MonotonicInstant::from_ticks(10_000))),
        services,
    )
}

include!("agent.rs");
include!("host.rs");
include!("selection.rs");
include!("cases.rs");

use futures_executor::block_on;
use futures_util::StreamExt;
use serde_json::{Value, json};
use std::collections::VecDeque;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Condvar, Mutex};
use std::thread::JoinHandle;
use swallowtail_adapter_grok::{
    GrokAcpDriver, grok_build_acp_binding, grok_build_acp_descriptor,
    grok_build_subscription_access_profile,
};
use swallowtail_core::{
    AccessRequirement, AccessStatus, Capability, CapabilityConstraint, CapabilityProfile,
    CapabilityRequirement, ConfiguredInstance, ConfiguredInstanceId, CredentialRef,
    CredentialState, DriverRole, EndpointAuthorization, EntitlementState, ExecutionHostId,
    ExecutionLayer, ExtensionNamespace, HarnessConfigurationPosture, HarnessIsolation,
    InstanceOwnership, InstancePolicyId, InstanceRevision, InstanceTargetRef, ModelId, ModelRoute,
    ModelRouteId, ModelRouteRevision, NamedCapabilityConstraint, OperationRequirements,
    OperationShape, PreflightContext, ProtocolFacadeId, ResourceAccess, ResourceRepresentation,
    RuntimeReadiness, SessionAccessPolicy, SessionProviderStatePolicy, SupportAuthority, preflight,
};
use swallowtail_runtime::{
    BoxFuture, CleanupOutcome, CredentialLease, CredentialService, Deadline, DeadlineObservation,
    DelegatedCredential, EnvironmentRef, HostServices, InteractiveSessionDriver, JoinedTask,
    MonotonicInstant, OpenSessionRequest, OperationContent, ProcessExit, ProcessHandle,
    ProcessInputChunk, ProcessOutputChunk, ProcessOutputStream, ProcessRequest, ProcessService,
    RequestId, ResourceLease, ResumeSessionRequest, RuntimeEventKind, RuntimeFailure,
    RuntimeTurnId, ScopeId, ScopedTaskService, SessionPlanAgreement, TerminalStatus, TimeService,
    TurnRequest, WorkingResourceIoService, WorkingResourceReadRequest, WorkingResourceRef,
    WorkingResourceService, WorkingResourceText, WorkingResourceWriteRequest,
};

include!("agent.rs");
include!("host.rs");
include!("selection.rs");
include!("cases.rs");

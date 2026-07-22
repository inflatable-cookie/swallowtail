//! Executor-neutral runtime contracts for Swallowtail drivers and hosts.
//!
//! This crate defines dynamic roles, scoped handles, host-service ports, and
//! lifecycle primitives. It does not provide an executor or concrete I/O.

#![forbid(unsafe_code)]

mod async_types;
mod attachment;
mod callback;
mod cancellation;
mod content;
mod event;
mod event_buffer;
mod event_channel;
mod failure;
mod handles;
mod host_reference;
mod host_registry;
mod host_traits;
mod identity;
mod input;
mod model_artifact;
mod network;
mod operation_policy;
mod outcome;
mod process_input;
mod process_io;
mod provider_observation;
mod realtime_media;
mod registration;
mod roles;
mod schema;
mod secret;
mod serving_endpoint;
mod serving_lifecycle;
mod session_access;
mod session_binding;
mod session_options;
mod session_provider_state;
mod session_replay;
mod time;
mod working_resource_io;

pub use async_types::{BoxCallbackStream, BoxEventStream, BoxFuture, BoxRealtimeMediaEventStream};
pub use attachment::{
    AttachmentDescriptor, AttachmentDigest, AttachmentRepresentation, AttachmentRole,
    LeaseCleanupAuthority,
};
pub use callback::{
    CallbackAbandonment, CallbackExchange, CallbackFailureKind, CallbackOperationId,
    CallbackPayload, CallbackRequest, CallbackRequestKind, CallbackResponder, CallbackResponse,
    CallbackResult, CallbackWaitState,
};
pub use cancellation::{CancellationAcknowledgement, CancellationControl, ImmediateCancellation};
pub use content::OperationContent;
pub use event::{EventDelivery, RuntimeEvent, RuntimeEventKind};
pub use event_buffer::{EventBufferFailure, EventBufferFailureKind, OrderedEventBuffer};
pub use event_channel::{RuntimeEventSender, RuntimeEventStream, runtime_event_channel};
pub use failure::RuntimeFailure;
pub use handles::{
    AttachedServingHandle, InteractiveSessionHandle, OwnedServingHandle,
    RealtimeMediaResponseHandle, RealtimeMediaSessionHandle, RunHandle, TurnHandle,
};
pub use host_reference::{
    AttachmentRef, EndpointRef, EnvironmentRef, ExecutableRef, MaterializedFileRef,
    MaterializedModelArtifactRef, MaterializedResourceRef, SchemaRef, WorkingResourceRef,
};
pub use host_registry::HostServices;
pub use host_traits::{
    AttachmentFileLease, AttachmentService, BlockingJob, BlockingWorkService, CredentialService,
    DiagnosticObserver, JoinedTask, ProcessHandle, ProcessService, ResourceLease, SchemaFileLease,
    SchemaService, ScopedTaskService, TimeService, WorkingResourceService,
};
pub use identity::{
    CallbackId, MediaStreamId, RequestId, RuntimeIdentityRequired, RuntimeRunId, RuntimeSessionId,
    RuntimeTurnId, ScopeId, ServingInstanceId,
};
pub use input::{InputLimitExceeded, InputValueRequired};
pub use model_artifact::{ModelArtifactAccess, ModelArtifactLease, ModelArtifactService};
pub use network::{AuthorizedEndpoint, NetworkGrant, NetworkPolicyService};
pub use operation_policy::{
    IncompatibleOperationPolicy, OperationPolicy, ProviderExecutionPolicy, ProviderRecoveryPolicy,
    ProviderRetentionPolicy, StreamReattachmentPolicy, validate_harness_isolation_policy,
};
pub use outcome::{
    CleanupOutcome, ProviderCancellationOutcome, ProviderRequestObservation,
    RemoteResourceDeletionOutcome, TerminalAlreadySet, TerminalOutcome, TerminalOutcomeFuture,
    TerminalOutcomeSender, TerminalStatus, terminal_outcome_channel,
};
pub use process_input::ProcessRequest;
pub use process_io::{ProcessExit, ProcessInputChunk, ProcessOutputChunk, ProcessOutputStream};
pub use provider_observation::{
    BilledCostObservation, BilledCostSemantics, BilledCostSource, Currency, ProviderObservation,
    QuotaObservation, QuotaState, RateLimitKind, RateLimitObservation, TokenUsage,
};
pub use realtime_media::{
    MediaChunk, MediaInputCommit, MediaTranscript, OpenRealtimeMediaSessionRequest,
    RealtimeMediaEvent, RealtimeMediaEventKind, RealtimeMediaFailure, RealtimeMediaFailureKind,
    RealtimeMediaResponseStatus, RealtimeMediaSessionState,
};
pub use registration::{DriverRegistration, RegistrationFailure};
pub use roles::{
    AttachServingRequest, DiscoveryDriver, DiscoveryRequest, InteractiveSessionDriver,
    LoadSessionRequest, LoadedSession, ModelCatalogDriver, ModelCatalogRequest, OpenSessionRequest,
    RealtimeMediaSessionDriver, ResumeSessionRequest, ServingInstanceDriver, StartServingRequest,
    StructuredRunDriver, StructuredRunRequest, TurnRequest,
};
pub use schema::{SchemaDocument, StructuredOutputDescriptor};
pub use secret::{CredentialLease, DelegatedCredential, SecretLease};
pub use serving_endpoint::{
    ObservedServingEndpoint, ServingEndpointBinding, ServingEndpointLease, ServingEndpointService,
};
pub use serving_lifecycle::validate_owned_serving_start;
pub use session_access::{validate_session_access_plan, validate_session_resource_lease};
pub use session_binding::SessionResumeBinding;
pub use session_options::{SessionOptions, ToolDeclaration};
pub use session_provider_state::validate_session_provider_state_plan;
pub use session_replay::{SessionReplayItem, SessionReplayKind};
pub use swallowtail_core::{
    CredentialRef, ExternalNetworkPolicy, ExternalSearchPolicy, FilesystemBoundary,
    HarnessIsolation, IncompatibleSessionAccessPolicy, ModelArtifactBinding,
    ModelArtifactDescriptor, ModelArtifactDigest, ModelArtifactFormat, ModelArtifactId,
    ModelArtifactRef, ModelArtifactRevision, OwnedRemoteResourceKind, ProviderApprovalPolicy,
    ProviderRequestHandling, ProviderRequestPolicy, ResourceAccess, ResourceRepresentation,
    SessionAccessPolicy, SessionProviderStatePolicy,
};
pub use time::{Deadline, DeadlineObservation, MonotonicInstant};
pub use working_resource_io::{
    WorkingResourceIoService, WorkingResourceLocator, WorkingResourceReadRequest,
    WorkingResourceText, WorkingResourceWriteRequest,
};

//! Executor-neutral runtime contracts for Swallowtail drivers and hosts.
//!
//! This crate defines dynamic roles, scoped handles, host-service ports, and
//! lifecycle primitives. It does not provide an executor or concrete I/O.

#![forbid(unsafe_code)]

mod activity;
mod async_types;
mod attachment;
mod callback;
mod cancellation;
mod content;
mod direct_continuation;
mod event;
mod event_buffer;
mod event_channel;
mod failure;
mod handles;
mod harness_rpc;
mod harness_user_input;
mod host_reference;
mod host_registry;
mod host_traits;
mod identity;
mod input;
mod installed_executable;
mod model_artifact;
mod negotiated_session_options;
mod network;
mod operation_policy;
mod outcome;
mod planned_connection_rollover;
mod preparation;
mod prepared_access;
mod prepared_operation;
mod process_input;
mod process_io;
mod provider_instance_catalogue;
mod provider_observation;
mod provider_session_import;
mod provider_session_management;
mod provider_session_operation;
mod provider_session_reconciliation;
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
mod session_plan_agreement;
mod session_provider_state;
mod session_replay;
mod subagent_directory;
mod time;
mod working_resource_io;

pub use activity::{
    ActivityActor, ActivityAssistantPhase, ActivityContent, ActivityContentChangeKind,
    ActivityContentStream, ActivityContentUpdate, ActivityCorrelation, ActivityDisclosure,
    ActivityId, ActivityKey, ActivityKind, ActivityLabel, ActivityLifecyclePhase,
    ActivityNamespace, ActivityObservation, ActivityOperationId, ActivityStatus,
    InvalidActivityRecord, SubagentControlActionKind, SubagentId, SubagentParent, SubagentSnapshot,
    SubagentStatus, TaskListItem, TaskListItemPriority, TaskListItemStatus, TaskListSnapshot,
};
pub use async_types::{
    BoxCallbackStream, BoxDirectToolCallStream, BoxEventStream, BoxFuture,
    BoxRealtimeMediaEventStream,
};
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
pub use direct_continuation::{
    DirectAttemptAuthorizationKind, DirectContinuationBinding, DirectContinuationState,
    DirectContinuationTurnRequest, DirectInferenceAttempt, DirectToolArguments, DirectToolCall,
    DirectToolExchange, DirectToolResult, DirectToolResultContent, DirectToolResultSubmitter,
    OpenDirectContinuationSessionRequest, ProviderPrivateContinuationRecord,
    validate_direct_continuation_plan,
};
pub use event::{EventDelivery, RuntimeEvent, RuntimeEventKind};
pub use event_buffer::{EventBufferFailure, EventBufferFailureKind, OrderedEventBuffer};
pub use event_channel::{RuntimeEventSender, RuntimeEventStream, runtime_event_channel};
pub use failure::RuntimeFailure;
pub use handles::{
    AttachedServingHandle, InteractiveSessionHandle, OwnedServingHandle,
    RealtimeMediaResponseHandle, RealtimeMediaSessionHandle, RunHandle, TurnHandle,
};
pub use harness_rpc::{
    HarnessCommandAcknowledgement, HarnessCommandResponse, HarnessScheduledMessage,
    HarnessUiDisplay, HarnessUiDisplayKind,
};
pub use harness_user_input::{
    HarnessUserInputAnswer, HarnessUserInputChoiceMode, HarnessUserInputInvalid,
    HarnessUserInputOption, HarnessUserInputQuestion, HarnessUserInputQuestionKind,
    HarnessUserInputRequest, HarnessUserInputResponse,
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
    AccessEvidenceSourceId, CallbackId, DirectInferenceAttemptId, DirectToolCallId,
    HarnessCommandId, HarnessQuestionId, HarnessQuestionOptionId, MediaStreamId,
    ProviderSessionCandidateId, ProviderSessionCatalogueId, RequestId, RuntimeIdentityRequired,
    RuntimeRunId, RuntimeSessionId, RuntimeTurnId, ScopeId, ServingInstanceId,
};
pub use input::{InputLimitExceeded, InputValueRequired};
pub use installed_executable::{
    DiscoveryCancellation, InstalledExecutableDiscoveryRequest, InstalledExecutableTarget,
    validate_installed_executable_discovery_services,
};
pub use model_artifact::{ModelArtifactAccess, ModelArtifactLease, ModelArtifactService};
pub use negotiated_session_options::{
    EffectiveReasoningSetup, NegotiatedReasoningSetup, NegotiatedSessionModelOption,
    NegotiatedSessionModelOptions, SessionLifecycleOperation, prepare_negotiated_reasoning_setup,
};
pub use network::{AuthorizedEndpoint, NetworkGrant, NetworkPolicyService};
pub use operation_policy::{
    IncompatibleOperationPolicy, OperationPolicy, ProviderExecutionPolicy, ProviderRecoveryPolicy,
    ProviderRetentionPolicy, StreamReattachmentPolicy, validate_attached_runtime_residency_policy,
    validate_harness_configuration_policy, validate_harness_isolation_policy,
};
pub use outcome::{
    CleanupOutcome, ProviderCancellationOutcome, ProviderRequestObservation,
    RemoteResourceDeletionOutcome, TerminalAlreadySet, TerminalOutcome, TerminalOutcomeFuture,
    TerminalOutcomeSender, TerminalStatus, terminal_outcome_channel,
};
pub use planned_connection_rollover::validate_planned_connection_rollover_plan;
pub use preparation::{PreparationFailure, PreparationStage};
pub use prepared_access::{AccessEvidenceProvenance, PreparedAccessEvidence};
pub use prepared_operation::{
    PreparedInterfaceCompatibility, PreparedOperationBinding, PreparedOperationEvidence,
};
pub use process_input::ProcessRequest;
pub use process_io::{ProcessExit, ProcessInputChunk, ProcessOutputChunk, ProcessOutputStream};
pub use provider_instance_catalogue::{
    ConfiguredProviderCredentialPosture, ConfiguredProviderInstanceAdmission,
    ConfiguredProviderInstanceCatalogue, ConfiguredProviderInstanceCatalogueFailure,
    ConfiguredProviderInstanceCatalogueFailureKind, ConfiguredProviderInstanceRecord,
    ConfiguredProviderInstanceRoute, ConfiguredProviderInstanceSelectionReadiness,
    ConfiguredProviderModelCatalogue, ConfiguredProviderModelCatalogueInput,
    ConfiguredProviderModelCatalogueState, ConfiguredProviderModelRoute,
    MAX_CONFIGURED_PROVIDER_INSTANCES, MAX_CONFIGURED_PROVIDER_MODELS_PER_INSTANCE,
    MAX_CONFIGURED_PROVIDER_ROUTES_PER_INSTANCE,
};
pub use provider_observation::{
    BilledCostObservation, BilledCostSemantics, BilledCostSource, Currency,
    DirectAttemptFinishObservation, DirectAttemptUsageObservation, ProviderFinishReason,
    ProviderObservation, QuotaObservation, QuotaState, RateLimitKind, RateLimitObservation,
    TokenUsage,
};
pub use provider_session_import::{
    PreparedProviderSessionCatalogueEvidence, PreparedProviderSessionImportEvidence,
    ProviderSessionCandidate, ProviderSessionCatalogueAgreement, ProviderSessionCatalogueOutcome,
    ProviderSessionCataloguePlan, ProviderSessionCatalogueRequest, ProviderSessionCatalogueScope,
    ProviderSessionCursor, ProviderSessionImportAgreement, ProviderSessionImportOutcome,
    ProviderSessionImportPlan, ProviderSessionImportRequest, ProviderSessionImportRevalidation,
    ProviderSessionOperationFailure, ProviderSessionOperationFailureStage,
    validate_provider_session_catalogue_execution, validate_provider_session_catalogue_request,
    validate_provider_session_import_execution, validate_provider_session_import_request,
};
pub use provider_session_management::{
    InvalidProviderSessionManagementBinding, InvalidProviderSessionManagementBindingKind,
    ProviderSessionManagementBinding,
};
pub use provider_session_operation::{
    ArchiveProviderSessionRequest, DeleteProviderSessionRequest,
    PreparedProviderSessionManagementEvidence, ProviderSessionManagementAgreement,
    ProviderSessionManagementOutcome, ProviderSessionManagementPlan, RestoreProviderSessionRequest,
    validate_provider_session_management_request,
};
pub use provider_session_reconciliation::{
    InterruptedTurnAttribution, InterruptedTurnState,
    PreparedProviderSessionReconciliationEvidence, ProviderSessionReconciliationAgreement,
    ProviderSessionReconciliationBounds, ProviderSessionReconciliationObservation,
    ProviderSessionReconciliationOutcome, ProviderSessionReconciliationPlan,
    ProviderSessionReconciliationRequest, bound_provider_session_replay_tail,
    validate_provider_session_reconciliation_execution,
    validate_provider_session_reconciliation_request,
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
    ProviderSessionCatalogueDriver, ProviderSessionImportDriver, ProviderSessionManagementDriver,
    ProviderSessionReconciliationDriver, RealtimeMediaSessionDriver, ResumeSessionRequest,
    ServingInstanceDriver, StartServingRequest, StructuredRunDriver, StructuredRunRequest,
    TurnRequest,
};
pub use schema::{SchemaDocument, StructuredOutputDescriptor};
pub use secret::{CredentialLease, DelegatedCredential, SecretLease};
pub use serving_endpoint::{
    ObservedServingEndpoint, ServingEndpointBinding, ServingEndpointLease, ServingEndpointService,
};
pub use serving_lifecycle::validate_owned_serving_start;
pub use session_access::{validate_session_access_plan, validate_session_resource_lease};
pub use session_binding::{
    PersistedSessionResumeBinding, SessionResumeBinding, SessionResumeBindingPersistenceFailure,
    SessionResumeBindingPersistenceFailureKind,
};
pub use session_options::{SessionOptions, ToolDeclaration};
pub use session_plan_agreement::{SessionPlanAgreement, validate_session_plan_agreement};
pub use session_provider_state::validate_session_provider_state_plan;
pub use session_replay::{SessionReplayItem, SessionReplayKind};
pub use subagent_directory::{
    SubagentDirectoryChange, SubagentDirectoryChangeKind, SubagentDirectoryDelta,
    SubagentDirectoryFailure, SubagentDirectoryFailureKind, SubagentDirectoryProjection,
};
pub use swallowtail_core::{
    ActivityCorrelationKind, ActivityInterfaceBasis, ActivityKindClass, ActivityKindProfile,
    ActivityLifecycleFidelity, ActivityUnknownEventPosture, AttachedRuntimeResidency,
    CredentialRef, ExternalNetworkPolicy, ExternalSearchPolicy, FilesystemBoundary,
    HarnessIsolation, IncompatibleSessionAccessPolicy, InvalidObservableActivityProfile,
    ModelArtifactBinding, ModelArtifactDescriptor, ModelArtifactDigest, ModelArtifactFormat,
    ModelArtifactId, ModelArtifactRef, ModelArtifactRevision, ObservableActivityAvailability,
    ObservableActivityProfile, OwnedRemoteResourceKind, PlannedConnectionRolloverPolicy,
    ProviderActivityRef, ProviderApprovalPolicy, ProviderInferenceCachePolicy,
    ProviderRequestHandling, ProviderRequestPolicy, ProviderSessionActivityEvidence,
    ProviderSessionActivityState, ProviderSessionAffectedScope, ProviderSessionBindingOrigin,
    ProviderSessionCancellationPosture, ProviderSessionCatalogueBounds,
    ProviderSessionDeletionStrength, ProviderSessionDiscoveryScope, ProviderSessionDisplayContent,
    ProviderSessionEffectTruth, ProviderSessionImportAvailability,
    ProviderSessionImportUnavailableReason, ProviderSessionInitialStateRequirement,
    ProviderSessionInterfaceCompatibility, ProviderSessionLifecycleState,
    ProviderSessionManagementAction, ProviderSessionManagementEffect, ResourceAccess,
    ResourceRepresentation, SessionAccessPolicy, SessionProviderStatePolicy,
};
pub use time::{Deadline, DeadlineObservation, MonotonicInstant};
pub use working_resource_io::{
    WorkingResourceIoService, WorkingResourceLocator, WorkingResourceReadRequest,
    WorkingResourceText, WorkingResourceWriteRequest,
};

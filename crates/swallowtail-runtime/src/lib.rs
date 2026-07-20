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
mod operation_policy;
mod outcome;
mod process_input;
mod process_io;
mod registration;
mod roles;
mod schema;
mod secret;
mod session_access;
mod session_binding;
mod session_options;
mod time;

pub use async_types::{BoxCallbackStream, BoxEventStream, BoxFuture};
pub use attachment::{
    AttachmentDescriptor, AttachmentDigest, AttachmentRepresentation, AttachmentRole,
    LeaseCleanupAuthority,
};
pub use callback::{
    CallbackAbandonment, CallbackExchange, CallbackFailureKind, CallbackPayload, CallbackRequest,
    CallbackRequestKind, CallbackResponder, CallbackResponse, CallbackResult, CallbackWaitState,
};
pub use cancellation::{CancellationAcknowledgement, CancellationControl, ImmediateCancellation};
pub use content::OperationContent;
pub use event::{EventDelivery, RuntimeEvent, RuntimeEventKind};
pub use event_buffer::{EventBufferFailure, EventBufferFailureKind, OrderedEventBuffer};
pub use event_channel::{RuntimeEventSender, RuntimeEventStream, runtime_event_channel};
pub use failure::RuntimeFailure;
pub use handles::{
    AttachedServingHandle, InteractiveSessionHandle, OwnedServingHandle, RunHandle, TurnHandle,
};
pub use host_reference::{
    AttachmentRef, CredentialRef, EndpointRef, EnvironmentRef, ExecutableRef, MaterializedFileRef,
    MaterializedResourceRef, SchemaRef, WorkingResourceRef,
};
pub use host_registry::HostServices;
pub use host_traits::{
    AttachmentFileLease, AttachmentService, BlockingJob, BlockingWorkService, CredentialService,
    DiagnosticObserver, JoinedTask, NetworkGrant, NetworkPolicyService, ProcessHandle,
    ProcessService, ResourceLease, SchemaFileLease, SchemaService, ScopedTaskService, TimeService,
    WorkingResourceService,
};
pub use identity::{
    CallbackId, RequestId, RuntimeIdentityRequired, RuntimeRunId, RuntimeSessionId, RuntimeTurnId,
    ScopeId, ServingInstanceId,
};
pub use input::{InputLimitExceeded, InputValueRequired};
pub use operation_policy::{IncompatibleOperationPolicy, OperationPolicy};
pub use outcome::{
    CleanupOutcome, ProviderRequestObservation, TerminalAlreadySet, TerminalOutcome,
    TerminalOutcomeFuture, TerminalOutcomeSender, TerminalStatus, terminal_outcome_channel,
};
pub use process_input::ProcessRequest;
pub use process_io::{ProcessExit, ProcessInputChunk, ProcessOutputChunk, ProcessOutputStream};
pub use registration::{DriverRegistration, RegistrationFailure};
pub use roles::{
    AttachServingRequest, DiscoveryDriver, DiscoveryRequest, InteractiveSessionDriver,
    ModelCatalogDriver, ModelCatalogRequest, OpenSessionRequest, ResumeSessionRequest,
    ServingInstanceDriver, StartServingRequest, StructuredRunDriver, StructuredRunRequest,
    TurnRequest,
};
pub use schema::{SchemaDocument, StructuredOutputDescriptor};
pub use secret::{CredentialLease, DelegatedCredential, SecretLease};
pub use session_access::{validate_session_access_plan, validate_session_resource_lease};
pub use session_binding::SessionResumeBinding;
pub use session_options::{SessionOptions, ToolDeclaration};
pub use swallowtail_core::{
    ExternalNetworkPolicy, ExternalSearchPolicy, FilesystemBoundary,
    IncompatibleSessionAccessPolicy, ProviderApprovalPolicy, ProviderRequestHandling,
    ProviderRequestPolicy, ResourceAccess, ResourceRepresentation, SessionAccessPolicy,
};
pub use time::{Deadline, DeadlineObservation, MonotonicInstant};

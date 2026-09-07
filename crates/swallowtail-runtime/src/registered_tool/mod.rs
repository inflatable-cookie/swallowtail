//! Optional Contract 063 registered tool and operation-bridge vocabulary.
//!
//! Everything here is additive and opt-in. Registration owns no runtime
//! resource, chooses no provider, model, route, credential, or server, and
//! claims no route support. `HostServiceKind` stays exhaustive and unchanged in
//! this slice: registered-profile availability is reported through the typed
//! [`RegisteredToolReadiness`] preflight instead of a new service-kind variant.

mod admission;
mod call;
mod declaration;
mod dispatch;
mod failure;
mod identity;
mod kernel;
mod lease;
mod limits;
mod payload;
mod preparation;
mod readiness;
mod schema;
mod selection;
mod service;
mod snapshot;

#[cfg(test)]
mod tests;

pub use admission::{
    AdmissionPhase, AdmissionVerdict, AdmittedAttemptId, AdmittedSessionId, AdmittedTaskId,
    ConsumerAdmissionBinding, ConsumerAdmissionHostService, ConsumerProcessIncarnation,
};
pub use call::{
    RegisteredToolCall, RegisteredToolCallRequest, RegisteredToolExecutionDisposition,
    RegisteredToolOutcome, RegisteredToolProgress, RegisteredToolResult,
    ValidatedRegisteredToolBinding,
};
pub use declaration::{
    RegisteredToolDeclaration, RegisteredToolEffectPosture, RegisteredToolEnforcedPosture,
    RegisteredToolRetryPosture,
};
pub use dispatch::{
    RegisteredToolCancellation, RegisteredToolCancellationSource, RegisteredToolDispatchContext,
    RegisteredToolDispatcher, RegisteredToolProgressChannel, RegisteredToolProgressSink,
};
pub use failure::{RegisteredToolFailure, RegisteredToolFailureKind};
pub use identity::{
    ConsumerTaskGeneration, ConsumerWorkspaceGeneration, RegisteredServerId,
    RegisteredServerRevision, RegisteredToolCallId, RegisteredToolExecutionKind, RegisteredToolId,
    RegisteredToolLeaseGeneration, RegisteredToolLocalName, RegisteredToolNamespace,
    RegisteredToolProtocolVersion, RegisteredToolReasonCode, RegisteredToolTransport,
    RegisteredToolTransportGeneration,
};
pub use kernel::{RegisteredToolOperationKernel, first_progress_sequence};
pub use lease::{
    RegisteredToolAdmissionState, RegisteredToolBridgeLease, RegisteredToolCleanupCause,
    RegisteredToolCompletionState, RegisteredToolLifecycleState, RegisteredToolOpenRequest,
};
pub use limits::{
    MAX_REGISTERED_TOOL_AGGREGATE_SCHEMA_BYTES, MAX_REGISTERED_TOOL_ARGUMENT_BYTES,
    MAX_REGISTERED_TOOL_CREDENTIAL_REFERENCES, MAX_REGISTERED_TOOL_DECLARATIONS,
    MAX_REGISTERED_TOOL_IDENTITY_BYTES, MAX_REGISTERED_TOOL_OUTSTANDING_CALLS,
    MAX_REGISTERED_TOOL_PROGRESS_ITEM_BYTES, MAX_REGISTERED_TOOL_PROTOCOL_VERSIONS,
    MAX_REGISTERED_TOOL_QUEUED_PROGRESS_ITEMS, MAX_REGISTERED_TOOL_RECIPE_REFERENCES,
    MAX_REGISTERED_TOOL_REQUIRED_SERVICES, MAX_REGISTERED_TOOL_RESULT_BYTES,
    MAX_REGISTERED_TOOL_SCHEMA_BYTES, MAX_REGISTERED_TOOL_SELECTED_TOOLS,
    MAX_REGISTERED_TOOL_TRANSPORTS, REGISTERED_TOOL_CLEANUP_BUDGET,
    REGISTERED_TOOL_MAX_CALL_DURATION, REGISTERED_TOOL_OPEN_BUDGET, RegisteredToolBounds,
    RegisteredToolLimits,
};
pub use payload::RegisteredToolPayload;
pub use preparation::{PreparedRegisteredToolBinding, RegisteredToolPreparation};
pub use readiness::{
    REGISTERED_TOOL_CONFORMANCE_PROTOCOL_VERSION, REGISTERED_TOOL_QUALIFIED_TRANSPORTS,
    RegisteredToolMountedTopology, RegisteredToolPortAvailability, RegisteredToolReadiness,
    RegisteredToolTopologyProof,
};
pub use schema::{
    RegisteredToolSchema, RegisteredToolSchemaDialect, RegisteredToolSchemaDigest,
    RegisteredToolSchemaDocument, RegisteredToolSchemaMediaType, RegisteredToolSchemaNamespace,
};
pub use selection::RegisteredToolSelection;
pub use service::RegisteredToolBridgeHostService;
pub use snapshot::{
    RegisteredToolSnapshot, RegisteredToolSnapshotInput, RegisteredToolSource,
    RegisteredToolSourceId, RegisteredToolTransportSupport,
};

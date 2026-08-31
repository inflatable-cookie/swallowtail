//! Contract 061 contributions emitted by prepared Codex operations.
//!
//! Every row is proved by one exact prepared plan or bound request. A row
//! backed only by a documentation matrix or route-wide posture is withheld.

#[path = "consumer_route_projection/builder.rs"]
mod builder;
#[path = "consumer_route_projection/controls.rs"]
mod controls;
#[path = "consumer_route_projection/exec.rs"]
mod exec;
#[path = "consumer_route_projection/facades.rs"]
mod facades;

pub(crate) use builder::CodexProjectionBuilder;
pub(crate) use controls::bounded;

use swallowtail_core::{
    AccessStatus, Capability, CredentialState, EndpointAuthorization, EntitlementState,
    RuntimeReadiness,
};
use swallowtail_runtime::{ConsumerRouteAvailability, ConsumerRouteFeatureId};

/// Maps one exact prepared capability to portable feature identity.
///
/// A capability without a portable feature identity stays withheld rather
/// than inventing a projected row. `ProviderSessionHistory` and
/// `ProviderSessionReconciliation` are outside the app-server and exec census
/// tranches, so they are withheld here at construction rather than emitted and
/// then filtered or exempted by a coverage test.
pub(crate) const fn feature_for(capability: Capability) -> Option<ConsumerRouteFeatureId> {
    Some(match capability {
        Capability::ModelCatalog => ConsumerRouteFeatureId::ModelCatalogue,
        Capability::StructuredRun => ConsumerRouteFeatureId::StructuredRun,
        Capability::InteractiveSession => ConsumerRouteFeatureId::InteractiveSession,
        Capability::RealtimeMedia => ConsumerRouteFeatureId::RealtimeMediaSession,
        Capability::StreamingEvents => ConsumerRouteFeatureId::StreamingEvents,
        Capability::UsageReporting => ConsumerRouteFeatureId::UsageEvidence,
        Capability::ObservableActivity => ConsumerRouteFeatureId::ActivityObservation,
        Capability::ReasoningSelection => ConsumerRouteFeatureId::ReasoningSelection,
        Capability::StructuredOutput => ConsumerRouteFeatureId::StructuredOutput,
        Capability::Attachments => ConsumerRouteFeatureId::Attachments,
        Capability::ToolCalls => ConsumerRouteFeatureId::ConsumerToolExchange,
        Capability::Interruption => ConsumerRouteFeatureId::CancellationOrInterruption,
        Capability::LoadSession => ConsumerRouteFeatureId::LoadSession,
        Capability::Resume => ConsumerRouteFeatureId::ResumeSession,
        Capability::ProviderSessionCatalogue => ConsumerRouteFeatureId::ProviderSessionCatalogue,
        Capability::ProviderSessionImport => ConsumerRouteFeatureId::ProviderSessionImport,
        Capability::ProviderSessionArchive => ConsumerRouteFeatureId::ProviderSessionArchive,
        Capability::ProviderSessionRestore => ConsumerRouteFeatureId::ProviderSessionRestore,
        Capability::ProviderSessionDelete => ConsumerRouteFeatureId::ProviderSessionDelete,
        Capability::ProviderDurableRetention => ConsumerRouteFeatureId::PersistentSessionPosture,
        Capability::WorkingResource => ConsumerRouteFeatureId::WorkingResource,
        Capability::WorkingResourceTextWrite => ConsumerRouteFeatureId::BoundedWorkspaceTextWrite,
        Capability::ExternalSearch => ConsumerRouteFeatureId::ExternalSearch,
        Capability::OutputTokenLimit => ConsumerRouteFeatureId::OutputTokenLimit,
        _ => return None,
    })
}

pub(crate) const fn availability(status: &AccessStatus) -> ConsumerRouteAvailability {
    if matches!(status.credential(), CredentialState::Ready)
        && matches!(status.entitlement(), EntitlementState::Available)
        && matches!(
            status.endpoint_authorization(),
            EndpointAuthorization::Allowed
        )
        && matches!(status.runtime_readiness(), RuntimeReadiness::Ready)
    {
        ConsumerRouteAvailability::Available
    } else {
        ConsumerRouteAvailability::Conditional
    }
}

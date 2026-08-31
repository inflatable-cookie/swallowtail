//! Contract 061 contribution emitted by the prepared ZCode app-server run.
//!
//! Every row is proved by the exact prepared run plan, its capability
//! requirements, or the mode bound into `ZcodePreparedEvidence`. Model
//! selection restates the exact prepared model route, and the app-server mode
//! restates the exact bound mode; neither is inferred from a descriptor or a
//! command default. Model-catalogue and persistence-posture rows are matrix
//! postures this prepared run cannot prove, so they are withheld at
//! construction rather than emitted and filtered.

#[path = "consumer_route_projection/builder.rs"]
mod builder;

use crate::ZcodePreparedRun;
use builder::RunProjection;
use swallowtail_core::{
    AccessStatus, Capability, CredentialState, EndpointAuthorization, EntitlementState,
    RuntimeReadiness,
};
use swallowtail_runtime::{
    ConsumerRouteAvailability, ConsumerRouteEnumerableValue, ConsumerRouteEnumeratedValues,
    ConsumerRouteFeatureId, ConsumerRouteNamespacedExtension, ConsumerRouteProjectionContribution,
    ConsumerRouteProjectionFailure, ConsumerRouteProjectionSourceId, ConsumerRouteValueDomain,
};

/// Exact census route the bounded app-server descriptors belong to.
pub(crate) const ZCODE_APP_SERVER_ROUTE: &str = "zcode.app-server";

impl ZcodePreparedRun {
    /// Emits only the app-server structured-run truth this prepared run proves.
    ///
    /// The supplied source identity is preserved as one adapter contribution.
    /// Both controls prove requested and prepared state only: the app-server
    /// route observes no acknowledgement, so no row claims pending,
    /// provider-effective, or rejected truth. Activity stays a post-open
    /// descriptor-only observation row.
    pub fn consumer_route_projection_contribution(
        &self,
        source_id: ConsumerRouteProjectionSourceId,
    ) -> Result<ConsumerRouteProjectionContribution, ConsumerRouteProjectionFailure> {
        let mut projection = RunProjection::new(self.plan(), source_id)
            .with_prepared_capabilities()
            .with_owned_runtime_lifecycle()
            .with_model_selection();
        projection.push_app_server_mode(self.mode().as_str())?;
        projection.build()
    }
}

/// Maps one exact prepared capability to portable feature identity.
///
/// `Capability::ModelCatalog` is deliberately absent: the prepared app-server
/// run carries no catalogue observation, so a model-catalogue row could never
/// be constructed even if a capability profile advertised one. A capability
/// without a `zcode.app-server` census row stays withheld the same way.
const fn feature_for(capability: Capability) -> Option<ConsumerRouteFeatureId> {
    Some(match capability {
        Capability::StructuredRun => ConsumerRouteFeatureId::StructuredRun,
        Capability::StreamingEvents => ConsumerRouteFeatureId::StreamingEvents,
        Capability::UsageReporting => ConsumerRouteFeatureId::UsageEvidence,
        Capability::Interruption => ConsumerRouteFeatureId::CancellationOrInterruption,
        Capability::WorkingResource => ConsumerRouteFeatureId::WorkingResource,
        Capability::ObservableActivity => ConsumerRouteFeatureId::ActivityObservation,
        _ => return None,
    })
}

/// Names one app-server-local descriptor by route, revision, and semantic id.
fn route_local(
    segment: &str,
    semantic_id: &str,
) -> Result<ConsumerRouteNamespacedExtension, ConsumerRouteProjectionFailure> {
    ConsumerRouteNamespacedExtension::new(ZCODE_APP_SERVER_ROUTE, segment, semantic_id)
}

/// Publishes the exact prepared value as the only admitted domain member.
fn exact(value: &str) -> Result<ConsumerRouteValueDomain, ConsumerRouteProjectionFailure> {
    Ok(ConsumerRouteValueDomain::Enumerated(
        ConsumerRouteEnumeratedValues::new([ConsumerRouteEnumerableValue::new(value)?])?,
    ))
}

/// Reports current availability without flattening the access dimensions.
///
/// The route is host-owned and unauthenticated, so `NotRequired` is the exact
/// satisfied credential state rather than a missing credential.
const fn availability(status: &AccessStatus) -> ConsumerRouteAvailability {
    if matches!(
        status.credential(),
        CredentialState::Ready | CredentialState::NotRequired
    ) && matches!(status.entitlement(), EntitlementState::Available)
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

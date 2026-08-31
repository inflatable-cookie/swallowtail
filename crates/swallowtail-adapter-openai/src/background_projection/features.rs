use swallowtail_core::{
    AccessStatus, Capability, CredentialState, EndpointAuthorization, EntitlementState,
    RuntimeReadiness,
};
use swallowtail_runtime::{
    ConsumerRouteAvailability, ConsumerRouteEnumerableValue, ConsumerRouteEnumeratedValues,
    ConsumerRouteFeatureId, ConsumerRouteNamespacedExtension, ConsumerRouteProjectionFailure,
    ConsumerRouteValueDomain,
};

/// Exact census route the bounded background-local descriptors belong to.
pub(crate) const BACKGROUND_ROUTE: &str = "openai.background";

/// Portable or route-local identity of one exact prepared capability.
pub(super) enum BackgroundFeature {
    /// The capability has closed portable feature identity.
    Portable(ConsumerRouteFeatureId),
    /// The capability keeps bounded route-local identity instead.
    RouteLocal(&'static str),
}

/// Maps one exact prepared capability to the identity it may be published under.
///
/// A capability without census identity on this route stays withheld at
/// construction rather than entering a projection under a borrowed name.
pub(super) const fn feature_for(capability: Capability) -> Option<BackgroundFeature> {
    Some(match capability {
        Capability::StructuredRun => {
            BackgroundFeature::Portable(ConsumerRouteFeatureId::StructuredRun)
        }
        Capability::StreamingEvents => {
            BackgroundFeature::Portable(ConsumerRouteFeatureId::StreamingEvents)
        }
        Capability::UsageReporting => {
            BackgroundFeature::Portable(ConsumerRouteFeatureId::UsageEvidence)
        }
        Capability::OutputTokenLimit => {
            BackgroundFeature::Portable(ConsumerRouteFeatureId::OutputTokenLimit)
        }
        Capability::ReasoningSelection => {
            BackgroundFeature::Portable(ConsumerRouteFeatureId::ReasoningSelection)
        }
        Capability::StructuredOutput => {
            BackgroundFeature::Portable(ConsumerRouteFeatureId::StructuredOutput)
        }
        Capability::Interruption => {
            BackgroundFeature::Portable(ConsumerRouteFeatureId::CancellationOrInterruption)
        }
        Capability::ObservableActivity => {
            BackgroundFeature::Portable(ConsumerRouteFeatureId::ActivityObservation)
        }
        Capability::ProviderBackgroundExecution => {
            BackgroundFeature::RouteLocal("feature.retained-background-execution")
        }
        Capability::StreamReattachment => {
            BackgroundFeature::RouteLocal("feature.stream-reattachment")
        }
        Capability::OwnedRemoteResourceDeletion => {
            BackgroundFeature::RouteLocal("feature.owned-remote-resource-cleanup")
        }
        _ => return None,
    })
}

pub(super) const fn availability(status: &AccessStatus) -> ConsumerRouteAvailability {
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

/// Names one background-local descriptor by route, revision, and semantic id.
pub(super) fn route_local(
    segment: &str,
    semantic_id: &str,
) -> Result<ConsumerRouteNamespacedExtension, ConsumerRouteProjectionFailure> {
    ConsumerRouteNamespacedExtension::new(BACKGROUND_ROUTE, segment, semantic_id)
}

/// Publishes the exact prepared value as the only admitted domain member.
pub(super) fn exact(
    value: &str,
) -> Result<ConsumerRouteValueDomain, ConsumerRouteProjectionFailure> {
    Ok(ConsumerRouteValueDomain::Enumerated(
        ConsumerRouteEnumeratedValues::new([ConsumerRouteEnumerableValue::new(value)?])?,
    ))
}

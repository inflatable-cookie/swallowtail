use swallowtail_runtime::{
    ConsumerRouteActorPosture, ConsumerRouteApplicability, ConsumerRouteAvailability,
    ConsumerRouteControlId, ConsumerRouteControlValue, ConsumerRouteEvidenceStrength,
    ConsumerRouteLifecycle, ConsumerRouteMutationAuthority, ConsumerRouteNamespacedExtension,
    ConsumerRouteOmissionSemantics, ConsumerRouteProjectionRow,
    ConsumerRouteProjectionSourceIdentity, ConsumerRouteProjectionSourceKind,
    ConsumerRouteRowIdentity, ConsumerRouteSourceClass, ConsumerRouteStateSupport,
    ConsumerRouteSupportPosture, ConsumerRouteValueDomain, ConsumerRouteValueKind,
};

use crate::consumer_route_projection_source;

pub(crate) const OPERATION_SOURCE: &str = "fixture.source.provider-operation";
pub(crate) const REPLACEMENT_OPERATION_SOURCE: &str =
    "fixture.source.provider-operation-replacement";

pub(crate) fn operation_source(id: &str) -> ConsumerRouteProjectionSourceIdentity {
    consumer_route_projection_source(
        id,
        ConsumerRouteProjectionSourceKind::ProviderOperationObservation,
    )
}

pub(crate) fn operation_row(
    applicability: &ConsumerRouteApplicability,
    source: ConsumerRouteProjectionSourceIdentity,
    semantic_id: impl Into<String>,
) -> ConsumerRouteProjectionRow {
    ConsumerRouteProjectionRow::new(
        ConsumerRouteRowIdentity::Control(ConsumerRouteControlId::Namespaced(
            ConsumerRouteNamespacedExtension::new(
                "fixture.provider-operation-route",
                "1.0.0",
                semantic_id,
            )
            .expect("extension is valid"),
        )),
        applicability.clone(),
        source,
        ConsumerRouteSourceClass::ProviderOperationOutcome,
        ConsumerRouteEvidenceStrength::CompletedProviderOperation,
        ConsumerRouteLifecycle::PostOperationObservationOnly,
    )
    .with_support(ConsumerRouteSupportPosture::Supported)
    .with_availability(ConsumerRouteAvailability::Available)
    .with_actor_posture(ConsumerRouteActorPosture::ObservationOnly)
    .with_state_support(ConsumerRouteStateSupport::descriptor_only().with_observed())
    .with_mutation_authority(ConsumerRouteMutationAuthority::Absent)
    .with_control_value(ConsumerRouteControlValue::new(
        ConsumerRouteValueKind::BoundedQuery,
        ConsumerRouteValueDomain::Descriptor,
        ConsumerRouteOmissionSemantics::NotSelectable,
    ))
}

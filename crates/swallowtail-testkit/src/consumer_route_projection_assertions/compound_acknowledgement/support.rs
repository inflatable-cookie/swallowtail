use swallowtail_runtime::{
    ConsumerRouteAcknowledgementState, ConsumerRouteActorPosture, ConsumerRouteApplicability,
    ConsumerRouteCompoundAcknowledgement, ConsumerRouteEnumerableValue,
    ConsumerRouteEvidenceStrength, ConsumerRouteFeatureId, ConsumerRouteLifecycle,
    ConsumerRouteMutationAuthority, ConsumerRouteProjectionRow,
    ConsumerRouteProjectionSourceIdentity, ConsumerRouteRowIdentity, ConsumerRouteSourceClass,
    ConsumerRouteStateSupport,
};

pub(super) fn effective_reasoning_rejected_plan() -> ConsumerRouteCompoundAcknowledgement {
    ConsumerRouteCompoundAcknowledgement::new(
        ConsumerRouteAcknowledgementState::effective(value("on")),
        ConsumerRouteAcknowledgementState::rejected(value("auto")),
    )
    .expect("the compound acknowledgement is valid")
}

pub(super) fn terminal_plan() -> ConsumerRouteCompoundAcknowledgement {
    ConsumerRouteCompoundAcknowledgement::new(
        ConsumerRouteAcknowledgementState::rejected(value("off")),
        ConsumerRouteAcknowledgementState::requested_not_dispatched(),
    )
    .expect("reasoning rejection can terminally prevent Plan dispatch")
}

pub(super) fn acknowledgement_row(
    applicability: &ConsumerRouteApplicability,
    source: ConsumerRouteProjectionSourceIdentity,
    acknowledgement: ConsumerRouteCompoundAcknowledgement,
) -> ConsumerRouteProjectionRow {
    ConsumerRouteProjectionRow::new(
        ConsumerRouteRowIdentity::Feature(
            ConsumerRouteFeatureId::ActiveSessionReasoningAcknowledgement,
        ),
        applicability.clone(),
        source.clone(),
        ConsumerRouteSourceClass::RouteAcknowledgementEvidence,
        ConsumerRouteEvidenceStrength::WireAcknowledgement,
        ConsumerRouteLifecycle::PostOpenObservationOnly,
    )
    .with_actor_posture(ConsumerRouteActorPosture::ObservationOnly)
    .with_mutation_authority(ConsumerRouteMutationAuthority::Acknowledged(
        source.id().clone(),
    ))
    .with_state_support(
        ConsumerRouteStateSupport::descriptor_only()
            .with_requested()
            .with_provider_effective()
            .with_rejected(),
    )
    .with_compound_acknowledgement(acknowledgement)
}

pub(super) fn value(value: &str) -> ConsumerRouteEnumerableValue {
    ConsumerRouteEnumerableValue::new(value).expect("the provider value is admitted")
}

pub(super) fn state_value(state: &ConsumerRouteAcknowledgementState) -> Option<&str> {
    match state {
        ConsumerRouteAcknowledgementState::Effective(value)
        | ConsumerRouteAcknowledgementState::Rejected(value) => Some(value.as_str()),
        ConsumerRouteAcknowledgementState::Absent
        | ConsumerRouteAcknowledgementState::RequestedNotDispatched => None,
    }
}

use super::failure::{ConsumerRouteProjectionFailure, ConsumerRouteProjectionFailureKind, failure};
use super::identity::ConsumerRouteProjectionSourceKind;
use super::row::ConsumerRouteProjectionRow;
use super::semantics::{
    ConsumerRouteActorPosture, ConsumerRouteEvidenceStrength, ConsumerRouteFeatureId,
    ConsumerRouteLifecycle, ConsumerRouteMutationAuthority, ConsumerRouteRowIdentity,
    ConsumerRouteSourceClass,
};
use super::value::ConsumerRouteEnumerableValue;

/// State of one independently acknowledged half of a compound route value.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ConsumerRouteAcknowledgementState {
    /// This half was not requested.
    Absent,
    /// The provider confirmed this exact value.
    Effective(ConsumerRouteEnumerableValue),
    /// The provider rejected this exact value.
    Rejected(ConsumerRouteEnumerableValue),
    /// An earlier rejection ended the lifecycle before this half was dispatched.
    RequestedNotDispatched,
}

impl ConsumerRouteAcknowledgementState {
    #[must_use]
    /// Creates an absent half with no provider value.
    pub const fn absent() -> Self {
        Self::Absent
    }

    #[must_use]
    /// Associates one exact provider-confirmed value with the effective state.
    pub fn effective(value: ConsumerRouteEnumerableValue) -> Self {
        Self::Effective(value)
    }

    #[must_use]
    /// Associates one exact provider-confirmed value with the rejected state.
    pub fn rejected(value: ConsumerRouteEnumerableValue) -> Self {
        Self::Rejected(value)
    }

    #[must_use]
    /// Records that the requested half terminally was not dispatched.
    pub const fn requested_not_dispatched() -> Self {
        Self::RequestedNotDispatched
    }
}

/// Independently state-associated reasoning and Plan acknowledgement halves.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConsumerRouteCompoundAcknowledgement {
    reasoning: ConsumerRouteAcknowledgementState,
    plan: ConsumerRouteAcknowledgementState,
}

impl ConsumerRouteCompoundAcknowledgement {
    /// Admits a reasoning-first compound acknowledgement.
    pub fn new(
        reasoning: ConsumerRouteAcknowledgementState,
        plan: ConsumerRouteAcknowledgementState,
    ) -> Result<Self, ConsumerRouteProjectionFailure> {
        let reasoning_not_dispatched = matches!(
            reasoning,
            ConsumerRouteAcknowledgementState::RequestedNotDispatched
        );
        let invalid_plan_not_dispatched =
            matches!(
                plan,
                ConsumerRouteAcknowledgementState::RequestedNotDispatched
            ) && !matches!(reasoning, ConsumerRouteAcknowledgementState::Rejected(_));
        if reasoning_not_dispatched || invalid_plan_not_dispatched {
            return Err(failure(
                ConsumerRouteProjectionFailureKind::ValueDomainInvalid,
                "swallowtail.consumer_route_projection.acknowledgement_state_invalid",
                "Compound acknowledgement carries an impossible half-state combination",
            ));
        }
        Ok(Self { reasoning, plan })
    }

    #[must_use]
    /// Returns the first, reasoning acknowledgement half.
    pub const fn reasoning(&self) -> &ConsumerRouteAcknowledgementState {
        &self.reasoning
    }

    #[must_use]
    /// Returns the second, Plan acknowledgement half.
    pub const fn plan(&self) -> &ConsumerRouteAcknowledgementState {
        &self.plan
    }
}

pub(super) fn admit_compound_acknowledgement(
    row: &ConsumerRouteProjectionRow,
) -> Result<(), ConsumerRouteProjectionFailure> {
    let Some(acknowledgement) = row.compound_acknowledgement() else {
        return Ok(());
    };
    if matches!(
        acknowledgement.plan(),
        ConsumerRouteAcknowledgementState::RequestedNotDispatched
    ) && row.state_support().pending()
    {
        return Err(failure(
            ConsumerRouteProjectionFailureKind::ValueDomainInvalid,
            "swallowtail.consumer_route_projection.acknowledgement_state_invalid",
            "Terminally undispatched Plan acknowledgement cannot be pending",
        ));
    }
    if !matches!(
        row.identity(),
        ConsumerRouteRowIdentity::Feature(
            ConsumerRouteFeatureId::ActiveSessionReasoningAcknowledgement
        )
    ) || row.lifecycle() != ConsumerRouteLifecycle::PostOpenObservationOnly
    {
        return Err(failure(
            ConsumerRouteProjectionFailureKind::ApplicabilityDisagreement,
            "swallowtail.consumer_route_projection.row_applicability_rejected",
            "A projected row is not applicable to its exact binding or view",
        ));
    }
    if row.source().kind() != ConsumerRouteProjectionSourceKind::ActiveSessionObservation {
        return Err(failure(
            ConsumerRouteProjectionFailureKind::IdentityInvalid,
            "swallowtail.consumer_route_projection.row_source_unknown",
            "A projected row names a source the contribution did not supply",
        ));
    }
    let acknowledged_by_row_source = matches!(
        row.mutation_authority(),
        ConsumerRouteMutationAuthority::Acknowledged(source) if source == row.source().id()
    );
    if row.source_class() != ConsumerRouteSourceClass::RouteAcknowledgementEvidence
        || row.evidence_strength() != ConsumerRouteEvidenceStrength::WireAcknowledgement
        || row.actor_posture() != ConsumerRouteActorPosture::ObservationOnly
        || !acknowledged_by_row_source
    {
        return Err(failure(
            ConsumerRouteProjectionFailureKind::MutationAuthorityAbsent,
            "swallowtail.consumer_route_projection.mutation_authority_absent",
            "A projected row claims selectable or acknowledged posture without exact authority",
        ));
    }
    Ok(())
}

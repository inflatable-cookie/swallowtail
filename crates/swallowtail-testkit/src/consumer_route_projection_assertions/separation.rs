use swallowtail_core::{
    CredentialState, EndpointAuthorization, EntitlementState, RuntimeReadiness, SupportAuthority,
};
use swallowtail_runtime::{
    ConsumerRouteActorPosture, ConsumerRouteAvailability, ConsumerRouteControlId,
    ConsumerRouteFeatureId, ConsumerRouteLifecycle, ConsumerRouteMutationAuthority,
    ConsumerRouteProjectionContribution, ConsumerRouteProjectionFailureKind,
    ConsumerRouteProjectionSourceId, ConsumerRouteStateSupport,
};

use crate::ConsumerRouteProjectionFixture;

use super::support::*;

pub(super) fn assert_view_and_lifecycle_separation() {
    let fixture = ConsumerRouteProjectionFixture::canonical();
    let applicability = fixture.applicability();
    let acknowledged = feature_row(
        &applicability,
        ConsumerRouteFeatureId::ActiveSessionReasoningAcknowledgement,
        ConsumerRouteLifecycle::PostOpenObservationOnly,
    )
    .with_actor_posture(ConsumerRouteActorPosture::ObservationOnly)
    .with_state_support(
        ConsumerRouteStateSupport::descriptor_only()
            .with_requested()
            .with_provider_effective(),
    )
    .with_mutation_authority(ConsumerRouteMutationAuthority::Acknowledged(
        ConsumerRouteProjectionSourceId::new(OBSERVATION_SOURCE).expect("source id is valid"),
    ));
    let per_turn = control_row(
        &applicability,
        ConsumerRouteControlId::UserInputExchange,
        ConsumerRouteLifecycle::PerTurn,
    )
    .with_actor_posture(ConsumerRouteActorPosture::ObservationOnly)
    .with_mutation_authority(ConsumerRouteMutationAuthority::Absent)
    .with_state_support(ConsumerRouteStateSupport::descriptor_only().with_observed());
    let admitted = contribution(
        &applicability,
        vec![feature_row(
            &applicability,
            ConsumerRouteFeatureId::InteractiveSession,
            ConsumerRouteLifecycle::SelectionSummary,
        )],
        vec![
            control_row(
                &applicability,
                ConsumerRouteControlId::ReasoningSelection,
                ConsumerRouteLifecycle::SessionStartOnly,
            ),
            per_turn,
        ],
        vec![acknowledged],
    )
    .expect("the lifecycle-separated contribution is admitted");
    let projection = compose(&fixture, &[&admitted]).expect("the projection composes");

    assert_eq!(projection.selection_summary().rows().len(), 1);
    assert_eq!(projection.session_start_controls().rows().len(), 2);
    assert_eq!(projection.active_session_state().rows().len(), 1);
    assert!(
        projection
            .selection_summary()
            .rows()
            .all(|row| row.lifecycle() == ConsumerRouteLifecycle::SelectionSummary)
    );
    assert!(projection.session_start_controls().rows().any(|row| {
        row.lifecycle() == ConsumerRouteLifecycle::PerTurn
            && row.state_support().observed()
            && !row.state_support().prepared()
    }));
    assert!(projection.active_session_state().rows().all(|row| {
        row.lifecycle() == ConsumerRouteLifecycle::PostOpenObservationOnly
            && row.mutation_authority().is_acknowledged()
    }));
    assert_eq!(projection.sources().len(), 4);
    assert_eq!(
        projection.identity().applicability(),
        &fixture.applicability()
    );
}

/// Proves the five access dimensions stay independently observable.
///
/// An aggregate availability summary never replaces, flattens, or erases the
/// exact owning access dimension.
pub(super) fn assert_exact_access_dimensions_stay_observable() {
    let ready = ConsumerRouteProjectionFixture::canonical().applicability();
    assert_eq!(ready.credential_state(), CredentialState::Ready);
    assert_eq!(ready.entitlement_state(), EntitlementState::Available);
    assert_eq!(
        ready.endpoint_authorization(),
        EndpointAuthorization::Allowed
    );
    assert_eq!(ready.runtime_readiness(), RuntimeReadiness::Ready);
    assert_eq!(
        ready.support_authority(),
        SupportAuthority::ProviderSupported
    );

    let degraded_fixture = ConsumerRouteProjectionFixture::degraded_runtime_readiness();
    let degraded = degraded_fixture.applicability();
    assert_eq!(degraded.runtime_readiness(), RuntimeReadiness::Degraded);
    assert_eq!(degraded.credential_state(), CredentialState::Ready);
    assert_eq!(degraded.entitlement_state(), EntitlementState::Available);
    assert_eq!(
        degraded.endpoint_authorization(),
        EndpointAuthorization::Allowed
    );

    let exhausted_fixture = ConsumerRouteProjectionFixture::exhausted_entitlement();
    let exhausted = exhausted_fixture.applicability();
    assert_eq!(exhausted.entitlement_state(), EntitlementState::Exhausted);
    assert_eq!(exhausted.runtime_readiness(), RuntimeReadiness::Ready);
    assert_ne!(
        degraded, exhausted,
        "two different degraded dimensions cannot collapse to one posture"
    );

    let conditional = feature_row(
        &degraded,
        ConsumerRouteFeatureId::InteractiveSession,
        ConsumerRouteLifecycle::SelectionSummary,
    )
    .with_availability(ConsumerRouteAvailability::Conditional);
    let admitted = ConsumerRouteProjectionContribution::new(
        degraded.clone(),
        [adapter_source()],
        vec![conditional],
        Vec::new(),
        Vec::new(),
    )
    .expect("the degraded contribution is admitted");
    let projection =
        compose(&degraded_fixture, &[&admitted]).expect("the degraded projection composes");
    let row = projection
        .selection_summary()
        .rows()
        .next()
        .expect("the conditional row survives");
    assert_eq!(row.availability(), ConsumerRouteAvailability::Conditional);
    assert_eq!(
        row.applicability().runtime_readiness(),
        RuntimeReadiness::Degraded,
        "a Conditional summary must not erase the owning access dimension"
    );
    assert_eq!(
        row.applicability().credential_state(),
        CredentialState::Ready
    );
}

/// Proves per-turn rows carry consumer-mediated authority and nothing wider.
pub(super) fn assert_consumer_mediated_per_turn_authority() {
    let fixture = ConsumerRouteProjectionFixture::canonical();
    let applicability = fixture.applicability();

    let admitted = contribution(
        &applicability,
        Vec::new(),
        vec![per_turn_control_row(
            &applicability,
            ConsumerRouteControlId::UserInputExchange,
        )],
        Vec::new(),
    )
    .expect("a consumer-mediated per-turn control is admitted");
    let projection = compose(&fixture, &[&admitted]).expect("the projection composes");
    let row = projection
        .session_start_controls()
        .rows()
        .next()
        .expect("the per-turn control survives");
    assert_eq!(row.lifecycle(), ConsumerRouteLifecycle::PerTurn);
    assert!(row.mutation_authority().is_consumer_mediated_per_turn());
    assert!(!row.mutation_authority().is_prepared_session_start());
    assert!(!row.mutation_authority().is_acknowledged());
    assert!(!row.state_support().prepared());
    assert!(!row.state_support().provider_effective());
    assert!(!row.state_support().rejected());

    assert_kind(
        &contribution(
            &applicability,
            Vec::new(),
            vec![control_row(
                &applicability,
                ConsumerRouteControlId::UserInputExchange,
                ConsumerRouteLifecycle::PerTurn,
            )],
            Vec::new(),
        )
        .expect_err("a per-turn row may not carry prepared session-start authority"),
        ConsumerRouteProjectionFailureKind::MutationAuthorityAbsent,
    );

    assert_kind(
        &contribution(
            &applicability,
            Vec::new(),
            vec![
                control_row(
                    &applicability,
                    ConsumerRouteControlId::SessionOptions,
                    ConsumerRouteLifecycle::SessionStartOnly,
                )
                .with_mutation_authority(
                    ConsumerRouteMutationAuthority::ConsumerMediatedPerTurn(
                        ConsumerRouteProjectionSourceId::new(ADAPTER_SOURCE)
                            .expect("source id is valid"),
                    ),
                ),
            ],
            Vec::new(),
        )
        .expect_err("session-start truth may not claim per-turn authority"),
        ConsumerRouteProjectionFailureKind::MutationAuthorityAbsent,
    );

    assert_kind(
        &contribution(
            &applicability,
            Vec::new(),
            vec![
                per_turn_control_row(&applicability, ConsumerRouteControlId::UserInputExchange)
                    .with_state_support(
                        ConsumerRouteStateSupport::descriptor_only()
                            .with_requested()
                            .with_provider_effective(),
                    ),
            ],
            Vec::new(),
        )
        .expect_err("a per-turn exchange may not claim provider-effective state"),
        ConsumerRouteProjectionFailureKind::MutationAuthorityAbsent,
    );
}

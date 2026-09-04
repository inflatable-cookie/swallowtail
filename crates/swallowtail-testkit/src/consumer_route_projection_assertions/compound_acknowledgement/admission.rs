use swallowtail_runtime::{
    ConsumerRouteActorPosture, ConsumerRouteEvidenceStrength, ConsumerRouteFeatureId,
    ConsumerRouteLifecycle, ConsumerRouteMutationAuthority, ConsumerRouteProjectionContribution,
    ConsumerRouteProjectionFailureKind, ConsumerRouteProjectionRow,
    ConsumerRouteProjectionSourceKind, ConsumerRouteRowIdentity, ConsumerRouteSourceClass,
};

use crate::{ConsumerRouteProjectionFixture, consumer_route_projection_source};

use super::super::support::{adapter_source, assert_kind, contribution, observation_source};
use super::support::{acknowledgement_row, effective_reasoning_rejected_plan};

/// Proves that compound truth requires completed active-session observation evidence.
pub fn assert_compound_acknowledgement_requires_observation_source() {
    let applicability = ConsumerRouteProjectionFixture::canonical().applicability();
    let prepared_source_row = acknowledgement_row(
        &applicability,
        adapter_source(),
        effective_reasoning_rejected_plan(),
    );
    assert_kind(
        &contribution(
            &applicability,
            Vec::new(),
            Vec::new(),
            vec![prepared_source_row],
        )
        .expect_err("a prepared source cannot publish compound acknowledgement truth"),
        ConsumerRouteProjectionFailureKind::IdentityInvalid,
    );

    let provider_operation_source = consumer_route_projection_source(
        "fixture.source.provider-operation",
        ConsumerRouteProjectionSourceKind::ProviderOperationObservation,
    );
    let provider_operation_row = acknowledgement_row(
        &applicability,
        provider_operation_source.clone(),
        effective_reasoning_rejected_plan(),
    );
    assert_kind(
        &ConsumerRouteProjectionContribution::new(
            applicability.clone(),
            [provider_operation_source],
            Vec::new(),
            Vec::new(),
            [provider_operation_row],
        )
        .expect_err("a provider-operation source cannot publish active-session truth"),
        ConsumerRouteProjectionFailureKind::ProviderOperationObservationInvalid,
    );

    let source = observation_source();
    let missing_source_row =
        acknowledgement_row(&applicability, source, effective_reasoning_rejected_plan());
    assert_kind(
        &ConsumerRouteProjectionContribution::new(
            applicability.clone(),
            [adapter_source()],
            Vec::new(),
            Vec::new(),
            [missing_source_row],
        )
        .expect_err("the active observation source must be supplied"),
        ConsumerRouteProjectionFailureKind::IdentityInvalid,
    );

    let source = observation_source();
    let mismatched_identity = ConsumerRouteProjectionRow::new(
        ConsumerRouteRowIdentity::Feature(ConsumerRouteFeatureId::PreparedFacade),
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
    .with_compound_acknowledgement(effective_reasoning_rejected_plan());
    assert_kind(
        &ConsumerRouteProjectionContribution::new(
            applicability.clone(),
            [source],
            Vec::new(),
            Vec::new(),
            [mismatched_identity],
        )
        .expect_err("a different row identity cannot carry compound acknowledgement truth"),
        ConsumerRouteProjectionFailureKind::ApplicabilityDisagreement,
    );

    let source = observation_source();
    let non_observation_evidence = ConsumerRouteProjectionRow::new(
        ConsumerRouteRowIdentity::Feature(
            ConsumerRouteFeatureId::ActiveSessionReasoningAcknowledgement,
        ),
        applicability.clone(),
        source.clone(),
        ConsumerRouteSourceClass::PreparedOperationRecord,
        ConsumerRouteEvidenceStrength::PreparedOperation,
        ConsumerRouteLifecycle::PostOpenObservationOnly,
    )
    .with_actor_posture(ConsumerRouteActorPosture::ObservationOnly)
    .with_mutation_authority(ConsumerRouteMutationAuthority::Acknowledged(
        source.id().clone(),
    ))
    .with_compound_acknowledgement(effective_reasoning_rejected_plan());
    assert_kind(
        &ConsumerRouteProjectionContribution::new(
            applicability,
            [source],
            Vec::new(),
            Vec::new(),
            [non_observation_evidence],
        )
        .expect_err("non-observation evidence cannot publish compound acknowledgement truth"),
        ConsumerRouteProjectionFailureKind::MutationAuthorityAbsent,
    );
}

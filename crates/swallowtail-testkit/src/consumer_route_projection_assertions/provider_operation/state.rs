use swallowtail_core::OperationShape;
use swallowtail_runtime::{
    ConsumerRouteActorPosture, ConsumerRouteEvidenceStrength, ConsumerRouteLifecycle,
    ConsumerRouteMutationAuthority, ConsumerRouteOmissionSemantics, ConsumerRouteProjectionInput,
    ConsumerRouteProviderOperationObservation, ConsumerRouteProviderOperationOutcome,
    ConsumerRouteSourceClass, ConsumerRouteValueDomain, ConsumerRouteValueKind,
    compose_consumer_route_projection,
};

use crate::consumer_route_projection_assertions::support::{evidence_source, record_source};

use super::support::{
    AccessCase, OPERATION_SOURCE, ProviderOperationFixture, operation_row, operation_source,
};

pub(super) fn assert_provider_operation_state_is_honest_descriptor_only_observation() {
    let catalogue =
        ProviderOperationFixture::new(OperationShape::ProviderSessionCatalogue, AccessCase::Ready);
    let catalogue_plan = catalogue.catalogue_plan();
    let catalogue_outcome = catalogue.catalogue_outcome(&catalogue_plan);
    let catalogue_evidence = catalogue.prepared();
    let catalogue_observation = ConsumerRouteProviderOperationObservation::new(
        &catalogue_evidence,
        ConsumerRouteProviderOperationOutcome::ProviderSessionCatalogue(&catalogue_outcome),
        operation_source(OPERATION_SOURCE),
        [operation_row(
            &swallowtail_runtime::ConsumerRouteApplicability::from_prepared_operation(
                &catalogue_evidence,
            ),
            operation_source(OPERATION_SOURCE),
            "control.provider-session-catalogue",
        )],
    )
    .expect("catalogue observation is admitted");
    let invalid_row = operation_row(
        &swallowtail_runtime::ConsumerRouteApplicability::from_prepared_operation(
            &catalogue_evidence,
        ),
        operation_source(OPERATION_SOURCE),
        "control.provider-session-catalogue-invalid",
    )
    .with_actor_posture(ConsumerRouteActorPosture::Informational);
    let invalid = ConsumerRouteProviderOperationObservation::new(
        &catalogue_evidence,
        ConsumerRouteProviderOperationOutcome::ProviderSessionCatalogue(&catalogue_outcome),
        operation_source(OPERATION_SOURCE),
        [invalid_row],
    )
    .expect_err("an operation row with non-observation actor posture is rejected");
    super::admission::assert_provider_operation_failure(
        &invalid,
        swallowtail_runtime::ConsumerRouteProjectionFailureKind::ProviderOperationObservationInvalid,
        "swallowtail.consumer_route_projection.provider_operation_row_rejected",
    );
    let catalogue_record = catalogue.record(&catalogue_evidence);
    let catalogue_projection = compose_consumer_route_projection(
        ConsumerRouteProjectionInput::new(
            &catalogue_record,
            record_source(),
            &catalogue_evidence,
            evidence_source(),
        )
        .with_provider_operation_observations([&catalogue_observation]),
    )
    .expect("catalogue projection composes");
    assert!(
        catalogue_projection
            .selection_summary()
            .rows()
            .next()
            .is_none()
    );
    assert!(
        catalogue_projection
            .session_start_controls()
            .rows()
            .next()
            .is_none()
    );
    assert!(
        catalogue_projection
            .active_session_state()
            .rows()
            .next()
            .is_none()
    );
    assert_observation_row(
        catalogue_projection
            .provider_operation_state()
            .rows()
            .next()
            .expect("catalogue row stays in the fourth view"),
    );

    let history =
        ProviderOperationFixture::new(OperationShape::ProviderSessionHistory, AccessCase::Ready);
    let history_plan = history.history_plan();
    let history_page = history.history_page(&history_plan);
    let history_evidence = history.prepared();
    let history_observation = ConsumerRouteProviderOperationObservation::new(
        &history_evidence,
        ConsumerRouteProviderOperationOutcome::ProviderSessionHistory(&history_page),
        operation_source(OPERATION_SOURCE),
        [operation_row(
            &swallowtail_runtime::ConsumerRouteApplicability::from_prepared_operation(
                &history_evidence,
            ),
            operation_source(OPERATION_SOURCE),
            "control.provider-session-history",
        )],
    )
    .expect("history observation is admitted");
    let history_record = history.record(&history_evidence);
    let history_projection = compose_consumer_route_projection(
        ConsumerRouteProjectionInput::new(
            &history_record,
            record_source(),
            &history_evidence,
            evidence_source(),
        )
        .with_provider_operation_observations([&history_observation]),
    )
    .expect("history projection composes separately");
    assert_observation_row(
        history_projection
            .provider_operation_state()
            .rows()
            .next()
            .expect("history row stays in its separate fourth view"),
    );

    let rendered = format!(
        "{catalogue_projection:?}{history_projection:?}{catalogue_outcome:?}{history_page:?}"
    );
    for forbidden in [
        "private/provider-operation-target",
        "provider/private/session",
        "fixture-catalogue-request",
        "fixture-history-request",
    ] {
        assert!(!rendered.contains(forbidden));
    }
}

fn assert_observation_row(row: &swallowtail_runtime::ConsumerRouteProjectionRow) {
    let value = row
        .control_value()
        .expect("observation row has a value descriptor");
    assert_eq!(value.kind(), ConsumerRouteValueKind::BoundedQuery);
    assert!(matches!(
        value.domain(),
        ConsumerRouteValueDomain::Descriptor
    ));
    assert_eq!(
        value.omission(),
        ConsumerRouteOmissionSemantics::NotSelectable
    );
    assert_eq!(
        row.actor_posture(),
        ConsumerRouteActorPosture::ObservationOnly
    );
    assert_eq!(
        row.lifecycle(),
        ConsumerRouteLifecycle::PostOperationObservationOnly
    );
    assert_eq!(
        row.source_class(),
        ConsumerRouteSourceClass::ProviderOperationOutcome
    );
    assert_eq!(
        row.evidence_strength(),
        ConsumerRouteEvidenceStrength::CompletedProviderOperation
    );
    let state = row.state_support();
    assert!(state.observed());
    assert!(!state.requested());
    assert!(!state.prepared());
    assert!(!state.pending());
    assert!(!state.provider_effective());
    assert!(!state.rejected());
    assert!(matches!(
        row.mutation_authority(),
        ConsumerRouteMutationAuthority::Absent
    ));
    assert_eq!(
        row.source().kind(),
        swallowtail_runtime::ConsumerRouteProjectionSourceKind::ProviderOperationObservation
    );
}

use swallowtail_core::OperationShape;
use swallowtail_runtime::{
    ConsumerRouteApplicability, ConsumerRouteFeatureId, ConsumerRouteLifecycle,
    ConsumerRouteProjectionContribution, ConsumerRouteProjectionFailureKind,
    ConsumerRouteProjectionInput, ConsumerRouteProjectionSourceKind,
    ConsumerRouteProviderOperationObservation, ConsumerRouteProviderOperationOutcome,
    compose_consumer_route_projection,
};

use crate::consumer_route_projection_assertions::support::{
    ADAPTER_SOURCE, adapter_source, evidence_source, feature_row, record_source,
};
use crate::consumer_route_projection_source;

use super::admission::{assert_non_outcome_source_is_rejected, assert_provider_operation_failure};
use super::support::{
    AccessCase, OPERATION_SOURCE, ProviderOperationFixture, REPLACEMENT_OPERATION_SOURCE,
    operation_row, operation_source,
};

pub(super) fn assert_provider_operation_sources_compose_without_merging() {
    let fixture =
        ProviderOperationFixture::new(OperationShape::ProviderSessionCatalogue, AccessCase::Ready);
    let plan = fixture.catalogue_plan();
    let outcome = fixture.catalogue_outcome(&plan);
    let evidence = fixture.prepared();
    let applicability = ConsumerRouteApplicability::from_prepared_operation(&evidence);
    let prepared = ConsumerRouteProjectionContribution::new(
        applicability.clone(),
        [adapter_source()],
        [feature_row(
            &applicability,
            ConsumerRouteFeatureId::ProviderSessionCatalogue,
            ConsumerRouteLifecycle::SelectionSummary,
        )],
        Vec::new(),
        Vec::new(),
    )
    .expect("ordinary prepared contribution is admitted");
    let initial_observation = observation(&evidence, &outcome, OPERATION_SOURCE);
    let record = fixture.record(&evidence);
    let before = compose_consumer_route_projection(
        ConsumerRouteProjectionInput::new(&record, record_source(), &evidence, evidence_source())
            .with_contributions([&prepared])
            .with_provider_operation_observations([&initial_observation]),
    )
    .expect("prepared and completed-outcome sources co-compose");
    assert_eq!(before.selection_summary().rows().len(), 1);
    assert_eq!(before.provider_operation_state().rows().len(), 1);
    assert!(
        before
            .sources()
            .any(|source| source.id().as_str() == ADAPTER_SOURCE)
    );
    assert!(before.sources().any(|source| {
        source.id().as_str() == OPERATION_SOURCE
            && source.kind() == ConsumerRouteProjectionSourceKind::ProviderOperationObservation
    }));

    let replacement = observation(&evidence, &outcome, REPLACEMENT_OPERATION_SOURCE);
    let after = compose_consumer_route_projection(
        ConsumerRouteProjectionInput::new(&record, record_source(), &evidence, evidence_source())
            .with_contributions([&prepared])
            .with_provider_operation_observations([&replacement]),
    )
    .expect("replacement outcome source composes");
    assert_ne!(before.identity(), after.identity());
    assert_eq!(
        before
            .provider_operation_state()
            .rows()
            .next()
            .map(|row| row.identity()),
        after
            .provider_operation_state()
            .rows()
            .next()
            .map(|row| row.identity())
    );
}

pub(super) fn assert_provider_operation_cross_access_and_source_disagreement_fail_closed() {
    let canonical =
        ProviderOperationFixture::new(OperationShape::ProviderSessionCatalogue, AccessCase::Ready);
    let canonical_plan = canonical.catalogue_plan();
    let canonical_outcome = canonical.catalogue_outcome(&canonical_plan);
    let canonical_evidence = canonical.prepared();
    let canonical_observation =
        observation(&canonical_evidence, &canonical_outcome, OPERATION_SOURCE);

    for access in [
        AccessCase::DegradedRuntime,
        AccessCase::ExhaustedEntitlement,
    ] {
        let divergent =
            ProviderOperationFixture::new(OperationShape::ProviderSessionCatalogue, access);
        let divergent_plan = divergent.catalogue_plan();
        let divergent_outcome = divergent.catalogue_outcome(&divergent_plan);
        let divergent_evidence = divergent.prepared();
        let divergent_observation =
            observation(&divergent_evidence, &divergent_outcome, OPERATION_SOURCE);
        assert_snapshot_disagreement(&canonical, &canonical_evidence, &divergent_observation);
        assert_snapshot_disagreement(&divergent, &divergent_evidence, &canonical_observation);
    }

    let applicability = ConsumerRouteApplicability::from_prepared_operation(&canonical_evidence);
    let repeated_id = consumer_route_projection_source(
        OPERATION_SOURCE,
        ConsumerRouteProjectionSourceKind::AdapterContribution,
    );
    let prepared = ConsumerRouteProjectionContribution::new(
        applicability,
        [repeated_id],
        Vec::new(),
        Vec::new(),
        Vec::new(),
    )
    .expect("ordinary source is internally valid");
    let record = canonical.record(&canonical_evidence);
    let duplicate = compose_consumer_route_projection(
        ConsumerRouteProjectionInput::new(
            &record,
            record_source(),
            &canonical_evidence,
            evidence_source(),
        )
        .with_contributions([&prepared])
        .with_provider_operation_observations([&canonical_observation]),
    )
    .expect_err("one source id cannot span prepared and outcome classes");
    assert_provider_operation_failure(
        &duplicate,
        ConsumerRouteProjectionFailureKind::DuplicateSource,
        "swallowtail.consumer_route_projection.source_identity_duplicate",
    );

    let history =
        ProviderOperationFixture::new(OperationShape::ProviderSessionHistory, AccessCase::Ready);
    let history_plan = history.history_plan();
    let history_page = history.history_page(&history_plan);
    let mismatch = ConsumerRouteProviderOperationObservation::new(
        &canonical_evidence,
        ConsumerRouteProviderOperationOutcome::ProviderSessionHistory(&history_page),
        operation_source(OPERATION_SOURCE),
        Vec::new(),
    )
    .expect_err("catalogue evidence cannot carry a history outcome");
    assert_provider_operation_failure(
        &mismatch,
        ConsumerRouteProjectionFailureKind::ProviderOperationObservationInvalid,
        "swallowtail.consumer_route_projection.provider_operation_evidence_rejected",
    );
    assert_non_outcome_source_is_rejected(
        &canonical_evidence,
        ConsumerRouteProviderOperationOutcome::ProviderSessionCatalogue(&canonical_outcome),
    );
}

fn observation(
    evidence: &swallowtail_runtime::PreparedOperationEvidence,
    outcome: &swallowtail_runtime::ProviderSessionCatalogueOutcome,
    source_id: &str,
) -> ConsumerRouteProviderOperationObservation {
    ConsumerRouteProviderOperationObservation::new(
        evidence,
        ConsumerRouteProviderOperationOutcome::ProviderSessionCatalogue(outcome),
        operation_source(source_id),
        [operation_row(
            &ConsumerRouteApplicability::from_prepared_operation(evidence),
            operation_source(source_id),
            "control.provider-session-catalogue",
        )],
    )
    .expect("provider-operation observation is admitted")
}

fn assert_snapshot_disagreement(
    fixture: &ProviderOperationFixture,
    evidence: &swallowtail_runtime::PreparedOperationEvidence,
    observation: &ConsumerRouteProviderOperationObservation,
) {
    let record = fixture.record(evidence);
    let failure = compose_consumer_route_projection(
        ConsumerRouteProjectionInput::new(&record, record_source(), evidence, evidence_source())
            .with_provider_operation_observations([observation]),
    )
    .expect_err("cross-access observation assembly fails closed");
    assert_provider_operation_failure(
        &failure,
        ConsumerRouteProjectionFailureKind::SnapshotIdentityDisagreement,
        "swallowtail.consumer_route_projection.snapshot_identity_rejected",
    );
}

use swallowtail_core::{OperationShape, SafeDiagnostic};
use swallowtail_runtime::{
    CleanupOutcome, ConsumerRouteApplicability, ConsumerRouteLifecycle,
    ConsumerRouteProjectionContribution, ConsumerRouteProjectionFailureKind,
    ConsumerRouteProjectionInput, ConsumerRouteProjectionSourceKind,
    ConsumerRouteProviderOperationObservation, ConsumerRouteProviderOperationOutcome,
    MAX_CONSUMER_ROUTE_NAMESPACED_EXTENSIONS, MAX_CONSUMER_ROUTE_PROVIDER_OPERATION_ROWS,
    ProviderSessionCatalogueOutcome, ProviderSessionOperationFailureStage,
    compose_consumer_route_projection,
};

use crate::{ConsumerRouteProjectionFixture, consumer_route_projection_source};

use super::support::{
    AccessCase, OPERATION_SOURCE, ProviderOperationFixture, operation_row, operation_source,
};
use crate::consumer_route_projection_assertions::support::{
    adapter_source, assert_kind, evidence_source, feature_row, namespaced_feature, record_source,
};

pub(super) fn assert_provider_operation_session_shape_is_rejected() {
    let catalogue =
        ProviderOperationFixture::new(OperationShape::ProviderSessionCatalogue, AccessCase::Ready);
    let catalogue_plan = catalogue.catalogue_plan();
    let catalogue_outcome = catalogue.catalogue_outcome(&catalogue_plan);
    for evidence in [
        ConsumerRouteProjectionFixture::canonical().prepared(),
        ProviderOperationFixture::new(OperationShape::StructuredRun, AccessCase::Ready).prepared(),
    ] {
        let failure = ConsumerRouteProviderOperationObservation::new(
            &evidence,
            ConsumerRouteProviderOperationOutcome::ProviderSessionCatalogue(&catalogue_outcome),
            operation_source(OPERATION_SOURCE),
            Vec::new(),
        )
        .expect_err("session-shaped and structured-run evidence are rejected");
        assert_provider_operation_failure(
            &failure,
            ConsumerRouteProjectionFailureKind::ProviderOperationObservationInvalid,
            "swallowtail.consumer_route_projection.provider_operation_shape_rejected",
        );
    }

    let catalogue_evidence = catalogue.prepared();
    ConsumerRouteProviderOperationObservation::new(
        &catalogue_evidence,
        ConsumerRouteProviderOperationOutcome::ProviderSessionCatalogue(&catalogue_outcome),
        operation_source(OPERATION_SOURCE),
        Vec::new(),
    )
    .expect("matching catalogue evidence and outcome are admitted");

    let divergent = ProviderOperationFixture::new(
        OperationShape::ProviderSessionCatalogue,
        AccessCase::DegradedRuntime,
    );
    let divergent_plan = divergent.catalogue_plan();
    let divergent_outcome = divergent.catalogue_outcome(&divergent_plan);
    let mismatch = ConsumerRouteProviderOperationObservation::new(
        &catalogue_evidence,
        ConsumerRouteProviderOperationOutcome::ProviderSessionCatalogue(&divergent_outcome),
        operation_source(OPERATION_SOURCE),
        Vec::new(),
    )
    .expect_err("same-shape outcome evidence from another plan is rejected");
    assert_provider_operation_failure(
        &mismatch,
        ConsumerRouteProjectionFailureKind::ProviderOperationObservationInvalid,
        "swallowtail.consumer_route_projection.provider_operation_evidence_rejected",
    );

    let history =
        ProviderOperationFixture::new(OperationShape::ProviderSessionHistory, AccessCase::Ready);
    let history_plan = history.history_plan();
    let history_page = history.history_page(&history_plan);
    let history_evidence = history.prepared();
    ConsumerRouteProviderOperationObservation::new(
        &history_evidence,
        ConsumerRouteProviderOperationOutcome::ProviderSessionHistory(&history_page),
        operation_source(OPERATION_SOURCE),
        Vec::new(),
    )
    .expect("matching history evidence and page are admitted");
}

pub(super) fn assert_prepared_record_cannot_masquerade_as_provider_operation_observation() {
    let fixture =
        ProviderOperationFixture::new(OperationShape::ProviderSessionCatalogue, AccessCase::Ready);
    let evidence = fixture.prepared();
    let applicability = ConsumerRouteApplicability::from_prepared_operation(&evidence);
    let source = operation_source(OPERATION_SOURCE);
    let failure = ConsumerRouteProjectionContribution::new(
        applicability,
        [source],
        Vec::new(),
        Vec::new(),
        Vec::new(),
    )
    .expect_err("prepared contribution cannot use the completed-outcome source kind");
    assert_provider_operation_failure(
        &failure,
        ConsumerRouteProjectionFailureKind::ProviderOperationObservationInvalid,
        "swallowtail.consumer_route_projection.provider_operation_source_rejected",
    );

    let plan = fixture.catalogue_plan();
    let request = swallowtail_runtime::ProviderSessionCatalogueRequest::from_plan(
        swallowtail_runtime::RequestId::new("failed-catalogue-request")
            .expect("request id is valid"),
        &plan,
        None,
    )
    .expect("request is valid");
    let failed = ProviderSessionCatalogueOutcome::new(
        &plan,
        &request,
        Vec::new(),
        None,
        CleanupOutcome::Failed(SafeDiagnostic::new(
            "fixture.cleanup_failed",
            "fixture cleanup failed",
        )),
    )
    .expect_err("a failed provider operation produces no successful outcome value");
    assert_eq!(
        failed.stage(),
        ProviderSessionOperationFailureStage::Cleanup
    );
}

pub(super) fn assert_provider_operation_row_maximum_is_fixed() {
    let fixture =
        ProviderOperationFixture::new(OperationShape::ProviderSessionCatalogue, AccessCase::Ready);
    let plan = fixture.catalogue_plan();
    let outcome = fixture.catalogue_outcome(&plan);
    let evidence = fixture.prepared();
    let applicability = ConsumerRouteApplicability::from_prepared_operation(&evidence);
    let rows = |count| {
        (0..count)
            .map(|index| {
                operation_row(
                    &applicability,
                    operation_source(OPERATION_SOURCE),
                    format!("control.provider-session-catalogue-{index}"),
                )
            })
            .collect::<Vec<_>>()
    };
    let exact = ConsumerRouteProviderOperationObservation::new(
        &evidence,
        ConsumerRouteProviderOperationOutcome::ProviderSessionCatalogue(&outcome),
        operation_source(OPERATION_SOURCE),
        rows(MAX_CONSUMER_ROUTE_PROVIDER_OPERATION_ROWS),
    )
    .expect("the fixed maximum is admitted without truncation");
    assert_eq!(
        exact.rows().len(),
        MAX_CONSUMER_ROUTE_PROVIDER_OPERATION_ROWS
    );

    let prepared = ConsumerRouteProjectionContribution::new(
        applicability.clone(),
        [adapter_source()],
        (0..MAX_CONSUMER_ROUTE_NAMESPACED_EXTENSIONS).map(|index| {
            feature_row(
                &applicability,
                namespaced_feature(index),
                ConsumerRouteLifecycle::SelectionSummary,
            )
        }),
        Vec::new(),
        Vec::new(),
    )
    .expect("the prepared contribution fills its extension budget");
    let record = fixture.record(&evidence);
    let aggregate = compose_consumer_route_projection(
        ConsumerRouteProjectionInput::new(&record, record_source(), &evidence, evidence_source())
            .with_contributions([&prepared])
            .with_provider_operation_observations([&exact]),
    )
    .expect_err("namespaced extensions are counted across all four views");
    assert_provider_operation_failure(
        &aggregate,
        ConsumerRouteProjectionFailureKind::LimitExceeded,
        "swallowtail.consumer_route_projection.namespaced_extension_limit_exceeded",
    );

    let failure = ConsumerRouteProviderOperationObservation::new(
        &evidence,
        ConsumerRouteProviderOperationOutcome::ProviderSessionCatalogue(&outcome),
        operation_source(OPERATION_SOURCE),
        rows(MAX_CONSUMER_ROUTE_PROVIDER_OPERATION_ROWS + 1),
    )
    .expect_err("maximum plus one is rejected");
    assert_provider_operation_failure(
        &failure,
        ConsumerRouteProjectionFailureKind::LimitExceeded,
        "swallowtail.consumer_route_projection.provider_operation_limit_exceeded",
    );
}

pub(super) fn assert_provider_operation_failure(
    failure: &swallowtail_runtime::ConsumerRouteProjectionFailure,
    kind: ConsumerRouteProjectionFailureKind,
    code: &str,
) {
    assert_kind(failure, kind);
    assert_eq!(failure.diagnostic().code(), code);
    let expected_message = match code {
        "swallowtail.consumer_route_projection.provider_operation_shape_rejected" => {
            Some("Provider-operation observation requires an admitted completed operation shape")
        }
        "swallowtail.consumer_route_projection.provider_operation_evidence_rejected" => {
            Some("Provider-operation observation does not match its completed outcome evidence")
        }
        "swallowtail.consumer_route_projection.provider_operation_row_rejected" => {
            Some("Provider-operation observation row claims incompatible lifecycle or authority")
        }
        "swallowtail.consumer_route_projection.provider_operation_source_rejected" => {
            Some("Prepared contribution cannot publish provider-operation observation")
        }
        "swallowtail.consumer_route_projection.provider_operation_limit_exceeded" => {
            Some("Projected provider-operation state exceeds the fixed row maximum")
        }
        _ => None,
    };
    if let Some(expected_message) = expected_message {
        assert_eq!(failure.diagnostic().message(), expected_message);
    }
}

pub(super) fn assert_non_outcome_source_is_rejected(
    evidence: &swallowtail_runtime::PreparedOperationEvidence,
    outcome: ConsumerRouteProviderOperationOutcome<'_>,
) {
    let failure = ConsumerRouteProviderOperationObservation::new(
        evidence,
        outcome,
        consumer_route_projection_source(
            "fixture.source.prepared-operation",
            ConsumerRouteProjectionSourceKind::PreparedOperation,
        ),
        Vec::new(),
    )
    .expect_err("a non-outcome source is rejected");
    assert_provider_operation_failure(
        &failure,
        ConsumerRouteProjectionFailureKind::ProviderOperationObservationInvalid,
        "swallowtail.consumer_route_projection.provider_operation_evidence_rejected",
    );
}

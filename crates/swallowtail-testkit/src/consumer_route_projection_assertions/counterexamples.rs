use swallowtail_core::SafeDiagnostic;
use swallowtail_runtime::{
    ConsumerRouteAvailabilityDimension, ConsumerRouteControlId, ConsumerRouteFeatureId,
    ConsumerRouteLifecycle, ConsumerRouteProjectionFailureKind, ConsumerRouteProjectionInput,
    ConsumerRouteProjectionSourceId, ConsumerRouteSafeReason, MAX_CONSUMER_ROUTE_SAFE_REASON_BYTES,
    compose_consumer_route_projection,
};

use crate::ConsumerRouteProjectionFixture;

use super::support::*;

pub(super) fn assert_named_counterexamples() {
    let fixture = ConsumerRouteProjectionFixture::canonical();
    let applicability = fixture.applicability();
    let superseded_fixture = ConsumerRouteProjectionFixture::superseded();

    let route_wide = feature_row(
        &superseded_fixture.applicability(),
        ConsumerRouteFeatureId::ExternalSearch,
        ConsumerRouteLifecycle::SelectionSummary,
    );
    assert_kind(
        &contribution(&applicability, vec![route_wide], Vec::new(), Vec::new())
            .expect_err("applicability disagreement rejects the row before publication"),
        ConsumerRouteProjectionFailureKind::ApplicabilityDisagreement,
    );

    let record = fixture.record();
    let stale = superseded_fixture.prepared();
    assert_kind(
        &compose_consumer_route_projection(ConsumerRouteProjectionInput::new(
            &record,
            record_source(),
            &stale,
            evidence_source(),
        ))
        .expect_err("a superseded revision rejects the whole assembly"),
        ConsumerRouteProjectionFailureKind::SnapshotIdentityDisagreement,
    );

    let post_open_selectable = control_row(
        &applicability,
        ConsumerRouteControlId::SessionOptions,
        ConsumerRouteLifecycle::PostOpenObservationOnly,
    );
    assert_kind(
        &contribution(
            &applicability,
            Vec::new(),
            Vec::new(),
            vec![post_open_selectable],
        )
        .expect_err("a post-open option list may not be presented as selectable"),
        ConsumerRouteProjectionFailureKind::MutationAuthorityAbsent,
    );

    let per_turn_as_session_start = control_row(
        &applicability,
        ConsumerRouteControlId::UserInputExchange,
        ConsumerRouteLifecycle::PerTurn,
    );
    assert_kind(
        &contribution(
            &applicability,
            Vec::new(),
            vec![per_turn_as_session_start],
            Vec::new(),
        )
        .expect_err("a per-turn exchange may not claim session-start preparation"),
        ConsumerRouteProjectionFailureKind::MutationAuthorityAbsent,
    );

    assert_kind(
        &ConsumerRouteSafeReason::new(
            ConsumerRouteAvailabilityDimension::CatalogueResult,
            ConsumerRouteProjectionSourceId::new(ADAPTER_SOURCE).expect("source id is valid"),
            SafeDiagnostic::new(
                "fixture.reason",
                "r".repeat(MAX_CONSUMER_ROUTE_SAFE_REASON_BYTES + 1),
            ),
        )
        .expect_err("an unbounded reason claim is rejected"),
        ConsumerRouteProjectionFailureKind::SafeReasonLimitExceeded,
    );

    let unnamed_reason = feature_row(
        &applicability,
        ConsumerRouteFeatureId::ModelCatalogue,
        ConsumerRouteLifecycle::SelectionSummary,
    )
    .with_safe_reason(
        ConsumerRouteSafeReason::new(
            ConsumerRouteAvailabilityDimension::CatalogueResult,
            ConsumerRouteProjectionSourceId::new("fixture.source.absent")
                .expect("source id is valid"),
            SafeDiagnostic::new("fixture.reason", "catalogue result is unknown"),
        )
        .expect("bounded reason is admitted"),
    );
    assert_kind(
        &contribution(&applicability, vec![unnamed_reason], Vec::new(), Vec::new())
            .expect_err("a reason no named source supplied is rejected"),
        ConsumerRouteProjectionFailureKind::IdentityInvalid,
    );

    let canonical_record = ConsumerRouteProjectionFixture::canonical();
    for divergent in [
        ConsumerRouteProjectionFixture::degraded_runtime_readiness(),
        ConsumerRouteProjectionFixture::exhausted_entitlement(),
    ] {
        assert_kind(
            &compose_across(&canonical_record, &divergent)
                .expect_err("identical identity with a differing access dimension fails closed"),
            ConsumerRouteProjectionFailureKind::SnapshotIdentityDisagreement,
        );
        assert_kind(
            &compose_across(&divergent, &canonical_record)
                .expect_err("the comparison fails closed in both directions"),
            ConsumerRouteProjectionFailureKind::SnapshotIdentityDisagreement,
        );
    }
}

use swallowtail_core::SafeDiagnostic;
use swallowtail_runtime::{
    ConsumerRouteAvailability, ConsumerRouteAvailabilityDimension, ConsumerRouteControlId,
    ConsumerRouteEvidenceStrength, ConsumerRouteFeatureId, ConsumerRouteLifecycle,
    ConsumerRouteProjectionFailureKind, ConsumerRouteProjectionInput, ConsumerRouteProjectionRow,
    ConsumerRouteProjectionSourceId, ConsumerRouteProjectionSourceIdentity,
    ConsumerRouteProjectionSourceKind, ConsumerRouteRowIdentity, ConsumerRouteSafeReason,
    ConsumerRouteSourceClass, ConsumerRouteSupportPosture, MAX_CONSUMER_ROUTE_SAFE_REASON_BYTES,
    compose_consumer_route_projection,
};

use crate::{ConsumerRouteProjectionFixture, consumer_route_projection_source};

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

/// Rejects a row that borrows a supplied source id under another evidence class.
///
/// A source id alone is not an identity. Admitting the id without its kind
/// would let one prepared adapter contribution masquerade as an active-session
/// observation, collapsing two independently replaceable evidence sources.
pub(super) fn assert_source_kind_is_part_of_identity() {
    let applicability = ConsumerRouteProjectionFixture::canonical().applicability();
    for (id, kind) in [
        (
            ADAPTER_SOURCE,
            ConsumerRouteProjectionSourceKind::ActiveSessionObservation,
        ),
        (
            OBSERVATION_SOURCE,
            ConsumerRouteProjectionSourceKind::AdapterContribution,
        ),
    ] {
        let borrowed = observation_row(consumer_route_projection_source(id, kind));
        assert_kind(
            &contribution(&applicability, Vec::new(), Vec::new(), vec![borrowed])
                .expect_err("a supplied source id under another evidence class is rejected"),
            ConsumerRouteProjectionFailureKind::IdentityInvalid,
        );
    }

    let exact = observation_row(observation_source());
    let admitted = contribution(&applicability, Vec::new(), Vec::new(), vec![exact])
        .expect("the exact supplied identity is admitted");
    let projection = ConsumerRouteProjectionFixture::canonical();
    let composed = compose(&projection, &[&admitted]).expect("the projection composes");
    let row = composed
        .active_session_state()
        .rows()
        .next()
        .expect("the observed row survives");
    assert_eq!(
        row.source().kind(),
        ConsumerRouteProjectionSourceKind::ActiveSessionObservation,
        "composition preserves the exact evidence class the source was supplied under"
    );
}

/// Builds one post-open observation row attributed to the supplied source.
fn observation_row(source: ConsumerRouteProjectionSourceIdentity) -> ConsumerRouteProjectionRow {
    ConsumerRouteProjectionRow::new(
        ConsumerRouteRowIdentity::Feature(
            ConsumerRouteFeatureId::ActiveSessionReasoningAcknowledgement,
        ),
        ConsumerRouteProjectionFixture::canonical().applicability(),
        source,
        ConsumerRouteSourceClass::RouteAcknowledgementEvidence,
        ConsumerRouteEvidenceStrength::WireAcknowledgement,
        ConsumerRouteLifecycle::PostOpenObservationOnly,
    )
    .with_support(ConsumerRouteSupportPosture::Supported)
    .with_availability(ConsumerRouteAvailability::Available)
}

use swallowtail_core::SafeDiagnostic;
use swallowtail_runtime::{
    ConsumerRouteAvailabilityDimension, ConsumerRouteEnumerableValue,
    ConsumerRouteEnumeratedValues, ConsumerRouteLifecycle, ConsumerRouteNamespacedExtension,
    ConsumerRouteProjectionContribution, ConsumerRouteProjectionFailureKind,
    ConsumerRouteProjectionSourceId, ConsumerRouteProjectionSourceKind, ConsumerRouteSafeReason,
    MAX_CONSUMER_ROUTE_ACTIVE_SESSION_ROWS, MAX_CONSUMER_ROUTE_ENUMERABLE_VALUE_BYTES,
    MAX_CONSUMER_ROUTE_ENUMERABLE_VALUES, MAX_CONSUMER_ROUTE_EXTENSION_TEXT_BYTES,
    MAX_CONSUMER_ROUTE_NAMESPACED_EXTENSIONS, MAX_CONSUMER_ROUTE_SAFE_REASON_BYTES,
    MAX_CONSUMER_ROUTE_SELECTION_SUMMARY_ROWS, MAX_CONSUMER_ROUTE_SESSION_START_ROWS,
    MAX_CONSUMER_ROUTE_SOURCE_ID_BYTES, MAX_CONSUMER_ROUTE_SOURCE_IDENTITIES,
};

use crate::{ConsumerRouteProjectionFixture, consumer_route_projection_source};

use super::support::*;

pub(super) fn assert_fixed_maxima() {
    let fixture = ConsumerRouteProjectionFixture::canonical();
    let applicability = fixture.applicability();

    assert_kind(
        &contribution(
            &applicability,
            rows(
                &applicability,
                MAX_CONSUMER_ROUTE_SELECTION_SUMMARY_ROWS + 1,
                ConsumerRouteLifecycle::SelectionSummary,
            ),
            Vec::new(),
            Vec::new(),
        )
        .expect_err("selection summary row maximum is fixed"),
        ConsumerRouteProjectionFailureKind::LimitExceeded,
    );
    assert_kind(
        &contribution(
            &applicability,
            Vec::new(),
            rows(
                &applicability,
                MAX_CONSUMER_ROUTE_SESSION_START_ROWS + 1,
                ConsumerRouteLifecycle::SessionStartOnly,
            ),
            Vec::new(),
        )
        .expect_err("session start row maximum is fixed"),
        ConsumerRouteProjectionFailureKind::LimitExceeded,
    );
    assert_kind(
        &contribution(
            &applicability,
            Vec::new(),
            Vec::new(),
            rows(
                &applicability,
                MAX_CONSUMER_ROUTE_ACTIVE_SESSION_ROWS + 1,
                ConsumerRouteLifecycle::PostOpenObservationOnly,
            ),
        )
        .expect_err("active session row maximum is fixed"),
        ConsumerRouteProjectionFailureKind::LimitExceeded,
    );

    assert_kind(
        &ConsumerRouteEnumeratedValues::new((0..=MAX_CONSUMER_ROUTE_ENUMERABLE_VALUES).map(
            |index| {
                ConsumerRouteEnumerableValue::new(format!("value-{index}")).expect("value is valid")
            },
        ))
        .expect_err("enumerable value maximum is fixed"),
        ConsumerRouteProjectionFailureKind::LimitExceeded,
    );
    assert_kind(
        &ConsumerRouteEnumerableValue::new(
            "v".repeat(MAX_CONSUMER_ROUTE_ENUMERABLE_VALUE_BYTES + 1),
        )
        .expect_err("enumerable value byte maximum is fixed"),
        ConsumerRouteProjectionFailureKind::LimitExceeded,
    );
    assert!(
        ConsumerRouteEnumerableValue::new("v".repeat(MAX_CONSUMER_ROUTE_ENUMERABLE_VALUE_BYTES))
            .is_ok(),
        "the exact maximum is admitted rather than truncated"
    );

    assert_kind(
        &ConsumerRouteNamespacedExtension::new(
            "fixture.route",
            "fixture-version-1",
            "x".repeat(MAX_CONSUMER_ROUTE_EXTENSION_TEXT_BYTES + 1),
        )
        .expect_err("extension text byte maximum is fixed"),
        ConsumerRouteProjectionFailureKind::LimitExceeded,
    );
    assert_kind(
        &contribution(
            &applicability,
            (0..=MAX_CONSUMER_ROUTE_NAMESPACED_EXTENSIONS)
                .map(|index| {
                    feature_row(
                        &applicability,
                        namespaced_feature(index),
                        ConsumerRouteLifecycle::SelectionSummary,
                    )
                })
                .collect(),
            Vec::new(),
            Vec::new(),
        )
        .expect_err("namespaced extension maximum is fixed"),
        ConsumerRouteProjectionFailureKind::LimitExceeded,
    );

    assert_kind(
        &ConsumerRouteProjectionSourceId::new("s".repeat(MAX_CONSUMER_ROUTE_SOURCE_ID_BYTES + 1))
            .expect_err("source id byte maximum is fixed"),
        ConsumerRouteProjectionFailureKind::LimitExceeded,
    );
    assert_kind(
        &ConsumerRouteProjectionContribution::new(
            applicability.clone(),
            (0..=MAX_CONSUMER_ROUTE_SOURCE_IDENTITIES).map(|index| {
                consumer_route_projection_source(
                    &format!("fixture.source.{index}"),
                    ConsumerRouteProjectionSourceKind::AdapterContribution,
                )
            }),
            Vec::new(),
            Vec::new(),
            Vec::new(),
        )
        .expect_err("source identity maximum is fixed"),
        ConsumerRouteProjectionFailureKind::LimitExceeded,
    );

    assert_kind(
        &ConsumerRouteSafeReason::new(
            ConsumerRouteAvailabilityDimension::Credential,
            ConsumerRouteProjectionSourceId::new(ADAPTER_SOURCE).expect("source id is valid"),
            SafeDiagnostic::new(
                "fixture.reason",
                "r".repeat(MAX_CONSUMER_ROUTE_SAFE_REASON_BYTES + 1),
            ),
        )
        .expect_err("safe reason byte maximum is fixed"),
        ConsumerRouteProjectionFailureKind::SafeReasonLimitExceeded,
    );
}

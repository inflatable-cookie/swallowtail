use swallowtail_runtime::{
    ConsumerRouteProjectionContribution, ConsumerRouteProjectionFailureKind,
};

use super::fixtures::{inference_contribution, serving_contribution};
use super::naming::all_rows;

const SHARED: &str = "llama-cpp.shared-source";

fn assert_rejects(
    applicability: swallowtail_runtime::ConsumerRouteApplicability,
    mine: &ConsumerRouteProjectionContribution,
    row: swallowtail_runtime::ConsumerRouteProjectionRow,
) {
    let rejection = ConsumerRouteProjectionContribution::new(
        applicability,
        mine.sources().cloned().collect::<Vec<_>>(),
        [row],
        [],
        [],
    )
    .expect_err("a row proved under other evidence cannot join this snapshot");
    assert_eq!(
        rejection.kind(),
        ConsumerRouteProjectionFailureKind::ApplicabilityDisagreement
    );
}

#[test]
fn attached_and_owned_rows_cannot_join_under_a_matching_source_id() {
    let attached = inference_contribution(SHARED);
    let owned = serving_contribution(SHARED);
    assert_ne!(
        attached.applicability().driver_identity(),
        owned.applicability().driver_identity()
    );
    let attached_row = all_rows(&attached)
        .next()
        .expect("attached publishes rows")
        .clone();
    let owned_row = all_rows(&owned)
        .next()
        .expect("owned publishes rows")
        .clone();
    assert_rejects(owned.applicability().clone(), &owned, attached_row);
    assert_rejects(attached.applicability().clone(), &attached, owned_row);
}

use swallowtail_adapter_qoder::qoder_headless_descriptor;
use swallowtail_core::SupportAuthority;
use swallowtail_runtime::ConsumerRouteApplicability;

use super::claims::all_rows;
use super::fixtures::{contribution, drifted_observations, prepared, run_input};
use super::mixture::{
    SHARED_SOURCE, assert_access_is_the_only_difference, assert_rejects,
    assert_route_identity_is_the_only_difference, instance, neighbour_descriptor,
    neighbour_driver_id, plan_with, ready, rebind,
};
use super::naming::semantic_id;

/// Proves a portable row carrying another route's applicability fails closed.
///
/// Both snapshots name the same source id, so a matching source identity does
/// not let one route publish another route's row, in either direction.
#[test]
fn a_portable_row_carrying_another_route_applicability_is_rejected() {
    let integration = prepared("1");
    let mine = contribution(
        &integration
            .prepare_run(run_input("assembly"))
            .expect("session prepares"),
        SHARED_SOURCE,
    );
    let local = ConsumerRouteApplicability::from_plan(
        &plan_with(
            &qoder_headless_descriptor(),
            &instance(&integration, None),
            &integration,
            ready(&integration),
        )
        .expect("the rebuilt local plan is well formed"),
    );
    assert_eq!(
        &local,
        mine.applicability(),
        "the rebuilt plan must reproduce the exact prepared applicability"
    );

    let neighbour = ConsumerRouteApplicability::from_plan(
        &plan_with(
            &neighbour_descriptor(),
            &instance(&integration, Some(neighbour_driver_id())),
            &integration,
            ready(&integration),
        )
        .expect("the neighbour route plan is well formed"),
    );
    assert_route_identity_is_the_only_difference(&neighbour, mine.applicability());

    let borrowed = rebind(
        all_rows(&mine)
            .find(|row| semantic_id(row.identity()) == "feature.streaming-events")
            .expect("this route publishes streaming events"),
        neighbour.clone(),
    );
    assert_rejects(mine.applicability().clone(), &mine, borrowed);

    let ours = all_rows(&mine)
        .find(|row| semantic_id(row.identity()) == "feature.prepared-facade")
        .expect("this route publishes its prepared facade")
        .clone();
    assert_rejects(neighbour, &mine, ours);
}

/// Proves one changed access dimension rejects mixed assembly at the composer.
///
/// Each drifted snapshot agrees on route, instance, revision, operation, and
/// resource evidence, so the rejection comes from the exact access dimension
/// rather than an identity difference or an aggregate availability summary.
#[test]
fn a_changed_access_dimension_rejects_mixed_assembly_under_a_matching_source_id() {
    let integration = prepared("1");
    let mine = contribution(
        &integration
            .prepare_run(run_input("assembly-access"))
            .expect("session prepares"),
        SHARED_SOURCE,
    );
    let row = all_rows(&mine)
        .find(|row| semantic_id(row.identity()) == "feature.working-resource")
        .expect("this route publishes its working resource")
        .clone();
    let mut formed = 0;
    for observed in drifted_observations() {
        let Ok(plan) = plan_with(
            &qoder_headless_descriptor(),
            &instance(&integration, None),
            &integration,
            &observed,
        ) else {
            assert_eq!(
                observed.support_authority(),
                SupportAuthority::ExperimentalObserved,
                "only a drifted support authority may fail before a snapshot exists"
            );
            continue;
        };
        formed += 1;
        let shifted = ConsumerRouteApplicability::from_plan(&plan);
        assert_access_is_the_only_difference(&shifted, mine.applicability());
        assert_rejects(shifted, &mine, row.clone());
    }
    assert_eq!(
        formed, 4,
        "credential, entitlement, endpoint, and readiness drift each form one snapshot"
    );
}

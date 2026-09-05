use super::common;
use swallowtail_core::ExecutionHostId;
use swallowtail_runtime::{
    ConsumerRouteProjectionContribution, ConsumerRouteProjectionFailureKind,
    ConsumerRouteProjectionSourceId,
};

#[test]
fn mixed_prepared_hosts_fail_at_applicability_admission() {
    let first_run = common::prepare(ExecutionHostId::new("muse.first.host").unwrap())
        .prepare_run(common::run_input(common::model(), "medium"))
        .expect("first run");
    let second_run = common::prepare(ExecutionHostId::new("muse.second.host").unwrap())
        .prepare_run(common::run_input(common::model(), "medium"))
        .expect("second run");
    let first = first_run
        .consumer_route_projection_contribution(ConsumerRouteProjectionSourceId::new("first").unwrap())
        .expect("first contribution");
    let second = second_run
        .consumer_route_projection_contribution(ConsumerRouteProjectionSourceId::new("second").unwrap())
        .expect("second contribution");
    let borrowed = first.selection_rows().next().unwrap().clone();
    let rejection = ConsumerRouteProjectionContribution::new(
        second.applicability().clone(),
        second.sources().cloned().collect::<Vec<_>>(),
        [borrowed],
        [],
        [],
    )
    .expect_err("cross-host assembly must fail closed");
    assert_eq!(rejection.kind(), ConsumerRouteProjectionFailureKind::ApplicabilityDisagreement);
}

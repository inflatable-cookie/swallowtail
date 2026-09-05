use super::{prepare, run_input};
use swallowtail_adapter_mistral_vibe::MistralVibeMaxTurns;
use swallowtail_core::ExecutionHostId;
use swallowtail_runtime::{
    ConsumerRouteProjectionContribution, ConsumerRouteProjectionFailureKind,
    ConsumerRouteProjectionSourceId,
};

#[test]
fn mixed_prepared_hosts_fail_at_applicability_admission() {
    let first = prepare(ExecutionHostId::new("mistral-vibe.first.host").expect("host"))
        .prepare_run(run_input("first").with_max_turns(MistralVibeMaxTurns::try_new(4).unwrap()))
        .expect("first run");
    let second = prepare(ExecutionHostId::new("mistral-vibe.second.host").expect("host"))
        .prepare_run(run_input("second").with_max_turns(MistralVibeMaxTurns::try_new(4).unwrap()))
        .expect("second run");
    let first_contribution = first
        .consumer_route_projection_contribution(ConsumerRouteProjectionSourceId::new("first").unwrap())
        .expect("first contribution");
    let second_contribution = second
        .consumer_route_projection_contribution(ConsumerRouteProjectionSourceId::new("second").unwrap())
        .expect("second contribution");
    let borrowed = first_contribution.selection_rows().next().unwrap().clone();
    let rejection = ConsumerRouteProjectionContribution::new(
        second_contribution.applicability().clone(),
        second_contribution.sources().cloned().collect::<Vec<_>>(),
        [borrowed],
        [],
        [],
    )
    .expect_err("cross-host assembly must fail closed");
    assert_eq!(rejection.kind(), ConsumerRouteProjectionFailureKind::ApplicabilityDisagreement);
}

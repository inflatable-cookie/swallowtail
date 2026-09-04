use swallowtail_runtime::{
    ConsumerRouteProjectionContribution, ConsumerRouteProjectionFailureKind,
};

use super::fixtures::inference_contribution;
use super::naming::{all_rows, source};

#[test]
fn matching_source_cross_instance_rows_fail_closed() {
    let left = super::fixtures::attached_with("host.other", "1", super::fixtures::ready_status())
        .expect("left prepares")
        .prepare_inference_attempt(super::fixtures::inference_input("left"))
        .expect("left inference prepares")
        .consumer_route_projection_contribution(source("llama-cpp.assembly.shared"))
        .expect("left contributes");
    let right = inference_contribution("llama-cpp.assembly.shared");
    assert_eq!(
        left.sources().next().unwrap().id().as_str(),
        right.sources().next().unwrap().id().as_str()
    );
    let rejection = ConsumerRouteProjectionContribution::new(
        right.applicability().clone(),
        right.sources().cloned().collect::<Vec<_>>(),
        [all_rows(&left).next().expect("left publishes").clone()],
        [],
        [],
    )
    .expect_err("a row from another instance cannot join this snapshot");
    assert_eq!(
        rejection.kind(),
        ConsumerRouteProjectionFailureKind::ApplicabilityDisagreement
    );
}

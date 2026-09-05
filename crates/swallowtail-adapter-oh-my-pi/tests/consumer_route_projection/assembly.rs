use super::{image, model, prepared};
use swallowtail_runtime::{
    ConsumerRouteProjectionContribution, ConsumerRouteProjectionFailureKind,
    ConsumerRouteProjectionSourceId, Deadline, MonotonicInstant, OperationContent, RequestId,
    SessionOptions, WorkingResourceRef,
};
use swallowtail_adapter_oh_my_pi::{OhMyPiRunProfileInput, OhMyPiSessionProfileInput};

#[test]
fn mixed_structured_and_interactive_assembly_fails_at_applicability_admission() {
    let run = prepared()
        .prepare_run(
            OhMyPiRunProfileInput::new(
                RequestId::new("run").unwrap(), model("mixed.route"),
                OperationContent::new("prompt").unwrap(), WorkingResourceRef::new("workspace").unwrap(),
                Deadline::at(MonotonicInstant::from_ticks(1_000)),
            )
            .with_attachments([image("image")]),
        )
        .expect("run")
        .consumer_route_projection_contribution(ConsumerRouteProjectionSourceId::new("run").unwrap())
        .expect("run contribution");
    let session = prepared()
        .prepare_session(
            OhMyPiSessionProfileInput::new(
                RequestId::new("session").unwrap(), model("mixed.route"),
                WorkingResourceRef::new("workspace").unwrap(), SessionOptions::default(),
            )
            .with_image_attachments(),
        )
        .expect("session")
        .consumer_route_projection_contribution(ConsumerRouteProjectionSourceId::new("session").unwrap())
        .expect("session contribution");
    let borrowed = run.selection_rows().next().unwrap().clone();
    let rejection = ConsumerRouteProjectionContribution::new(
        session.applicability().clone(), session.sources().cloned().collect::<Vec<_>>(),
        [borrowed], [], [],
    )
    .expect_err("cross-operation assembly must fail closed");
    assert_eq!(rejection.kind(), ConsumerRouteProjectionFailureKind::ApplicabilityDisagreement);
}

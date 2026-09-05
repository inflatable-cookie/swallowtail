use super::{model, prepared};
use swallowtail_adapter_oh_my_pi::OhMyPiRunProfileInput;
use swallowtail_core::ReasoningMode;
use swallowtail_runtime::{
    ConsumerRouteFeatureId, ConsumerRouteRowIdentity, Deadline, MonotonicInstant,
    OperationContent, RequestId, WorkingResourceRef,
};

#[test]
fn attachment_rows_are_absent_when_the_consumer_did_not_mediate_them() {
    let run = prepared()
        .prepare_run(
            OhMyPiRunProfileInput::new(
                RequestId::new("no-attachments").unwrap(), model("negative.route"),
                OperationContent::new("prompt").unwrap(), WorkingResourceRef::new("workspace").unwrap(),
                Deadline::at(MonotonicInstant::from_ticks(1_000)),
            )
            .with_reasoning_mode(ReasoningMode::new("low").unwrap()),
        )
        .expect("run")
        .consumer_route_projection_contribution(
            swallowtail_runtime::ConsumerRouteProjectionSourceId::new("negative").unwrap(),
        )
        .expect("contribution");
    assert!(!run.selection_rows().any(|row| {
        row.identity() == &ConsumerRouteRowIdentity::Feature(ConsumerRouteFeatureId::Attachments)
    }));
    assert!(!run.session_start_rows().any(|row| {
        row.identity().namespaced_extension().is_some_and(|extension| extension.semantic_id() == "control.attachments")
    }));
    assert!(run.session_start_rows().all(|row| !row.mutation_authority().is_consumer_mediated_per_turn()));
}

use super::{FakeProcessService, PendingTimeService, host_services_for, preparation_input, probe};
use futures_executor::block_on;
use std::sync::Arc;
use swallowtail_adapter_qwen::{QwenModelSelection, QwenRunProfileInput, prepare_qwen_headless};
use swallowtail_core::{ExecutionHostId, ModelId, ModelRouteId, ModelRouteRevision, ProviderId};
use swallowtail_runtime::OperationContent;
use swallowtail_runtime::{ConsumerRouteFeatureId, ConsumerRouteRowIdentity, Deadline, MonotonicInstant, RequestId, WorkingResourceRef};

#[test]
fn unrequested_reasoning_and_harness_controls_are_absent() {
    let host_id = ExecutionHostId::new("qwen.negative.host").unwrap();
    let (process, _) = FakeProcessService::completed("0.19.11\n");
    let (services, _) = host_services_for(host_id.clone(), process, Arc::new(PendingTimeService));
    let prepared = block_on(prepare_qwen_headless(preparation_input(host_id), probe(), services)).unwrap();
    let run = prepared.prepare_run(QwenRunProfileInput::new(
        RequestId::new("negative").unwrap(),
        QwenModelSelection::new(
            ModelRouteId::new("qwen.negative.route").unwrap(), ModelRouteRevision::new("1").unwrap(),
            ProviderId::new("alibaba-modelstudio").unwrap(), ModelId::new("qwen3-coder-plus").unwrap(),
        ),
        OperationContent::new("prompt").unwrap(), WorkingResourceRef::new("workspace").unwrap(),
        Deadline::at(MonotonicInstant::from_ticks(1_000)),
    )).unwrap();
    let contribution = run.consumer_route_projection_contribution(
        swallowtail_runtime::ConsumerRouteProjectionSourceId::new("negative").unwrap(),
    ).unwrap();
    assert!(!contribution.selection_rows().any(|row| row.identity() == &ConsumerRouteRowIdentity::Feature(ConsumerRouteFeatureId::ReasoningSelection)));
    assert!(!contribution.session_start_rows().any(|row| row.identity().namespaced_extension().is_some_and(|extension| extension.semantic_id() == "control.harness-mode")));
    assert!(!contribution.selection_rows().any(|row| row.identity() == &ConsumerRouteRowIdentity::Feature(ConsumerRouteFeatureId::UsageEvidence)));
    assert!(contribution.session_start_rows().all(|row| !row.mutation_authority().is_consumer_mediated_per_turn()));
}

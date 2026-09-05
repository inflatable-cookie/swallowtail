use super::{FakeProcessService, PendingTimeService, host_services_for, preparation_input, probe};
use futures_executor::block_on;
use std::sync::Arc;
use swallowtail_adapter_qwen::{QwenModelSelection, QwenRunProfileInput, QwenSessionProfileInput, prepare_qwen_headless};
use swallowtail_core::{ExecutionHostId, ModelId, ModelRouteId, ModelRouteRevision, ProviderId};
use swallowtail_runtime::{ConsumerRouteProjectionContribution, ConsumerRouteProjectionFailureKind, ConsumerRouteProjectionSourceId, Deadline, MonotonicInstant, OperationContent, RequestId, WorkingResourceRef};

fn prepared(host: &str) -> swallowtail_adapter_qwen::QwenPreparedIntegration {
    let host_id = ExecutionHostId::new(host).unwrap();
    let (process, _) = FakeProcessService::completed("0.21.15\n");
    let (services, _) = host_services_for(host_id.clone(), process, Arc::new(PendingTimeService));
    block_on(prepare_qwen_headless(preparation_input(host_id), probe(), services)).unwrap()
}

fn model() -> QwenModelSelection {
    QwenModelSelection::new(
        ModelRouteId::new("qwen.mixed.route").unwrap(), ModelRouteRevision::new("1").unwrap(),
        ProviderId::new("alibaba-modelstudio").unwrap(), ModelId::new("qwen3-coder-plus").unwrap(),
    )
}

#[test]
fn mixed_structured_and_interactive_assembly_fails_at_applicability_admission() {
    let run = prepared("qwen.run.host")
        .prepare_run(QwenRunProfileInput::new(
            RequestId::new("run").unwrap(), model(), OperationContent::new("prompt").unwrap(),
            WorkingResourceRef::new("workspace").unwrap(), Deadline::at(MonotonicInstant::from_ticks(1_000)),
        ))
        .unwrap()
        .consumer_route_projection_contribution(ConsumerRouteProjectionSourceId::new("run").unwrap())
        .unwrap();
    let session = prepared("qwen.session.host")
        .prepare_session(QwenSessionProfileInput::new(
            RequestId::new("session").unwrap(), model(), WorkingResourceRef::new("workspace").unwrap(),
        ))
        .unwrap()
        .consumer_route_projection_contribution(ConsumerRouteProjectionSourceId::new("session").unwrap())
        .unwrap();
    let borrowed = run.selection_rows().next().unwrap().clone();
    let rejection = ConsumerRouteProjectionContribution::new(
        session.applicability().clone(), session.sources().cloned().collect::<Vec<_>>(),
        [borrowed], [], [],
    ).expect_err("cross-operation assembly must fail closed");
    assert_eq!(rejection.kind(), ConsumerRouteProjectionFailureKind::ApplicabilityDisagreement);
}

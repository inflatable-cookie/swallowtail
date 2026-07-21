use super::*;
use futures_executor::block_on;
use futures_util::StreamExt;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use support::{
    FakeProcessService, ImmediateTimeService, PendingTimeService, host_services_for, plan_for,
    request_for,
};
use swallowtail_adapter_qwen::QwenHeadlessDriver;
use swallowtail_core::HarnessIsolation;
use swallowtail_runtime::{
    CancellationAcknowledgement, CleanupOutcome, Deadline, EnvironmentRef, MonotonicInstant,
    OperationContent, OperationPolicy, ProcessExit, ProviderRetentionPolicy, RequestId,
    RuntimeEvent, StructuredRunDriver, StructuredRunRequest, TerminalOutcome, TerminalStatus,
};
use swallowtail_testkit::ExecutionTopologyFixture;

const ROOT: &str = "tests/fixtures/qwen-code-v0.19.11";

pub struct RunEvidence {
    pub events: Vec<RuntimeEvent>,
    pub outcome: TerminalOutcome,
}

pub fn completed(
    topology: &ExecutionTopologyFixture,
    output: &str,
    exit: ProcessExit,
    id: &str,
) -> RunEvidence {
    let (process, state) = FakeProcessService::with_exit(output, exit);
    let (services, task) = host_services_for(
        topology.execution_host_id().clone(),
        process,
        Arc::new(PendingTimeService),
    );
    let mut handle = block_on(driver().start_run(
        plan_for(topology),
        request_for(id, topology.working_resource().clone()),
        services,
    ))
    .expect("conformance run starts");
    assert!(handle.provider_run_ref().is_none());
    let events = block_on(
        handle
            .take_events()
            .expect("event stream is available")
            .collect::<Vec<_>>(),
    )
    .into_iter()
    .collect::<Result<Vec<_>, _>>()
    .expect("events are valid");
    let outcome = block_on(
        handle
            .take_terminal_outcome()
            .expect("terminal outcome is available"),
    );
    assert_eq!(block_on(handle.close()), CleanupOutcome::Clean);
    assert!(state.waited());
    assert!(task.joined());
    let observed = state.request();
    assert_eq!(
        observed.executable,
        topology.instance_target().as_host_value()
    );
    assert_eq!(
        observed.working_resource.as_deref(),
        Some(topology.working_resource().as_host_value())
    );
    assert_eq!(state.stdin(), b"fixture-private-prompt");
    assert!(state.stdin_closed());
    for forbidden in ["--sandbox", "--continue", "--resume", "--yolo"] {
        assert!(
            !observed
                .arguments
                .iter()
                .any(|argument| argument == forbidden)
        );
    }
    RunEvidence { events, outcome }
}

pub fn cancelled(topology: &ExecutionTopologyFixture) -> TerminalOutcome {
    let (process, state) = FakeProcessService::held_open();
    let (services, task) = host_services_for(
        topology.execution_host_id().clone(),
        process,
        Arc::new(PendingTimeService),
    );
    let mut handle = block_on(driver().start_run(
        plan_for(topology),
        request_for("cancel", topology.working_resource().clone()),
        services,
    ))
    .expect("cancellable run starts");
    assert_eq!(
        block_on(handle.cancellation().request()).expect("cancellation succeeds"),
        CancellationAcknowledgement::Requested
    );
    let outcome = block_on(
        handle
            .take_terminal_outcome()
            .expect("terminal outcome is available"),
    );
    assert_eq!(block_on(handle.close()), CleanupOutcome::Clean);
    assert!(state.force_stopped());
    assert!(state.waited());
    assert!(task.joined());
    outcome
}

pub fn timed_out(topology: &ExecutionTopologyFixture) -> TerminalOutcome {
    let (process, state) = FakeProcessService::held_open();
    let (services, task) = host_services_for(
        topology.execution_host_id().clone(),
        process,
        Arc::new(ImmediateTimeService),
    );
    let mut handle = block_on(driver().start_run(
        plan_for(topology),
        request_for("timeout", topology.working_resource().clone()),
        services,
    ))
    .expect("deadline-bound run starts");
    let outcome = block_on(
        handle
            .take_terminal_outcome()
            .expect("terminal outcome is available"),
    );
    assert_eq!(block_on(handle.close()), CleanupOutcome::Clean);
    assert!(state.force_stopped());
    assert!(state.waited());
    assert!(task.joined());
    outcome
}

pub fn isolation_rejected(topology: &ExecutionTopologyFixture) {
    for isolation in [
        HarnessIsolation::ProviderEnforced,
        HarnessIsolation::HostEnforced,
    ] {
        let (process, state) = FakeProcessService::completed("");
        let (services, _task) = host_services_for(
            topology.execution_host_id().clone(),
            process,
            Arc::new(PendingTimeService),
        );
        let request = StructuredRunRequest::new(
            RequestId::new(format!("isolation-{isolation:?}")).expect("request id is valid"),
            OperationContent::new("fixture-private-prompt").expect("content is valid"),
            OperationPolicy::offline()
                .with_provider_retention(ProviderRetentionPolicy::DurableAllowed)
                .with_harness_isolation(isolation),
        )
        .with_working_resource(topology.working_resource().clone())
        .with_deadline(Deadline::at(MonotonicInstant::from_ticks(1_000)));
        let result = block_on(driver().start_run(plan_for(topology), request, services));
        assert!(result.is_err());
        assert!(!state.started());
    }
}

pub fn assert_status_code(terminal: &TerminalOutcome, expected: &str, provider: bool) {
    let diagnostic = match terminal.status() {
        TerminalStatus::ProviderFailed(diagnostic) if provider => diagnostic,
        TerminalStatus::RuntimeFailed(diagnostic) if !provider => diagnostic,
        status => panic!("unexpected terminal status {status:?}"),
    };
    assert_eq!(diagnostic.code(), expected);
}

pub fn fixture(name: &str) -> String {
    std::fs::read_to_string(path(name))
        .unwrap_or_else(|error| panic!("failed to read {name}: {error}"))
}

fn path(name: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join(ROOT).join(name)
}

fn driver() -> QwenHeadlessDriver {
    QwenHeadlessDriver::new(
        EnvironmentRef::new("qwen-saved-environment").expect("environment is valid"),
    )
}

#[path = "driver/failures.rs"]
mod failures;
mod support;

use futures_executor::block_on;
use futures_util::StreamExt;
use serde_json::Value;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use support::{
    FakeProcessService, ImmediateTimeService, PendingTimeService, ProcessState, host_services,
    plan, request,
};
use swallowtail_adapter_qwen::QwenHeadlessDriver;
use swallowtail_core::HarnessIsolation;
use swallowtail_runtime::{
    CancellationAcknowledgement, CleanupOutcome, EnvironmentRef, OperationPolicy, ProcessExit,
    ProviderObservation, ProviderRetentionPolicy, RuntimeEvent, RuntimeEventKind,
    StructuredRunDriver, TerminalOutcome, TerminalStatus,
};

const ROOT: &str = "tests/fixtures/qwen-code-v0.19.11";

#[test]
fn success_uses_the_frozen_invocation_and_normalizes_output_and_usage() {
    let (process, state) = FakeProcessService::completed(&fixture("success.jsonl"));
    let (events, terminal, cleanup) = run_completed(process, Arc::clone(&state), "success");

    assert_eq!(terminal.status(), &TerminalStatus::Completed);
    assert_eq!(
        terminal.output().map(|value| value.as_str()),
        Some("fixture answer")
    );
    assert_eq!(cleanup, CleanupOutcome::Clean);
    let expected = protocol_arguments();
    let observed = state.request();
    assert_eq!(observed.executable, "qwen-executable");
    assert_eq!(observed.arguments, expected);
    assert_eq!(observed.environments, ["qwen-saved-environment"]);
    assert_eq!(observed.working_resource.as_deref(), Some("workspace.main"));
    assert_eq!(state.stdin(), b"fixture-private-prompt");
    assert!(state.stdin_closed());
    assert!(state.waited());
    assert!(events.iter().any(|event| {
        matches!(
            event.kind(),
            RuntimeEventKind::ProviderObservation(ProviderObservation::Usage(usage))
                if usage.input_tokens() == Some(12) && usage.output_tokens() == Some(2)
        )
    }));
    assert!(events.iter().any(|event| {
        event.kind() == &RuntimeEventKind::OutputAvailable
            && event
                .content()
                .is_some_and(|value| value.as_str() == "fixture answer")
    }));
    assert!(!observed.arguments.iter().any(|argument| {
        argument.contains("fixture-private-prompt")
            || argument == "--sandbox"
            || argument == "--yolo"
    }));
}

#[test]
fn cancellation_force_stops_waits_and_joins_the_process_task() {
    let (process, state) = FakeProcessService::held_open();
    let mut handle = block_on(driver().start_run(
        plan(),
        request("cancel"),
        host_services(process, Arc::new(PendingTimeService)),
    ))
    .expect("run starts");
    assert_eq!(
        block_on(handle.cancellation().request()).expect("cancellation succeeds"),
        CancellationAcknowledgement::Requested
    );
    assert_eq!(
        block_on(handle.cancellation().request()).expect("repeat cancellation succeeds"),
        CancellationAcknowledgement::AlreadyRequested
    );
    let terminal = block_on(
        handle
            .take_terminal_outcome()
            .expect("terminal outcome is available"),
    );

    assert_eq!(terminal.status(), &TerminalStatus::Cancelled);
    assert_eq!(block_on(handle.close()), CleanupOutcome::Clean);
    assert!(state.force_stopped());
    assert!(state.waited());
}

#[test]
fn host_deadline_force_stops_waits_and_reports_timeout() {
    let (process, state) = FakeProcessService::held_open();
    let mut handle = block_on(driver().start_run(
        plan(),
        request("timeout"),
        host_services(process, Arc::new(ImmediateTimeService)),
    ))
    .expect("run starts");
    let terminal = block_on(
        handle
            .take_terminal_outcome()
            .expect("terminal outcome is available"),
    );

    assert_eq!(terminal.status(), &TerminalStatus::TimedOut);
    assert_eq!(block_on(handle.close()), CleanupOutcome::Clean);
    assert!(state.force_stopped());
    assert!(state.waited());
}

#[test]
fn invalid_lifecycle_and_isolation_fail_before_process_start() {
    let cases = [
        request("missing-deadline").without_deadline_for_test(),
        swallowtail_runtime::StructuredRunRequest::new(
            swallowtail_runtime::RequestId::new("retention-prohibited")
                .expect("request id is valid"),
            swallowtail_runtime::OperationContent::new("fixture-private-prompt")
                .expect("content is valid"),
            OperationPolicy::offline().with_harness_isolation(HarnessIsolation::AmbientHost),
        )
        .with_working_resource(support::working_resource())
        .with_deadline(swallowtail_runtime::Deadline::at(
            swallowtail_runtime::MonotonicInstant::from_ticks(1_000),
        )),
        swallowtail_runtime::StructuredRunRequest::new(
            swallowtail_runtime::RequestId::new("wrong-isolation").expect("request id is valid"),
            swallowtail_runtime::OperationContent::new("fixture-private-prompt")
                .expect("content is valid"),
            OperationPolicy::offline()
                .with_provider_retention(ProviderRetentionPolicy::DurableAllowed)
                .with_harness_isolation(HarnessIsolation::HostEnforced),
        )
        .with_working_resource(support::working_resource())
        .with_deadline(swallowtail_runtime::Deadline::at(
            swallowtail_runtime::MonotonicInstant::from_ticks(1_000),
        )),
    ];

    for request in cases {
        let (process, state) = FakeProcessService::completed("");
        let result = block_on(driver().start_run(
            plan(),
            request,
            host_services(process, Arc::new(PendingTimeService)),
        ));
        assert!(result.is_err());
        assert!(!state.started());
    }
}

fn run_completed(
    process: Arc<FakeProcessService>,
    _state: Arc<ProcessState>,
    id: &str,
) -> (Vec<RuntimeEvent>, TerminalOutcome, CleanupOutcome) {
    let mut handle = block_on(driver().start_run(
        plan(),
        request(id),
        host_services(process, Arc::new(PendingTimeService)),
    ))
    .expect("run starts");
    let events = block_on(
        handle
            .take_events()
            .expect("event stream is available")
            .collect::<Vec<_>>(),
    )
    .into_iter()
    .collect::<Result<Vec<_>, _>>()
    .expect("events are valid");
    let terminal = block_on(
        handle
            .take_terminal_outcome()
            .expect("terminal outcome is available"),
    );
    let cleanup = block_on(handle.close());
    (events, terminal, cleanup)
}

fn driver() -> QwenHeadlessDriver {
    QwenHeadlessDriver::new(
        EnvironmentRef::new("qwen-saved-environment").expect("environment is valid"),
    )
}

fn fixture(name: &str) -> String {
    std::fs::read_to_string(path(name))
        .unwrap_or_else(|error| panic!("failed to read {name}: {error}"))
}

fn path(name: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join(ROOT).join(name)
}

fn protocol_arguments() -> Vec<String> {
    let protocol: Value =
        serde_json::from_str(&fixture("protocol.json")).expect("protocol fixture is valid");
    protocol["invocation"]["arguments"]
        .as_array()
        .expect("arguments are an array")
        .iter()
        .map(|value| {
            value
                .as_str()
                .expect("argument is text")
                .replace("{model_id}", "qwen3-coder-plus")
        })
        .collect()
}

fn assert_status_code(terminal: &TerminalOutcome, expected: &str, provider: bool) {
    let diagnostic = match terminal.status() {
        TerminalStatus::ProviderFailed(diagnostic) if provider => diagnostic,
        TerminalStatus::RuntimeFailed(diagnostic) if !provider => diagnostic,
        status => panic!("unexpected terminal status {status:?}"),
    };
    assert_eq!(diagnostic.code(), expected);
}

trait WithoutDeadlineForTest {
    fn without_deadline_for_test(self) -> Self;
}

impl WithoutDeadlineForTest for swallowtail_runtime::StructuredRunRequest {
    fn without_deadline_for_test(self) -> Self {
        swallowtail_runtime::StructuredRunRequest::new(
            self.request_id().clone(),
            self.content().clone(),
            self.policy().clone(),
        )
        .with_working_resource(
            self.working_resource()
                .cloned()
                .expect("fixture working resource is present"),
        )
    }
}

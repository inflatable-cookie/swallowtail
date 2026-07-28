use super::{
    FakeProcessService, ImmediateTimeService, PendingTimeService, host_services_for, plan_for,
    request_for, session_id,
};
use futures_executor::block_on;
use futures_util::StreamExt;
use std::sync::Arc;
use swallowtail_adapter_gemini::GeminiHeadlessDriver;
use swallowtail_runtime::{
    CancellationAcknowledgement, CleanupOutcome, EnvironmentRef, ProcessExit, RuntimeEvent,
    StructuredRunDriver, TerminalOutcome, TerminalStatus,
};
use swallowtail_testkit::ExecutionTopologyFixture;

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
    assert_eq!(
        handle
            .provider_run_ref()
            .map(|value| value.as_provider_value()),
        Some(session_id(id).as_str())
    );
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
    assert_eq!(
        observed.arguments,
        [
            "--output-format",
            "stream-json",
            "--model",
            "gemini-2.5-flash",
            "--approval-mode",
            "plan",
            "--extensions",
            "none",
            "--allowed-mcp-server-names",
            "",
            "--skip-trust",
            "--session-id",
            &session_id(id),
        ]
    );
    for forbidden in ["--sandbox", "--yolo", "--resume", "--acp"] {
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
        request_for("gemini-cancel", topology.working_resource().clone()),
        services,
    ))
    .expect("cancellable run starts");
    assert_eq!(
        block_on(handle.cancellation().request()).expect("cancellation succeeds"),
        CancellationAcknowledgement::Requested
    );
    assert_eq!(
        block_on(handle.cancellation().request()).expect("repeat cancellation succeeds"),
        CancellationAcknowledgement::AlreadyRequested
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
        request_for("gemini-timeout", topology.working_resource().clone()),
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

pub fn driver() -> GeminiHeadlessDriver {
    GeminiHeadlessDriver::new(
        EnvironmentRef::new("gemini.fixture.environment").expect("environment is valid"),
        swallowtail_core::CredentialRef::new("gemini.fixture.api-key")
            .expect("credential is valid"),
    )
}

pub fn assert_status_code(outcome: &TerminalOutcome, expected: &str, provider: bool) {
    let diagnostic = match outcome.status() {
        TerminalStatus::ProviderFailed(diagnostic) if provider => diagnostic,
        TerminalStatus::RuntimeFailed(diagnostic) if !provider => diagnostic,
        status => panic!("unexpected terminal status {status:?}"),
    };
    assert_eq!(diagnostic.code(), expected);
}

pub fn assert_redacted(events: &[RuntimeEvent], outcome: &TerminalOutcome) {
    let public = format!("{events:?}{outcome:?}");
    for private in [
        "fixture-private-prompt",
        "fixture-private-workspace",
        "fixture-provider-secret-never-diagnose",
        "fixture answer",
    ] {
        assert!(!public.contains(private));
    }
}

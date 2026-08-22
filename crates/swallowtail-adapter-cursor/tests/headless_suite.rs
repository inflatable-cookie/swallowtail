mod plan;
mod support;

use futures_executor::block_on;
use futures_util::StreamExt;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use support::{FixtureHost, ImmediateTime};
use swallowtail_adapter_cursor::CursorHeadlessDriver;
use swallowtail_core::{
    ExecutionHostId, HarnessConfigurationPosture, HarnessIsolation, ReasoningMode, ResourceAccess,
};
use swallowtail_runtime::{
    ActivityKind, CancellationAcknowledgement, CleanupOutcome, Deadline, EnvironmentRef,
    MonotonicInstant, OperationContent, OperationPolicy, ProcessExit, ProcessOutputChunk,
    ProcessOutputStream, ProviderObservation, ProviderRetentionPolicy, RequestId, RuntimeEvent,
    RuntimeEventKind, StructuredRunDriver, StructuredRunRequest, TerminalOutcome, TerminalStatus,
    WorkingResourceRef,
};

const FIXTURE: &str = "tests/fixtures/cursor-agent-2026.07.01-41b2de7/headless-success.jsonl";

#[test]
fn read_only_run_uses_plan_mode_and_projects_bounded_activity_and_usage() {
    let host_id = local_host();
    let host = FixtureHost::completed([stdout(&fixture())]);
    let (events, terminal, cleanup) = completed_run(
        &host,
        plan::headless_plan(
            host_id.clone(),
            "cursor.fixture.executable",
            ResourceAccess::Read,
        ),
        request("read-success"),
        host.services(host_id),
    );

    assert_eq!(terminal.status(), &TerminalStatus::Completed);
    assert_eq!(
        terminal.output().map(OperationContent::as_str),
        Some("Part done.")
    );
    assert_eq!(cleanup, CleanupOutcome::Clean);
    let observed = host.observed();
    assert_eq!(observed.executable, "cursor.fixture.executable");
    assert_eq!(
        observed.arguments,
        [
            "--print",
            "--output-format",
            "stream-json",
            "--model",
            "fixture-model",
            "--trust",
            "--mode",
            "plan",
        ]
    );
    assert_eq!(observed.environments, ["cursor.fixture.environment"]);
    assert_eq!(observed.working_resource.as_deref(), Some("workspace.main"));
    assert_eq!(host.stdin(), b"fixture-private-prompt");
    assert!(host.stdin_closed());
    assert!(host.waited());
    assert!(host.joined());
    assert!(events.iter().any(|event| {
        matches!(
            event.kind(),
            RuntimeEventKind::ProviderObservation(ProviderObservation::Usage(usage))
                if usage.input_tokens() == Some(12)
                    && usage.output_tokens() == Some(3)
                    && usage.cache_read_input_tokens() == Some(2)
                    && usage.cache_write_input_tokens() == Some(1)
        )
    }));
    assert!(events.iter().any(|event| {
        matches!(
            event.kind(),
            RuntimeEventKind::Activity(activity)
                if activity.kind() == &ActivityKind::ReasoningSummary
        )
    }));
    assert!(events.iter().any(|event| {
        matches!(
            event.kind(),
            RuntimeEventKind::Activity(activity)
                if activity.kind() == &ActivityKind::ProviderOwnedTool
                    && activity.provider_activity_ref().is_some_and(|value| value.as_provider_value() == "fixture-call-1")
        )
    }));
    let public = format!("{events:?}{terminal:?}");
    assert!(!public.contains("private-fixture-path"));
    assert!(!public.contains("private-provider-tool-result"));
    assert!(!public.contains("fixture-private-prompt"));
}

#[test]
fn write_run_omits_plan_mode_without_selecting_force_or_sandbox() {
    let host_id = local_host();
    let host = FixtureHost::completed([stdout(&fixture())]);
    let (_, terminal, cleanup) = completed_run(
        &host,
        plan::headless_plan(
            host_id.clone(),
            "cursor.fixture.executable",
            ResourceAccess::ReadWrite,
        ),
        request("write-success"),
        host.services(host_id),
    );

    assert_eq!(terminal.status(), &TerminalStatus::Completed);
    assert_eq!(cleanup, CleanupOutcome::Clean);
    let arguments = host.observed().arguments;
    assert!(!arguments.iter().any(|value| value == "--mode"));
    for rejected in ["--force", "--yolo", "--sandbox", "--stream-partial-output"] {
        assert!(!arguments.iter().any(|value| value == rejected));
    }
}

#[test]
fn cancellation_and_deadline_force_stop_wait_and_join() {
    let host_id = local_host();
    let cancelled = FixtureHost::held_open();
    let mut handle = block_on(driver().start_run(
        plan::headless_plan(
            host_id.clone(),
            "cursor.fixture.executable",
            ResourceAccess::Read,
        ),
        request("cancelled"),
        cancelled.services(host_id.clone()),
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
    let terminal = block_on(handle.take_terminal_outcome().expect("terminal future"));
    assert_eq!(terminal.status(), &TerminalStatus::Cancelled);
    assert_eq!(block_on(handle.close()), CleanupOutcome::Clean);
    assert!(cancelled.force_stopped());
    assert!(cancelled.waited());
    assert!(cancelled.joined());

    let timed_out = FixtureHost::held_open();
    let mut handle = block_on(driver().start_run(
        plan::headless_plan(
            host_id.clone(),
            "cursor.fixture.executable",
            ResourceAccess::Read,
        ),
        request("timed-out"),
        timed_out.services_with_time(host_id, Arc::new(ImmediateTime)),
    ))
    .expect("run starts");
    let terminal = block_on(handle.take_terminal_outcome().expect("terminal future"));
    assert_eq!(terminal.status(), &TerminalStatus::TimedOut);
    assert_eq!(block_on(handle.close()), CleanupOutcome::Clean);
    assert!(timed_out.force_stopped());
    assert!(timed_out.waited());
    assert!(timed_out.joined());
}

#[test]
fn nonzero_exit_and_malformed_stream_are_typed_and_sanitized() {
    let host_id = local_host();
    let failed = FixtureHost::with_exit(
        [stderr("provider-private-stderr")],
        ProcessExit::new(false, Some(7)),
    );
    let (_, terminal, cleanup) = completed_run(
        &failed,
        plan::headless_plan(
            host_id.clone(),
            "cursor.fixture.executable",
            ResourceAccess::Read,
        ),
        request("provider-failure"),
        failed.services(host_id.clone()),
    );
    assert_status_code(
        &terminal,
        "swallowtail.cursor.headless.process_failed",
        true,
    );
    assert_eq!(cleanup, CleanupOutcome::Clean);
    assert!(!format!("{terminal:?}").contains("provider-private-stderr"));

    let malformed = FixtureHost::completed([stdout(
        "{\"type\":\"system\",\"subtype\":\"init\",\"session_id\":\"private-session\"}\n",
    )]);
    let (_, terminal, cleanup) = completed_run(
        &malformed,
        plan::headless_plan(
            host_id.clone(),
            "cursor.fixture.executable",
            ResourceAccess::Read,
        ),
        request("malformed"),
        malformed.services(host_id),
    );
    assert_status_code(
        &terminal,
        "swallowtail.cursor.headless.malformed_stream",
        false,
    );
    assert_eq!(cleanup, CleanupOutcome::Clean);
    assert!(!format!("{terminal:?}").contains("private-session"));
}

#[test]
fn local_and_remote_authoritative_hosts_use_the_same_explicit_route() {
    for (host_value, target) in [
        ("host.local", "cursor.local.executable"),
        ("host.remote", "cursor.remote.executable"),
    ] {
        let host_id = ExecutionHostId::new(host_value).expect("host id");
        let host = FixtureHost::completed([stdout(&fixture())]);
        let (_, terminal, cleanup) = completed_run(
            &host,
            plan::headless_plan(host_id.clone(), target, ResourceAccess::Read),
            request(host_value),
            host.services(host_id),
        );
        assert_eq!(terminal.status(), &TerminalStatus::Completed);
        assert_eq!(cleanup, CleanupOutcome::Clean);
        assert_eq!(host.observed().executable, target);
    }
}

#[test]
fn low_level_driver_rejects_unqualified_parameterized_plans_before_process_work() {
    let host_id = local_host();
    for (model, reasoning, request_id) in [
        ("composer-2.5[fast=true]", None, "bad-fast"),
        (
            "claude-opus-5[effort=high,context=300k]",
            Some("high"),
            "bad-order",
        ),
        (
            "claude-opus-5[context=300k,effort=high,effort=high]",
            Some("high"),
            "duplicate-effort",
        ),
    ] {
        let host = FixtureHost::completed([stdout(&fixture())]);
        let err = match block_on(driver().start_run(
            plan::headless_plan_with_model(
                host_id.clone(),
                "cursor.fixture.executable",
                ResourceAccess::ReadWrite,
                model,
                reasoning,
            ),
            parameterized_request(request_id, reasoning),
            host.services(host_id.clone()),
        )) {
            Err(error) => error,
            Ok(_) => panic!("expected failure for {model}"),
        };
        assert_eq!(
            err.diagnostic().code(),
            "swallowtail.cursor.headless.model_parameter_rejected",
            "{model}"
        );
        assert!(!host.started(), "{model}");
    }
}

#[test]
fn low_level_driver_rejects_effort_mismatch_before_process_work() {
    let host_id = local_host();
    let host = FixtureHost::completed([stdout(&fixture())]);
    let model = "claude-opus-5[context=300k,effort=high]";
    let err = match block_on(driver().start_run(
        plan::headless_plan_with_model(
            host_id.clone(),
            "cursor.fixture.executable",
            ResourceAccess::ReadWrite,
            model,
            Some("high"),
        ),
        parameterized_request("effort-mismatch", Some("low")),
        host.services(host_id),
    )) {
        Err(error) => error,
        Ok(_) => panic!("expected effort mismatch failure"),
    };
    assert_eq!(
        err.diagnostic().code(),
        "swallowtail.cursor.headless.request_plan_mismatch"
    );
    assert!(!host.started());
}

fn completed_run(
    _host: &FixtureHost,
    plan: swallowtail_core::PreflightPlan,
    request: StructuredRunRequest,
    services: swallowtail_runtime::HostServices,
) -> (Vec<RuntimeEvent>, TerminalOutcome, CleanupOutcome) {
    let mut handle = block_on(driver().start_run(plan, request, services)).expect("run starts");
    let events = block_on(
        handle
            .take_events()
            .expect("event stream")
            .collect::<Vec<_>>(),
    )
    .into_iter()
    .collect::<Result<Vec<_>, _>>()
    .expect("events parse");
    let terminal = block_on(handle.take_terminal_outcome().expect("terminal future"));
    let cleanup = block_on(handle.close());
    (events, terminal, cleanup)
}

fn driver() -> CursorHeadlessDriver {
    CursorHeadlessDriver::new(
        EnvironmentRef::new("cursor.fixture.environment").expect("environment"),
    )
}

fn request(id: &str) -> StructuredRunRequest {
    parameterized_request(id, None)
}

fn parameterized_request(id: &str, reasoning: Option<&str>) -> StructuredRunRequest {
    let mut policy = OperationPolicy::offline()
        .with_provider_retention(ProviderRetentionPolicy::DurableAllowed)
        .with_harness_isolation(HarnessIsolation::AmbientHost)
        .with_harness_configuration_posture(HarnessConfigurationPosture::Ambient);
    if let Some(reasoning) = reasoning {
        policy = policy.with_reasoning_mode(ReasoningMode::new(reasoning).expect("reasoning"));
    }
    StructuredRunRequest::new(
        RequestId::new(id).expect("request id"),
        OperationContent::new("fixture-private-prompt").expect("prompt"),
        policy,
    )
    .with_working_resource(WorkingResourceRef::new("workspace.main").expect("resource"))
    .with_deadline(Deadline::at(MonotonicInstant::from_ticks(1_000)))
}

fn fixture() -> String {
    std::fs::read_to_string(fixture_path()).expect("headless fixture reads")
}

fn fixture_path() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join(FIXTURE)
}

fn stdout(value: &str) -> ProcessOutputChunk {
    ProcessOutputChunk::new(ProcessOutputStream::Stdout, value.as_bytes().to_vec())
}

fn stderr(value: &str) -> ProcessOutputChunk {
    ProcessOutputChunk::new(ProcessOutputStream::Stderr, value.as_bytes().to_vec())
}

fn local_host() -> ExecutionHostId {
    ExecutionHostId::new("host.local").expect("host id")
}

fn assert_status_code(terminal: &TerminalOutcome, expected: &str, provider: bool) {
    let diagnostic = match terminal.status() {
        TerminalStatus::ProviderFailed(diagnostic) if provider => diagnostic,
        TerminalStatus::RuntimeFailed(diagnostic) if !provider => diagnostic,
        status => panic!("unexpected terminal status {status:?}"),
    };
    assert_eq!(diagnostic.code(), expected);
}

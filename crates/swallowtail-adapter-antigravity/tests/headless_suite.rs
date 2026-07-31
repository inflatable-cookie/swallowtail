mod plan;
mod support;

use futures_executor::block_on;
use futures_util::StreamExt;
use std::sync::Arc;
use support::{FixtureHost, ImmediateTime};
use swallowtail_adapter_antigravity::AntigravityHeadlessDriver;
use swallowtail_core::{
    ExecutionHostId, HarnessConfigurationPosture, HarnessIsolation, ReasoningMode, ResourceAccess,
};
use swallowtail_runtime::{
    ActivityKind, CancellationAcknowledgement, CleanupOutcome, Deadline, EnvironmentRef,
    MonotonicInstant, OperationContent, OperationPolicy, ProcessExit, ProcessOutputChunk,
    ProcessOutputStream, ProviderObservation, ProviderRetentionPolicy, RequestId, RuntimeEvent,
    RuntimeEventKind, SchemaDocument, StructuredOutputDescriptor, StructuredRunDriver,
    StructuredRunRequest, TerminalOutcome, TerminalStatus, WorkingResourceRef,
};

const SUCCESS: &str = include_str!("fixtures/antigravity-cli-1.1.9/headless-success.jsonl");
const STRUCTURED: &str = include_str!("fixtures/antigravity-cli-1.1.9/headless-structured.jsonl");
const INVALID_MODEL: &str =
    include_str!("fixtures/antigravity-cli-1.1.9/headless-invalid-model.jsonl");
const SCHEMA: &str =
    r#"{"type":"object","properties":{"answer":{"type":"string"}},"required":["answer"]}"#;

#[test]
fn ambient_read_run_projects_steps_subagents_and_exact_usage() {
    let host_id = local_host();
    let host = FixtureHost::completed([stdout(SUCCESS)]);
    let (events, terminal, cleanup) = completed_run(
        plan::headless_plan(
            host_id.clone(),
            "antigravity.fixture.executable",
            ResourceAccess::Read,
            HarnessIsolation::AmbientHost,
            Some("high"),
            false,
        ),
        request(
            "ambient-read",
            HarnessIsolation::AmbientHost,
            Some("high"),
            false,
        ),
        host.services(host_id),
    );

    assert_eq!(terminal.status(), &TerminalStatus::Completed);
    assert_eq!(
        terminal.output().map(OperationContent::as_str),
        Some("Done.")
    );
    assert_eq!(cleanup, CleanupOutcome::Clean);
    let observed = host.observed();
    assert_eq!(observed.executable, "antigravity.fixture.executable");
    assert!(
        observed
            .arguments
            .windows(2)
            .any(|pair| pair == ["--print", "fixture-private-prompt"])
    );
    assert!(
        observed
            .arguments
            .windows(2)
            .any(|pair| pair == ["--mode", "plan"])
    );
    assert!(
        observed
            .arguments
            .windows(2)
            .any(|pair| pair == ["--effort", "high"])
    );
    assert!(!observed.arguments.iter().any(|value| value == "--sandbox"));
    assert!(
        !observed
            .arguments
            .iter()
            .any(|value| value == "--dangerously-skip-permissions")
    );
    assert_eq!(observed.working_resource.as_deref(), Some("workspace.main"));
    assert!(host.stdin_closed());
    assert!(host.waited());
    assert!(host.joined());

    assert!(events.iter().any(|event| {
        matches!(
            event.kind(),
            RuntimeEventKind::ProviderObservation(ProviderObservation::Usage(usage))
                if usage.input_tokens() == Some(12)
                    && usage.output_tokens() == Some(5)
                    && usage.reasoning_tokens() == Some(2)
                    && usage.cache_read_input_tokens() == Some(3)
        )
    }));
    assert!(events.iter().any(|event| {
        matches!(
            event.kind(),
            RuntimeEventKind::Activity(activity)
                if activity.kind() == &ActivityKind::ProviderOwnedTool
                    && activity.label().is_some_and(|label| label.as_str() == "run_command")
        )
    }));
    assert!(events.iter().any(|event| {
        matches!(
            event.kind(),
            RuntimeEventKind::Activity(activity)
                if activity.kind() == &ActivityKind::SubagentOrCollaboration
                    && activity.subagents().len() == 1
        )
    }));
    let public = format!("{events:?}{terminal:?}");
    for private in [
        "/private/workspace",
        "/private/secret",
        "private-tool-output",
        "private/child.log",
        "fixture-private-prompt",
    ] {
        assert!(!public.contains(private));
    }
}

#[test]
fn provider_sandbox_write_run_binds_schema_without_permission_bypass() {
    let host_id = local_host();
    let host = FixtureHost::completed([stdout(STRUCTURED)]);
    let (_, terminal, cleanup) = completed_run(
        plan::headless_plan(
            host_id.clone(),
            "antigravity.fixture.executable",
            ResourceAccess::ReadWrite,
            HarnessIsolation::ProviderEnforced,
            None,
            true,
        ),
        request(
            "sandboxed-structured",
            HarnessIsolation::ProviderEnforced,
            None,
            true,
        ),
        host.services(host_id),
    );

    assert_eq!(terminal.status(), &TerminalStatus::Completed);
    assert_eq!(
        terminal.output().map(OperationContent::as_str),
        Some(r#"{"answer":"yes"}"#)
    );
    assert_eq!(cleanup, CleanupOutcome::Clean);
    let arguments = host.observed().arguments;
    assert!(arguments.iter().any(|value| value == "--sandbox"));
    assert!(
        arguments
            .windows(2)
            .any(|pair| pair == ["--json-schema", SCHEMA])
    );
    assert!(!arguments.iter().any(|value| value == "--mode"));
    assert!(
        !arguments
            .iter()
            .any(|value| value == "--dangerously-skip-permissions")
    );
}

#[test]
fn invalid_model_and_permission_bypass_streams_are_typed_without_raw_payloads() {
    let host_id = local_host();
    let invalid = FixtureHost::with_exit([stdout(INVALID_MODEL)], ProcessExit::new(false, Some(1)));
    let (_, terminal, cleanup) = completed_run(
        plan::headless_plan(
            host_id.clone(),
            "antigravity.fixture.executable",
            ResourceAccess::Read,
            HarnessIsolation::AmbientHost,
            None,
            false,
        ),
        request("invalid-model", HarnessIsolation::AmbientHost, None, false),
        invalid.services(host_id.clone()),
    );
    assert_status_code(
        &terminal,
        "swallowtail.antigravity.headless.invalid_model",
        true,
    );
    assert_eq!(cleanup, CleanupOutcome::Clean);
    assert!(!format!("{terminal:?}").contains("private available model list"));

    let bypass = SUCCESS.replace("request-review", "always-proceed");
    let host = FixtureHost::completed([stdout(&bypass)]);
    let (_, terminal, cleanup) = completed_run(
        plan::headless_plan(
            host_id.clone(),
            "antigravity.fixture.executable",
            ResourceAccess::Read,
            HarnessIsolation::AmbientHost,
            None,
            false,
        ),
        request(
            "permission-bypass",
            HarnessIsolation::AmbientHost,
            None,
            false,
        ),
        host.services(host_id),
    );
    assert_status_code(
        &terminal,
        "swallowtail.antigravity.headless.malformed_stream",
        false,
    );
    assert_eq!(cleanup, CleanupOutcome::Clean);
}

#[test]
fn a_second_terminal_result_is_rejected() {
    let host_id = local_host();
    let terminal_line = SUCCESS.lines().last().expect("terminal fixture line");
    let duplicate = format!("{SUCCESS}{terminal_line}\n");
    let host = FixtureHost::completed([stdout(&duplicate)]);
    let (_, terminal, cleanup) = completed_run(
        plan::headless_plan(
            host_id.clone(),
            "antigravity.fixture.executable",
            ResourceAccess::Read,
            HarnessIsolation::AmbientHost,
            None,
            false,
        ),
        request(
            "duplicate-terminal",
            HarnessIsolation::AmbientHost,
            None,
            false,
        ),
        host.services(host_id),
    );
    assert_status_code(
        &terminal,
        "swallowtail.antigravity.headless.stream_limit",
        false,
    );
    assert_eq!(cleanup, CleanupOutcome::Clean);
}

#[test]
fn cancellation_and_deadline_force_stop_wait_and_join() {
    let host_id = local_host();
    let cancelled = FixtureHost::held_open();
    let mut handle = block_on(driver().start_run(
        plan::headless_plan(
            host_id.clone(),
            "antigravity.fixture.executable",
            ResourceAccess::Read,
            HarnessIsolation::AmbientHost,
            None,
            false,
        ),
        request("cancelled", HarnessIsolation::AmbientHost, None, false),
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
            "antigravity.fixture.executable",
            ResourceAccess::Read,
            HarnessIsolation::AmbientHost,
            None,
            false,
        ),
        request("timed-out", HarnessIsolation::AmbientHost, None, false),
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
fn local_and_remote_authoritative_hosts_use_the_same_explicit_route() {
    for (host_value, target) in [
        ("host.local", "antigravity.local.executable"),
        ("host.remote", "antigravity.remote.executable"),
    ] {
        let host_id = ExecutionHostId::new(host_value).expect("host id");
        let host = FixtureHost::completed([stdout(SUCCESS)]);
        let (_, terminal, cleanup) = completed_run(
            plan::headless_plan(
                host_id.clone(),
                target,
                ResourceAccess::Read,
                HarnessIsolation::AmbientHost,
                None,
                false,
            ),
            request(host_value, HarnessIsolation::AmbientHost, None, false),
            host.services(host_id),
        );
        assert_eq!(terminal.status(), &TerminalStatus::Completed);
        assert_eq!(cleanup, CleanupOutcome::Clean);
        assert_eq!(host.observed().executable, target);
    }
}

fn completed_run(
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

fn driver() -> AntigravityHeadlessDriver {
    AntigravityHeadlessDriver::new(
        EnvironmentRef::new("antigravity.fixture.environment").expect("environment"),
    )
}

fn request(
    id: &str,
    isolation: HarnessIsolation,
    effort: Option<&str>,
    structured: bool,
) -> StructuredRunRequest {
    let mut policy = OperationPolicy::offline()
        .with_provider_retention(ProviderRetentionPolicy::DurableAllowed)
        .with_harness_isolation(isolation)
        .with_harness_configuration_posture(HarnessConfigurationPosture::Ambient);
    if let Some(effort) = effort {
        policy = policy.with_reasoning_mode(ReasoningMode::new(effort).expect("effort"));
    }
    let mut request = StructuredRunRequest::new(
        RequestId::new(id).expect("request id"),
        OperationContent::new("fixture-private-prompt").expect("prompt"),
        policy,
    )
    .with_working_resource(WorkingResourceRef::new("workspace.main").expect("resource"))
    .with_deadline(Deadline::at(MonotonicInstant::from_ticks(1_000)));
    if structured {
        request = request.with_structured_output(
            StructuredOutputDescriptor::new(
                SchemaDocument::inline(SCHEMA.as_bytes().to_vec(), 16 * 1024).expect("schema"),
                "application/schema+json",
                "json-schema-2020-12",
            )
            .expect("structured output"),
        );
    }
    request
}

fn stdout(value: &str) -> ProcessOutputChunk {
    ProcessOutputChunk::new(ProcessOutputStream::Stdout, value.as_bytes().to_vec())
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

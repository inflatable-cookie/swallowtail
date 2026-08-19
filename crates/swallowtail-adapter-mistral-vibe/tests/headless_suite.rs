#[path = "support/headless.rs"]
mod support;

use futures_executor::block_on;
use futures_util::StreamExt;
use std::sync::Arc;
use support::{FIXTURE_CWD, FixtureHost, ImmediateTime};
use swallowtail_adapter_mistral_vibe::MistralVibeHeadlessDriver;
use swallowtail_core::{ExecutionHostId, HarnessConfigurationPosture, HarnessIsolation};
use swallowtail_runtime::{
    CancellationAcknowledgement, CleanupOutcome, Deadline, EnvironmentRef, MonotonicInstant,
    OperationContent, OperationPolicy, ProcessExit, ProviderRetentionPolicy, RequestId,
    RuntimeEventKind, StructuredRunDriver, StructuredRunRequest, TerminalStatus,
};

const SUCCESS: &str = include_str!("fixtures/mistral-vibe-headless-2.24.2/success.jsonl");
const STDERR: &str = include_str!("fixtures/mistral-vibe-headless-2.24.2/stderr-error.txt");
const LIMIT: &str = include_str!("fixtures/mistral-vibe-headless-2.24.2/limit-stderr.txt");

#[test]
fn success_run_uses_streaming_plan_trust_and_joins_cleanup() {
    let host_id = ExecutionHostId::new("fixture.host.local").expect("host");
    let selected = support::selection(host_id.clone());
    let host = FixtureHost::scripted([SUCCESS]);
    let mut handle = block_on(driver().start_run(
        selected.plan,
        request("success", selected.resource),
        host.services(host_id),
    ))
    .expect("run starts");
    let outcome = block_on(
        handle
            .take_terminal_outcome()
            .expect("terminal outcome is available"),
    );
    assert_eq!(outcome.status(), &TerminalStatus::Completed);
    assert_eq!(
        outcome.output().map(OperationContent::as_str),
        Some("Vibe display text.")
    );
    let mut events = handle.take_events().expect("events are available");
    let events = block_on(async move {
        let mut seen = Vec::new();
        while let Some(event) = events.next().await {
            seen.push(event.expect("event is valid"));
        }
        seen
    });
    assert!(
        events
            .iter()
            .any(|event| matches!(event.kind(), RuntimeEventKind::OutputDelta))
    );
    assert!(!format!("{events:?}").contains("private fixture prompt"));
    assert!(!format!("{outcome:?}").contains("Vibe display text"));
    let observed = host.observed();
    assert_eq!(
        observed.arguments,
        [
            "--prompt",
            "private fixture prompt",
            "--output",
            "streaming",
            "--max-turns",
            "8",
            "--trust",
            "--agent",
            "plan",
            "--workdir",
            FIXTURE_CWD,
        ]
    );
    for forbidden in [
        "vibe-acp",
        "--continue",
        "--resume",
        "--teleport",
        "--auto-approve",
        "--yolo",
        "json",
        "text",
    ] {
        assert!(
            !observed
                .arguments
                .iter()
                .any(|argument| argument == forbidden)
        );
    }
    assert!(host.stdin().is_empty());
    assert!(host.stdin_closed());
    assert!(host.waited());
    assert_eq!(block_on(handle.close()), CleanupOutcome::Clean);
    assert!(host.joined());
}

#[test]
fn stderr_error_fails_without_leaking_native_message() {
    let host_id = ExecutionHostId::new("fixture.host.local").expect("host");
    let selected = support::selection(host_id.clone());
    let host = FixtureHost::with_exit(
        [support::stderr_chunk(STDERR.as_bytes().to_vec())],
        ProcessExit::new(false, Some(1)),
    );
    let mut handle = block_on(driver().start_run(
        selected.plan,
        request("stderr", selected.resource),
        host.services(host_id),
    ))
    .expect("run starts");
    let outcome = block_on(
        handle
            .take_terminal_outcome()
            .expect("terminal outcome is available"),
    );
    assert!(matches!(
        outcome.status(),
        TerminalStatus::ProviderFailed(_)
    ));
    assert!(!format!("{outcome:?}").contains("prompt provided"));
}

#[test]
fn limit_stderr_is_bounded_failure_not_end_turn() {
    let host_id = ExecutionHostId::new("fixture.host.local").expect("host");
    let selected = support::selection(host_id.clone());
    let host = FixtureHost::with_exit(
        [support::stderr_chunk(LIMIT.as_bytes().to_vec())],
        ProcessExit::new(false, Some(1)),
    );
    let mut handle = block_on(driver().start_run(
        selected.plan,
        request("limit", selected.resource),
        host.services(host_id),
    ))
    .expect("run starts");
    let outcome = block_on(
        handle
            .take_terminal_outcome()
            .expect("terminal outcome is available"),
    );
    match outcome.status() {
        TerminalStatus::ProviderFailed(diagnostic) => {
            assert_eq!(
                diagnostic.code(),
                "swallowtail.mistral-vibe.headless.max_turns"
            );
        }
        other => panic!("expected provider failed, got {other:?}"),
    }
}

#[test]
fn cancellation_force_stops_and_joins() {
    let host_id = ExecutionHostId::new("fixture.host.local").expect("host");
    let selected = support::selection(host_id.clone());
    let host = FixtureHost::held_open();
    let mut handle = block_on(driver().start_run(
        selected.plan,
        request("cancel", selected.resource),
        host.services(host_id),
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
    let outcome = block_on(
        handle
            .take_terminal_outcome()
            .expect("terminal outcome is available"),
    );
    assert_eq!(outcome.status(), &TerminalStatus::Cancelled);
    assert_eq!(block_on(handle.close()), CleanupOutcome::Clean);
    assert!(host.force_stopped());
    assert!(host.waited());
}

#[test]
fn host_deadline_times_out() {
    let host_id = ExecutionHostId::new("fixture.host.local").expect("host");
    let selected = support::selection(host_id.clone());
    let host = FixtureHost::held_open();
    let mut handle = block_on(driver().start_run(
        selected.plan,
        request("timeout", selected.resource),
        host.services_with_time(host_id, Arc::new(ImmediateTime)),
    ))
    .expect("run starts");
    let outcome = block_on(
        handle
            .take_terminal_outcome()
            .expect("terminal outcome is available"),
    );
    assert_eq!(outcome.status(), &TerminalStatus::TimedOut);
    assert_eq!(block_on(handle.close()), CleanupOutcome::Clean);
    assert!(host.force_stopped());
}

#[test]
fn malformed_and_wrong_wire_fail_before_useful_work() {
    let cases = [
        ("malformed", "{\n"),
        ("array", "[{\"type\":\"message\"}]\n"),
        (
            "acp",
            "{\"jsonrpc\":\"2.0\",\"id\":1,\"method\":\"initialize\",\"params\":{}}\n",
        ),
    ];
    for (id, body) in cases {
        let host_id = ExecutionHostId::new("fixture.host.local").expect("host");
        let selected = support::selection(host_id.clone());
        let host = FixtureHost::scripted([body]);
        let mut handle = block_on(driver().start_run(
            selected.plan,
            request(id, selected.resource),
            host.services(host_id),
        ))
        .expect("run starts");
        let outcome = block_on(
            handle
                .take_terminal_outcome()
                .expect("terminal outcome is available"),
        );
        assert!(
            matches!(outcome.status(), TerminalStatus::RuntimeFailed(_)),
            "{id}"
        );
    }
}

#[test]
fn missing_deadline_fails_before_process_start() {
    let host_id = ExecutionHostId::new("fixture.host.local").expect("host");
    let selected = support::selection(host_id.clone());
    let host = FixtureHost::scripted([SUCCESS]);
    let policy = OperationPolicy::offline()
        .with_provider_retention(ProviderRetentionPolicy::Prohibited)
        .with_harness_isolation(HarnessIsolation::AmbientHost)
        .with_harness_configuration_posture(HarnessConfigurationPosture::Ambient);
    let request = StructuredRunRequest::new(
        RequestId::new("mistral-vibe-headless-missing-deadline").expect("request"),
        OperationContent::new("private fixture prompt").expect("prompt"),
        policy,
    )
    .with_working_resource(selected.resource);
    let result = block_on(driver().start_run(selected.plan, request, host.services(host_id)));
    assert!(result.is_err());
    assert!(!host.started());
}

fn driver() -> MistralVibeHeadlessDriver {
    MistralVibeHeadlessDriver::new(
        EnvironmentRef::new("mistral-vibe.fixture.isolated").expect("environment"),
    )
}

fn request(id: &str, resource: swallowtail_runtime::WorkingResourceRef) -> StructuredRunRequest {
    let policy = OperationPolicy::offline()
        .with_provider_retention(ProviderRetentionPolicy::Prohibited)
        .with_harness_isolation(HarnessIsolation::AmbientHost)
        .with_harness_configuration_posture(HarnessConfigurationPosture::Ambient);
    StructuredRunRequest::new(
        RequestId::new(format!("mistral-vibe-headless-{id}")).expect("request"),
        OperationContent::new("private fixture prompt").expect("prompt"),
        policy,
    )
    .with_working_resource(resource)
    .with_deadline(Deadline::at(MonotonicInstant::from_ticks(1_000)))
}

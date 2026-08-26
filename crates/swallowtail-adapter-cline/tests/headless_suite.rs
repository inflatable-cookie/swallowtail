#[path = "support/headless.rs"]
mod support;

use futures_executor::block_on;
use futures_util::StreamExt;
use std::sync::Arc;
use support::{FIXTURE_CWD, FixtureHost, ImmediateTime};
use swallowtail_adapter_cline::ClineHeadlessDriver;
use swallowtail_core::{ExecutionHostId, HarnessConfigurationPosture, HarnessIsolation};
use swallowtail_runtime::{
    CancellationAcknowledgement, CleanupOutcome, Deadline, EnvironmentRef, MonotonicInstant,
    OperationContent, OperationPolicy, ProcessExit, ProviderRetentionPolicy, RequestId,
    RuntimeEventKind, StructuredRunDriver, StructuredRunRequest, TerminalStatus,
};

const SUCCESS: &str = include_str!("fixtures/cline-headless-3.0.55/success.jsonl");
const ABORT: &str = include_str!("fixtures/cline-headless-3.0.55/abort.jsonl");
const STDERR: &str = include_str!("fixtures/cline-headless-3.0.55/stderr-error.jsonl");

#[test]
fn success_run_uses_json_auto_approve_false_and_joins_cleanup() {
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
        Some("Cline display text.")
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
    assert!(!format!("{outcome:?}").contains("Cline display text"));
    let observed = host.observed();
    assert_eq!(
        observed.arguments,
        [
            "--json",
            "--auto-approve",
            "false",
            "-c",
            FIXTURE_CWD,
            "private fixture prompt"
        ]
    );
    for forbidden in ["--acp", "--id", "--yolo", "true"] {
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
fn abort_stream_cancels_without_selecting_acp() {
    let host_id = ExecutionHostId::new("fixture.host.local").expect("host");
    let selected = support::selection(host_id.clone());
    let host = FixtureHost::scripted([ABORT]);
    let mut handle = block_on(driver().start_run(
        selected.plan,
        request("abort", selected.resource),
        host.services(host_id),
    ))
    .expect("run starts");
    let outcome = block_on(
        handle
            .take_terminal_outcome()
            .expect("terminal outcome is available"),
    );
    assert_eq!(outcome.status(), &TerminalStatus::Cancelled);
    assert_eq!(block_on(handle.close()), CleanupOutcome::Clean);
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
    assert!(!format!("{outcome:?}").contains("prompt argument"));
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
        (
            "ask-say",
            "{\"type\":\"say\",\"text\":\"docs schema\",\"ts\":0,\"say\":\"text\"}\n",
        ),
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
        RequestId::new("cline-headless-missing-deadline").expect("request"),
        OperationContent::new("private fixture prompt").expect("prompt"),
        policy,
    )
    .with_working_resource(selected.resource);
    let result = block_on(driver().start_run(selected.plan, request, host.services(host_id)));
    assert!(result.is_err());
    assert!(!host.started());
}

#[test]
fn plan_places_canonical_flag_before_cwd_and_does_not_select_act_flags() {
    let host_id = ExecutionHostId::new("fixture.host.plan").expect("host");
    let selected = support::selection_with_plan(host_id.clone());
    let host = FixtureHost::scripted([SUCCESS]);
    let mut handle = block_on(driver().start_run(
        selected.plan,
        request_with_plan("plan", selected.resource),
        host.services(host_id),
    ))
    .expect("plan run starts");
    let outcome = block_on(
        handle
            .take_terminal_outcome()
            .expect("terminal outcome is available"),
    );
    assert_eq!(outcome.status(), &TerminalStatus::Completed);
    let observed = host.observed();
    assert_eq!(
        observed.arguments,
        [
            "--json",
            "--auto-approve",
            "false",
            "--plan",
            "-c",
            FIXTURE_CWD,
            "private fixture prompt"
        ]
    );
    for forbidden in ["--acp", "--id", "--yolo", "--zen", "-p"] {
        assert!(
            !observed
                .arguments
                .iter()
                .any(|argument| argument == forbidden),
            "{forbidden} must not be selected for cline.headless Plan"
        );
    }
    assert!(
        !observed
            .arguments
            .windows(2)
            .any(|pair| pair == ["--auto-approve", "true"])
    );
    assert_eq!(block_on(handle.close()), CleanupOutcome::Clean);
    assert!(host.joined());
}

#[test]
fn plan_without_capability_rejects_before_process_start() {
    let host_id = ExecutionHostId::new("fixture.host.plan.unadvertised").expect("host");
    let selected = support::selection(host_id.clone());
    let host = FixtureHost::scripted([SUCCESS]);
    let error = block_on(driver().start_run(
        selected.plan,
        request_with_plan("plan-unadvertised", selected.resource),
        host.services(host_id),
    ))
    .err()
    .expect("Plan request cannot use a plan without HarnessModeSelection");
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.cline.headless.harness_mode_mismatch"
    );
    assert!(!host.started());
}

#[test]
fn omitted_request_rejects_plan_bearing_plan_before_process_start() {
    let host_id = ExecutionHostId::new("fixture.host.omit.plan-plan").expect("host");
    let selected = support::selection_with_plan(host_id.clone());
    let host = FixtureHost::scripted([SUCCESS]);
    let error = block_on(driver().start_run(
        selected.plan,
        request("omit-vs-plan", selected.resource),
        host.services(host_id),
    ))
    .err()
    .expect("omitted request cannot use a Plan-bearing plan");
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.cline.headless.harness_mode_mismatch"
    );
    assert!(!host.started());
}

fn driver() -> ClineHeadlessDriver {
    ClineHeadlessDriver::new(EnvironmentRef::new("cline.fixture.isolated").expect("environment"))
}

fn request(id: &str, resource: swallowtail_runtime::WorkingResourceRef) -> StructuredRunRequest {
    request_with_policy(id, resource, omit_policy())
}

fn request_with_plan(
    id: &str,
    resource: swallowtail_runtime::WorkingResourceRef,
) -> StructuredRunRequest {
    request_with_policy(
        id,
        resource,
        omit_policy().with_harness_mode(swallowtail_core::HarnessMode::Plan),
    )
}

fn omit_policy() -> OperationPolicy {
    OperationPolicy::offline()
        .with_provider_retention(ProviderRetentionPolicy::Prohibited)
        .with_harness_isolation(HarnessIsolation::AmbientHost)
        .with_harness_configuration_posture(HarnessConfigurationPosture::Ambient)
}

fn request_with_policy(
    id: &str,
    resource: swallowtail_runtime::WorkingResourceRef,
    policy: OperationPolicy,
) -> StructuredRunRequest {
    StructuredRunRequest::new(
        RequestId::new(format!("cline-headless-{id}")).expect("request"),
        OperationContent::new("private fixture prompt").expect("prompt"),
        policy,
    )
    .with_working_resource(resource)
    .with_deadline(Deadline::at(MonotonicInstant::from_ticks(1_000)))
}

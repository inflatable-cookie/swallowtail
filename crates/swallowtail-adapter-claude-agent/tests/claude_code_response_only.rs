#![allow(dead_code, unused_imports)]

mod claude_code_support;

use claude_code_support::{
    FakeProcessService, ImmediateTimeService, PendingTimeService, host_services, response_fixture,
    response_preparation_input, response_preparation_probe,
};
use futures_executor::block_on;
use futures_util::StreamExt;
use std::sync::Arc;
use swallowtail_adapter_claude_agent::{
    ClaudeCodeResponseModelSelection, ClaudeCodeResponsePreparedIntegration,
    ClaudeCodeResponsePreparedRun, ClaudeCodeResponseProfileInput,
    prepare_claude_code_response_only,
};
use swallowtail_core::{
    Capability, HarnessConfigurationPosture, HarnessIsolation, ModelId, ModelRouteId,
    ModelRouteRevision, ObservableActivityAvailability,
};
use swallowtail_runtime::{
    CleanupOutcome, Deadline, MonotonicInstant, OperationContent, ProcessExit, RequestId,
    RuntimeEventKind, TerminalStatus,
};

#[test]
fn prepared_route_returns_one_ordinary_text_result_without_authority() {
    let host = swallowtail_core::ExecutionHostId::new("host.fixture").expect("host is valid");
    let prepared = prepared(host.clone());
    let run = profile(&prepared, "complete");
    for forbidden in [
        Capability::WorkingResource,
        Capability::StructuredOutput,
        Capability::HarnessModeSelection,
    ] {
        assert!(
            !run.plan()
                .requirements()
                .capabilities()
                .any(|requirement| requirement.capability() == forbidden)
        );
    }
    assert_eq!(
        run.plan().harness_configuration_posture(),
        Some(HarnessConfigurationPosture::ProviderSuppressed)
    );
    assert_eq!(
        run.plan().requirements().harness_isolation(),
        Some(HarnessIsolation::AmbientHost)
    );
    assert_eq!(
        run.evidence()
            .operation()
            .observable_activity()
            .availability(),
        ObservableActivityAvailability::Available
    );
    assert!(run.request().working_resource().is_none());
    assert!(run.request().structured_output().is_none());
    assert_eq!(run.request().attachments().len(), 0);
    assert_eq!(run.request().tools().len(), 0);

    let (process, state) =
        FakeProcessService::completed(&response_fixture("response-complete.jsonl"));
    let (services, task) = host_services(host, process, Arc::new(PendingTimeService));
    let mut handle = block_on(run.start_run(services)).expect("response-only run starts");
    assert!(handle.take_callbacks().is_none());
    assert!(handle.take_management_binding().is_none());
    assert!(handle.detachment().is_none());
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
    assert_eq!(outcome.status(), &TerminalStatus::Completed);
    assert_eq!(
        outcome.output().map(OperationContent::as_str),
        Some(r#"{"decision":"accept","score":7}"#)
    );
    assert_eq!(
        events
            .iter()
            .filter(|event| event.kind() == &RuntimeEventKind::OutputAvailable)
            .count(),
        1
    );
    assert_eq!(
        events
            .iter()
            .filter(|event| matches!(event.kind(), RuntimeEventKind::Activity(_)))
            .count(),
        1
    );
    assert_eq!(block_on(handle.close()), CleanupOutcome::Clean);
    assert!(state.waited());
    assert!(task.joined());
    let request = state.request();
    assert!(request.working_resource.is_none());
    assert!(
        request
            .arguments
            .windows(2)
            .any(|pair| pair == ["--tools", ""])
    );
    assert!(request.arguments.iter().any(|value| value == "--safe-mode"));
    assert!(
        request
            .arguments
            .iter()
            .any(|value| value == "--no-session-persistence")
    );
    assert!(
        request
            .arguments
            .iter()
            .any(|value| value == "--strict-mcp-config")
    );
    assert!(
        request
            .arguments
            .windows(2)
            .any(|pair| pair == ["--mcp-config", r#"{"mcpServers":{}}"#])
    );
    for forbidden in [
        "--json-schema",
        "--permission-mode",
        "--resume",
        "--continue",
        "--fallback-model",
    ] {
        assert!(
            !request
                .arguments
                .iter()
                .any(|argument| argument == forbidden)
        );
    }
    assert_eq!(state.stdin(), b"return JSON-shaped text".to_vec());
    assert!(state.stdin_closed());
}

#[test]
fn response_stream_fails_closed_on_authority_or_turn_drift() {
    let cases = [
        (r#""tools":[]"#, r#""tools":["Read"]"#),
        (
            r#""mcp_servers":[]"#,
            r#""mcp_servers":[{"name":"fixture"}]"#,
        ),
        (r#""num_turns":1"#, r#""num_turns":2"#),
        (r#""structured_output":null"#, r#""structured_output":{}"#),
        (
            r#""claude_code_version":"2.1.227""#,
            r#""claude_code_version":"2.1.228""#,
        ),
    ];
    for (from, to) in cases {
        let fixture = response_fixture("response-complete.jsonl").replacen(from, to, 1);
        assert_malformed(fixture);
    }
    let duplicate = response_fixture("response-complete.jsonl");
    let assistant = duplicate.lines().nth(1).expect("assistant fixture line");
    let duplicate = duplicate.replacen(
        duplicate.lines().nth(2).expect("result fixture line"),
        &format!("{assistant}\n{}", duplicate.lines().nth(2).unwrap()),
        1,
    );
    assert_malformed(duplicate);

    let base = response_fixture("response-complete.jsonl");
    let result = base.lines().nth(2).expect("result fixture line");
    let user = r#"{"type":"user","message":{"role":"user","content":[]},"session_id":"fixture-response-session"}"#;
    assert_malformed(base.replacen(result, &format!("{user}\n{result}"), 1));
    assert_malformed(base.replacen(
        r#"{"type":"text","text":"{\"decision\":\"accept\",\"score\":7}"}"#,
        r#"{"type":"tool_use","id":"tool-fixture","name":"Read"}"#,
        1,
    ));
    assert_malformed(base.replacen(r#""text":"{\"decision\""#, r#""missing":"{\"decision\""#, 1));
    assert_malformed("{".to_owned());
    assert_malformed(format!("{{\"padding\":\"{}\"}}\n", "x".repeat(1024 * 1024)));
}

#[test]
fn unsuccessful_process_is_redacted_and_never_becomes_output() {
    let host = swallowtail_core::ExecutionHostId::new("host.failure").expect("host is valid");
    let run = profile(&prepared(host.clone()), "failure");
    let (process, state) = FakeProcessService::with_exit("", ProcessExit::new(false, Some(1)));
    let (services, _task) = host_services(host, process, Arc::new(PendingTimeService));
    let mut handle = block_on(run.start_run(services)).expect("response-only run starts");
    let _events = block_on(
        handle
            .take_events()
            .expect("event stream is available")
            .collect::<Vec<_>>(),
    );
    let outcome = block_on(
        handle
            .take_terminal_outcome()
            .expect("terminal outcome is available"),
    );
    let diagnostic = match outcome.status() {
        TerminalStatus::ProviderFailed(diagnostic) => diagnostic,
        status => panic!("unexpected status: {status:?}"),
    };
    assert_eq!(
        diagnostic.code(),
        "swallowtail.claude_code.response_only.process_failed"
    );
    assert!(!diagnostic.message().contains("return JSON-shaped text"));
    assert!(outcome.output().is_none());
    assert_eq!(block_on(handle.close()), CleanupOutcome::Clean);
    assert!(state.waited());
}

#[test]
fn cancellation_stops_and_reaps_the_provider_process() {
    let host = swallowtail_core::ExecutionHostId::new("host.cancel").expect("host is valid");
    let run = profile(&prepared(host.clone()), "cancel");
    let (process, state) = FakeProcessService::held_open();
    let (services, task) = host_services(host, process, Arc::new(PendingTimeService));
    let mut handle = block_on(run.start_run(services)).expect("response-only run starts");
    let acknowledgement = block_on(handle.cancellation().request()).expect("cancellation succeeds");
    assert_eq!(
        acknowledgement,
        swallowtail_runtime::CancellationAcknowledgement::Requested
    );
    let _events = block_on(
        handle
            .take_events()
            .expect("event stream is available")
            .collect::<Vec<_>>(),
    );
    let outcome = block_on(
        handle
            .take_terminal_outcome()
            .expect("terminal outcome is available"),
    );
    assert!(matches!(outcome.status(), TerminalStatus::Cancelled));
    assert_eq!(block_on(handle.close()), CleanupOutcome::Clean);
    assert!(state.force_stopped());
    assert!(state.waited());
    assert!(task.joined());
}

#[test]
fn deadline_stops_and_reaps_the_provider_process() {
    let host = swallowtail_core::ExecutionHostId::new("host.timeout").expect("host is valid");
    let run = profile(&prepared(host.clone()), "timeout");
    let (process, state) = FakeProcessService::held_open();
    let (services, task) = host_services(host, process, Arc::new(ImmediateTimeService));
    let mut handle = block_on(run.start_run(services)).expect("response-only run starts");
    let _events = block_on(
        handle
            .take_events()
            .expect("event stream is available")
            .collect::<Vec<_>>(),
    );
    let outcome = block_on(
        handle
            .take_terminal_outcome()
            .expect("terminal outcome is available"),
    );
    assert_eq!(outcome.status(), &TerminalStatus::TimedOut);
    assert_eq!(block_on(handle.close()), CleanupOutcome::Clean);
    assert!(state.force_stopped());
    assert!(state.waited());
    assert!(task.joined());
}

fn prepared(host: swallowtail_core::ExecutionHostId) -> ClaudeCodeResponsePreparedIntegration {
    let (process, state) = FakeProcessService::completed("2.1.227 (Claude Code)\n");
    let (services, task) = host_services(host.clone(), process, Arc::new(PendingTimeService));
    let prepared = block_on(prepare_claude_code_response_only(
        response_preparation_input(host),
        response_preparation_probe(),
        services,
    ))
    .expect("Claude Code response-only prepares");
    assert_eq!(state.request().arguments, ["--version"]);
    assert!(state.waited());
    assert!(task.joined());
    prepared
}

fn profile(
    prepared: &ClaudeCodeResponsePreparedIntegration,
    id: &str,
) -> ClaudeCodeResponsePreparedRun {
    prepared
        .prepare_run(ClaudeCodeResponseProfileInput::new(
            RequestId::new(format!("claude-code-response-{id}")).expect("request is valid"),
            ClaudeCodeResponseModelSelection::new(
                ModelRouteId::new(format!("claude-code-response.{id}")).expect("route is valid"),
                ModelRouteRevision::new("1").expect("route revision is valid"),
                ModelId::new("claude-sonnet-5").expect("model is valid"),
            ),
            OperationContent::new("return JSON-shaped text").expect("content is valid"),
            Deadline::at(MonotonicInstant::from_ticks(1_000)),
        ))
        .expect("response-only run prepares")
}

fn assert_malformed(output: String) {
    let host = swallowtail_core::ExecutionHostId::new("host.malformed").expect("host is valid");
    let run = profile(&prepared(host.clone()), "malformed");
    let (process, state) = FakeProcessService::with_exit(&output, ProcessExit::new(true, Some(0)));
    let (services, _task) = host_services(host, process, Arc::new(PendingTimeService));
    let mut handle = block_on(run.start_run(services)).expect("response-only run starts");
    let events = block_on(
        handle
            .take_events()
            .expect("event stream is available")
            .collect::<Vec<_>>(),
    );
    assert!(events.iter().all(Result::is_ok));
    let outcome = block_on(
        handle
            .take_terminal_outcome()
            .expect("terminal outcome is available"),
    );
    assert!(matches!(outcome.status(), TerminalStatus::RuntimeFailed(_)));
    assert_eq!(block_on(handle.close()), CleanupOutcome::Clean);
    assert!(state.waited());
}

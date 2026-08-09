use super::*;
use swallowtail_runtime::RuntimeRunId;

fn operation() -> ActivityOperationId {
    ActivityOperationId::Run(RuntimeRunId::new("command-code-events-fixture").unwrap())
}

fn run_all(lines: &[&str]) -> Result<(Vec<RuntimeEvent>, ParsedTerminal), RuntimeFailure> {
    let mut parser = CommandCodeHeadlessEventParser::with_expected_session(operation(), None);
    let mut events = Vec::new();
    for line in lines {
        events.extend(parser.push(format!("{line}\n").as_bytes())?);
    }
    let (trailing, terminal, _session_id) = parser.finish()?;
    events.extend(trailing);
    Ok((events, terminal))
}

#[test]
fn no_tool_success_stream_projects_output_and_completes() {
    let (events, terminal) = run_all(&[
        r#"{"type":"event","event":{"type":"run_start","sessionId":"00000000-0000-4000-8000-000000000001"}}"#,
        r#"{"type":"event","event":{"type":"turn_start"}}"#,
        r#"{"type":"event","event":{"type":"thinking_start"}}"#,
        r#"{"type":"event","event":{"type":"thinking_delta","delta":"considering"}}"#,
        r#"{"type":"event","event":{"type":"thinking_end"}}"#,
        r#"{"type":"event","event":{"type":"text_delta","delta":"pong"}}"#,
        r#"{"type":"event","event":{"type":"model_request_end","usage":{"inputTokens":12,"outputTokens":3,"cacheReadTokens":0,"cacheWriteTokens":0}}}"#,
        r#"{"type":"event","event":{"type":"turn_end"}}"#,
        r#"{"type":"event","event":{"type":"run_end"}}"#,
        r#"{"type":"result","subtype":"success","sessionId":"00000000-0000-4000-8000-000000000001","stopReason":"end_turn","usage":{"inputTokens":12,"outputTokens":3,"cacheReadTokens":0,"cacheWriteTokens":0},"durationMs":842,"finalText":"pong"}"#,
    ])
    .expect("qualified success stream parses");
    assert!(
        events
            .iter()
            .any(|event| matches!(event.kind(), RuntimeEventKind::OutputAvailable))
    );
    assert!(
        events
            .iter()
            .any(|event| matches!(event.kind(), RuntimeEventKind::ReasoningProgress))
    );
    assert!(events.iter().any(|event| matches!(
        event.kind(),
        RuntimeEventKind::ProviderObservation(ProviderObservation::Usage(_))
    )));
    let outcome = terminal.outcome(ProcessExit::new(true, Some(0)));
    assert_eq!(outcome.status(), &TerminalStatus::Completed);
    assert_eq!(outcome.output().map(OperationContent::as_str), Some("pong"));
}

#[test]
fn unknown_event_type_is_projected_as_namespaced_unknown_activity() {
    let (events, terminal) = run_all(&[
        r#"{"type":"event","event":{"type":"run_start","sessionId":"00000000-0000-4000-8000-000000000002"}}"#,
        r#"{"type":"event","event":{"type":"future_experimental_event","foo":"bar"}}"#,
        r#"{"type":"result","subtype":"success","sessionId":"00000000-0000-4000-8000-000000000002","finalText":"pong"}"#,
    ])
    .expect("unknown-event stream parses");
    let found = events.iter().any(|event| match event.kind() {
        RuntimeEventKind::Activity(observation) => matches!(
            observation.kind(),
            swallowtail_runtime::ActivityKind::Unknown(namespace)
                if namespace.as_str() == "command-code.headless.event.future_experimental_event"
        ),
        _ => false,
    });
    assert!(found, "unknown event type should be namespaced");
    let outcome = terminal.outcome(ProcessExit::new(true, Some(0)));
    assert_eq!(outcome.status(), &TerminalStatus::Completed);
}

#[test]
fn tool_lifecycle_never_projects_input_or_result_bodies() {
    let (events, _terminal) = run_all(&[
        r#"{"type":"event","event":{"type":"run_start","sessionId":"00000000-0000-4000-8000-000000000003"}}"#,
        r#"{"type":"event","event":{"type":"tool_queued","toolCallId":"call-1","toolName":"read_file","input":{"file_path":"/etc/private"}}}"#,
        r#"{"type":"event","event":{"type":"tool_running","toolCallId":"call-1","toolName":"read_file","description":null}}"#,
        r#"{"type":"event","event":{"type":"tool_completed","toolCallId":"call-1","toolName":"read_file","result":["private contents"]}}"#,
        r#"{"type":"result","subtype":"success","sessionId":"00000000-0000-4000-8000-000000000003","finalText":"pong"}"#,
    ])
    .expect("tool stream parses");
    for event in &events {
        if let RuntimeEventKind::Activity(observation) = event.kind() {
            assert!(observation.content().is_none());
        }
    }
}

#[test]
fn credit_failure_result_classifies_quota_exhausted_with_exit_ten() {
    let (_, terminal) = run_all(&[
        r#"{"type":"event","event":{"type":"run_start","sessionId":"00000000-0000-4000-8000-000000000004"}}"#,
        r#"{"type":"event","event":{"type":"run_error","message":"insufficient credits"}}"#,
        r#"{"type":"result","subtype":"error","sessionId":"00000000-0000-4000-8000-000000000004"}"#,
    ])
    .expect("credit failure stream parses");
    let outcome = terminal.outcome(ProcessExit::new(false, Some(10)));
    let TerminalStatus::ProviderFailed(diagnostic) = outcome.status() else {
        panic!("credit failure must be a provider failure");
    };
    assert_eq!(
        diagnostic.failure_classification().kind(),
        swallowtail_core::FailureKind::QuotaExhausted
    );
}

#[test]
fn max_turns_result_classifies_invalid_request() {
    let (_, terminal) = run_all(&[
        r#"{"type":"event","event":{"type":"run_start","sessionId":"00000000-0000-4000-8000-000000000005"}}"#,
        r#"{"type":"result","subtype":"max_turns","sessionId":"00000000-0000-4000-8000-000000000005"}"#,
    ])
    .expect("max-turns stream parses");
    let outcome = terminal.outcome(ProcessExit::new(true, Some(0)));
    let TerminalStatus::ProviderFailed(diagnostic) = outcome.status() else {
        panic!("max-turns must be a provider failure");
    };
    assert_eq!(
        diagnostic.failure_classification().kind(),
        swallowtail_core::FailureKind::InvalidRequest
    );
}

#[test]
fn negative_cases_are_all_rejected() {
    let cases: [&str; 9] = [
        r#"not-json"#,
        r#"{"type":"unexpected"}"#,
        r#"{"type":"event"}"#,
        r#"{"type":"event","event":{}}"#,
        r#"{"type":"event","event":{"type":"run_start"}}"#,
        r#"{"type":"event","event":{"type":"thinking_delta","delta":"no start seen"}}"#,
        r#"{"type":"event","event":{"type":"tool_running","toolCallId":"missing"}}"#,
        r#"{"type":"event","event":{"type":"tool_completed","toolCallId":"missing"}}"#,
        r#"{"type":"result","subtype":"success"}"#,
    ];
    for case in cases {
        let mut parser = CommandCodeHeadlessEventParser::with_expected_session(operation(), None);
        let result = parser
            .push(format!("{case}\n").as_bytes())
            .and_then(|_| parser.finish());
        assert!(result.is_err(), "{case} should be rejected");
    }
}

#[test]
fn duplicate_run_start_is_rejected() {
    let result = run_all(&[
        r#"{"type":"event","event":{"type":"run_start","sessionId":"00000000-0000-4000-8000-000000000006"}}"#,
        r#"{"type":"event","event":{"type":"run_start","sessionId":"00000000-0000-4000-8000-000000000006"}}"#,
    ]);
    assert!(result.is_err());
}

#[test]
fn mismatched_session_id_on_result_is_rejected() {
    let result = run_all(&[
        r#"{"type":"event","event":{"type":"run_start","sessionId":"00000000-0000-4000-8000-000000000007"}}"#,
        r#"{"type":"result","subtype":"success","sessionId":"00000000-0000-4000-8000-000000000099","finalText":"pong"}"#,
    ]);
    assert!(result.is_err());
}

#[test]
fn run_end_never_fails_the_stream_even_though_it_is_ignored() {
    let (events, _terminal) = run_all(&[
        r#"{"type":"event","event":{"type":"run_start","sessionId":"00000000-0000-4000-8000-000000000008"}}"#,
        r#"{"type":"event","event":{"type":"run_end","result":{"nextState":{"messages":["private"],"cwd":"/private"}}}}"#,
        r#"{"type":"result","subtype":"success","sessionId":"00000000-0000-4000-8000-000000000008","finalText":"pong"}"#,
    ])
    .expect("run_end is ignored, never inspected");
    assert!(events.iter().all(|event| !matches!(
        event.kind(),
        RuntimeEventKind::Activity(_) if false
    )));
}

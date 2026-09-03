#[test]
fn qwen_reasoning_session_repeats_exact_setup_across_resume_and_replacement() {
    let host_id = ExecutionHostId::new("fixture.qwen.reasoning.session").expect("valid host");
    let (discovery_process, _) = FakeProcessService::completed("0.21.15\n");
    let (discovery_services, _) = host_services_for(
        host_id.clone(),
        discovery_process,
        Arc::new(PendingTimeService),
    );
    let mode = ReasoningMode::new("high").expect("mode is valid");
    let prepared = block_on(prepare_qwen_headless(
        preparation_input(host_id.clone()),
        probe(),
        discovery_services,
    ))
    .expect("Qwen prepares");
    let profile = prepared
        .prepare_session(
            QwenSessionProfileInput::new(
                RequestId::new("qwen-reasoning-session").expect("valid request"),
                QwenModelSelection::new(
                    ModelRouteId::new("qwen.reasoning.session.route").expect("valid route"),
                    ModelRouteRevision::new("1").expect("valid route revision"),
                    ProviderId::new("alibaba-modelstudio").expect("valid provider"),
                    ModelId::new("qwen3.8-max").expect("valid model"),
                ),
                WorkingResourceRef::new("qwen.reasoning.session.workspace")
                    .expect("valid resource"),
            )
            .with_reasoning_mode(mode.clone()),
        )
        .expect("qualified reasoning session prepares");
    assert_eq!(profile.evidence().reasoning_mode(), Some(&mode));
    assert_eq!(profile.request().options().reasoning_mode(), Some(&mode));
    let interrupted = RuntimeTurnId::new("qwen-reasoning-interrupted").expect("valid turn");
    let restoration = profile.prepare_working_state_restoration(interrupted.clone());
    assert_eq!(
        restoration.method(),
        WorkingStateRestorationMethod::FreshSessionReplacement
    );

    let control = |session_id: &str, payload: &str| {
        let initialize = serde_json::json!({
            "type": "control_response",
            "response": {
                "subtype": "success",
                "request_id": "swallowtail-initialize",
                "response": {
                    "subtype": "initialize",
                    "session_id": session_id,
                    "capabilities": {"can_set_effort": true}
                }
            }
        });
        let effort = serde_json::json!({
            "type": "control_response",
            "response": {
                "subtype": "success",
                "request_id": "swallowtail-reasoning",
                "response": {
                    "subtype": "set_effort",
                    "effort": "high",
                    "applied": true,
                    "override": null
                }
            }
        });
        format!("{}\n{}\n{}", initialize, effort, payload)
    };
    let outputs = [
        control(
            "fixture-control-session-1",
            include_str!("../fixtures/qwen-code-0.21.15/reasoning-success.jsonl"),
        ),
        control(
            "fixture-control-session-2",
            include_str!("../fixtures/qwen-code-0.21.15/reasoning-continued.jsonl"),
        ),
        control(
            "fixture-control-session-replacement",
            include_str!("../fixtures/qwen-code-0.21.15/reasoning-success.jsonl"),
        ),
    ];
    let output_refs = outputs.iter().map(String::as_str).collect::<Vec<_>>();
    let (process, states) = ScriptedProcessService::completed(&output_refs);
    let (services, _) = host_services_for(host_id, process, Arc::new(PendingTimeService));
    let mut session = block_on(profile.open_session(services.clone())).expect("session opens");
    for (index, content) in ["first reasoning prompt", "second reasoning prompt"]
        .into_iter()
        .enumerate()
    {
        let mut turn = block_on(
            session.start_turn(
                TurnRequest::new(
                    RuntimeTurnId::new(format!("qwen-reasoning-turn-{}", index + 1))
                        .expect("valid turn"),
                    OperationContent::new(content).expect("valid content"),
                )
                .with_deadline(Deadline::at(MonotonicInstant::from_ticks(1_000))),
                services.clone(),
            ),
        )
        .expect("turn starts");
        let terminal = block_on(
            turn.take_terminal_outcome()
                .expect("terminal outcome is available"),
        );
        assert_eq!(terminal.status(), &TerminalStatus::Completed);
        assert_eq!(block_on(turn.close()), CleanupOutcome::Clean);
    }
    assert!(!states[0].request().arguments.iter().any(|arg| arg == "--resume"));
    assert_eq!(
        states[1]
            .request()
            .arguments
            .windows(2)
            .find(|pair| pair[0] == "--resume"),
        Some(["--resume".to_owned(), "reasoning-session-1".to_owned()].as_slice())
    );
    assert_eq!(
        block_on(close_session(session, services.clone())),
        CleanupOutcome::Clean
    );

    let restored = block_on(restoration.restore(services.clone())).expect("replacement opens");
    let WorkingStateRestorationOutcome::SessionReplaced(replaced) = restored else {
        panic!("reasoning restoration reports a fresh replacement");
    };
    assert_eq!(replaced.interrupted_turn_id(), &interrupted);
    let (_, mut replacement) = replaced.into_parts();
    let mut turn = block_on(
        replacement.start_turn(
            TurnRequest::new(
                RuntimeTurnId::new("qwen-reasoning-replacement-turn").expect("valid turn"),
                OperationContent::new("replacement reasoning prompt").expect("valid content"),
            )
            .with_deadline(Deadline::at(MonotonicInstant::from_ticks(1_000))),
            services.clone(),
        ),
    )
    .expect("replacement turn starts");
    let terminal = block_on(
        turn.take_terminal_outcome()
            .expect("replacement terminal is available"),
    );
    assert_eq!(terminal.status(), &TerminalStatus::Completed);
    assert_eq!(block_on(turn.close()), CleanupOutcome::Clean);

    let replacement_arguments = states[2].request().arguments;
    assert!(replacement_arguments
        .windows(2)
        .any(|pair| pair == ["--input-format", "stream-json"]));
    assert!(!replacement_arguments.iter().any(|arg| arg == "--resume"));
    assert!(!replacement_arguments.iter().any(|arg| arg == "--continue"));
    let messages = String::from_utf8(states[2].stdin())
        .expect("replacement control stdin is UTF-8")
        .lines()
        .map(|line| serde_json::from_str::<serde_json::Value>(line).expect("valid message"))
        .collect::<Vec<_>>();
    assert_eq!(messages.len(), 3);
    assert_eq!(
        messages[0]
            .pointer("/request/subtype")
            .and_then(|value| value.as_str()),
        Some("initialize")
    );
    assert_eq!(
        messages[1]
            .pointer("/request/subtype")
            .and_then(|value| value.as_str()),
        Some("set_effort")
    );
    assert_eq!(
        messages[1]
            .pointer("/request/effort")
            .and_then(|value| value.as_str()),
        Some("high")
    );
    assert_eq!(messages[2].get("type").and_then(|value| value.as_str()), Some("user"));
    assert_eq!(
        messages[2]
            .get("session_id")
            .and_then(|value| value.as_str()),
        Some("fixture-control-session-replacement")
    );
    assert_eq!(
        block_on(close_session(replacement, services)),
        CleanupOutcome::Clean
    );
}

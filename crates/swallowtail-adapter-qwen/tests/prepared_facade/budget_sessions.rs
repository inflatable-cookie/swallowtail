#[test]
fn selected_qwen_budgets_compose_with_reasoning_and_session_children() {
    let turns = QwenSessionTurnBudget::try_new(8).expect("admitted turns");
    let tools = QwenToolCallBudget::try_new(4).expect("admitted tools");
    let host_id = ExecutionHostId::new("fixture.qwen.budget.session").expect("valid host");
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
                RequestId::new("qwen-budget-session").expect("valid request"),
                QwenModelSelection::new(
                    ModelRouteId::new("qwen.budget.session.route").expect("valid route"),
                    ModelRouteRevision::new("1").expect("valid route revision"),
                    ProviderId::new("alibaba-modelstudio").expect("valid provider"),
                    ModelId::new("qwen3.8-max").expect("valid model"),
                ),
                WorkingResourceRef::new("qwen.budget.session.workspace").expect("valid resource"),
            )
            .with_reasoning_mode(mode.clone())
            .with_session_turn_budget(turns)
            .with_tool_call_budget(tools),
        )
        .expect("composed session prepares");
    assert_eq!(profile.evidence().reasoning_mode(), Some(&mode));
    assert_eq!(profile.evidence().budgets().session_turns(), Some(turns));
    assert_eq!(profile.evidence().budgets().tool_calls(), Some(tools));

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
            "fixture-budget-session-1",
            include_str!("../fixtures/qwen-code-0.21.15/reasoning-success.jsonl"),
        ),
        control(
            "fixture-budget-session-2",
            include_str!("../fixtures/qwen-code-0.21.15/reasoning-continued.jsonl"),
        ),
        control(
            "fixture-budget-replacement",
            include_str!("../fixtures/qwen-code-0.21.15/reasoning-success.jsonl"),
        ),
    ];
    let output_refs = outputs.iter().map(String::as_str).collect::<Vec<_>>();
    let (process, states) = ScriptedProcessService::completed(&output_refs);
    let (services, _) = host_services_for(host_id, process, Arc::new(PendingTimeService));
    let mut session = block_on(profile.open_session(services.clone())).expect("session opens");
    for (index, content) in ["first budget prompt", "second budget prompt"]
        .into_iter()
        .enumerate()
    {
        let mut turn = block_on(
            session.start_turn(
                TurnRequest::new(
                    RuntimeTurnId::new(format!("qwen-budget-turn-{}", index + 1))
                        .expect("valid turn"),
                    OperationContent::new(content).expect("valid content"),
                )
                .with_deadline(Deadline::at(MonotonicInstant::from_ticks(1_000))),
                services.clone(),
            ),
        )
        .expect("turn starts");
        assert_eq!(
            block_on(
                turn.take_terminal_outcome()
                    .expect("turn terminal is available")
            )
            .status(),
            &TerminalStatus::Completed
        );
        assert_eq!(block_on(turn.close()), CleanupOutcome::Clean);
    }
    assert_eq!(block_on(session.close()), CleanupOutcome::Clean);

    let interrupted = RuntimeTurnId::new("qwen-budget-interrupted").expect("valid turn");
    let restoration = profile.prepare_working_state_restoration(interrupted);
    let restored = block_on(restoration.restore(services.clone())).expect("replacement opens");
    let WorkingStateRestorationOutcome::SessionReplaced(replaced) = restored else {
        panic!("budget restoration reports a fresh replacement");
    };
    let (_, mut replacement) = replaced.into_parts();
    let mut turn = block_on(
        replacement.start_turn(
            TurnRequest::new(
                RuntimeTurnId::new("qwen-budget-replacement-turn").expect("valid turn"),
                OperationContent::new("replacement budget prompt").expect("valid content"),
            )
            .with_deadline(Deadline::at(MonotonicInstant::from_ticks(1_000))),
            services,
        ),
    )
    .expect("replacement turn starts");
    assert_eq!(
        block_on(
            turn.take_terminal_outcome()
                .expect("replacement terminal is available")
        )
        .status(),
        &TerminalStatus::Completed
    );
    assert_eq!(block_on(turn.close()), CleanupOutcome::Clean);

    for (index, arguments) in states.iter().map(|state| state.request().arguments).enumerate()
    {
        assert_eq!(flag_value(&arguments, "--max-wall-time"), Some("60s"));
        assert_eq!(flag_value(&arguments, "--max-tool-calls"), Some("4"));
        assert_eq!(flag_value(&arguments, "--max-session-turns"), Some("8"));
        assert_eq!(flag_value(&arguments, "--input-format"), Some("stream-json"));
        if index == 1 {
            assert_eq!(
                arguments
                    .windows(2)
                    .find(|pair| pair[0] == "--resume")
                    .map(|pair| pair[1].as_str()),
                Some("reasoning-session-1")
            );
        } else {
            assert!(!arguments.iter().any(|argument| argument == "--resume"));
        }
    }
}

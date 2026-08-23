fn ordinary_success_jsonl(version: &str) -> String {
    include_str!("../fixtures/qwen-code-v0.19.11/success.jsonl").replace(
        "\"qwen_code_version\":\"0.19.11\"",
        &format!("\"qwen_code_version\":\"{version}\""),
    )
}

fn flag_value<'a>(arguments: &'a [String], flag: &str) -> Option<&'a str> {
    arguments.windows(2).find_map(|pair| {
        (pair[0] == flag).then_some(pair[1].as_str())
    })
}

fn budget_run_input(host_suffix: &str) -> QwenRunProfileInput {
    QwenRunProfileInput::new(
        RequestId::new(format!("qwen-budget-run-{host_suffix}")).expect("valid request"),
        QwenModelSelection::new(
            ModelRouteId::new(format!("qwen.budget.route.{host_suffix}")).expect("valid route"),
            ModelRouteRevision::new("1").expect("valid route revision"),
            ProviderId::new("alibaba-modelstudio").expect("valid provider"),
            ModelId::new("qwen3-coder-plus").expect("valid model"),
        ),
        OperationContent::new("budget fixture prompt").expect("valid prompt"),
        WorkingResourceRef::new("qwen.budget.workspace").expect("valid resource"),
        Deadline::at(MonotonicInstant::from_ticks(1_000)),
    )
}

#[test]
fn omitted_qwen_budgets_keep_current_argv_on_qualified_and_baseline_packages() {
    for version in ["0.19.11", "0.21.15"] {
        let host_id = ExecutionHostId::new(format!("fixture.qwen.budget.omit.{version}"))
            .expect("valid host");
        let (discovery_process, _) = FakeProcessService::completed(&format!("{version}\n"));
        let (discovery_services, _) = host_services_for(
            host_id.clone(),
            discovery_process,
            Arc::new(PendingTimeService),
        );
        let prepared = block_on(prepare_qwen_headless(
            preparation_input(host_id.clone()),
            probe(),
            discovery_services,
        ))
        .expect("Qwen prepares");
        let profile = prepared
            .prepare_run(budget_run_input(version))
            .expect("omitted budgets prepare");
        assert_eq!(profile.evidence().budgets(), QwenHeadlessBudgets::omitted());

        let (process, state) = FakeProcessService::completed(&ordinary_success_jsonl(version));
        let (services, _) =
            host_services_for(host_id, process, Arc::new(PendingTimeService));
        let mut run = block_on(profile.start_run(services)).expect("run starts");
        let terminal = block_on(
            run.take_terminal_outcome()
                .expect("terminal outcome is available"),
        );
        assert_eq!(terminal.status(), &TerminalStatus::Completed);
        assert_eq!(block_on(run.close()), CleanupOutcome::Clean);
        let arguments = state.request().arguments;
        assert_eq!(flag_value(&arguments, "--max-wall-time"), Some("60s"));
        assert_eq!(flag_value(&arguments, "--max-tool-calls"), Some("16"));
        assert_eq!(flag_value(&arguments, "--max-session-turns"), Some("24"));
        assert_eq!(flag_value(&arguments, "--input-format"), Some("text"));
    }
}

#[test]
fn independently_omitted_qwen_budget_keeps_the_other_current_argv() {
    let host_id = ExecutionHostId::new("fixture.qwen.budget.independent").expect("valid host");
    let (discovery_process, _) = FakeProcessService::completed("0.21.15\n");
    let (discovery_services, _) = host_services_for(
        host_id.clone(),
        discovery_process,
        Arc::new(PendingTimeService),
    );
    let prepared = block_on(prepare_qwen_headless(
        preparation_input(host_id.clone()),
        probe(),
        discovery_services,
    ))
    .expect("Qwen 0.21.15 prepares");
    let profile = prepared
        .prepare_run(
            budget_run_input("independent").with_session_turn_budget(
                QwenSessionTurnBudget::try_new(12).expect("admitted turns"),
            ),
        )
        .expect("independent turn budget prepares");
    assert_eq!(
        profile.evidence().budgets().session_turns(),
        QwenSessionTurnBudget::try_new(12)
    );
    assert_eq!(profile.evidence().budgets().tool_calls(), None);

    let (process, state) = FakeProcessService::completed(&ordinary_success_jsonl("0.21.15"));
    let (services, _) = host_services_for(host_id, process, Arc::new(PendingTimeService));
    let mut run = block_on(profile.start_run(services)).expect("run starts");
    assert_eq!(
        block_on(
            run.take_terminal_outcome()
                .expect("terminal outcome is available")
        )
        .status(),
        &TerminalStatus::Completed
    );
    assert_eq!(block_on(run.close()), CleanupOutcome::Clean);
    let arguments = state.request().arguments;
    assert_eq!(flag_value(&arguments, "--max-wall-time"), Some("60s"));
    assert_eq!(flag_value(&arguments, "--max-tool-calls"), Some("16"));
    assert_eq!(flag_value(&arguments, "--max-session-turns"), Some("12"));
}

#[test]
fn selected_qwen_budgets_dispatch_exact_child_argv_on_0_21_15() {
    let turns = QwenSessionTurnBudget::try_new(1).expect("admitted turns");
    let tools = QwenToolCallBudget::try_new(0).expect("admitted tools");
    let host_id = ExecutionHostId::new("fixture.qwen.budget.selected").expect("valid host");
    let (discovery_process, _) = FakeProcessService::completed("0.21.15\n");
    let (discovery_services, _) = host_services_for(
        host_id.clone(),
        discovery_process,
        Arc::new(PendingTimeService),
    );
    let prepared = block_on(prepare_qwen_headless(
        preparation_input(host_id.clone()),
        probe(),
        discovery_services,
    ))
    .expect("Qwen 0.21.15 prepares");
    let profile = prepared
        .prepare_run(
            budget_run_input("selected")
                .with_session_turn_budget(turns)
                .with_tool_call_budget(tools),
        )
        .expect("selected budgets prepare");
    assert_eq!(profile.evidence().budgets().session_turns(), Some(turns));
    assert_eq!(profile.evidence().budgets().tool_calls(), Some(tools));

    let (process, state) = FakeProcessService::completed(&ordinary_success_jsonl("0.21.15"));
    let (services, _) = host_services_for(host_id, process, Arc::new(PendingTimeService));
    let mut run = block_on(profile.start_run(services)).expect("run starts");
    assert_eq!(
        block_on(
            run.take_terminal_outcome()
                .expect("terminal outcome is available")
        )
        .status(),
        &TerminalStatus::Completed
    );
    assert_eq!(block_on(run.close()), CleanupOutcome::Clean);
    let arguments = state.request().arguments;
    assert_eq!(flag_value(&arguments, "--max-wall-time"), Some("60s"));
    assert_eq!(flag_value(&arguments, "--max-tool-calls"), Some("0"));
    assert_eq!(flag_value(&arguments, "--max-session-turns"), Some("1"));
    assert_eq!(flag_value(&arguments, "--approval-mode"), Some("default"));
}

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

#[test]
fn selected_qwen_budgets_reject_unqualified_version_and_keep_native_terminals() {
    let turns = QwenSessionTurnBudget::try_new(2).expect("admitted turns");
    let host_id = ExecutionHostId::new("fixture.qwen.budget.reject.0.19.11").expect("valid host");
    let (discovery_process, _) = FakeProcessService::completed("0.19.11\n");
    let (discovery_services, _) = host_services_for(
        host_id.clone(),
        discovery_process,
        Arc::new(PendingTimeService),
    );
    let prepared = block_on(prepare_qwen_headless(
        preparation_input(host_id),
        probe(),
        discovery_services,
    ))
    .expect("baseline Qwen prepares");
    let error = prepared
        .prepare_run(budget_run_input("reject").with_session_turn_budget(turns))
        .expect_err("selected budgets require 0.21.15");
    assert_eq!(
        error.diagnostic().safe().code(),
        "swallowtail.qwen.preparation.budget_unsupported"
    );

    let host_id = ExecutionHostId::new("fixture.qwen.budget.terminal").expect("valid host");
    let (discovery_process, _) = FakeProcessService::completed("0.21.15\n");
    let (discovery_services, _) = host_services_for(
        host_id.clone(),
        discovery_process,
        Arc::new(PendingTimeService),
    );
    let prepared = block_on(prepare_qwen_headless(
        preparation_input(host_id.clone()),
        probe(),
        discovery_services,
    ))
    .expect("Qwen 0.21.15 prepares");
    let profile = prepared
        .prepare_run(
            budget_run_input("terminal").with_tool_call_budget(
                QwenToolCallBudget::try_new(0).expect("admitted tools"),
            ),
        )
        .expect("zero-tool budget prepares");
    let (process, state) = FakeProcessService::with_exit(
        "",
        swallowtail_runtime::ProcessExit::new(false, Some(55)),
    );
    let (services, _) = host_services_for(host_id, process, Arc::new(PendingTimeService));
    let mut run = block_on(profile.start_run(services)).expect("run starts");
    let terminal = block_on(
        run.take_terminal_outcome()
            .expect("terminal outcome is available"),
    );
    match terminal.status() {
        TerminalStatus::ProviderFailed(diagnostic) => {
            assert_eq!(diagnostic.code(), "swallowtail.qwen.headless.native_budget")
        }
        status => panic!("expected native budget terminal, got {status:?}"),
    }
    assert_eq!(block_on(run.close()), CleanupOutcome::Clean);
    assert_eq!(flag_value(&state.request().arguments, "--max-tool-calls"), Some("0"));
}

#[test]
fn selected_qwen_budget_driver_rejects_older_plan_before_process_start() {
    let host_new = ExecutionHostId::new("fixture.qwen.budget.mismatch.new").expect("valid host");
    let (discovery_new, _) = FakeProcessService::completed("0.21.15\n");
    let (services_new, _) = host_services_for(
        host_new.clone(),
        discovery_new,
        Arc::new(PendingTimeService),
    );
    let prepared_new = block_on(prepare_qwen_headless(
        preparation_input(host_new),
        probe(),
        services_new,
    ))
    .expect("0.21.15 prepares");
    let selected = prepared_new
        .prepare_run(
            budget_run_input("mismatch-new").with_tool_call_budget(
                QwenToolCallBudget::try_new(1).expect("admitted tools"),
            ),
        )
        .expect("selected budgets prepare");

    let host_old = ExecutionHostId::new("fixture.qwen.budget.mismatch.old").expect("valid host");
    let (discovery_old, _) = FakeProcessService::completed("0.19.11\n");
    let (services_old, _) = host_services_for(
        host_old.clone(),
        discovery_old,
        Arc::new(PendingTimeService),
    );
    let prepared_old = block_on(prepare_qwen_headless(
        preparation_input(host_old.clone()),
        probe(),
        services_old,
    ))
    .expect("0.19.11 prepares");
    let omitted = prepared_old
        .prepare_run(budget_run_input("mismatch-old"))
        .expect("omitted budgets prepare");

    let (process, state) = FakeProcessService::completed("");
    let (services, _) = host_services_for(host_old, process, Arc::new(PendingTimeService));
    let error = match block_on(selected.low_level_driver().start_run(
        omitted.plan().clone(),
        omitted.request().clone(),
        services,
    )) {
        Ok(_) => panic!("budget-selected driver must reject an older plan"),
        Err(error) => error,
    };
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.qwen.headless.budget_version_mismatch"
    );
    assert!(!state.started());
}

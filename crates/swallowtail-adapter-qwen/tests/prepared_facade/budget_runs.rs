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
fn selected_qwen_budgets_reject_unqualified_version_and_keep_native_terminals() {
    let turns = QwenSessionTurnBudget::try_new(2).expect("admitted turns");
    for version in ["0.19.11", "0.22.0", "0.22.1"] {
        let host_id = ExecutionHostId::new(format!("fixture.qwen.budget.reject.{version}"))
            .expect("valid host");
        let (discovery_process, _) = FakeProcessService::completed(&format!("{version}\n"));
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
        .expect("Qwen prepares");
        let error = prepared
            .prepare_run(budget_run_input("reject").with_session_turn_budget(turns))
            .expect_err("selected budgets require exact 0.21.15");
        assert_eq!(
            error.diagnostic().safe().code(),
            "swallowtail.qwen.preparation.budget_unsupported"
        );
    }

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

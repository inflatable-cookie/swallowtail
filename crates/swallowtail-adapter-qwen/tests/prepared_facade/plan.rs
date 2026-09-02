fn plan_success_jsonl(version: &str) -> String {
    include_str!("../fixtures/qwen-code-0.21.15/plan-success.jsonl").replace("0.21.15", version)
}

fn plan_first_turn_jsonl(version: &str) -> String {
    include_str!("../fixtures/qwen-code-0.21.15/plan-interactive-first-turn.jsonl")
        .replace("0.21.15", version)
}

fn plan_continued_turn_jsonl(version: &str) -> String {
    include_str!("../fixtures/qwen-code-0.21.15/plan-interactive-continued-turn.jsonl")
        .replace("0.21.15", version)
}

#[test]
fn qwen_plan_dispatches_canonical_approval_mode_on_every_admitted_package() {
    for version in ["0.21.15", "0.22.0", "0.22.1", "0.22.2", "0.22.3"] {
        let host_id =
            ExecutionHostId::new(format!("fixture.qwen.plan.{version}")).expect("valid host");
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
        .expect("qualified Qwen prepares");
        let profile = prepared
            .prepare_run(
                QwenRunProfileInput::new(
                    RequestId::new(format!("qwen-plan-run-{version}")).expect("valid request"),
                    QwenModelSelection::new(
                        ModelRouteId::new(format!("qwen.plan.route.{version}"))
                            .expect("valid route"),
                        ModelRouteRevision::new("1").expect("valid route revision"),
                        ProviderId::new("alibaba-modelstudio").expect("valid provider"),
                        ModelId::new("qwen3-coder-plus").expect("valid model"),
                    ),
                    OperationContent::new("plan fixture prompt").expect("valid prompt"),
                    WorkingResourceRef::new("qwen.plan.workspace").expect("valid resource"),
                    Deadline::at(MonotonicInstant::from_ticks(1_000)),
                )
                .with_harness_mode(HarnessMode::Plan),
            )
            .expect("qualified Plan prepares");
        assert_eq!(profile.evidence().harness_mode(), Some(HarnessMode::Plan));
        assert_eq!(
            profile.request().policy().harness_mode(),
            Some(HarnessMode::Plan)
        );
        assert!(profile.plan().requirements().capabilities().any(
            |requirement| requirement.capability() == swallowtail_core::Capability::HarnessModeSelection
        ));
        assert_prepared_operation_evidence_matches_plan(
            profile.evidence().operation(),
            profile.plan(),
        );

        let (operation_process, operation_state) =
            FakeProcessService::completed(&plan_success_jsonl(version));
        let (operation_services, _) =
            host_services_for(host_id, operation_process, Arc::new(PendingTimeService));
        let mut run = block_on(profile.start_run(operation_services)).expect("plan run starts");
        let terminal = block_on(
            run.take_terminal_outcome()
                .expect("terminal outcome is available"),
        );
        assert_eq!(terminal.status(), &TerminalStatus::Completed);
        assert_eq!(block_on(run.close()), CleanupOutcome::Clean);
        assert_eq!(flag_value(&operation_state.request().arguments, "--approval-mode"), Some("plan"));
        assert!(operation_state
            .request()
            .arguments
            .iter()
            .any(|argument| argument == "--safe-mode"));
        assert_eq!(operation_state.stdin(), b"plan fixture prompt");
    }
}

#[test]
fn qwen_plan_rejects_unqualified_versions_before_process_work() {
    for version in ["0.19.11", "0.21.14"] {
        let host_id = ExecutionHostId::new(format!("fixture.qwen.plan.reject.{version}"))
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
        .expect("Qwen discovery prepares");
        let error = prepared
            .prepare_run(
                QwenRunProfileInput::new(
                    RequestId::new("qwen-plan-reject").expect("valid request"),
                    QwenModelSelection::new(
                        ModelRouteId::new("qwen.plan.reject.route").expect("valid route"),
                        ModelRouteRevision::new("1").expect("valid route revision"),
                        ProviderId::new("alibaba-modelstudio").expect("valid provider"),
                        ModelId::new("qwen3-coder-plus").expect("valid model"),
                    ),
                    OperationContent::new("rejected plan prompt").expect("valid prompt"),
                    WorkingResourceRef::new("qwen.plan.reject.workspace").expect("valid resource"),
                    Deadline::at(MonotonicInstant::from_ticks(1_000)),
                )
                .with_harness_mode(HarnessMode::Plan),
            )
            .expect_err("unqualified Plan must reject");
        assert_eq!(
            error.diagnostic().safe().code(),
            "swallowtail.qwen.preparation.harness_mode_unsupported"
        );
    }
}

#[test]
fn qwen_plan_session_reapplies_the_same_approval_mode_on_resume_and_replacement() {
    let host_id = ExecutionHostId::new("fixture.qwen.plan.session").expect("valid host");
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
        .prepare_session(
            QwenSessionProfileInput::new(
                RequestId::new("qwen-plan-session").expect("valid request"),
                QwenModelSelection::new(
                    ModelRouteId::new("qwen.plan.session.route").expect("valid route"),
                    ModelRouteRevision::new("1").expect("valid route revision"),
                    ProviderId::new("alibaba-modelstudio").expect("valid provider"),
                    ModelId::new("qwen3-coder-plus").expect("valid model"),
                ),
                WorkingResourceRef::new("qwen.plan.session.workspace").expect("valid resource"),
            )
            .with_harness_mode(HarnessMode::Plan),
        )
        .expect("qualified Plan session prepares");
    assert_eq!(profile.evidence().harness_mode(), Some(HarnessMode::Plan));
    assert_eq!(
        profile.request().options().harness_mode(),
        Some(HarnessMode::Plan)
    );
    let interrupted = RuntimeTurnId::new("lost-qwen-plan-turn").expect("valid turn");
    let restoration = profile.prepare_working_state_restoration(interrupted.clone());
    assert_eq!(
        restoration.method(),
        WorkingStateRestorationMethod::FreshSessionReplacement
    );

    let first_turn = plan_first_turn_jsonl("0.21.15");
    let continued_turn = plan_continued_turn_jsonl("0.21.15");
    let (process, states) =
        ScriptedProcessService::completed(&[&first_turn, &continued_turn, &first_turn]);
    let (services, _) = host_services_for(host_id, process, Arc::new(PendingTimeService));
    let mut session = block_on(profile.open_session(services.clone())).expect("session opens");

    for (index, content) in ["first plan prompt", "second plan prompt"]
        .into_iter()
        .enumerate()
    {
        let mut turn = block_on(
            session.start_turn(
                TurnRequest::new(
                    RuntimeTurnId::new(format!("qwen-plan-turn-{}", index + 1)).expect("valid turn"),
                    OperationContent::new(content).expect("valid content"),
                )
                .with_deadline(Deadline::at(MonotonicInstant::from_ticks(1_000))),
                services.clone(),
            ),
        )
        .expect("turn starts");
        let terminal = block_on(
            turn.take_terminal_outcome()
                .expect("turn terminal is available"),
        );
        assert_eq!(terminal.status(), &TerminalStatus::Completed);
        assert_eq!(block_on(turn.close()), CleanupOutcome::Clean);
    }

    assert_eq!(flag_value(&states[0].request().arguments, "--approval-mode"), Some("plan"));
    assert!(!states[0]
        .request()
        .arguments
        .iter()
        .any(|argument| argument == "--resume"));
    assert_eq!(flag_value(&states[1].request().arguments, "--approval-mode"), Some("plan"));
    assert_eq!(
        flag_value(&states[1].request().arguments, "--resume"),
        Some("123e4567-e89b-12d3-a456-426614174000")
    );
    assert_eq!(
        block_on(close_session(session, services.clone())),
        CleanupOutcome::Clean
    );

    let restored = block_on(restoration.restore(services.clone())).expect("replacement opens");
    let WorkingStateRestorationOutcome::SessionReplaced(replaced) = restored else {
        panic!("plan restoration reports a fresh replacement");
    };
    assert_eq!(replaced.interrupted_turn_id(), &interrupted);
    let (_, mut replacement) = replaced.into_parts();
    let mut turn = block_on(
        replacement.start_turn(
            TurnRequest::new(
                RuntimeTurnId::new("qwen-plan-replacement-turn").expect("valid turn"),
                OperationContent::new("replacement plan prompt").expect("valid content"),
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
    assert_eq!(
        flag_value(&replacement_arguments, "--approval-mode"),
        Some("plan")
    );
    assert!(!replacement_arguments
        .iter()
        .any(|argument| argument == "--resume"));
    assert!(!replacement_arguments
        .iter()
        .any(|argument| argument == "--continue"));
    assert_eq!(
        block_on(close_session(replacement, services)),
        CleanupOutcome::Clean
    );
}

#[test]
fn qwen_plan_composes_with_exact_reasoning_on_0_21_15() {
    let host_id = ExecutionHostId::new("fixture.qwen.plan.reasoning").expect("valid host");
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
    let mode = ReasoningMode::new("high").expect("canonical mode is valid");
    let profile = prepared
        .prepare_run(
            QwenRunProfileInput::new(
                RequestId::new("qwen-plan-reasoning").expect("valid request"),
                QwenModelSelection::new(
                    ModelRouteId::new("qwen.plan.reasoning.route").expect("valid route"),
                    ModelRouteRevision::new("1").expect("valid route revision"),
                    ProviderId::new("alibaba-modelstudio").expect("valid provider"),
                    ModelId::new("qwen3.8-max").expect("valid model"),
                ),
                OperationContent::new("plan reasoning prompt").expect("valid prompt"),
                WorkingResourceRef::new("qwen.plan.reasoning.workspace").expect("valid resource"),
                Deadline::at(MonotonicInstant::from_ticks(1_000)),
            )
            .with_harness_mode(HarnessMode::Plan)
            .with_reasoning_mode(mode.clone()),
        )
        .expect("Plan plus reasoning prepares");
    assert_eq!(profile.evidence().harness_mode(), Some(HarnessMode::Plan));
    assert_eq!(profile.evidence().reasoning_mode(), Some(&mode));

    let initialize = serde_json::json!({
        "type": "control_response",
        "response": {
            "subtype": "success",
            "request_id": "swallowtail-initialize",
            "response": {
                "subtype": "initialize",
                "session_id": "fixture-control-session",
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
    let mut stream = Vec::new();
    stream.extend_from_slice(initialize.to_string().as_bytes());
    stream.push(b'\n');
    stream.extend_from_slice(effort.to_string().as_bytes());
    stream.push(b'\n');
    stream.extend_from_slice(
        include_str!("../fixtures/qwen-code-0.21.15/reasoning-success.jsonl")
            .replace("\"permission_mode\":\"default\"", "\"permission_mode\":\"plan\"")
            .as_bytes(),
    );
    let (operation_process, operation_state) =
        FakeProcessService::completed(&String::from_utf8(stream).expect("utf8"));
    let (operation_services, _) =
        host_services_for(host_id, operation_process, Arc::new(PendingTimeService));
    let mut run = block_on(profile.start_run(operation_services)).expect("composed run starts");
    let terminal = block_on(
        run.take_terminal_outcome()
            .expect("terminal outcome is available"),
    );
    assert_eq!(terminal.status(), &TerminalStatus::Completed);
    assert_eq!(block_on(run.close()), CleanupOutcome::Clean);
    assert_eq!(flag_value(&operation_state.request().arguments, "--approval-mode"), Some("plan"));
    assert_eq!(
        flag_value(&operation_state.request().arguments, "--input-format"),
        Some("stream-json")
    );
}

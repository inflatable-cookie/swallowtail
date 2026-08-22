#[test]
fn prepared_runs_preserve_qwen_stdin_budgets_and_ambient_truth_in_both_topologies() {
    for host_value in [
        "fixture.qwen.prepared.local",
        "fixture.qwen.prepared.remote",
    ] {
        let host_id = ExecutionHostId::new(host_value).expect("valid host");
        let (discovery_process, discovery_state) = FakeProcessService::completed("0.19.11\n");
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
        assert_eq!(discovery_state.request().arguments, ["--version"]);

        let profile = prepared
            .prepare_run(QwenRunProfileInput::new(
                RequestId::new("qwen-prepared-run").expect("valid request"),
                QwenModelSelection::new(
                    ModelRouteId::new("qwen.prepared.route").expect("valid route"),
                    ModelRouteRevision::new("1").expect("valid route revision"),
                    ProviderId::new("alibaba-modelstudio").expect("valid provider"),
                    ModelId::new("qwen3-coder-plus").expect("valid model"),
                ),
                OperationContent::new("prepared private prompt").expect("valid prompt"),
                WorkingResourceRef::new("qwen.prepared.workspace").expect("valid resource"),
                Deadline::at(MonotonicInstant::from_ticks(1_000)),
            ))
            .expect("Qwen run profile prepares");
        assert_eq!(
            profile.plan().harness_configuration_posture(),
            Some(HarnessConfigurationPosture::Ambient)
        );
        assert_eq!(
            profile.plan().requirements().harness_isolation(),
            Some(HarnessIsolation::AmbientHost)
        );
        assert_eq!(
            profile.plan().provider_id().map(ProviderId::as_str),
            Some("alibaba-modelstudio")
        );
        assert_prepared_operation_evidence_matches_plan(
            profile.evidence().operation(),
            profile.plan(),
        );
        assert_eq!(
            profile.evidence().observable_activity().availability(),
            ObservableActivityAvailability::Available
        );

        let (operation_process, operation_state) = FakeProcessService::completed(include_str!(
            "../fixtures/qwen-code-v0.19.11/success.jsonl"
        ));
        let (operation_services, _) =
            host_services_for(host_id, operation_process, Arc::new(PendingTimeService));
        let mut run = block_on(profile.start_run(operation_services)).expect("prepared run starts");
        let terminal = block_on(
            run.take_terminal_outcome()
                .expect("terminal outcome is available"),
        );
        assert_eq!(terminal.status(), &TerminalStatus::Completed);
        assert_eq!(block_on(run.close()), CleanupOutcome::Clean);
        assert_eq!(operation_state.stdin(), b"prepared private prompt");
        let arguments = operation_state.request().arguments;
        for exact in [
            "--input-format",
            "text",
            "--output-format",
            "stream-json",
            "--max-wall-time",
            "60s",
            "--max-tool-calls",
            "16",
            "--max-session-turns",
            "24",
        ] {
            assert!(arguments.iter().any(|argument| argument == exact));
        }
        assert!(!arguments.iter().any(|argument| argument == "--sandbox"));
    }
}

#[test]
fn qwen_reasoning_dispatches_each_canonical_mode_through_exact_control_handshake() {
    for (model_index, model) in ["qwen3.8-max", "qwen3.8-max-preview"]
        .into_iter()
        .enumerate()
    {
        for (value_index, value) in ["low", "medium", "high", "xhigh", "max"]
            .into_iter()
            .enumerate()
        {
            let index = model_index * 5 + value_index;
            let host_id = ExecutionHostId::new(format!("fixture.qwen.reasoning.{index}"))
                .expect("valid host");
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
        let mode = ReasoningMode::new(value).expect("canonical mode is valid");
        let profile = prepared
            .prepare_run(
                QwenRunProfileInput::new(
                    RequestId::new(format!("qwen-reasoning-run-{index}"))
                        .expect("valid request"),
                    QwenModelSelection::new(
                        ModelRouteId::new(format!("qwen.reasoning.route.{index}"))
                            .expect("valid route"),
                        ModelRouteRevision::new("1").expect("valid route revision"),
                        ProviderId::new("alibaba-modelstudio").expect("valid provider"),
                        ModelId::new(model).expect("valid model"),
                    ),
                    OperationContent::new("reasoning fixture prompt").expect("valid prompt"),
                    WorkingResourceRef::new("qwen.reasoning.workspace").expect("valid resource"),
                    Deadline::at(MonotonicInstant::from_ticks(1_000)),
                )
                .with_reasoning_mode(mode.clone()),
            )
            .expect("qualified reasoning prepares");
        assert_eq!(profile.evidence().reasoning_mode(), Some(&mode));
        assert_eq!(profile.request().policy().reasoning_mode(), Some(&mode));
        assert!(profile
            .plan()
            .requirements()
            .capabilities()
            .any(|requirement| requirement.capability()
                == swallowtail_core::Capability::ReasoningSelection));

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
                    "effort": value,
                    "applied": true,
                    "override": null
                }
            }
        });
        let output = format!(
                    "{}\n{}\n{}",
            initialize,
            effort,
            if model == "qwen3.8-max" {
                include_str!("../fixtures/qwen-code-0.21.15/reasoning-success.jsonl")
            } else {
                include_str!("../fixtures/qwen-code-0.21.15/reasoning-preview-success.jsonl")
            }
        );
        let (operation_process, operation_state) = FakeProcessService::completed(&output);
        let (operation_services, _) =
            host_services_for(host_id, operation_process, Arc::new(PendingTimeService));
        let mut run = block_on(profile.start_run(operation_services)).expect("run starts");
        let terminal = block_on(
            run.take_terminal_outcome()
                .expect("terminal outcome is available"),
        );
        assert_eq!(terminal.status(), &TerminalStatus::Completed);
        assert_eq!(block_on(run.close()), CleanupOutcome::Clean);

        let arguments = operation_state.request().arguments;
        assert!(arguments.windows(2).any(|pair| pair == ["--input-format", "stream-json"]));
        assert!(!arguments.iter().any(|argument| argument == "text"));
        assert!(arguments
            .windows(2)
            .any(|pair| pair == ["--model", model]));
        let messages = String::from_utf8(operation_state.stdin())
            .expect("control stdin is UTF-8")
            .lines()
            .map(|line| serde_json::from_str::<serde_json::Value>(line).expect("valid message"))
            .collect::<Vec<_>>();
        assert_eq!(messages.len(), 3);
        assert_eq!(messages[0].pointer("/request/subtype").and_then(|v| v.as_str()), Some("initialize"));
        assert_eq!(messages[1].pointer("/request/subtype").and_then(|v| v.as_str()), Some("set_effort"));
        assert_eq!(messages[1].pointer("/request/effort").and_then(|v| v.as_str()), Some(value));
        assert_eq!(messages[2].get("type").and_then(|v| v.as_str()), Some("user"));
        assert_eq!(messages[2].get("session_id").and_then(|v| v.as_str()), Some("fixture-control-session"));
        }
    }
}

#[test]
fn qwen_reasoning_rejects_unqualified_version_model_and_alias_before_operation_work() {
    for (version, provider, model, value) in [
        ("0.19.11", "alibaba-modelstudio", "qwen3.8-max", "high"),
        ("0.21.15", "alibaba-modelstudio", "qwen3-coder-plus", "high"),
        ("0.21.15", "alibaba-modelstudio", "qwen3.8-max", "med"),
        ("0.21.15", "other-provider", "qwen3.8-max", "high"),
    ] {
        let host_id = ExecutionHostId::new(format!("fixture.qwen.reasoning.reject.{version}.{model}"))
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
                    RequestId::new("qwen-reasoning-reject").expect("valid request"),
                    QwenModelSelection::new(
                        ModelRouteId::new("qwen.reasoning.reject.route").expect("valid route"),
                        ModelRouteRevision::new("1").expect("valid route revision"),
                        ProviderId::new(provider).expect("valid provider"),
                        ModelId::new(model).expect("valid model"),
                    ),
                    OperationContent::new("rejected reasoning prompt").expect("valid prompt"),
                    WorkingResourceRef::new("qwen.reasoning.reject.workspace")
                        .expect("valid resource"),
                    Deadline::at(MonotonicInstant::from_ticks(1_000)),
                )
                .with_reasoning_mode(ReasoningMode::new(value).expect("mode is valid")),
            )
            .expect_err("unqualified reasoning must reject");
        assert_eq!(
            error.diagnostic().safe().code(),
            "swallowtail.qwen.preparation.reasoning_unsupported"
        );
    }
}

#[test]
fn qwen_reasoning_request_mismatch_fails_before_process_start() {
    let host_id = ExecutionHostId::new("fixture.qwen.reasoning.mismatch").expect("valid host");
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
    .expect("Qwen prepares");
    let profile = prepared
        .prepare_run(
            QwenRunProfileInput::new(
                RequestId::new("qwen-reasoning-mismatch").expect("valid request"),
                QwenModelSelection::new(
                    ModelRouteId::new("qwen.reasoning.mismatch.route").expect("valid route"),
                    ModelRouteRevision::new("1").expect("valid route revision"),
                    ProviderId::new("alibaba-modelstudio").expect("valid provider"),
                    ModelId::new("qwen3.8-max").expect("valid model"),
                ),
                OperationContent::new("mismatch prompt").expect("valid prompt"),
                WorkingResourceRef::new("qwen.reasoning.mismatch.workspace")
                    .expect("valid resource"),
                Deadline::at(MonotonicInstant::from_ticks(1_000)),
            )
            .with_reasoning_mode(ReasoningMode::new("high").expect("mode is valid")),
        )
        .expect("qualified reasoning prepares");
    let request = StructuredRunRequest::new(
        RequestId::new("qwen-reasoning-mismatch-request").expect("valid request"),
        OperationContent::new("mismatch prompt").expect("valid prompt"),
        OperationPolicy::offline()
            .with_reasoning_mode(ReasoningMode::new("low").expect("mode is valid"))
            .with_provider_retention(ProviderRetentionPolicy::DurableAllowed)
            .with_harness_isolation(HarnessIsolation::AmbientHost)
            .with_harness_configuration_posture(HarnessConfigurationPosture::Ambient),
    )
    .with_working_resource(WorkingResourceRef::new("qwen.reasoning.mismatch.workspace").expect("valid resource"))
    .with_deadline(Deadline::at(MonotonicInstant::from_ticks(1_000)));
    let (process, state) = FakeProcessService::completed("");
    let (services, _) = host_services_for(host_id, process, Arc::new(PendingTimeService));
    let error = match block_on(
        QwenHeadlessDriver::new(
            EnvironmentRef::new("qwen.environment").expect("valid environment"),
        )
        .start_run(profile.plan().clone(), request, services),
    ) {
        Ok(_) => panic!("mismatched reasoning must reject"),
        Err(error) => error,
    };
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.qwen.headless.reasoning_plan_mismatch"
    );
    assert!(!state.started());
}

#[test]
fn qwen_reasoning_rejects_ambient_override_or_provider_substitution_before_prompt() {
    for (reported_effort, applied, override_value) in [
        ("low", true, serde_json::Value::Null),
        ("high", false, serde_json::json!("samplingParams.enable_thinking")),
    ] {
        let host_id = ExecutionHostId::new(format!(
            "fixture.qwen.reasoning.control-reject.{}",
            reported_effort
        ))
        .expect("valid host");
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
        .expect("Qwen prepares");
        let profile = prepared
            .prepare_run(
                QwenRunProfileInput::new(
                    RequestId::new(format!("qwen-reasoning-control-{reported_effort}"))
                        .expect("valid request"),
                    QwenModelSelection::new(
                        ModelRouteId::new("qwen.reasoning.control.route").expect("valid route"),
                        ModelRouteRevision::new("1").expect("valid route revision"),
                        ProviderId::new("alibaba-modelstudio").expect("valid provider"),
                        ModelId::new("qwen3.8-max").expect("valid model"),
                    ),
                    OperationContent::new("control rejection prompt").expect("valid prompt"),
                    WorkingResourceRef::new("qwen.reasoning.control.workspace")
                        .expect("valid resource"),
                    Deadline::at(MonotonicInstant::from_ticks(1_000)),
                )
                .with_reasoning_mode(ReasoningMode::new("high").expect("mode is valid")),
            )
            .expect("qualified reasoning prepares");
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
                    "effort": reported_effort,
                    "applied": applied,
                    "override": override_value
                }
            }
        });
        let output = format!("{}\n{}\n", initialize, effort);
        let (process, state) = FakeProcessService::completed(&output);
        let (services, _) = host_services_for(host_id, process, Arc::new(PendingTimeService));
        let error = match block_on(profile.start_run(services)) {
            Ok(_) => panic!("control mismatch must reject"),
            Err(error) => error,
        };
        assert_eq!(
            error.diagnostic().code(),
            "swallowtail.qwen.headless.reasoning_not_applied"
        );
        let stdin = String::from_utf8(state.stdin()).expect("control stdin is UTF-8");
        let messages = stdin
            .lines()
            .collect::<Vec<_>>();
        assert_eq!(messages.len(), 2);
        assert!(state.force_stopped());
    }
}

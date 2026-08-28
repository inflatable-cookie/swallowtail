#[test]
fn qwen_reasoning_rejects_unqualified_version_model_and_alias_before_operation_work() {
    for (version, provider, model, value) in [
        ("0.19.11", "alibaba-modelstudio", "qwen3.8-max", "high"),
        ("0.22.0", "alibaba-modelstudio", "qwen3.8-max", "high"),
        ("0.22.1", "alibaba-modelstudio", "qwen3.8-max", "high"),
        ("0.22.2", "alibaba-modelstudio", "qwen3.8-max", "high"),
        ("0.22.3", "alibaba-modelstudio", "qwen3.8-max", "high"),
        ("0.21.15", "alibaba-modelstudio", "qwen3-coder-plus", "high"),
        ("0.21.15", "alibaba-modelstudio", "qwen3.8-max", "med"),
        ("0.21.15", "other-provider", "qwen3.8-max", "high"),
    ] {
        let host_id = ExecutionHostId::new(format!(
            "fixture.qwen.reasoning.reject.{version}.{model}"
        ))
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
                        ModelRouteId::new("qwen.reasoning.reject.route")
                            .expect("valid route"),
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
    .with_working_resource(
        WorkingResourceRef::new("qwen.reasoning.mismatch.workspace").expect("valid resource"),
    )
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
        (
            "high",
            false,
            serde_json::json!("samplingParams.enable_thinking"),
        ),
    ] {
        let host_id = ExecutionHostId::new(format!(
            "fixture.qwen.reasoning.control-reject.{reported_effort}"
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
                        ModelRouteId::new("qwen.reasoning.control.route")
                            .expect("valid route"),
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
        let messages = stdin.lines().collect::<Vec<_>>();
        assert_eq!(messages.len(), 2);
        assert!(state.force_stopped());
    }
}

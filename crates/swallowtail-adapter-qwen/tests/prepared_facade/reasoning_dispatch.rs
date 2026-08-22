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
                        OperationContent::new("reasoning fixture prompt")
                            .expect("valid prompt"),
                        WorkingResourceRef::new("qwen.reasoning.workspace")
                            .expect("valid resource"),
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
            assert!(arguments
                .windows(2)
                .any(|pair| pair == ["--input-format", "stream-json"]));
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
                Some(value)
            );
            assert_eq!(messages[2].get("type").and_then(|value| value.as_str()), Some("user"));
            assert_eq!(
                messages[2]
                    .get("session_id")
                    .and_then(|value| value.as_str()),
                Some("fixture-control-session")
            );
        }
    }
}

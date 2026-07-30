#[test]
fn prepared_session_opt_in_exposes_one_shot_permission_exchange() {
    let host_id = ExecutionHostId::new("fixture.session.consumer-mediated").expect("valid host");
    let preparation_host = FixtureHost::new(Scenario::Version, "0.61.0");
    let prepared = block_on(prepare_claude_agent(
        preparation_input(host_id.clone()),
        probe(),
        preparation_host.services(host_id.clone()),
    ))
    .expect("Claude Agent prepares");
    let profile = prepared
        .prepare_session(
            ClaudeAgentSessionProfileInput::new(
                RequestId::new("claude-agent-session-consumer-mediated").expect("valid request"),
                ClaudeAgentModelSelection::new(
                    ModelRouteId::new("claude-agent.prepared.route").expect("valid route"),
                    ModelRouteRevision::new("1").expect("valid route revision"),
                    ModelId::new("claude-sonnet-4-6").expect("valid model"),
                ),
                WorkingResourceRef::new("claude-agent.prepared.workspace").expect("valid resource"),
                SessionOptions::default(),
            )
            .with_consumer_mediated_permissions(),
        )
        .expect("consumer-mediated session prepares");
    assert_eq!(
        profile
            .plan()
            .requirements()
            .extension_namespaces()
            .map(swallowtail_core::ExtensionNamespace::as_str)
            .collect::<Vec<_>>(),
        vec!["acp/session/request-permission"]
    );
    assert_eq!(
        profile.request().access_policy(),
        &SessionAccessPolicy::ambient_harness_with_consumer_mediated_requests(
            ResourceAccess::Read,
            [swallowtail_adapter_claude_agent::claude_agent_permission_namespace()],
        )
    );

    let operation_host = FixtureHost::new(Scenario::Permission, "0.61.0");
    let services = operation_host.services(host_id);
    let mut session =
        block_on(profile.open_session(services.clone())).expect("prepared session opens");
    let binding = session
        .resume_binding()
        .expect("session retains attachment authority")
        .clone();
    let mut turn = block_on(session.start_turn(
        TurnRequest::new(
            RuntimeTurnId::new("claude-agent-session-permission-turn").expect("valid turn"),
            OperationContent::new("request one provider permission").expect("valid prompt"),
        ),
        services,
    ))
    .expect("session turn starts");
    let mut callbacks = turn.take_callbacks().expect("permission callbacks exist");
    let mut requests = callbacks
        .take_requests()
        .expect("callback request stream exists");
    let callback = block_on(requests.next())
        .expect("permission callback arrives")
        .expect("permission callback is valid");
    let callback_id = callback.callback_id().clone();
    let turn_id = callback.turn_id().expect("callback retains turn").clone();
    let swallowtail_runtime::CallbackRequestKind::Extension(extension) = callback.kind() else {
        panic!("permission is a provider extension");
    };
    assert_eq!(
        extension.namespace().as_str(),
        "acp/session/request-permission"
    );
    block_on(
        callbacks.responder().respond(CallbackResponse::new(
            callback_id.clone(),
            turn_id,
            CallbackResult::Success(
                CallbackPayload::new(br#"{"optionId":"allow-once"}"#, 256)
                    .expect("selection is bounded"),
            ),
        )),
    )
    .expect("permission selection is transported");

    let mut events = turn.take_events().expect("events");
    let terminal = turn.take_terminal_outcome().expect("terminal");
    let (observed, outcome) = block_on(async {
        let mut observed = Vec::new();
        while let Some(event) = events.next().await {
            observed.push(event.expect("event succeeds"));
        }
        (observed, terminal.await)
    });
    assert!(observed.iter().any(|event| {
        matches!(
            event.kind(),
            swallowtail_runtime::RuntimeEventKind::CallbackRequested(id)
                if id == &callback_id
        )
    }));
    assert_eq!(outcome.status(), &TerminalStatus::Completed);
    assert_eq!(block_on(turn.close()), CleanupOutcome::NotApplicable);
    assert_eq!(block_on(session.close()), CleanupOutcome::Clean);
    assert_eq!(
        profile
            .load_request(
                RequestId::new("claude-agent-session-permission-load").expect("valid request"),
                binding.clone(),
            )
            .expect("consumer-mediated load derives")
            .access_policy(),
        profile.request().access_policy()
    );
    assert_eq!(
        profile
            .resume_request(
                RequestId::new("claude-agent-session-permission-resume").expect("valid request"),
                binding,
            )
            .expect("consumer-mediated resume derives")
            .access_policy(),
        profile.request().access_policy()
    );
    let writes = operation_host.writes();
    assert!(writes.iter().any(|message| {
        message.get("id").and_then(serde_json::Value::as_u64) == Some(900)
            && message["result"]["outcome"]["optionId"] == "allow-once"
    }));
    assert!(!writes.iter().any(|message| {
        message.get("method").and_then(serde_json::Value::as_str) == Some("session/cancel")
    }));
}

#[test]
fn prepared_structured_run_opt_in_exposes_one_shot_permission_exchange() {
    let host_id = ExecutionHostId::new("fixture.run.consumer-mediated").expect("valid host");
    let preparation_host = FixtureHost::new(Scenario::Version, "0.61.0");
    let prepared = block_on(prepare_claude_agent(
        preparation_input(host_id.clone()),
        probe(),
        preparation_host.services(host_id.clone()),
    ))
    .expect("Claude Agent prepares");
    let profile = prepared
        .prepare_run(
            ClaudeAgentRunProfileInput::new(
                RequestId::new("claude-agent-consumer-mediated").expect("valid request"),
                ClaudeAgentModelSelection::new(
                    ModelRouteId::new("claude-agent.prepared.route").expect("valid route"),
                    ModelRouteRevision::new("1").expect("valid route revision"),
                    ModelId::new("claude-sonnet-4-6").expect("valid model"),
                ),
                OperationContent::new("request one provider permission").expect("valid prompt"),
                WorkingResourceRef::new("claude-agent.prepared.workspace").expect("valid resource"),
                Some(Deadline::at(MonotonicInstant::from_ticks(u64::MAX))),
            )
            .with_consumer_mediated_permissions(),
        )
        .expect("consumer-mediated run prepares");
    assert_eq!(
        profile
            .plan()
            .requirements()
            .extension_namespaces()
            .map(swallowtail_core::ExtensionNamespace::as_str)
            .collect::<Vec<_>>(),
        vec!["acp/session/request-permission"]
    );

    let operation_host = FixtureHost::new(Scenario::Permission, "0.61.0");
    let mut run = block_on(profile.start_run(operation_host.services(host_id)))
        .expect("structured run starts");
    let mut callbacks = run.take_callbacks().expect("permission callbacks exist");
    let mut requests = callbacks
        .take_requests()
        .expect("callback request stream exists");
    let callback = block_on(requests.next())
        .expect("permission callback arrives")
        .expect("permission callback is valid");
    let callback_id = callback.callback_id().clone();
    let turn_id = callback.turn_id().expect("callback retains turn").clone();
    assert_eq!(
        callback.deadline(),
        Some(Deadline::at(MonotonicInstant::from_ticks(u64::MAX)))
    );
    let swallowtail_runtime::CallbackRequestKind::Extension(extension) = callback.kind() else {
        panic!("permission is a provider extension");
    };
    assert_eq!(
        extension.namespace().as_str(),
        "acp/session/request-permission"
    );
    let payload: serde_json::Value =
        serde_json::from_slice(extension.payload()).expect("permission payload is JSON");
    assert_eq!(payload["toolCall"]["toolCallId"], "shell-1");
    assert_eq!(payload["options"].as_array().expect("options").len(), 2);

    let responder = callbacks.responder();
    assert!(
        block_on(
            responder.respond(CallbackResponse::new(
                callback_id.clone(),
                swallowtail_runtime::RuntimeTurnId::new("wrong-turn").expect("valid turn"),
                CallbackResult::Success(
                    CallbackPayload::new(br#"{"optionId":"allow-once"}"#, 256)
                        .expect("selection is bounded"),
                ),
            ))
        )
        .is_err()
    );
    assert!(
        block_on(
            responder.respond(CallbackResponse::new(
                callback_id.clone(),
                turn_id.clone(),
                CallbackResult::Success(
                    CallbackPayload::new(br#"{"optionId":"allow-always"}"#, 256)
                        .expect("selection is bounded"),
                ),
            ))
        )
        .is_err()
    );
    let selection = CallbackResponse::new(
        callback_id.clone(),
        turn_id,
        CallbackResult::Success(
            CallbackPayload::new(br#"{"optionId":"allow-once"}"#, 256)
                .expect("selection is bounded"),
        ),
    );
    block_on(responder.respond(selection.clone())).expect("permission selection is transported");
    assert!(block_on(responder.respond(selection)).is_err());

    let mut events = run.take_events().expect("events");
    let terminal = run.take_terminal_outcome().expect("terminal");
    let (observed, outcome) = block_on(async {
        let mut observed = Vec::new();
        while let Some(event) = events.next().await {
            observed.push(event.expect("event succeeds"));
        }
        (observed, terminal.await)
    });
    assert!(observed.iter().any(|event| {
        matches!(
            event.kind(),
            swallowtail_runtime::RuntimeEventKind::CallbackRequested(id)
                if id == &callback_id
        )
    }));
    assert_eq!(outcome.status(), &TerminalStatus::Completed);
    assert_eq!(block_on(run.close()), CleanupOutcome::Clean);
    let writes = operation_host.writes();
    assert!(writes.iter().any(|message| {
        message.get("id").and_then(serde_json::Value::as_u64) == Some(900)
            && message["result"]["outcome"]["optionId"] == "allow-once"
    }));
    assert!(!writes.iter().any(|message| {
        message.get("method").and_then(serde_json::Value::as_str) == Some("session/cancel")
    }));
}


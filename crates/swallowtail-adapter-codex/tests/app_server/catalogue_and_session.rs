#[test]
fn model_catalog_initializes_pages_and_cleans_up() {
    let (process, state) = ScriptedAppServer::new(AppServerMode::CompleteTurn);
    let models = block_on(driver().list_models(
        app_server_plan(DriverRole::ModelCatalog),
        ModelCatalogRequest::new(RequestId::new("catalog-1").expect("request id is valid")),
        host_services(process),
    ))
    .expect("catalog succeeds");

    assert_eq!(
        models
            .iter()
            .map(|model| model.id().as_str())
            .collect::<Vec<_>>(),
        ["gpt-5.4-mini", "gpt-5.4"]
    );
    assert_eq!(models[0].metadata().display_name(), Some("GPT-5.4 Mini"));
    assert_eq!(
        models[0].metadata().description(),
        Some("Fast structured work")
    );
    assert!(models[0].metadata().is_default());
    assert!(!models[1].metadata().is_default());
    let reasoning = models[0]
        .metadata()
        .reasoning()
        .expect("reasoning catalog evidence is present");
    assert_eq!(
        reasoning
            .supported_modes()
            .map(|mode| mode.as_str())
            .collect::<Vec<_>>(),
        ["low", "medium"]
    );
    assert_eq!(
        reasoning.default_mode().map(|mode| mode.as_str()),
        Some("medium")
    );
    assert_eq!(
        state.methods(),
        ["initialize", "initialized", "model/list", "model/list"]
    );
    let initialize = state
        .messages()
        .into_iter()
        .find(|message| {
            message.get("method").and_then(serde_json::Value::as_str) == Some("initialize")
        })
        .expect("initialize was sent");
    assert!(initialize["params"].get("capabilities").is_none());
    assert!(state.waited());
    let process_request = state.request();
    assert_eq!(process_request.executable, "codex-app-server-executable");
    assert_eq!(
        process_request.arguments,
        ["app-server", "--listen", "stdio://"]
    );
    assert!(process_request.working_resource.is_none());
}

#[test]
fn model_catalog_deadline_closes_and_joins_the_connection() {
    let recording = RecordingHostServices::default();
    let (process, state) = ScriptedAppServer::new(AppServerMode::HoldCatalog);
    let failure = block_on(
        driver().list_models(
            app_server_plan_with(DriverRole::ModelCatalog, [], [HostServiceKind::Time]),
            ModelCatalogRequest::new(
                RequestId::new("catalog-timeout").expect("request id is valid"),
            )
            .with_deadline(Deadline::at(MonotonicInstant::from_ticks(20))),
            host_services_with(process, &recording, [HostServiceKind::Time]),
        ),
    )
    .expect_err("catalog deadline expires");

    assert_eq!(
        failure.diagnostic().code(),
        "swallowtail.codex.app_server.catalog_timed_out"
    );
    assert!(state.waited());
}

#[test]
fn session_turn_streams_output_and_preserves_provider_ids() {
    let (process, state) = ScriptedAppServer::new(AppServerMode::CompleteTurn);
    let services = host_services(process);
    let mut session = block_on(driver().open_session(
        app_server_plan(DriverRole::InteractiveSession),
        read_only_open_request(
            RequestId::new("session-1").expect("request id is valid"),
            working_resource(),
            None,
        ),
        services.clone(),
    ))
    .expect("session opens");
    assert_eq!(
        session
            .provider_session_ref()
            .expect("provider session id is present")
            .as_provider_value(),
        "thread-provider-new"
    );
    assert_ne!(
        session.session_id().as_str(),
        session
            .provider_session_ref()
            .expect("provider session id is present")
            .as_provider_value()
    );

    let mut turn = block_on(session.start_turn(
        TurnRequest::new(
            RuntimeTurnId::new("turn-runtime-1").expect("turn id is valid"),
            OperationContent::new("private prompt").expect("content is valid"),
        ),
        services.clone(),
    ))
    .expect("turn starts");
    assert_eq!(
        turn.provider_turn_ref()
            .expect("provider turn id is present")
            .as_provider_value(),
        "turn-provider-1"
    );
    let events = block_on(
        turn.take_events()
            .expect("event stream is available")
            .collect::<Vec<_>>(),
    );
    let terminal = block_on(
        turn.take_terminal_outcome()
            .expect("terminal outcome is available"),
    );
    assert!(events.iter().all(Result::is_ok));
    assert!(events.iter().any(|event| {
        event
            .as_ref()
            .is_ok_and(|event| event.kind() == &RuntimeEventKind::OutputDelta)
    }));
    let activities = events
        .iter()
        .filter_map(|event| match event.as_ref().ok()?.kind() {
            RuntimeEventKind::Activity(activity) => Some(activity),
            _ => None,
        })
        .collect::<Vec<_>>();
    assert!(activities.iter().any(|activity| {
        matches!(activity.kind(), ActivityKind::AssistantMessage)
            && activity.assistant_phase() == Some(ActivityAssistantPhase::ProviderUnspecified)
            && activity
                .provider_activity_ref()
                .is_some_and(|reference| reference.as_provider_value() == "item-1")
    }));
    assert_eq!(terminal.status(), &TerminalStatus::Completed);
    assert_eq!(
        terminal.output().map(|output| output.as_str()),
        Some("final answer")
    );
    assert!(!format!("{terminal:?}").contains("final answer"));
    assert_eq!(block_on(turn.close()), CleanupOutcome::NotApplicable);
    assert_eq!(
        block_on(support::close_session(session, services)),
        CleanupOutcome::Clean
    );
    assert!(state.waited());
    assert_eq!(
        state.request().working_resource.as_deref(),
        Some("workspace.main")
    );
    assert!(state.methods().contains(&"turn/start".to_owned()));
}

#[test]
fn session_options_and_dynamic_tool_callback_round_trip() {
    let (process, state) = ScriptedAppServer::gate_enforcing(AppServerMode::DynamicToolCall);
    let services = host_services(process);
    let plan = app_server_plan_with(
        DriverRole::InteractiveSession,
        [
            reasoning_capability(),
            harness_mode_capability(),
            tool_capability(),
        ],
        [],
    );
    let mut session = block_on(
        driver().open_session(
            plan,
            read_only_open_request(
                RequestId::new("session-tools").expect("request id is valid"),
                working_resource(),
                None,
            )
            .with_options(
                session_options("task_ledger").with_harness_mode(HarnessMode::Plan),
            ),
            services.clone(),
        ),
    )
    .expect("tool-enabled session opens");

    let messages = state.messages();
    let initialize = messages
        .iter()
        .find(|message| {
            message.get("method").and_then(serde_json::Value::as_str) == Some("initialize")
        })
        .expect("initialize was sent");
    assert_eq!(
        initialize["params"]["capabilities"]["experimentalApi"],
        true
    );
    let thread_start = messages
        .iter()
        .find(|message| {
            message.get("method").and_then(serde_json::Value::as_str) == Some("thread/start")
        })
        .expect("thread/start was sent");
    assert_eq!(
        thread_start["params"]["developerInstructions"],
        "private session instructions"
    );
    assert!(
        thread_start["params"]
            .get("allowProviderModelFallback")
            .is_none()
    );
    assert_eq!(
        thread_start["params"]["dynamicTools"][0]["type"],
        "function"
    );
    assert_eq!(
        thread_start["params"]["dynamicTools"][0]["name"],
        "task_ledger"
    );

    let mut turn = block_on(session.start_turn(
        TurnRequest::new(
            RuntimeTurnId::new("turn-tools").expect("turn id is valid"),
            OperationContent::new("list tasks").expect("content is valid"),
        ),
        services.clone(),
    ))
    .expect("turn starts");
    let mut callbacks = turn
        .take_callbacks()
        .expect("tool-enabled turn exposes callbacks");
    let mut requests = callbacks
        .take_requests()
        .expect("callback request stream is available");
    let request = block_on(requests.next())
        .expect("callback request arrives")
        .expect("callback request is valid");
    match request.kind() {
        CallbackRequestKind::ToolCall {
            tool_name,
            arguments,
        } => {
            assert_eq!(tool_name, "task_ledger");
            assert_eq!(
                serde_json::from_slice::<serde_json::Value>(arguments.as_bytes())
                    .expect("arguments remain JSON"),
                serde_json::json!({"operation": "list"})
            );
        }
        CallbackRequestKind::Extension(_) => panic!("expected a dynamic tool callback"),
        CallbackRequestKind::HarnessUserInput(_) => {
            panic!("expected a dynamic tool callback")
        }
    }
    let response = CallbackResponse::new(
        request.callback_id().clone(),
        request
            .turn_id()
            .expect("callback belongs to a turn")
            .clone(),
        CallbackResult::Success(
            CallbackPayload::new(br#"{"tasks":[]}"#.to_vec(), 128)
                .expect("callback result is bounded"),
        ),
    );
    block_on(callbacks.responder().respond(response.clone()))
        .expect("callback response is accepted");
    assert!(block_on(callbacks.responder().respond(response)).is_err());

    let terminal = block_on(
        turn.take_terminal_outcome()
            .expect("terminal outcome is available"),
    );
    assert_eq!(terminal.status(), &TerminalStatus::Completed);
    let turn_start = state
        .messages()
        .into_iter()
        .find(|message| {
            message.get("method").and_then(serde_json::Value::as_str) == Some("turn/start")
        })
        .expect("turn/start was sent");
    assert_eq!(turn_start["params"]["effort"], "low");
    assert_eq!(turn_start["params"]["collaborationMode"]["mode"], "plan");
    assert_eq!(
        turn_start["params"]["collaborationMode"]["settings"]["model"],
        "gpt-5.4-mini"
    );
    assert_eq!(
        turn_start["params"]["collaborationMode"]["settings"]["reasoning_effort"],
        "low"
    );
    assert_eq!(
        turn_start["params"]["collaborationMode"]["settings"]["developer_instructions"],
        "private session instructions"
    );
    let provider_response = state
        .messages()
        .into_iter()
        .find(|message| {
            message.get("id").and_then(serde_json::Value::as_str) == Some("callback-900")
                && message.get("result").is_some()
        })
        .expect("provider callback response was sent");
    assert_eq!(provider_response["result"]["success"], true);
    assert_eq!(block_on(turn.close()), CleanupOutcome::NotApplicable);
    assert_eq!(
        block_on(support::close_session(session, services)),
        CleanupOutcome::Clean
    );
}

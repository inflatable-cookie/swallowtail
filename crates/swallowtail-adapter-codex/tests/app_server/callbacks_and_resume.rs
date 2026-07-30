#[test]
fn gate_fixture_accepts_stable_session_without_experimental_fields() {
    let (process, state) = ScriptedAppServer::gate_enforcing(AppServerMode::CompleteTurn);
    let result = block_on(driver().open_session(
        app_server_plan(DriverRole::InteractiveSession),
        read_only_open_request(
            RequestId::new("session-experimental-gate").expect("request id is valid"),
            working_resource(),
            None,
        ),
        host_services(process),
    ));

    let session = result.expect("stable session opens without experimental negotiation");
    let initialize = state
        .messages()
        .into_iter()
        .find(|message| message["method"] == "initialize")
        .expect("initialize was sent");
    assert!(
        initialize
            .pointer("/params/capabilities/experimentalApi")
            .is_none()
    );
    let thread_start = state
        .messages()
        .into_iter()
        .find(|message| message["method"] == "thread/start")
        .expect("thread/start was sent");
    assert!(
        thread_start["params"]
            .get("allowProviderModelFallback")
            .is_none()
    );
    assert_eq!(block_on(session.close()), CleanupOutcome::Clean);
    assert!(state.waited());
}

#[test]
fn undeclared_dynamic_tool_never_reaches_the_consumer() {
    let (process, state) = ScriptedAppServer::new(AppServerMode::DynamicToolCall);
    let services = host_services(process);
    let mut session = block_on(
        driver().open_session(
            app_server_plan_with(
                DriverRole::InteractiveSession,
                [reasoning_capability(), tool_capability()],
                [],
            ),
            read_only_open_request(
                RequestId::new("session-unknown-tool").expect("request id is valid"),
                working_resource(),
                None,
            )
            .with_options(session_options("different_tool")),
            services.clone(),
        ),
    )
    .expect("declared session opens");
    let mut turn = block_on(session.start_turn(
        TurnRequest::new(
            RuntimeTurnId::new("turn-unknown-tool").expect("turn id is valid"),
            OperationContent::new("try a tool").expect("content is valid"),
        ),
        services,
    ))
    .expect("turn starts");
    let mut callbacks = turn.take_callbacks().expect("callback exchange exists");
    let mut requests = callbacks.take_requests().expect("request stream exists");
    let terminal = block_on(
        turn.take_terminal_outcome()
            .expect("terminal outcome is available"),
    );

    assert!(matches!(
        terminal.status(),
        TerminalStatus::RuntimeFailed(_)
    ));
    assert!(block_on(requests.next()).is_none());
    assert!(state.forced());
    assert!(state.messages().iter().any(|message| {
        message.get("id").and_then(serde_json::Value::as_str) == Some("callback-900")
            && message.get("error").is_some()
    }));
    assert_eq!(block_on(turn.close()), CleanupOutcome::NotApplicable);
    assert_eq!(block_on(session.close()), CleanupOutcome::Clean);
}

#[test]
fn resumed_turn_uses_native_interruption_without_stopping_session() {
    let (process, state) = ScriptedAppServer::new(AppServerMode::HoldTurn);
    let services = host_services(process);
    let plan = app_server_plan_with(DriverRole::InteractiveSession, [reasoning_capability()], []);
    let binding = session_resume_binding(&plan, "thread-provider-existing");
    let mut session = block_on(
        driver().resume_session(
            plan,
            read_only_resume_request(
                RequestId::new("session-resume").expect("request id is valid"),
                binding,
                working_resource(),
                None,
            )
            .with_options(
                SessionOptions::default()
                    .with_developer_instructions(
                        OperationContent::new("resumed instructions")
                            .expect("instructions are valid"),
                    )
                    .with_reasoning_mode(
                        ReasoningMode::new("low").expect("reasoning mode is valid"),
                    ),
            ),
            services.clone(),
        ),
    )
    .expect("session resumes");
    assert_eq!(
        session
            .provider_session_ref()
            .expect("provider ref is present")
            .as_provider_value(),
        "thread-provider-existing"
    );
    let resume = state
        .messages()
        .into_iter()
        .find(|message| {
            message.get("method").and_then(serde_json::Value::as_str) == Some("thread/resume")
        })
        .expect("thread/resume was sent");
    assert_eq!(
        resume["params"]["developerInstructions"],
        "resumed instructions"
    );
    let mut turn = block_on(session.start_turn(
        TurnRequest::new(
            RuntimeTurnId::new("turn-runtime-cancel").expect("turn id is valid"),
            OperationContent::new("keep working").expect("content is valid"),
        ),
        services,
    ))
    .expect("turn starts");
    let resumed_turn = state
        .messages()
        .into_iter()
        .find(|message| {
            message.get("method").and_then(serde_json::Value::as_str) == Some("turn/start")
        })
        .expect("turn/start was sent");
    assert_eq!(resumed_turn["params"]["effort"], "low");
    assert_eq!(
        block_on(turn.cancellation().request()).expect("turn cancellation succeeds"),
        CancellationAcknowledgement::Requested
    );
    let terminal = block_on(
        turn.take_terminal_outcome()
            .expect("terminal outcome is available"),
    );
    assert_eq!(terminal.status(), &TerminalStatus::Cancelled);
    assert!(!state.forced());
    assert_eq!(block_on(turn.close()), CleanupOutcome::NotApplicable);
    assert_eq!(block_on(session.close()), CleanupOutcome::Clean);
    assert!(state.methods().contains(&"turn/interrupt".to_owned()));
}

#[test]
fn cancellation_abandons_pending_callback_and_rejects_late_response() {
    let (process, state) = ScriptedAppServer::new(AppServerMode::HoldDynamicToolCall);
    let services = host_services(process);
    let mut session = block_on(
        driver().open_session(
            app_server_plan_with(
                DriverRole::InteractiveSession,
                [reasoning_capability(), tool_capability()],
                [],
            ),
            read_only_open_request(
                RequestId::new("session-cancel-tool").expect("request id is valid"),
                working_resource(),
                None,
            )
            .with_options(session_options("task_ledger")),
            services.clone(),
        ),
    )
    .expect("tool-enabled session opens");
    let mut turn = block_on(session.start_turn(
        TurnRequest::new(
            RuntimeTurnId::new("turn-cancel-tool").expect("turn id is valid"),
            OperationContent::new("wait for tool").expect("content is valid"),
        ),
        services,
    ))
    .expect("turn starts");
    let mut callbacks = turn.take_callbacks().expect("callback exchange exists");
    let mut requests = callbacks.take_requests().expect("request stream exists");
    let request = block_on(requests.next())
        .expect("callback request arrives")
        .expect("callback request is valid");

    assert_eq!(
        block_on(turn.cancellation().request()).expect("turn cancellation succeeds"),
        CancellationAcknowledgement::Requested
    );
    let terminal = block_on(
        turn.take_terminal_outcome()
            .expect("terminal outcome is available"),
    );
    assert_eq!(terminal.status(), &TerminalStatus::Cancelled);
    let late = CallbackResponse::new(
        request.callback_id().clone(),
        request
            .turn_id()
            .expect("callback belongs to a turn")
            .clone(),
        CallbackResult::Success(
            CallbackPayload::new(b"late".to_vec(), 16).expect("payload is bounded"),
        ),
    );
    assert!(block_on(callbacks.responder().respond(late)).is_err());
    assert!(state.messages().iter().any(|message| {
        message.get("id").and_then(serde_json::Value::as_str) == Some("callback-900")
            && message.get("error").is_some()
    }));
    assert_eq!(block_on(turn.close()), CleanupOutcome::NotApplicable);
    assert_eq!(block_on(session.close()), CleanupOutcome::Clean);
}


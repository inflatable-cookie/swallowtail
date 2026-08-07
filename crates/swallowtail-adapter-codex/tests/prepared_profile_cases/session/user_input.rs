#[test]
fn prepared_session_exchanges_typed_user_input_without_enabling_approvals() {
    let prepared_app = prepared(
        CodexPreparedDriver::AppServer,
        "0.146.0",
        &RecordingHostServices::default(),
        false,
    );
    let profile = prepared_app
        .prepare_read_only_session(
            CodexSessionProfileInput::new(
                RequestId::new("user-input").unwrap(),
                model(),
                working_resource(),
                None,
                SessionOptions::default(),
            )
            .with_user_input_exchange(),
        )
        .expect("user-input exchange prepares");
    let policy = profile.request().access_policy();
    assert_eq!(
        policy
            .provider_requests()
            .handling_for(&swallowtail_adapter_codex::codex_user_input_request_extension()),
        swallowtail_core::ProviderRequestHandling::Exchange
    );
    assert_eq!(
        policy
            .provider_requests()
            .handling_for(&swallowtail_adapter_codex::codex_approval_request_extension()),
        swallowtail_core::ProviderRequestHandling::Reject
    );

    let (process, state) = ScriptedAppServer::gate_enforcing(AppServerMode::ExchangeUserInput);
    let services = support::host_services(process);
    let mut session = block_on(profile.open_session(services.clone()))
        .expect("prepared user-input session opens");
    let mut turn = block_on(session.start_turn(
        TurnRequest::new(
            RuntimeTurnId::new("user-input-turn").unwrap(),
            OperationContent::new("ask me").unwrap(),
        ),
        services,
    ))
    .expect("turn starts");
    let mut callbacks = turn.take_callbacks().expect("callback exchange is exposed");
    let mut requests = callbacks
        .take_requests()
        .expect("request stream is exposed");
    let request = block_on(requests.next())
        .expect("question arrives")
        .expect("question is valid");
    let user_input = match request.kind() {
        swallowtail_runtime::CallbackRequestKind::HarnessUserInput(request) => request,
        kind => panic!("expected typed user input, got {kind:?}"),
    };
    assert_eq!(user_input.questions().len(), 1);
    assert_eq!(user_input.auto_resolution_ms(), Some(60_000));
    assert!(!format!("{request:?}").contains("Choose a scope"));

    let invalid = HarnessUserInputResponse::new(
        [HarnessUserInputAnswer::selected(
            HarnessQuestionId::new("scope").unwrap(),
            [HarnessQuestionOptionId::new("not-offered").unwrap()],
            None,
        )],
        16,
        64 * 1024,
    )
    .unwrap();
    let failure = block_on(callbacks.responder().respond(
        swallowtail_runtime::CallbackResponse::new(
            request.callback_id().clone(),
            request.turn_id().expect("turn callback").clone(),
            swallowtail_runtime::CallbackResult::UserInput(invalid),
        ),
    ))
    .expect_err("an unoffered option is rejected");
    assert_eq!(
        failure.diagnostic().code(),
        "swallowtail.codex.app_server.user_input_response_invalid"
    );

    let response = HarnessUserInputResponse::new(
        [HarnessUserInputAnswer::selected(
            HarnessQuestionId::new("scope").unwrap(),
            [HarnessQuestionOptionId::new("Tests").unwrap()],
            None,
        )],
        16,
        64 * 1024,
    )
    .unwrap();
    block_on(
        callbacks
            .responder()
            .respond(swallowtail_runtime::CallbackResponse::new(
                request.callback_id().clone(),
                request.turn_id().expect("turn callback").clone(),
                swallowtail_runtime::CallbackResult::UserInput(response),
            )),
    )
    .expect("typed response is accepted");
    let terminal = block_on(
        turn.take_terminal_outcome()
            .expect("terminal outcome is available"),
    );
    assert_eq!(
        terminal.status(),
        &swallowtail_runtime::TerminalStatus::Completed
    );
    let provider_response = state
        .messages()
        .into_iter()
        .find(|message| {
            message.get("id").and_then(serde_json::Value::as_str) == Some("input-900")
                && message.get("result").is_some()
        })
        .expect("provider response is sent");
    assert_eq!(
        provider_response["result"]["answers"]["scope"]["answers"],
        serde_json::json!(["Tests"])
    );
    assert_eq!(block_on(turn.close()), CleanupOutcome::NotApplicable);
    assert_eq!(block_on(session.close()), CleanupOutcome::Clean);
}

#[test]
fn numeric_provider_request_id_resumes_and_completes_the_same_activity() {
    let prepared_app = prepared(
        CodexPreparedDriver::AppServer,
        "0.146.0",
        &RecordingHostServices::default(),
        false,
    );
    let profile = prepared_app
        .prepare_read_only_session(
            CodexSessionProfileInput::new(
                RequestId::new("numeric-user-input").unwrap(),
                model(),
                working_resource(),
                None,
                SessionOptions::default(),
            )
            .with_user_input_exchange(),
        )
        .expect("numeric user-input exchange prepares");
    let (process, state) =
        ScriptedAppServer::gate_enforcing(AppServerMode::ExchangeUserInputNumericRequestId);
    let services = support::host_services(process);
    let mut session =
        block_on(profile.open_session(services.clone())).expect("numeric user-input session opens");
    let mut turn = block_on(session.start_turn(
        TurnRequest::new(
            RuntimeTurnId::new("numeric-user-input-turn").unwrap(),
            OperationContent::new("ask me").unwrap(),
        ),
        services,
    ))
    .expect("numeric user-input turn starts");
    let mut events = turn.take_events().expect("turn events are exposed");
    let mut callbacks = turn.take_callbacks().expect("callback exchange is exposed");
    let mut requests = callbacks
        .take_requests()
        .expect("request stream is exposed");
    let request = block_on(requests.next())
        .expect("numeric question arrives")
        .expect("numeric question is valid");
    assert_eq!(
        request
            .provider_request_ref()
            .expect("provider request correlation is retained")
            .as_provider_value(),
        "900"
    );
    assert_eq!(
        request
            .provider_request_ref()
            .expect("provider request correlation is retained")
            .representation(),
        swallowtail_core::ProviderRequestRepresentation::SignedInteger
    );
    let response = HarnessUserInputResponse::new(
        [HarnessUserInputAnswer::selected(
            HarnessQuestionId::new("scope").unwrap(),
            [HarnessQuestionOptionId::new("Tests").unwrap()],
            None,
        )],
        16,
        64 * 1024,
    )
    .unwrap();
    block_on(
        callbacks
            .responder()
            .respond(swallowtail_runtime::CallbackResponse::new(
                request.callback_id().clone(),
                request.turn_id().expect("turn callback").clone(),
                swallowtail_runtime::CallbackResult::UserInput(response),
            )),
    )
    .expect("numeric typed response is accepted");

    let terminal = block_on(
        turn.take_terminal_outcome()
            .expect("terminal outcome is available"),
    );
    assert_eq!(
        terminal.status(),
        &swallowtail_runtime::TerminalStatus::Completed
    );

    let mut request_activity = Vec::new();
    block_on(async {
        while let Some(event) = events.next().await {
            let event = event.expect("runtime event is valid");
            let swallowtail_runtime::RuntimeEventKind::Activity(activity) = event.kind() else {
                continue;
            };
            let Some(swallowtail_runtime::ActivityCorrelation::ProviderRequest(reference)) =
                activity.correlation()
            else {
                continue;
            };
            if reference.as_provider_value() == "900"
                && reference.representation()
                    == swallowtail_core::ProviderRequestRepresentation::SignedInteger
            {
                request_activity.push((activity.activity_id().clone(), activity.phase()));
            }
        }
    });
    assert_eq!(request_activity.len(), 2);
    assert_eq!(
        request_activity[0].1,
        swallowtail_runtime::ActivityLifecyclePhase::Started
    );
    assert_eq!(
        request_activity[1].1,
        swallowtail_runtime::ActivityLifecyclePhase::Completed
    );
    assert_eq!(request_activity[0].0, request_activity[1].0);

    let provider_responses = state
        .messages()
        .into_iter()
        .filter(|message| {
            message.get("id").and_then(serde_json::Value::as_i64) == Some(900)
                && message.get("result").is_some()
        })
        .collect::<Vec<_>>();
    assert_eq!(provider_responses.len(), 1);
    assert_eq!(
        provider_responses[0]["result"]["answers"]["scope"]["answers"],
        serde_json::json!(["Tests"])
    );
    assert_eq!(block_on(turn.close()), CleanupOutcome::NotApplicable);
    assert_eq!(block_on(session.close()), CleanupOutcome::Clean);
}


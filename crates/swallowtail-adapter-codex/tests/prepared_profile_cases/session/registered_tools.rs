fn registered_source_id() -> swallowtail_runtime::ConsumerRouteProjectionSourceId {
    swallowtail_runtime::ConsumerRouteProjectionSourceId::new("codex.app-server.registered-tools")
        .expect("fixture source id is valid")
}

#[test]
fn a_registered_preparation_declares_its_tools_and_runs_the_real_call() {
    let admission = std::sync::Arc::new(swallowtail_testkit::ScriptedAdmissionPort::current());
    let dispatcher = support::registered::RouteDispatcher::new(|call| {
        Ok(support::registered::text_result(call, "three open tasks"))
    });
    let prepared_app = prepared(
        CodexPreparedDriver::AppServer,
        "0.145.0",
        &RecordingHostServices::default(),
        false,
    );
    let profile = prepared_app
        .prepare_read_only_session(
            CodexSessionProfileInput::new(
                RequestId::new("registered-session").unwrap(),
                model(),
                working_resource(),
                None,
                SessionOptions::default(),
            )
            .with_registered_tools(support::registered::registered_preparation(
                &admission,
                swallowtail_runtime::RegisteredToolExecutionKind::NativeClient,
            )),
        )
        .expect("a registered preparation prepares");

    // The registered selection is what the prepared plan and request declare.
    assert_eq!(profile.request().options().tools().len(), 1);
    assert_eq!(
        profile
            .request()
            .options()
            .tools()
            .next()
            .expect("one registered tool is declared")
            .name(),
        support::registered::REGISTERED_TOOL_WIRE_NAME
    );
    assert!(
        profile
            .plan()
            .requirements()
            .capabilities()
            .any(|requirement| requirement.capability() == Capability::ToolCalls)
    );
    assert!(profile.registered_tools().is_some());

    let (process, state) = ScriptedAppServer::new(AppServerMode::RegisteredToolCall);
    let (services, local) = support::registered::registered_services(process, dispatcher.clone());
    let mut session = block_on(profile.open_session(services.clone()))
        .expect("the prepared registered session opens");
    let mut turn = block_on(session.start_turn(
        TurnRequest::new(
            RuntimeTurnId::new("turn-registered-prepared").expect("turn id is valid"),
            OperationContent::new("use the registered tool").expect("content is valid"),
        ),
        services.clone(),
    ))
    .expect("turn starts");
    let response = support::registered::tool_response(&state);
    let _terminal = block_on(
        turn.take_terminal_outcome()
            .expect("terminal outcome is available"),
    );

    assert_eq!(response["result"]["success"], true);
    assert_eq!(
        support::registered::result_text(&response),
        "three open tasks"
    );
    assert_eq!(dispatcher.dispatches(), 1);
    assert_eq!(block_on(turn.close()), CleanupOutcome::Clean);
    assert_eq!(local.registered_tool_lease_count(), 0);
    assert_eq!(
        block_on(support::close_session(session, services)),
        CleanupOutcome::Clean
    );
}

#[test]
fn registered_and_unregistered_declarations_cannot_share_one_session() {
    let admission = std::sync::Arc::new(swallowtail_testkit::ScriptedAdmissionPort::current());
    let prepared_app = prepared(
        CodexPreparedDriver::AppServer,
        "0.145.0",
        &RecordingHostServices::default(),
        false,
    );
    let failure = prepared_app
        .prepare_read_only_session(
            CodexSessionProfileInput::new(
                RequestId::new("registered-conflict").unwrap(),
                model(),
                working_resource(),
                None,
                SessionOptions::default().with_tools([tool("lookup")]),
            )
            .with_registered_tools(support::registered::registered_preparation(
                &admission,
                swallowtail_runtime::RegisteredToolExecutionKind::NativeClient,
            )),
        )
        .expect_err("mixed tool declarations are refused");

    assert_eq!(
        failure.diagnostic().safe().code(),
        "swallowtail.codex.preparation.registered_tools_conflict"
    );
}

#[test]
fn a_registered_mcp_selection_is_refused_at_preparation() {
    let admission = std::sync::Arc::new(swallowtail_testkit::ScriptedAdmissionPort::current());
    let prepared_app = prepared(
        CodexPreparedDriver::AppServer,
        "0.145.0",
        &RecordingHostServices::default(),
        false,
    );
    let failure = prepared_app
        .prepare_read_only_session(
            CodexSessionProfileInput::new(
                RequestId::new("registered-mcp").unwrap(),
                model(),
                working_resource(),
                None,
                SessionOptions::default(),
            )
            .with_registered_tools(support::registered::registered_preparation(
                &admission,
                swallowtail_runtime::RegisteredToolExecutionKind::Mcp,
            )),
        )
        .expect_err("provider-direct MCP registration stays withheld");

    assert_eq!(
        failure.diagnostic().safe().code(),
        "swallowtail.codex.app_server.registered_mcp_withheld"
    );
}

#[test]
fn a_registered_session_cannot_redeclare_its_tools_on_resume_or_load() {
    let admission = std::sync::Arc::new(swallowtail_testkit::ScriptedAdmissionPort::current());
    let prepared_app = prepared(
        CodexPreparedDriver::AppServer,
        "0.145.0",
        &RecordingHostServices::default(),
        false,
    );
    let profile = prepared_app
        .prepare_read_only_session(
            CodexSessionProfileInput::new(
                RequestId::new("registered-continuation").unwrap(),
                model(),
                working_resource(),
                None,
                SessionOptions::default(),
            )
            .with_registered_tools(support::registered::registered_preparation(
                &admission,
                swallowtail_runtime::RegisteredToolExecutionKind::NativeClient,
            )),
        )
        .expect("a registered preparation prepares");
    let binding = support::session_resume_binding(profile.plan(), "thread-provider-existing");

    assert!(
        profile
            .resume_request(RequestId::new("resume").unwrap(), binding.clone())
            .is_err()
    );
    assert!(
        profile
            .load_request(RequestId::new("load").unwrap(), binding)
            .is_err()
    );
}

#[test]
fn the_registered_capability_projection_publishes_this_route_only() {
    let admission = std::sync::Arc::new(swallowtail_testkit::ScriptedAdmissionPort::current());
    let dispatcher = support::registered::RouteDispatcher::new(|call| {
        Ok(support::registered::text_result(call, "unused"))
    });
    let prepared_app = prepared(
        CodexPreparedDriver::AppServer,
        "0.145.0",
        &RecordingHostServices::default(),
        false,
    );
    let profile = prepared_app
        .prepare_read_only_session(
            CodexSessionProfileInput::new(
                RequestId::new("registered-projection").unwrap(),
                model(),
                working_resource(),
                None,
                SessionOptions::default(),
            )
            .with_registered_tools(support::registered::registered_preparation(
                &admission,
                swallowtail_runtime::RegisteredToolExecutionKind::NativeClient,
            )),
        )
        .expect("a registered preparation prepares");
    let (process, _state) = ScriptedAppServer::new(AppServerMode::CompleteTurn);
    let (services, _local) = support::registered::registered_services(process, dispatcher);
    let contribution = profile
        .registered_capability_projection_contribution(
            registered_source_id(),
            &services,
        )
        .expect("a registered session publishes its capability")
        .expect("the contribution is valid");

    assert!(
        contribution.selection_rows().len() > 0,
        "the registered capability publishes selection rows"
    );
}

#[test]
fn a_session_without_registered_tools_publishes_nothing_new() {
    let prepared_app = prepared(
        CodexPreparedDriver::AppServer,
        "0.145.0",
        &RecordingHostServices::default(),
        false,
    );
    let profile = prepared_app
        .prepare_read_only_session(CodexSessionProfileInput::new(
            RequestId::new("unregistered-projection").unwrap(),
            model(),
            working_resource(),
            None,
            SessionOptions::default(),
        ))
        .expect("an ordinary session prepares");
    let (process, _state) = ScriptedAppServer::new(AppServerMode::CompleteTurn);

    assert!(profile.registered_tools().is_none());
    assert!(
        profile
            .registered_capability_projection_contribution(
                registered_source_id(),
                &host_services(process),
            )
            .is_none()
    );
}

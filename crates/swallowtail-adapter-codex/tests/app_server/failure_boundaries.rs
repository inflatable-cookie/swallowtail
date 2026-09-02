#[test]
fn callback_wait_ends_when_the_host_deadline_is_observed() {
    let (process, state) = ScriptedAppServer::new(AppServerMode::HoldDynamicToolCall);
    let clock = ControllableTime::new(0);
    let services = host_services(process)
        .with_time(Arc::new(clock.clone()) as Arc<dyn TimeService>);
    let mut session = block_on(
        driver().open_session(
            app_server_plan_with(
                DriverRole::InteractiveSession,
                [reasoning_capability(), tool_capability()],
                [HostServiceKind::Time],
            ),
            read_only_open_request(
                RequestId::new("session-timeout-tool").expect("request id is valid"),
                working_resource(),
                None,
            )
            .with_options(session_options("task_ledger")),
            services.clone(),
        ),
    )
    .expect("deadline-capable session opens");
    let mut turn = block_on(
        session.start_turn(
            TurnRequest::new(
                RuntimeTurnId::new("turn-timeout-tool").expect("turn id is valid"),
                OperationContent::new("wait for tool").expect("content is valid"),
            )
            .with_deadline(Deadline::at(MonotonicInstant::from_ticks(50))),
            services.clone(),
        ),
    )
    .expect("turn starts");
    let mut callbacks = turn.take_callbacks().expect("callback exchange exists");
    let mut requests = callbacks.take_requests().expect("request stream exists");
    let callback = block_on(requests.next())
        .expect("tool callback arrives before the deadline fires")
        .expect("tool callback is valid");
    assert!(matches!(
        callback.kind(),
        CallbackRequestKind::ToolCall { .. }
    ));
    clock.advance_to(50);
    let terminal = block_on(
        turn.take_terminal_outcome()
            .expect("terminal outcome is available"),
    );

    assert_eq!(terminal.status(), &TerminalStatus::TimedOut);
    assert!(block_on(requests.next()).is_none());
    assert!(state.methods().contains(&"turn/interrupt".to_owned()));
    state.wait_for_message(|message| {
        message.get("id").and_then(serde_json::Value::as_str) == Some("callback-900")
            && message.get("error").is_some()
    });
    assert_eq!(block_on(turn.close()), CleanupOutcome::NotApplicable);
    assert_eq!(
        block_on(support::close_session(session, services)),
        CleanupOutcome::Clean
    );
}

#[test]
fn whole_session_cancellation_force_stops_and_joins() {
    let (process, state) = ScriptedAppServer::new(AppServerMode::HoldTurn);
    let services = host_services(process);
    let session = block_on(driver().open_session(
        app_server_plan(DriverRole::InteractiveSession),
        read_only_open_request(
            RequestId::new("session-cancel").expect("request id is valid"),
            working_resource(),
            None,
        ),
        services.clone(),
    ))
    .expect("session opens");
    assert_eq!(
        block_on(session.cancellation().request()).expect("session cancellation succeeds"),
        CancellationAcknowledgement::Requested
    );
    assert_eq!(
        block_on(session.cancellation().request()).expect("repeat cancellation succeeds"),
        CancellationAcknowledgement::AlreadyRequested
    );
    assert_eq!(
        block_on(support::close_session(session, services)),
        CleanupOutcome::Clean
    );
    assert!(state.forced());
    assert!(state.waited());
}

#[test]
fn unsupported_server_request_fails_instead_of_hanging() {
    let (process, state) = ScriptedAppServer::new(AppServerMode::RequestCallback);
    let services = host_services(process);
    let mut session = block_on(driver().open_session(
        app_server_plan(DriverRole::InteractiveSession),
        read_only_open_request(
            RequestId::new("session-callback").expect("request id is valid"),
            working_resource(),
            None,
        ),
        services.clone(),
    ))
    .expect("session opens");
    let mut turn = block_on(session.start_turn(
        TurnRequest::new(
            RuntimeTurnId::new("turn-callback").expect("turn id is valid"),
            OperationContent::new("trigger callback").expect("content is valid"),
        ),
        services.clone(),
    ))
    .expect("turn response remains correlated");
    let terminal = block_on(
        turn.take_terminal_outcome()
            .expect("terminal outcome is available"),
    );

    assert!(matches!(
        terminal.status(),
        TerminalStatus::RuntimeFailed(_)
    ));
    assert_eq!(block_on(turn.close()), CleanupOutcome::NotApplicable);
    assert_eq!(
        block_on(support::close_session(session, services)),
        CleanupOutcome::Clean
    );
    assert!(state.forced());
    assert!(state.waited());
}

#[test]
fn unsupported_session_input_fails_before_process_start() {
    let (process, state) = ScriptedAppServer::new(AppServerMode::CompleteTurn);
    let result = block_on(driver().open_session(
        app_server_plan(DriverRole::InteractiveSession),
        read_only_open_request(
            RequestId::new("session-deadline").expect("request id is valid"),
            working_resource(),
            Some(Deadline::at(MonotonicInstant::from_ticks(10))),
        ),
        host_services(process),
    ));

    assert!(result.is_err());
    assert!(!state.started());
}

#[test]
fn session_options_without_matching_preflight_fail_before_process_start() {
    let (process, state) = ScriptedAppServer::new(AppServerMode::CompleteTurn);
    let request = read_only_open_request(
        RequestId::new("session-options").expect("request id is valid"),
        working_resource(),
        None,
    )
    .with_options(
        SessionOptions::default()
            .with_reasoning_mode(ReasoningMode::new("low").expect("reasoning mode is valid")),
    );
    let result = block_on(driver().open_session(
        app_server_plan(DriverRole::InteractiveSession),
        request,
        host_services(process),
    ));

    assert!(result.is_err());
    assert!(!state.started());
}

#[test]
fn resumed_dynamic_tools_fail_before_process_start_when_schema_cannot_redeclare_them() {
    let (process, state) = ScriptedAppServer::new(AppServerMode::CompleteTurn);
    let plan = app_server_plan_with(
        DriverRole::InteractiveSession,
        [reasoning_capability(), tool_capability()],
        [],
    );
    let binding = session_resume_binding(&plan, "thread-provider-existing");
    let result = block_on(
        driver().resume_session(
            plan,
            read_only_resume_request(
                RequestId::new("resume-tools").expect("request id is valid"),
                binding,
                working_resource(),
                None,
            )
            .with_options(session_options("task_ledger")),
            host_services(process),
        ),
    );

    assert!(result.is_err());
    assert!(!state.started());
}

#[test]
fn structured_output_is_rejected_before_turn_provider_work() {
    let (process, state) = ScriptedAppServer::new(AppServerMode::CompleteTurn);
    let services = host_services(process);
    let mut session = block_on(driver().open_session(
        app_server_plan(DriverRole::InteractiveSession),
        read_only_open_request(
            RequestId::new("session-structured-output").expect("request id is valid"),
            working_resource(),
            None,
        ),
        services.clone(),
    ))
    .expect("session opens");
    let methods_before_turn = state.methods();
    let schema = StructuredOutputDescriptor::new(
        SchemaDocument::inline(b"{}".to_vec(), 16).expect("schema is within bound"),
        "application/schema+json",
        "json-schema-2020-12",
    )
    .expect("schema descriptor is valid");
    let result = block_on(
        session.start_turn(
            TurnRequest::new(
                RuntimeTurnId::new("turn-structured-output").expect("turn id is valid"),
                OperationContent::new("return structured output").expect("content is valid"),
            )
            .with_structured_output(schema),
            services.clone(),
        ),
    );

    assert!(result.is_err());
    assert_eq!(state.methods(), methods_before_turn);
    assert_eq!(
        block_on(support::close_session(session, services)),
        CleanupOutcome::Clean
    );
}

#[test]
fn malformed_notification_carries_bounded_method_context_and_stderr_tail() {
    let (process, state) = ScriptedAppServer::new(AppServerMode::MalformedNotification);
    let services = host_services(process);
    let mut session = block_on(driver().open_session(
        app_server_plan(DriverRole::InteractiveSession),
        read_only_open_request(
            RequestId::new("session-malformed").expect("request id is valid"),
            working_resource(),
            None,
        ),
        services.clone(),
    ))
    .expect("session opens");
    let mut turn = block_on(session.start_turn(
        TurnRequest::new(
            RuntimeTurnId::new("turn-malformed").expect("turn id is valid"),
            OperationContent::new("drifted notification").expect("content is valid"),
        ),
        services.clone(),
    ))
    .expect("turn starts");
    let terminal = block_on(
        turn.take_terminal_outcome()
            .expect("terminal outcome is available"),
    );

    let TerminalStatus::RuntimeFailed(diagnostic) = terminal.status() else {
        panic!("malformed notification fails the turn");
    };
    assert_eq!(
        diagnostic.code(),
        "swallowtail.codex.app_server.malformed_notification"
    );
    let message = diagnostic.message();
    assert!(message.contains("method `item/plan/delta`"));
    assert!(message.contains("excerpt `"));
    assert!(message.contains("; stderr: "));
    assert!(message.contains("unrecognized plan delta field"));
    assert!(message.contains("[stderr truncated]"));
    assert!(!message.contains("xxx"));
    assert!(message.chars().count() <= 640);

    let result = block_on(session.start_turn(
        TurnRequest::new(
            RuntimeTurnId::new("turn-after-poison").expect("turn id is valid"),
            OperationContent::new("after poison").expect("content is valid"),
        ),
        services.clone(),
    ));
    let error = result
        .err()
        .expect("poisoned session rejects the next request");
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.codex.app_server.connection_closed"
    );
    assert_eq!(block_on(turn.close()), CleanupOutcome::NotApplicable);
    assert_eq!(
        block_on(support::close_session(session, services)),
        CleanupOutcome::Clean
    );
    assert!(state.forced());
    assert!(state.waited());
}

#[test]
fn malformed_notification_emits_correlated_debug_observations_when_observer_registered() {
    let (process, state) = ScriptedAppServer::new(AppServerMode::MalformedNotification);
    let observer = Arc::new(CapturingDebugObserver::default());
    let services = host_services(process).with_diagnostic_observer(observer.clone());
    let mut session = block_on(driver().open_session(
        app_server_plan(DriverRole::InteractiveSession),
        read_only_open_request(
            RequestId::new("session-malformed-debug").expect("request id is valid"),
            working_resource(),
            None,
        ),
        services.clone(),
    ))
    .expect("session opens");
    let mut turn = block_on(session.start_turn(
        TurnRequest::new(
            RuntimeTurnId::new("turn-malformed-debug").expect("turn id is valid"),
            OperationContent::new("drifted notification").expect("content is valid"),
        ),
        services.clone(),
    ))
    .expect("turn starts");
    let terminal = block_on(
        turn.take_terminal_outcome()
            .expect("terminal outcome is available"),
    );

    let TerminalStatus::RuntimeFailed(diagnostic) = terminal.status() else {
        panic!("malformed notification fails the turn");
    };
    assert_eq!(
        diagnostic.code(),
        "swallowtail.codex.app_server.malformed_notification"
    );
    assert!(diagnostic.message().contains("method `item/plan/delta`"));
    assert!(!diagnostic.message().contains("xxx"));

    let observations = observer.observations();
    assert!(
        observations.iter().any(|observation| {
            observation.kind() == DebugObservationKind::ProtocolParse
                && observation.correlated_code()
                    == Some("swallowtail.codex.app_server.malformed_notification")
                && observation.detail().contains("method=item/plan/delta")
                && !observation.detail().contains("xxx")
        }),
        "expected protocol-parse debug observation, got {observations:?}"
    );
    assert!(
        observations.iter().any(|observation| {
            observation.kind() == DebugObservationKind::StderrRing
                && observation.correlated_code()
                    == Some("swallowtail.codex.app_server.malformed_notification")
                && observation.detail().contains("unrecognized plan delta field")
        }),
        "expected stderr-ring debug observation, got {observations:?}"
    );

    let result = block_on(session.start_turn(
        TurnRequest::new(
            RuntimeTurnId::new("turn-after-poison-debug").expect("turn id is valid"),
            OperationContent::new("after poison").expect("content is valid"),
        ),
        services.clone(),
    ));
    assert_eq!(
        result
            .err()
            .expect("poisoned session rejects the next request")
            .diagnostic()
            .code(),
        "swallowtail.codex.app_server.connection_closed"
    );
    assert_eq!(block_on(turn.close()), CleanupOutcome::NotApplicable);
    assert_eq!(
        block_on(support::close_session(session, services)),
        CleanupOutcome::Clean
    );
    assert!(state.forced());
    assert!(state.waited());
}

#[derive(Default)]
struct CapturingDebugObserver {
    observations: Mutex<Vec<DebugObservation>>,
}

impl CapturingDebugObserver {
    fn observations(&self) -> Vec<DebugObservation> {
        self.observations
            .lock()
            .expect("observation lock")
            .clone()
    }
}

impl DiagnosticObserver for CapturingDebugObserver {
    fn observe(&self, _diagnostic: &Diagnostic) {}

    fn observe_debug(&self, observation: &DebugObservation) {
        self.observations
            .lock()
            .expect("observation lock")
            .push(observation.clone());
    }
}

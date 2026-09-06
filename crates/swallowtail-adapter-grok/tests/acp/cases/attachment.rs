#[test]
fn exact_attachment_activates_once_runs_activity_and_joins_owned_leases() {
    let (host, services, mut session) = open(Scenario::Success);
    assert_eq!(
        session
            .negotiated_model_options()
            .expect("models")
            .current_value(),
        "grok-4.5"
    );
    let mut turn = start(&mut *session, services.clone(), "grok-success-turn");
    let outcome = block_on(
        turn.take_terminal_outcome()
            .expect("terminal outcome available"),
    );
    assert_eq!(outcome.status(), &TerminalStatus::Completed);
    assert_eq!(
        outcome.output().expect("output").as_str(),
        "Fixture response."
    );
    let mut events = turn.take_events().expect("events");
    let events = block_on(async move {
        let mut collected = Vec::new();
        while let Some(event) = events.next().await {
            collected.push(event.expect("event"));
        }
        collected
    });
    assert!(
        events
            .iter()
            .any(|event| matches!(event.kind(), RuntimeEventKind::ReasoningProgress))
    );
    assert!(
        events
            .iter()
            .filter(|event| matches!(event.kind(), RuntimeEventKind::Activity(_)))
            .count()
            >= 5
    );
    let plan = events
        .iter()
        .find_map(|event| match event.kind() {
            RuntimeEventKind::Activity(activity) => activity.task_list(),
            _ => None,
        })
        .expect("ACP plan carries a typed task-list replacement");
    let task = plan.items().next().expect("fixture task");
    assert_eq!(task.content().as_str(), "Inspect fixture");
    assert_eq!(
        task.status(),
        swallowtail_runtime::TaskListItemStatus::InProgress
    );
    assert_eq!(
        task.priority(),
        Some(swallowtail_runtime::TaskListItemPriority::Medium)
    );
    assert!(!format!("{events:?}").contains("private fixture prompt"));
    assert!(!format!("{outcome:?}").contains("Fixture response"));
    assert_eq!(block_on(turn.close()), CleanupOutcome::NotApplicable);
    assert_eq!(block_on(close_session(session, services)), CleanupOutcome::Clean);

    let process = host
        .process
        .lock()
        .expect("process lock")
        .clone()
        .expect("process observed");
    assert_eq!(process.arguments, ["--no-auto-update", "agent", "stdio"]);
    assert_eq!(process.environment_count, 1);
    assert_eq!(
        process.resource,
        Some(WorkingResourceRef::new("grok.fixture.workspace").expect("resource"))
    );
    assert_eq!(host.credential_acquires.load(Ordering::SeqCst), 1);
    assert_eq!(host.credential_releases.load(Ordering::SeqCst), 1);
    assert_eq!(host.resource_releases.load(Ordering::SeqCst), 1);
    let writes = host.writes();
    assert_eq!(
        writes
            .iter()
            .filter(|message| message["method"] == "authenticate")
            .count(),
        1
    );
    let auth = writes
        .iter()
        .find(|message| message["method"] == "authenticate")
        .expect("auth request");
    assert_eq!(auth["params"]["methodId"], "cached_token");
    assert_eq!(auth["params"]["_meta"]["headless"], true);
    assert!(!format!("{writes:?}").contains("must-be-discarded"));
}

#[test]
fn exact_attachment_recovery_discards_large_history_and_returns_the_bound_session() {
    let (_opened_host, opened_services, session) = open(Scenario::Success);
    let binding = session
        .resume_binding()
        .expect("opened Grok session exposes a durable binding")
        .clone();
    assert_eq!(
        binding.model_id().map(swallowtail_core::ModelId::as_str),
        Some("grok-4.5")
    );
    assert_eq!(
        block_on(close_session(session, opened_services)),
        CleanupOutcome::Clean
    );

    let host_id = ExecutionHostId::new("fixture.host.grok").expect("host");
    let selected = selection(host_id.clone());
    let persisted = binding
        .export_persisted(&selected.plan)
        .expect("Grok binding persists");
    let binding = swallowtail_runtime::SessionResumeBinding::restore_persisted(
        &persisted,
        &selected.plan,
        &selected.resource,
        &SessionAccessPolicy::ambient_harness(ResourceAccess::ReadWrite),
    )
    .expect("Grok binding restores after a process restart");
    let host = FixtureHost::new(Scenario::Success);
    let services = host.services(host_id);
    let request = ResumeSessionRequest::from_plan(
        &selected.plan,
        RequestId::new("grok-recover").expect("request"),
        binding.clone(),
        selected.resource,
        None,
    )
    .expect("recovery request");
    let driver = GrokAcpDriver::new(
        EnvironmentRef::new("grok.fixture.ambient").expect("environment"),
        selected.credential,
    );
    let recovered = block_on(driver.recover_session_attachment(
        selected.plan,
        request,
        services.clone(),
    ))
    .expect("exact attachment recovers");
    assert_eq!(
        recovered.provider_session_ref(),
        Some(binding.provider_session_ref())
    );
    assert_eq!(recovered.resume_binding(), Some(&binding));
    assert_eq!(
        block_on(close_session(recovered, services)),
        CleanupOutcome::Clean
    );
    assert_eq!(
        host.writes()
            .iter()
            .filter_map(|message| message.get("method").and_then(Value::as_str))
            .collect::<Vec<_>>(),
        ["initialize", "authenticate", "session/load"]
    );
}

#[test]
fn attachment_recovery_rejects_invalid_or_incomplete_loads_without_a_handle() {
    let (_host, services, session) = open(Scenario::Success);
    let binding = session.resume_binding().expect("binding").clone();
    assert_eq!(block_on(close_session(session, services)), CleanupOutcome::Clean);

    for (scenario, suffix) in [
        (Scenario::RecoveryForeign, "session_mismatch"),
        (Scenario::RecoveryCallback, "callback_rejected"),
        (Scenario::RecoveryMalformed, "response_malformed"),
        (Scenario::RecoveryOversized, "limit_exceeded"),
        (Scenario::RecoveryLate, "late_update"),
        (Scenario::RecoveryDisconnect, "connection_ended"),
        (Scenario::RecoveryResponseMismatch, "response_mismatch"),
    ] {
        let host_id = ExecutionHostId::new("fixture.host.grok").expect("host");
        let selected = selection(host_id.clone());
        let host = FixtureHost::new(scenario);
        let services = host.services(host_id);
        let request = ResumeSessionRequest::from_plan(
            &selected.plan,
            RequestId::new(format!("grok-recover-{suffix}")).expect("request"),
            binding.clone(),
            selected.resource,
            None,
        )
        .expect("recovery request");
        let driver = GrokAcpDriver::new(
            EnvironmentRef::new("grok.fixture.ambient").expect("environment"),
            selected.credential,
        );
        let error = match block_on(driver.recover_session_attachment(
            selected.plan,
            request,
            services,
        )) {
            Ok(_) => panic!("{scenario:?} recovery must return no handle"),
            Err(error) => error,
        };
        assert!(
            error.diagnostic().code().ends_with(suffix),
            "unexpected diagnostic: {}",
            error.diagnostic().code()
        );
        assert_eq!(host.credential_releases.load(Ordering::SeqCst), 1);
        assert_eq!(host.resource_releases.load(Ordering::SeqCst), 1);
    }
}

#[test]
fn permission_is_observed_and_cancelled_without_ambient_approval() {
    let (host, services, mut session) = open(Scenario::Permission);
    let mut turn = start(&mut *session, services.clone(), "grok-permission-turn");
    assert!(turn.take_callbacks().is_none());
    let outcome = block_on(
        turn.take_terminal_outcome()
            .expect("terminal outcome available"),
    );
    assert!(matches!(
        outcome.status(),
        TerminalStatus::ProviderRequestObserved(_)
    ));
    assert!(host.writes().iter().any(|message| {
        message.get("method").and_then(Value::as_str) == Some("session/cancel")
    }));
    assert!(host.writes().iter().any(|message| {
        message.get("id").and_then(Value::as_u64) == Some(900)
            && message["result"]["outcome"]["outcome"] == "cancelled"
    }));
    assert_eq!(block_on(turn.close()), CleanupOutcome::NotApplicable);
    assert_eq!(block_on(close_session(session, services)), CleanupOutcome::Clean);
}

#[test]
fn consumer_permission_exchange_answers_each_one_shot_option_and_hides_persistent_choices() {
    for option_id in ["allow-once", "reject-once"] {
        let (host, services, mut session) = open_consumer(Scenario::Permission);
        let mut turn = start(&mut *session, services.clone(), "grok-permission-exchange-turn");
        let mut callbacks = turn.take_callbacks().expect("consumer exchange exists");
        let mut requests = callbacks
            .take_requests()
            .expect("permission request stream exists");
        let request = block_on(requests.next())
            .expect("permission callback arrives")
            .expect("permission callback is valid");
        let swallowtail_runtime::CallbackRequestKind::Extension(extension) = request.kind() else {
            panic!("permission is a provider extension");
        };
        assert_eq!(extension.namespace().as_str(), "acp/session/request-permission");
        let payload: Value = serde_json::from_slice(extension.payload()).expect("JSON payload");
        let options = payload["options"].as_array().expect("options");
        assert_eq!(
            options
                .iter()
                .map(|option| option["kind"].as_str().expect("option kind"))
                .collect::<Vec<_>>(),
            ["allow_once", "reject_once"]
        );
        assert_eq!(
            request
                .provider_request_ref()
                .expect("provider request correlation")
                .as_provider_value(),
            "acp:number:900"
        );
        block_on(callbacks.responder().respond(CallbackResponse::for_request(
            &request,
            CallbackResult::Success(
                CallbackPayload::new(
                    format!(r#"{{"optionId":"{option_id}"}}"#).into_bytes(),
                    256,
                )
                .expect("selection is bounded"),
            ),
        )))
        .expect("permission answer reaches the fixture");
        let outcome = block_on(
            turn.take_terminal_outcome()
                .expect("terminal outcome available"),
        );
        assert_eq!(outcome.status(), &TerminalStatus::Completed);
        assert!(host.writes().iter().any(|message| {
            message.get("id").and_then(Value::as_u64) == Some(900)
                && message["result"]["outcome"]["optionId"] == option_id
        }));
        assert!(!host.writes().iter().any(|message| {
            message.get("method").and_then(Value::as_str) == Some("session/cancel")
        }));
        assert_eq!(block_on(turn.close()), CleanupOutcome::NotApplicable);
        assert_eq!(block_on(close_session(session, services)), CleanupOutcome::Clean);
    }
}

#[test]
fn consumer_permission_abandonment_closes_the_exchange_without_granting_access() {
    let (host, services, mut session) = open_consumer(Scenario::Permission);
    let mut turn = start(&mut *session, services.clone(), "grok-permission-abandon-turn");
    let mut callbacks = turn.take_callbacks().expect("consumer exchange exists");
    let mut requests = callbacks
        .take_requests()
        .expect("permission request stream exists");
    let request = block_on(requests.next())
        .expect("permission callback arrives")
        .expect("permission callback is valid");
    let responder = callbacks.responder();
    block_on(turn.cancellation().request()).expect("turn cancellation requested");
    assert!(block_on(requests.next()).is_none());
    assert!(block_on(responder.respond(CallbackResponse::for_request(
        &request,
        CallbackResult::Success(
            CallbackPayload::new(br#"{"optionId":"allow-once"}"#.to_vec(), 256)
                .expect("selection is bounded"),
        ),
    ))).is_err());
    let outcome = block_on(
        turn.take_terminal_outcome()
            .expect("terminal outcome available"),
    );
    assert_eq!(outcome.status(), &TerminalStatus::Cancelled);
    assert!(host.writes().iter().any(|message| {
        message.get("method").and_then(Value::as_str) == Some("session/cancel")
    }));
    assert!(!host.writes().iter().any(|message| {
        message.get("id").and_then(Value::as_u64) == Some(900)
            && message["result"]["outcome"]["outcome"] == "selected"
    }));
    assert_eq!(block_on(turn.close()), CleanupOutcome::NotApplicable);
    assert_eq!(block_on(close_session(session, services)), CleanupOutcome::Clean);
}

#[test]
fn consumer_permission_timeout_abandons_the_bounded_exchange() {
    let (host, services, mut session) = open_consumer(Scenario::PermissionTimeout);
    let mut turn = start_with_deadline(
        &mut *session,
        services.clone(),
        "grok-permission-timeout-turn",
        Some(Deadline::at(MonotonicInstant::from_ticks(10))),
    );
    let mut callbacks = turn.take_callbacks().expect("consumer exchange exists");
    let mut requests = callbacks
        .take_requests()
        .expect("permission request stream exists");
    let request = block_on(requests.next())
        .expect("permission callback arrives before timeout")
        .expect("permission callback is valid");
    assert_eq!(
        request.deadline(),
        Some(Deadline::at(MonotonicInstant::from_ticks(10)))
    );
    let responder = callbacks.responder();
    host.release_deadline();
    let outcome = block_on(
        turn.take_terminal_outcome()
            .expect("terminal outcome available"),
    );
    assert_eq!(outcome.status(), &TerminalStatus::TimedOut);
    assert!(block_on(requests.next()).is_none());
    assert!(block_on(responder.respond(CallbackResponse::for_request(
        &request,
        CallbackResult::Failure {
            kind: swallowtail_runtime::CallbackFailureKind::TimedOut,
            detail: None,
        },
    ))).is_err());
    assert!(host.wait_for_write(|writes| writes.iter().any(|message| {
        message.get("method").and_then(Value::as_str) == Some("session/cancel")
    })));
    assert_eq!(block_on(turn.close()), CleanupOutcome::NotApplicable);
    assert_eq!(block_on(close_session(session, services)), CleanupOutcome::Clean);
}

#[test]
fn consumer_permission_malformed_request_fails_closed() {
    let (_host, services, mut session) = open_consumer(Scenario::PermissionMalformed);
    let mut turn = start(&mut *session, services.clone(), "grok-permission-malformed-turn");
    let mut callbacks = turn.take_callbacks().expect("consumer exchange exists");
    let mut requests = callbacks
        .take_requests()
        .expect("permission request stream exists");
    let outcome = block_on(
        turn.take_terminal_outcome()
            .expect("terminal outcome available"),
    );
    assert!(matches!(
        outcome.status(),
        TerminalStatus::RuntimeFailed(diagnostic)
            if diagnostic.code() == "swallowtail.grok.acp.response_malformed"
    ));
    assert!(block_on(requests.next()).is_none());
    assert_eq!(block_on(turn.close()), CleanupOutcome::NotApplicable);
    assert_eq!(block_on(close_session(session, services)), CleanupOutcome::Clean);
}

#[test]
fn permission_without_active_turn_is_rejected_by_the_fixture_exchange() {
    let (host, services, session) = open(Scenario::PermissionWithoutTurn);
    host.emit_permission_without_turn();
    assert!(host.wait_for_write(|writes| {
        writes.iter().any(|message| {
            message.get("id").and_then(Value::as_u64) == Some(902)
                && message["error"]["code"].as_i64() == Some(-32600)
        })
    }));
    let _ = block_on(close_session(session, services));
}

#[test]
fn active_turn_cancellation_waits_for_native_cancelled_result() {
    let (_host, services, mut session) = open(Scenario::Cancellation);
    let mut turn = start(&mut *session, services.clone(), "grok-cancel-turn");
    block_on(turn.cancellation().request()).expect("cancel sent");
    let outcome = block_on(
        turn.take_terminal_outcome()
            .expect("terminal outcome available"),
    );
    assert_eq!(outcome.status(), &TerminalStatus::Cancelled);
    assert_eq!(block_on(turn.close()), CleanupOutcome::NotApplicable);
    assert_eq!(block_on(close_session(session, services)), CleanupOutcome::Clean);
}

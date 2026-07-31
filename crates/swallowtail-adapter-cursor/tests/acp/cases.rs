#[test]
fn exact_attachment_streams_activity_and_joins_owned_resources() {
    let (host, services, mut session) = open(Scenario::Success);
    assert!(session.negotiated_model_options().is_none());
    let mut turn = start(&mut *session, services, "cursor-success-turn");
    let outcome = block_on(
        turn.take_terminal_outcome()
            .expect("terminal outcome available"),
    );
    assert_eq!(outcome.status(), &TerminalStatus::Completed);
    assert_eq!(
        outcome.output().expect("output").as_str(),
        "Fixture response."
    );
    let mut stream = turn.take_events().expect("events");
    let events = block_on(async move {
        let mut collected = Vec::new();
        while let Some(event) = stream.next().await {
            collected.push(event.expect("event"));
        }
        collected
    });
    assert!(events.iter().any(|event| {
        matches!(event.kind(), RuntimeEventKind::ReasoningProgress)
    }));
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
    assert_eq!(block_on(session.close()), CleanupOutcome::Clean);

    let process = host
        .process
        .lock()
        .expect("process lock")
        .clone()
        .expect("process observed");
    assert_eq!(process.arguments, ["acp"]);
    assert_eq!(process.environment_count, 1);
    assert_eq!(
        process.resource,
        Some(WorkingResourceRef::new("cursor.fixture.workspace").expect("resource"))
    );
    assert_eq!(host.resource_releases.load(Ordering::SeqCst), 1);
    let writes = host.writes();
    assert!(!writes.iter().any(|message| {
        message.get("method").and_then(Value::as_str) == Some("authenticate")
    }));
    assert_eq!(
        writes
            .iter()
            .filter_map(|message| message.get("method").and_then(Value::as_str))
            .collect::<Vec<_>>(),
        ["initialize", "session/new", "session/prompt"]
    );
    assert_eq!(
        writes[0]["params"]["clientCapabilities"]["fs"],
        json!({"readTextFile": true, "writeTextFile": false})
    );
}

#[test]
fn permission_is_observed_and_cancelled_without_ambient_approval() {
    let (host, services, mut session) = open(Scenario::Permission);
    let mut turn = start(&mut *session, services, "cursor-permission-turn");
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
    assert_eq!(block_on(session.close()), CleanupOutcome::Clean);
}

#[test]
fn active_turn_cancellation_waits_for_native_cancelled_result() {
    let (_host, services, mut session) = open(Scenario::Cancellation);
    let mut turn = start(&mut *session, services, "cursor-cancel-turn");
    block_on(turn.cancellation().request()).expect("cancel sent");
    let outcome = block_on(
        turn.take_terminal_outcome()
            .expect("terminal outcome available"),
    );
    assert_eq!(outcome.status(), &TerminalStatus::Cancelled);
    assert_eq!(block_on(turn.close()), CleanupOutcome::NotApplicable);
    assert_eq!(block_on(session.close()), CleanupOutcome::Clean);
}

#[test]
fn local_and_remote_authoritative_hosts_share_the_same_wire_shape() {
    for host_id in ["fixture.host.local", "fixture.host.remote-authoritative"] {
        let host_id = ExecutionHostId::new(host_id).expect("host");
        let (host, services, mut session) = open_on(host_id.clone(), Scenario::Success);
        let mut turn = start(&mut *session, services, "cursor-topology-turn");
        let outcome = block_on(turn.take_terminal_outcome().expect("terminal"));
        assert_eq!(outcome.status(), &TerminalStatus::Completed);
        assert_eq!(block_on(turn.close()), CleanupOutcome::NotApplicable);
        assert_eq!(block_on(session.close()), CleanupOutcome::Clean);
        assert_eq!(
            selection(host_id.clone()).plan.execution_host_id(),
            &host_id
        );
        assert_eq!(
            host.process
                .lock()
                .expect("process lock")
                .as_ref()
                .expect("process")
                .arguments,
            ["acp"]
        );
    }
}

#[test]
fn malformed_negotiation_and_disconnect_are_sanitized_and_joined() {
    let host_id = ExecutionHostId::new("fixture.host.cursor.malformed").expect("host");
    let selected = selection(host_id.clone());
    let malformed = FixtureHost::new(Scenario::Malformed);
    let services = malformed.services(host_id);
    let driver = CursorAcpDriver::new(
        EnvironmentRef::new("cursor.fixture.ambient").expect("environment"),
    );
    let request = OpenSessionRequest::new(
        RequestId::new("cursor-malformed").expect("request"),
        selected.resource,
        None,
        SessionPlanAgreement::explicit(
            SessionAccessPolicy::ambient_harness(ResourceAccess::ReadWrite),
            Some(SessionProviderStatePolicy::DurableProviderSessionPreserved),
            Some(HarnessConfigurationPosture::Ambient),
        ),
    );
    let error = match block_on(driver.open_session(selected.plan, request, services)) {
        Ok(_) => panic!("malformed initialization must fail"),
        Err(error) => error,
    };
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.cursor.acp.response_malformed"
    );
    assert!(!format!("{error:?}").contains("private-fixture-secret"));
    assert_eq!(malformed.resource_releases.load(Ordering::SeqCst), 1);

    let (_host, services, mut session) = open(Scenario::Disconnect);
    let mut turn = start(&mut *session, services, "cursor-disconnect-turn");
    let outcome = block_on(turn.take_terminal_outcome().expect("terminal"));
    assert!(matches!(outcome.status(), TerminalStatus::RuntimeFailed(_)));
    assert_eq!(block_on(turn.close()), CleanupOutcome::NotApplicable);
    assert_eq!(block_on(session.close()), CleanupOutcome::Clean);
}

#[test]
fn descriptor_and_conformance_keep_the_route_bounded() {
    let descriptor = cursor_acp_descriptor();
    assert!(descriptor.supports_role(DriverRole::Discovery));
    assert!(descriptor.supports_role(DriverRole::InteractiveSession));
    assert!(!descriptor.supports_role(DriverRole::StructuredRun));
    assert!(!descriptor
        .required_host_services(DriverRole::InteractiveSession)
        .any(|service| service == swallowtail_core::HostServiceKind::Credential));

    use swallowtail_testkit::{ConformanceAssertion, run_acp_single_turn_projection_assertions};
    let report = run_acp_single_turn_projection_assertions();
    for assertion in [
        ConformanceAssertion::PreflightBeforeSideEffects,
        ConformanceAssertion::OrderedEvents,
        ConformanceAssertion::SingleTerminalOutcome,
        ConformanceAssertion::CancellationAndTimeoutDistinct,
        ConformanceAssertion::CleanupRemainsVisible,
        ConformanceAssertion::DurableRetentionExplicit,
        ConformanceAssertion::NoTranscriptDeletionClaim,
    ] {
        assert!(report.covers(assertion), "missing {assertion:?}");
    }
}

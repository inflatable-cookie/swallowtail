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
    let mut turn = start(&mut *session, services, "grok-success-turn");
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
    assert_eq!(block_on(session.close()), CleanupOutcome::Clean);

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
fn permission_is_observed_and_cancelled_without_ambient_approval() {
    let (host, services, mut session) = open(Scenario::Permission);
    let mut turn = start(&mut *session, services, "grok-permission-turn");
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
    let mut turn = start(&mut *session, services, "grok-cancel-turn");
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
fn structured_projection_preserves_events_output_and_durable_provider_state() {
    let host = FixtureHost::new(Scenario::Success);
    let host_id = ExecutionHostId::new("fixture.host.grok.structured").expect("host");
    let mut run = start_run(host_id, &host, "0.2.114", None);
    assert_eq!(
        run.cancellation().scope(),
        swallowtail_core::CancellationScope::StructuredRun
    );
    assert!(run.take_callbacks().is_none());
    let mut events = run.take_events().expect("events");
    let events = block_on(async move {
        let mut collected = Vec::new();
        while let Some(event) = events.next().await {
            collected.push(event.expect("event"));
        }
        collected
    });
    let outcome = block_on(run.take_terminal_outcome().expect("terminal"));
    assert_eq!(outcome.status(), &TerminalStatus::Completed);
    assert_eq!(
        outcome.output().expect("output").as_str(),
        "Fixture response."
    );
    assert!(events.iter().any(|event| {
        matches!(
            event.kind(),
            RuntimeEventKind::Activity(_) | RuntimeEventKind::ReasoningProgress
        )
    }));
    assert_eq!(outcome.cleanup(), &CleanupOutcome::Clean);
    assert_eq!(block_on(run.close()), CleanupOutcome::Clean);
    assert_eq!(host.credential_releases.load(Ordering::SeqCst), 1);
    assert_eq!(host.resource_releases.load(Ordering::SeqCst), 1);
}

#[test]
fn structured_cancellation_deadline_and_provider_request_remain_distinct() {
    for (scenario, expected, deadline) in [
        (Scenario::Cancellation, "cancelled", None),
        (
            Scenario::Deadline,
            "timed-out",
            Some(Deadline::at(MonotonicInstant::from_ticks(10))),
        ),
        (Scenario::Permission, "provider-request", None),
    ] {
        let host = FixtureHost::new(scenario);
        let host_id = ExecutionHostId::new(format!("fixture.host.grok.{expected}")).expect("host");
        let mut run = start_run(host_id, &host, "0.2.114", deadline);
        if matches!(scenario, Scenario::Cancellation) {
            block_on(run.cancellation().request()).expect("cancel requested");
        }
        let outcome = block_on(run.take_terminal_outcome().expect("terminal"));
        match expected {
            "cancelled" => assert_eq!(outcome.status(), &TerminalStatus::Cancelled),
            "timed-out" => assert_eq!(outcome.status(), &TerminalStatus::TimedOut),
            "provider-request" => assert!(matches!(
                outcome.status(),
                TerminalStatus::ProviderRequestObserved(_)
            )),
            _ => unreachable!(),
        }
        assert_eq!(outcome.cleanup(), &CleanupOutcome::Clean);
        assert_eq!(block_on(run.close()), CleanupOutcome::Clean);
    }
}

#[test]
fn exact_structured_projection_runs_on_both_authoritative_host_topologies() {
    for host in [
        "fixture.host.local",
        "fixture.host.remote-authoritative",
    ] {
        let host = ExecutionHostId::new(host).expect("host");
        let fixture = FixtureHost::new(Scenario::Success);
        let mut run = start_run(host.clone(), &fixture, "0.2.114", None);
        let outcome = block_on(run.take_terminal_outcome().expect("terminal"));
        assert_eq!(outcome.status(), &TerminalStatus::Completed);
        assert_eq!(outcome.cleanup(), &CleanupOutcome::Clean);
        assert_eq!(block_on(run.close()), CleanupOutcome::Clean);
        let selected = run_selection(host.clone(), "0.2.114");
        assert_eq!(selected.plan.execution_host_id(), &host);
    }
}

#[test]
fn unverified_newer_executes_without_becoming_guaranteed_support() {
    let claim = swallowtail_adapter_grok::grok_build_acp_claim();
    let version = swallowtail_core::InterfaceVersion::new("0.2.115").expect("version");
    assert!(!claim.supports(&version));
    assert!(claim.permits(&version));
    assert!(matches!(
        claim.assess(&version),
        swallowtail_core::InterfaceCompatibilityAssessment::UnverifiedNewer(_)
    ));
    let host = FixtureHost::with_version(Scenario::Success, "0.2.115");
    let host_id = ExecutionHostId::new("fixture.host.grok.unverified").expect("host");
    let mut run = start_run(host_id, &host, "0.2.115", None);
    let outcome = block_on(run.take_terminal_outcome().expect("terminal"));
    assert_eq!(outcome.status(), &TerminalStatus::Completed);
    assert_eq!(outcome.cleanup(), &CleanupOutcome::Clean);
    assert_eq!(block_on(run.close()), CleanupOutcome::Clean);
}

#[test]
fn excluded_version_is_rejected_before_an_attachment_can_be_planned() {
    let claim = swallowtail_adapter_grok::grok_build_acp_claim();
    let version = swallowtail_core::InterfaceVersion::new("0.2.113").expect("version");
    assert!(!claim.permits(&version));
    assert!(matches!(
        claim.assess(&version),
        swallowtail_core::InterfaceCompatibilityAssessment::Incompatible
    ));
}

#[test]
fn disconnect_and_malformed_protocol_are_distinct_and_cleanup_stays_joined() {
    let disconnected = FixtureHost::new(Scenario::Disconnect);
    let host_id = ExecutionHostId::new("fixture.host.grok.disconnect").expect("host");
    let mut run = start_run(host_id, &disconnected, "0.2.114", None);
    let outcome = block_on(run.take_terminal_outcome().expect("terminal"));
    assert!(matches!(
        outcome.status(),
        TerminalStatus::RuntimeFailed(_)
    ));
    assert_eq!(outcome.cleanup(), &CleanupOutcome::Clean);
    assert_eq!(block_on(run.close()), CleanupOutcome::Clean);

    let malformed = FixtureHost::new(Scenario::Malformed);
    let host_id = ExecutionHostId::new("fixture.host.grok.malformed").expect("host");
    let error = match try_start_run(host_id, &malformed, "0.2.114", None) {
        Ok(_) => panic!("malformed initialization must fail"),
        Err(error) => error,
    };
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.grok.acp.response_malformed"
    );
    assert!(!format!("{error:?}").contains("private-fixture-secret"));
    assert_eq!(malformed.credential_releases.load(Ordering::SeqCst), 1);
    assert_eq!(malformed.resource_releases.load(Ordering::SeqCst), 1);
}

#[test]
fn provider_neutral_acp_projection_assertions_cover_grok_run_invariants() {
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

#[test]
fn descriptor_keeps_discovery_interactive_and_structured_roles_separate() {
    let descriptor = grok_build_acp_descriptor();
    assert!(descriptor.supports_role(DriverRole::Discovery));
    assert!(descriptor.supports_role(DriverRole::InteractiveSession));
    assert!(descriptor.supports_role(DriverRole::StructuredRun));
    let services = descriptor
        .required_host_services(DriverRole::StructuredRun)
        .collect::<Vec<_>>();
    assert!(services.contains(&swallowtail_core::HostServiceKind::Credential));
    assert!(services.contains(&swallowtail_core::HostServiceKind::WorkingResourceIo));
    assert!(services.contains(&swallowtail_core::HostServiceKind::Time));
}

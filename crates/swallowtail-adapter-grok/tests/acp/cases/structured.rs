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


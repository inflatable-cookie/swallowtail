#[test]
fn disconnect_and_malformed_protocol_are_distinct_and_cleanup_stays_joined() {
    let disconnected = FixtureHost::new(Scenario::Disconnect);
    let host_id = ExecutionHostId::new("fixture.host.grok.disconnect").expect("host");
    let mut run = start_run(host_id, &disconnected, "0.2.114", Some(run_deadline()));
    let outcome = block_on(run.take_terminal_outcome().expect("terminal"));
    assert!(matches!(
        outcome.status(),
        TerminalStatus::RuntimeFailed(_)
    ));
    assert_eq!(outcome.cleanup(), &CleanupOutcome::Clean);
    assert_eq!(block_on(run.close()), CleanupOutcome::Clean);

    let malformed = FixtureHost::new(Scenario::Malformed);
    let host_id = ExecutionHostId::new("fixture.host.grok.malformed").expect("host");
    let error = match try_start_run(host_id, &malformed, "0.2.114", Some(run_deadline())) {
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

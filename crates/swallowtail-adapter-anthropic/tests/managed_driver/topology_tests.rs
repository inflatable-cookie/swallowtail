#[test]
fn provider_managed_profile_runs_the_production_driver_in_both_topologies() {
    let report = run_provider_managed_harness_profile();
    for assertion in [
        ConformanceAssertion::ProviderManagedHarnessLifecycle,
        ConformanceAssertion::DurableRetentionExplicit,
        ConformanceAssertion::ManagedRecoveryExplicit,
        ConformanceAssertion::OwnedRemoteDeletionTruth,
        ConformanceAssertion::CallbackExchange,
        ConformanceAssertion::HostTopologyPreserved,
    ] {
        assert!(report.covers(assertion), "missing {assertion:?}");
    }

    for topology in [
        ExecutionTopologyFixture::local(),
        ExecutionTopologyFixture::remote_authoritative(),
    ] {
        let fixture = Fixture::for_topology(topology.clone());
        let plan = fixture.plan();
        assert_eq!(plan.execution_host_id(), topology.execution_host_id());
        assert_eq!(plan.instance_id(), topology.configured_instance_id());
        assert_eq!(plan.instance_target_ref(), topology.instance_target());
        let (run, events, outcome) = complete(
            &fixture,
            fixture.request(&format!(
                "managed-topology-{}",
                topology.configured_instance_id().as_str()
            )),
        );
        assert_eq!(outcome.status(), &TerminalStatus::Completed);
        assert!(events.iter().any(|event| matches!(
            event.kind(),
            swallowtail_runtime::RuntimeEventKind::ProviderObservation(ProviderObservation::Usage(
                _
            ))
        )));
        assert_eq!(fixture.server.state().session_creations, 1);
        assert_eq!(fixture.server.state().stream_attachments, 1);
        assert_eq!(fixture.credential_releases(), 1);
        assert!(matches!(block_on(run.close()), CleanupOutcome::Clean));
    }
}

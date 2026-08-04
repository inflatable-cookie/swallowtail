#[test]
fn prepared_temporary_profile_attempts_one_delete_without_stateful_reconciliation() {
    let topology = ExecutionTopologyFixture::local();
    let (probe_process, _) = FakeProcessService::completed("0.52.0\n");
    let (probe_services, _) = host_services_for(
        topology.execution_host_id().clone(),
        probe_process,
        Arc::new(PendingTimeService),
    );
    let GeminiCliPreparedIntegration::Headless(prepared) = block_on(prepare_gemini_cli(
        headless_support::cli_preparation_input(topology.execution_host_id().clone()),
        headless_support::cli_probe(),
        probe_services,
    ))
    .expect("Gemini headless prepares") else {
        panic!("headless route remains selected");
    };
    let profile = prepared
        .prepare_run(
            GeminiHeadlessRunProfileInput::new(
                RequestId::new("gemini-owned-cleanup").expect("request id is valid"),
                GeminiHeadlessModelSelection::new(
                    ModelRouteId::new("gemini.prepared.route").expect("route id is valid"),
                    ModelRouteRevision::new("1").expect("route revision is valid"),
                    ProviderId::new("gemini").expect("provider id is valid"),
                    ModelId::new("gemini-2.5-flash").expect("model id is valid"),
                ),
                OperationContent::new("temporary private prompt").expect("prompt is valid"),
                topology.working_resource().clone(),
                Deadline::at(MonotonicInstant::from_ticks(1_000)),
            )
            .with_owned_transcript_cleanup(),
        )
        .expect("temporary run prepares");
    assert_eq!(
        profile.request().policy().provider_retention(),
        ProviderRetentionPolicy::TemporaryAllowed
    );
    let provider_id = headless_support::session_id("gemini-owned-cleanup");
    let (process, states) = ScriptedProcessService::new([
        (
            fixture("success.jsonl", "gemini-owned-cleanup"),
            String::new(),
            ProcessExit::new(true, Some(0)),
        ),
        (
            "Deleted session 1: private".to_owned(),
            String::new(),
            ProcessExit::new(true, Some(0)),
        ),
    ]);
    let (services, task) = host_services_for(
        topology.execution_host_id().clone(),
        process,
        Arc::new(PendingTimeService),
    );
    let mut run = block_on(profile.start_run(services)).expect("temporary run starts");
    let outcome = block_on(
        run.take_terminal_outcome()
            .expect("terminal outcome is available"),
    );
    assert_eq!(outcome.status(), &TerminalStatus::Completed);
    assert_eq!(
        outcome.remote_resource_deletion(OwnedRemoteResourceKind::Session),
        Some(RemoteResourceDeletionOutcome::Unconfirmed)
    );
    assert!(matches!(outcome.cleanup(), CleanupOutcome::Degraded(_)));
    assert!(run.take_management_binding().is_none());
    assert_eq!(block_on(run.close()), CleanupOutcome::Clean);
    assert!(task.joined());
    assert_eq!(
        states[1].request().arguments,
        ["--delete-session", provider_id.as_str()]
    );
    assert_eq!(states.len(), 2);
    assert!(states.iter().all(|state| state.waited()));
}

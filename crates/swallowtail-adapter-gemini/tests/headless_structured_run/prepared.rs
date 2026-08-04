#[test]
fn prepared_facade_discovers_exact_version_and_starts_a_bound_run() {
    let descriptor = swallowtail_adapter_gemini::gemini_headless_descriptor();
    assert!(!descriptor.supports_role(DriverRole::ProviderSessionManagement));
    assert!(!descriptor.supports_operation_shape(OperationShape::ProviderSessionManagement));
    for topology in [
        ExecutionTopologyFixture::local(),
        ExecutionTopologyFixture::remote_authoritative(),
    ] {
        let (probe_process, probe_state) = FakeProcessService::completed("0.52.0\n");
        let (probe_services, _) = host_services_for(
            topology.execution_host_id().clone(),
            probe_process,
            Arc::new(PendingTimeService),
        );
        let selected = block_on(prepare_gemini_cli(
            headless_support::cli_preparation_input(topology.execution_host_id().clone()),
            headless_support::cli_probe(),
            probe_services,
        ))
        .expect("Gemini headless prepares");
        assert_eq!(selected.driver(), GeminiCliPreparedDriver::Headless);
        let GeminiCliPreparedIntegration::Headless(prepared) = selected else {
            panic!("headless route remains explicitly selected");
        };
        assert_eq!(probe_state.request().arguments, ["--version"]);
        assert_eq!(
            prepared.observation().version().version().as_str(),
            "0.52.0"
        );

        let profile = prepared
            .prepare_run(GeminiHeadlessRunProfileInput::new(
                RequestId::new("gemini-prepared-run").expect("request id is valid"),
                GeminiHeadlessModelSelection::new(
                    ModelRouteId::new("gemini.prepared.route").expect("route id is valid"),
                    ModelRouteRevision::new("1").expect("route revision is valid"),
                    ProviderId::new("gemini").expect("provider id is valid"),
                    ModelId::new("gemini-2.5-flash").expect("model id is valid"),
                ),
                OperationContent::new("prepared private prompt").expect("prompt is valid"),
                WorkingResourceRef::new(topology.working_resource().as_host_value())
                    .expect("working resource is valid"),
                Deadline::at(MonotonicInstant::from_ticks(1_000)),
            ))
            .expect("Gemini headless run prepares");
        assert_eq!(
            profile.plan().harness_configuration_posture(),
            Some(HarnessConfigurationPosture::Ambient)
        );
        assert_eq!(
            profile.plan().requirements().harness_isolation(),
            Some(HarnessIsolation::AmbientHost)
        );
        assert_prepared_operation_evidence_matches_plan(
            profile.evidence().operation(),
            profile.plan(),
        );
        assert_eq!(
            profile.evidence().observable_activity().availability(),
            ObservableActivityAvailability::Available
        );

        let output = fixture("success.jsonl", "gemini-prepared-run");
        let (run_process, run_state) = FakeProcessService::completed(&output);
        let (run_services, task) = host_services_for(
            topology.execution_host_id().clone(),
            run_process,
            Arc::new(PendingTimeService),
        );
        let mut run = block_on(profile.start_run(run_services)).expect("prepared run starts");
        assert!(run.provider_run_ref().is_some());
        assert!(run.take_management_binding().is_none());
        let terminal = block_on(
            run.take_terminal_outcome()
                .expect("terminal outcome is available"),
        );
        assert_eq!(terminal.status(), &TerminalStatus::Completed);
        assert!(run.take_management_binding().is_none());
        assert_eq!(block_on(run.close()), CleanupOutcome::Clean);
        assert!(run_state.waited());
        assert!(task.joined());
        assert_eq!(run_state.stdin(), b"prepared private prompt");
    }
}

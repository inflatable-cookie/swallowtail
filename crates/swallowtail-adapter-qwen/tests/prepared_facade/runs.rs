#[test]
fn prepared_runs_preserve_qwen_stdin_budgets_and_ambient_truth_in_both_topologies() {
    for host_value in [
        "fixture.qwen.prepared.local",
        "fixture.qwen.prepared.remote",
    ] {
        let host_id = ExecutionHostId::new(host_value).expect("valid host");
        let (discovery_process, discovery_state) = FakeProcessService::completed("0.19.11\n");
        let (discovery_services, _) = host_services_for(
            host_id.clone(),
            discovery_process,
            Arc::new(PendingTimeService),
        );
        let prepared = block_on(prepare_qwen_headless(
            preparation_input(host_id.clone()),
            probe(),
            discovery_services,
        ))
        .expect("Qwen prepares");
        assert_eq!(discovery_state.request().arguments, ["--version"]);

        let profile = prepared
            .prepare_run(QwenRunProfileInput::new(
                RequestId::new("qwen-prepared-run").expect("valid request"),
                QwenModelSelection::new(
                    ModelRouteId::new("qwen.prepared.route").expect("valid route"),
                    ModelRouteRevision::new("1").expect("valid route revision"),
                    ProviderId::new("alibaba-modelstudio").expect("valid provider"),
                    ModelId::new("qwen3-coder-plus").expect("valid model"),
                ),
                OperationContent::new("prepared private prompt").expect("valid prompt"),
                WorkingResourceRef::new("qwen.prepared.workspace").expect("valid resource"),
                Deadline::at(MonotonicInstant::from_ticks(1_000)),
            ))
            .expect("Qwen run profile prepares");
        assert_eq!(
            profile.plan().harness_configuration_posture(),
            Some(HarnessConfigurationPosture::Ambient)
        );
        assert_eq!(
            profile.plan().requirements().harness_isolation(),
            Some(HarnessIsolation::AmbientHost)
        );
        assert_eq!(
            profile.plan().provider_id().map(ProviderId::as_str),
            Some("alibaba-modelstudio")
        );
        assert_prepared_operation_evidence_matches_plan(
            profile.evidence().operation(),
            profile.plan(),
        );
        assert_eq!(
            profile.evidence().observable_activity().availability(),
            ObservableActivityAvailability::Available
        );

        let (operation_process, operation_state) = FakeProcessService::completed(include_str!(
            "../fixtures/qwen-code-v0.19.11/success.jsonl"
        ));
        let (operation_services, _) =
            host_services_for(host_id, operation_process, Arc::new(PendingTimeService));
        let mut run = block_on(profile.start_run(operation_services)).expect("prepared run starts");
        let terminal = block_on(
            run.take_terminal_outcome()
                .expect("terminal outcome is available"),
        );
        assert_eq!(terminal.status(), &TerminalStatus::Completed);
        assert_eq!(block_on(run.close()), CleanupOutcome::Clean);
        assert_eq!(operation_state.stdin(), b"prepared private prompt");
        let arguments = operation_state.request().arguments;
        for exact in [
            "--input-format",
            "text",
            "--output-format",
            "stream-json",
            "--max-wall-time",
            "60s",
            "--max-tool-calls",
            "16",
            "--max-session-turns",
            "24",
        ] {
            assert!(arguments.iter().any(|argument| argument == exact));
        }
        assert!(!arguments.iter().any(|argument| argument == "--sandbox"));
    }
}

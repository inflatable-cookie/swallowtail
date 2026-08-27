fn prepared_at(host: swallowtail_core::ExecutionHostId, version: &str) -> ClaudeCodePreparedIntegration
{
    let (process, state) = FakeProcessService::completed(&format!("{version} (Claude Code)\n"));
    let (services, task) = host_services(host.clone(), process, Arc::new(PendingTimeService));
    let prepared = block_on(prepare_claude_code_headless(
        preparation_input(host),
        preparation_probe(),
        services,
    ))
    .expect("Claude Code headless prepares");
    assert!(state.waited());
    assert!(task.joined());
    assert_eq!(prepared.observation().version().version().as_str(), version);
    prepared
}

fn run_profile_input(
    resource: WorkingResourceRef,
    id: &str,
) -> swallowtail_adapter_claude_agent::ClaudeCodeRunProfileInput {
    ClaudeCodeRunProfileInput::new(
        RequestId::new(format!("claude-code-{id}")).expect("request is valid"),
        ClaudeCodeModelSelection::new(
            ModelRouteId::new(format!("claude-code.{id}")).expect("route is valid"),
            ModelRouteRevision::new("1").expect("route revision is valid"),
            ModelId::new("claude-opus-5").expect("model is valid"),
        ),
        OperationContent::new("private Claude fixture prompt").expect("content is valid"),
        resource,
        Deadline::at(MonotonicInstant::from_ticks(1_000)),
    )
}

#[test]
fn admitted_maximum_turns_dispatch_one_canonical_argument_and_omission_is_byte_identical() {
    let topology = ExecutionTopologyFixture::local();
    let prepared = prepared(topology.execution_host_id().clone());

    let omitted = prepared
        .prepare_run(run_profile_input(
            topology.working_resource().clone(),
            "turns-omitted",
        ))
        .expect("Claude Code run prepares");
    assert_eq!(omitted.maximum_turns(), None);
    assert_eq!(omitted.evidence().maximum_turns(), None);

    let omitted_run = execute(
        &omitted,
        topology.execution_host_id().clone(),
        &fixture("headless-complete.jsonl"),
        ProcessExit::new(true, Some(0)),
    );
    assert_eq!(omitted_run.outcome.status(), &TerminalStatus::Completed);
    assert!(
        !omitted_run
            .request
            .arguments
            .iter()
            .any(|argument| argument == "--max-turns")
    );
    assert_eq!(
        omitted_run.request.environments,
        ["claude.fixture.local-subscription-environment"]
    );

    for admitted in [1_u64, 3, 30, u64::from(u32::MAX)] {
        let selection = ClaudeCodeMaximumTurns::from_u64(admitted).expect("value is admitted");
        let selected = prepared
            .prepare_run(
                run_profile_input(topology.working_resource().clone(), "turns-selected")
                    .with_maximum_turns(selection),
            )
            .expect("Claude Code run prepares");
        assert_eq!(selected.maximum_turns(), Some(selection));
        assert_eq!(selected.evidence().maximum_turns(), Some(selection));

        let run = execute(
            &selected,
            topology.execution_host_id().clone(),
            &fixture("headless-complete.jsonl"),
            ProcessExit::new(true, Some(0)),
        );
        let expected = admitted.to_string();
        assert_eq!(
            run.request.arguments[..omitted_run.request.arguments.len()],
            omitted_run.request.arguments[..]
        );
        assert_eq!(
            run.request.arguments[omitted_run.request.arguments.len()..],
            ["--max-turns", expected.as_str()]
        );
        assert_eq!(
            run.request
                .arguments
                .iter()
                .filter(|argument| *argument == "--max-turns")
                .count(),
            1
        );
        assert_eq!(
            run.request.environments,
            ["claude.fixture.local-subscription-environment"]
        );
        assert_eq!(run.outcome.status(), &TerminalStatus::Completed);
        assert!(run.stdin_closed);
    }
}

#[test]
fn maximum_turns_preserves_reasoning_model_authority_and_low_level_driver_agreement() {
    let topology = ExecutionTopologyFixture::local();
    let prepared = prepared(topology.execution_host_id().clone());
    let selection = ClaudeCodeMaximumTurns::from_u64(4).expect("value is admitted");
    let selected = prepared
        .prepare_run(
            run_profile_input(topology.working_resource().clone(), "turns-compose")
                .with_reasoning_mode(ReasoningMode::new("high").expect("reasoning mode is valid"))
                .with_maximum_turns(selection),
        )
        .expect("Claude Code run prepares");

    assert_eq!(
        selected.request().policy().harness_mode(),
        Some(HarnessMode::Plan)
    );
    assert_eq!(
        selected.request().policy().provider_retention(),
        ProviderRetentionPolicy::Prohibited
    );
    assert_eq!(
        selected.plan().harness_configuration_posture(),
        Some(HarnessConfigurationPosture::Ambient)
    );
    assert_eq!(
        selected.plan().requirements().harness_isolation(),
        Some(HarnessIsolation::AmbientHost)
    );

    let run = execute(
        &selected,
        topology.execution_host_id().clone(),
        &fixture("headless-complete.jsonl"),
        ProcessExit::new(true, Some(0)),
    );
    assert_eq!(
        run.request.arguments,
        [
            "-p",
            "--input-format",
            "text",
            "--output-format",
            "stream-json",
            "--verbose",
            "--no-session-persistence",
            "--model",
            "claude-opus-5",
            "--effort",
            "high",
            "--permission-mode",
            "plan",
            "--tools",
            "Read,Glob,Grep",
            "--setting-sources",
            "user,project,local",
            "--mcp-config",
            r#"{"mcpServers":{}}"#,
            "--strict-mcp-config",
            "--max-turns",
            "4",
        ]
    );

    // The extracted low-level driver is deliberately unbound, even paired with
    // this run's own plan and request. Prepared `start_run` is the only path
    // that dispatches a bound.
    let (process, state) = FakeProcessService::with_exit(
        &fixture("headless-complete.jsonl"),
        ProcessExit::new(true, Some(0)),
    );
    let (services, task) = host_services(
        topology.execution_host_id().clone(),
        process,
        Arc::new(PendingTimeService),
    );
    let mut low_level = block_on(selected.low_level_driver().start_run(
        selected.plan().clone(),
        selected.request().clone(),
        services,
    ))
    .expect("low-level run starts");
    let _ = block_on(
        low_level
            .take_terminal_outcome()
            .expect("terminal outcome is available"),
    );
    assert_eq!(block_on(low_level.close()), CleanupOutcome::Clean);
    assert!(task.joined());
    assert!(
        !state
            .request()
            .arguments
            .iter()
            .any(|argument| argument == "--max-turns")
    );
    // Everything else about the extracted driver's command is unchanged.
    assert_eq!(
        state.request().arguments,
        run.request.arguments[..run.request.arguments.len() - 2]
    );
}

struct RunEvidence {
    events: Vec<RuntimeEvent>,
    outcome: TerminalOutcome,
    request: claude_code_support::ObservedProcessRequest,
    stdin: Vec<u8>,
    stdin_closed: bool,
}

fn execute(
    profile: &ClaudeCodePreparedRun,
    host: swallowtail_core::ExecutionHostId,
    output: &str,
    exit: ProcessExit,
) -> RunEvidence {
    let (process, state) = FakeProcessService::with_exit(output, exit);
    let (services, task) = host_services(host, process, Arc::new(PendingTimeService));
    let mut run = block_on(profile.start_run(services)).expect("run starts");
    assert!(run.provider_run_ref().is_none());
    let events = block_on(
        run.take_events()
            .expect("event stream is available")
            .collect::<Vec<_>>(),
    )
    .into_iter()
    .collect::<Result<Vec<_>, _>>()
    .expect("events are valid");
    let outcome = block_on(
        run.take_terminal_outcome()
            .expect("terminal outcome is available"),
    );
    assert_eq!(block_on(run.close()), CleanupOutcome::Clean);
    assert!(state.waited());
    assert!(task.joined());
    RunEvidence {
        events,
        outcome,
        request: state.request(),
        stdin: state.stdin(),
        stdin_closed: state.stdin_closed(),
    }
}

fn prepared(host: swallowtail_core::ExecutionHostId) -> ClaudeCodePreparedIntegration {
    let (process, state) = FakeProcessService::completed("2.1.220 (Claude Code)\n");
    let (services, task) = host_services(host.clone(), process, Arc::new(PendingTimeService));
    let prepared = block_on(prepare_claude_code_headless(
        preparation_input(host),
        preparation_probe(),
        services,
    ))
    .expect("Claude Code headless prepares");
    assert_eq!(state.request().arguments, ["--version"]);
    assert!(state.waited());
    assert!(task.joined());
    assert_eq!(
        prepared.observation().version().version().as_str(),
        "2.1.220"
    );
    prepared
}

fn profile(
    prepared: &ClaudeCodePreparedIntegration,
    resource: WorkingResourceRef,
    id: &str,
    reasoning: Option<&str>,
) -> ClaudeCodePreparedRun {
    let input = ClaudeCodeRunProfileInput::new(
        RequestId::new(format!("claude-code-{id}")).expect("request is valid"),
        ClaudeCodeModelSelection::new(
            ModelRouteId::new(format!("claude-code.{id}")).expect("route is valid"),
            ModelRouteRevision::new("1").expect("route revision is valid"),
            ModelId::new("claude-opus-5").expect("model is valid"),
        ),
        OperationContent::new("private Claude fixture prompt").expect("content is valid"),
        resource,
        Deadline::at(MonotonicInstant::from_ticks(1_000)),
    );
    let input = match reasoning {
        Some(reasoning) => input
            .with_reasoning_mode(ReasoningMode::new(reasoning).expect("reasoning mode is valid")),
        None => input,
    };
    prepared
        .prepare_run(input)
        .expect("Claude Code run prepares")
}

fn assert_status(outcome: &TerminalOutcome, code: &str, provider: bool) {
    let diagnostic = match outcome.status() {
        TerminalStatus::ProviderFailed(diagnostic) if provider => diagnostic,
        TerminalStatus::RuntimeFailed(diagnostic) if !provider => diagnostic,
        status => panic!("unexpected status {status:?}"),
    };
    assert_eq!(diagnostic.code(), code);
}

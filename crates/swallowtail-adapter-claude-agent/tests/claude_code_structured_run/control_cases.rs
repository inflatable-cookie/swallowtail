#[test]
fn unsupported_input_cancellation_and_timeout_are_bounded_before_cleanup() {
    let topology = ExecutionTopologyFixture::local();
    let prepared = prepared(topology.execution_host_id().clone());
    let profile = profile(
        &prepared,
        topology.working_resource().clone(),
        "bounded",
        None,
    );

    let request =
        profile
            .request()
            .clone()
            .with_tools([swallowtail_runtime::ToolDeclaration::new(
                "consumer-tool",
                swallowtail_runtime::SchemaDocument::inline(
                    br#"{"type":"object"}"#.to_vec(),
                    1_024,
                )
                .expect("schema is valid"),
                "application/schema+json",
                "json-schema-2020-12",
            )
            .expect("tool is valid")]);
    let (process, state) = FakeProcessService::completed("");
    let (services, _) = host_services(
        topology.execution_host_id().clone(),
        process,
        Arc::new(PendingTimeService),
    );
    let result = block_on(profile.low_level_driver().start_run(
        profile.plan().clone(),
        request,
        services,
    ));
    assert!(result.is_err());
    assert!(!state.started());

    let (process, state) = FakeProcessService::held_open();
    let (services, task) = host_services(
        topology.execution_host_id().clone(),
        process,
        Arc::new(PendingTimeService),
    );
    let mut run = block_on(profile.start_run(services)).expect("cancellable run starts");
    assert_eq!(
        block_on(run.cancellation().request()).expect("cancellation succeeds"),
        CancellationAcknowledgement::Requested
    );
    assert_eq!(
        block_on(run.cancellation().request()).expect("repeat cancellation succeeds"),
        CancellationAcknowledgement::AlreadyRequested
    );
    let cancelled = block_on(
        run.take_terminal_outcome()
            .expect("terminal outcome is available"),
    );
    assert_eq!(cancelled.status(), &TerminalStatus::Cancelled);
    assert_eq!(block_on(run.close()), CleanupOutcome::Clean);
    assert!(state.force_stopped());
    assert!(state.waited());
    assert!(task.joined());

    let (process, state) = FakeProcessService::held_open();
    let (services, task) = host_services(
        topology.execution_host_id().clone(),
        process,
        Arc::new(ImmediateTimeService),
    );
    let mut run = block_on(profile.start_run(services)).expect("deadline-bound run starts");
    let timed_out = block_on(
        run.take_terminal_outcome()
            .expect("terminal outcome is available"),
    );
    assert_eq!(timed_out.status(), &TerminalStatus::TimedOut);
    assert_eq!(block_on(run.close()), CleanupOutcome::Clean);
    assert!(state.force_stopped());
    assert!(state.waited());
    assert!(task.joined());
}


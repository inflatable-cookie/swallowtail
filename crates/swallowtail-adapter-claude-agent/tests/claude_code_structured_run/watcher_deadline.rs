#[test]
fn deadline_with_an_active_watcher_emits_exact_host_watcher_lifecycle() {
    let topology = ExecutionTopologyFixture::local();
    let (prepared, local) =
        prepared_at_with_watchers(topology.execution_host_id().clone(), "2.1.251");
    let profile = watcher_profile(
        &prepared,
        topology.working_resource().clone(),
        "watchers-deadline-active",
        true,
    )
    .expect("opt-in prepares");
    let (process, state, _completer) = FakeProcessService::controllable();
    let time = Arc::new(ControllableTimeService::new());
    let (services, task) = watcher_host_services(
        topology.execution_host_id().clone(),
        process,
        Arc::clone(&time) as Arc<dyn swallowtail_runtime::TimeService>,
        &local,
    );
    let mut run = block_on(profile.start_run(services)).expect("run starts");
    let started = std::time::Instant::now();
    while !state.started() {
        assert!(started.elapsed() < Duration::from_secs(2), "process starts");
        std::thread::yield_now();
    }
    let mcp_path = argument_after(&state.request().arguments, "--mcp-config").to_owned();
    let (endpoint, bearer) = read_mcp_authority(&mcp_path);
    let _ = start_sleep_watcher(&endpoint, &bearer);
    time.fire();
    let events = block_on(run.take_events().expect("events").collect::<Vec<_>>())
        .into_iter()
        .collect::<Result<Vec<_>, _>>()
        .expect("events remain valid");
    assert_exact_host_watcher_lifecycle(&events);
    let outcome = block_on(run.take_terminal_outcome().expect("terminal"));
    assert_eq!(outcome.status(), &TerminalStatus::TimedOut);
    assert_eq!(block_on(run.close()), CleanupOutcome::Clean);
    assert!(task.joined());
    assert!(!Path::new(&mcp_path).exists());
}

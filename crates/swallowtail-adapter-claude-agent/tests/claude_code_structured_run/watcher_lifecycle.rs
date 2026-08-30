fn start_sleep_watcher(endpoint: &str, bearer: &str) -> String {
    handshake(endpoint, bearer);
    let (status, body) = post_json(
        endpoint,
        bearer,
        &tool_call(
            2,
            WATCHER_BRIDGE_TOOL_START,
            serde_json::json!({"operation_data": "sleep-operation"}),
        ),
    );
    assert_eq!(status, 200, "{body}");
    tool_payload(&body)["watcher_id"]
        .as_str()
        .expect("watcher id")
        .to_owned()
}

fn host_watcher_phases(events: &[RuntimeEvent]) -> Vec<ActivityLifecyclePhase> {
    events
        .iter()
        .filter_map(|event| match event.kind() {
            RuntimeEventKind::Activity(observation)
                if observation.kind() == &ActivityKind::HostWatcher =>
            {
                Some(observation.phase())
            }
            _ => None,
        })
        .collect()
}

fn assert_exact_host_watcher_lifecycle(events: &[RuntimeEvent]) {
    let phases = host_watcher_phases(events);
    let started = phases
        .iter()
        .filter(|phase| **phase == ActivityLifecyclePhase::Started)
        .count();
    let updated = phases
        .iter()
        .filter(|phase| **phase == ActivityLifecyclePhase::Updated)
        .count();
    let completed = phases
        .iter()
        .filter(|phase| **phase == ActivityLifecyclePhase::Completed)
        .count();
    assert_eq!(started, 1, "started once: {phases:?}");
    assert_eq!(updated, 1, "updated once: {phases:?}");
    assert_eq!(completed, 1, "completed once: {phases:?}");
    let start = phases
        .iter()
        .position(|phase| *phase == ActivityLifecyclePhase::Started)
        .expect("started");
    let update = phases
        .iter()
        .position(|phase| *phase == ActivityLifecyclePhase::Updated)
        .expect("updated");
    let complete = phases
        .iter()
        .position(|phase| *phase == ActivityLifecyclePhase::Completed)
        .expect("completed");
    assert!(
        start < update && update < complete,
        "started → updated → completed: {phases:?}"
    );
}

#[test]
fn silent_provider_fast_watcher_emits_complete_host_watcher_lifecycle() {
    let topology = ExecutionTopologyFixture::local();
    let (prepared, local) =
        prepared_at_with_watchers(topology.execution_host_id().clone(), "2.1.251");
    let profile = watcher_profile(
        &prepared,
        topology.working_resource().clone(),
        "watchers-silent",
        true,
    )
    .expect("opt-in prepares");
    let (process, state, completer) = FakeProcessService::controllable();
    let (services, task) = watcher_host_services(
        topology.execution_host_id().clone(),
        process,
        Arc::new(PendingTimeService),
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
    handshake(&endpoint, &bearer);
    let (status, body) = post_json(
        &endpoint,
        &bearer,
        &tool_call(
            2,
            WATCHER_BRIDGE_TOOL_START,
            serde_json::json!({"operation_data": "exit-zero-operation"}),
        ),
    );
    assert_eq!(status, 200, "{body}");
    let watcher_id = tool_payload(&body)["watcher_id"]
        .as_str()
        .expect("watcher id")
        .to_owned();
    let (status, body) = post_json(
        &endpoint,
        &bearer,
        &tool_call(
            3,
            WATCHER_BRIDGE_TOOL_WAIT,
            serde_json::json!({"watcher_id": watcher_id}),
        ),
    );
    assert_eq!(status, 200, "{body}");
    completer.complete(&fixture("headless-complete.jsonl"), ProcessExit::new(true, Some(0)));
    let events = block_on(run.take_events().expect("events").collect::<Vec<_>>())
        .into_iter()
        .collect::<Result<Vec<_>, _>>()
        .expect("events remain valid");
    assert_exact_host_watcher_lifecycle(&events);
    let outcome = block_on(run.take_terminal_outcome().expect("terminal"));
    assert_eq!(outcome.status(), &TerminalStatus::Completed);
    assert_eq!(block_on(run.close()), CleanupOutcome::Clean);
    assert!(task.joined());
    assert!(!Path::new(&mcp_path).exists());
}

#[test]
fn cancellation_with_an_active_watcher_emits_terminal_host_watcher_activity() {
    let topology = ExecutionTopologyFixture::local();
    let (prepared, local) =
        prepared_at_with_watchers(topology.execution_host_id().clone(), "2.1.251");
    let profile = watcher_profile(
        &prepared,
        topology.working_resource().clone(),
        "watchers-cancel-active",
        true,
    )
    .expect("opt-in prepares");
    let (process, state, _completer) = FakeProcessService::controllable();
    let (services, task) = watcher_host_services(
        topology.execution_host_id().clone(),
        process,
        Arc::new(PendingTimeService),
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
    assert_eq!(
        block_on(run.cancellation().request()).expect("cancel"),
        CancellationAcknowledgement::Requested
    );
    let events = block_on(run.take_events().expect("events").collect::<Vec<_>>())
        .into_iter()
        .collect::<Result<Vec<_>, _>>()
        .expect("events remain valid");
    assert_exact_host_watcher_lifecycle(&events);
    let outcome = block_on(run.take_terminal_outcome().expect("terminal"));
    assert_eq!(outcome.status(), &TerminalStatus::Cancelled);
    assert_eq!(block_on(run.close()), CleanupOutcome::Clean);
    assert!(task.joined());
    assert!(!Path::new(&mcp_path).exists());
}

#[test]
fn provider_failure_with_an_active_watcher_emits_terminal_host_watcher_activity() {
    let topology = ExecutionTopologyFixture::local();
    let (prepared, local) =
        prepared_at_with_watchers(topology.execution_host_id().clone(), "2.1.251");
    let profile = watcher_profile(
        &prepared,
        topology.working_resource().clone(),
        "watchers-provider-failure-active",
        true,
    )
    .expect("opt-in prepares");
    let (process, state, completer) = FakeProcessService::controllable();
    let (services, task) = watcher_host_services(
        topology.execution_host_id().clone(),
        process,
        Arc::new(PendingTimeService),
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
    completer.complete(
        &fixture("headless-provider-failure.jsonl"),
        ProcessExit::new(false, Some(1)),
    );
    let events = block_on(run.take_events().expect("events").collect::<Vec<_>>())
        .into_iter()
        .collect::<Result<Vec<_>, _>>()
        .expect("events remain valid");
    assert_exact_host_watcher_lifecycle(&events);
    let outcome = block_on(run.take_terminal_outcome().expect("terminal"));
    assert!(matches!(outcome.status(), TerminalStatus::ProviderFailed(_)));
    assert_eq!(block_on(run.close()), CleanupOutcome::Clean);
    assert!(task.joined());
    assert!(!Path::new(&mcp_path).exists());
}

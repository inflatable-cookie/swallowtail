#[test]
fn fake_provider_direct_gate_is_not_stop_reentry() {
    let topology = ExecutionTopologyFixture::local();
    let (prepared, local) =
        prepared_at_with_watchers(topology.execution_host_id().clone(), "2.1.251");
    let profile = watcher_profile(
        &prepared,
        topology.working_resource().clone(),
        "watchers-direct-gate",
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
    let turn = RuntimeTurnId::new("claude-code-headless:claude-code-watchers-direct-gate")
        .expect("turn");
    let mut recorder =
        WatcherProofRecorder::new("claude-code-headless:claude-code-watchers-direct-gate");
    let mut events = run.take_events().expect("events");
    handshake(&endpoint, &bearer);
    tools_list(&endpoint, &bearer, 2);
    let (status, body) = post_json(
        &endpoint,
        &bearer,
        &tool_call(
            3,
            WATCHER_BRIDGE_TOOL_START,
            serde_json::json!({"operation_data": "sleep-operation"}),
        ),
    );
    assert_eq!(status, 200, "{body}");
    wait_for_fact(&mut events, &mut recorder, &local, &turn, |fact| {
        matches!(fact, WatcherProofFact::WatcherStarted { .. })
    });
    let (_, blocked) = stop_continuation(&endpoint, &bearer, 4);
    assert_eq!(blocked["allows_successful_completion"], false);
    wait_for_fact(&mut events, &mut recorder, &local, &turn, |fact| {
        matches!(fact, WatcherProofFact::DirectGateActive { .. })
    });
    let watcher_id = tool_payload(&body)["watcher_id"]
        .as_str()
        .expect("watcher id")
        .to_owned();
    let (status, stop_body) = post_json(
        &endpoint,
        &bearer,
        &tool_call(
            5,
            WATCHER_BRIDGE_TOOL_STOP,
            serde_json::json!({"watcher_id": watcher_id}),
        ),
    );
    assert_eq!(status, 200, "{stop_body}");
    let (status, wait_body) = post_json(
        &endpoint,
        &bearer,
        &tool_call(
            6,
            WATCHER_BRIDGE_TOOL_WAIT,
            serde_json::json!({"watcher_id": watcher_id}),
        ),
    );
    assert_eq!(status, 200, "{wait_body}");
    completer.complete(
        concat!(
            "{\"type\":\"system\",\"subtype\":\"init\",\"session_id\":\"fixture-session\",\"model\":\"claude-opus-5\",\"permissionMode\":\"plan\",\"claude_code_version\":\"2.1.251\",\"cwd\":\"/fixture\",\"tools\":[\"Read\"],\"mcp_servers\":[]}\n",
            "{\"type\":\"result\",\"subtype\":\"success\",\"is_error\":false,\"result\":\"WATCHER_LIVE_OK\",\"usage\":{\"input_tokens\":12,\"output_tokens\":3,\"cache_read_input_tokens\":4,\"cache_creation_input_tokens\":1},\"session_id\":\"fixture-session\"}\n",
        ),
        ProcessExit::new(true, Some(0)),
    );
    let remaining = block_on(events.collect::<Vec<_>>())
        .into_iter()
        .collect::<Result<Vec<_>, _>>()
        .expect("events remain valid");
    for event in remaining {
        recorder.ingest_event(&event);
    }
    recorder.ingest_bridge(&local.watcher_bridge_proof(&turn));
    let outcome = block_on(run.take_terminal_outcome().expect("terminal"));
    recorder.ingest_terminal(&outcome);
    assert!(
        recorder
            .facts()
            .iter()
            .any(|fact| matches!(fact, WatcherProofFact::DirectGateActive { .. })),
        "{:?}",
        recorder.facts()
    );
    assert!(
        assert_stop_reentry_proof(recorder.facts()).is_err(),
        "{:?}",
        recorder.facts()
    );
    assert_eq!(block_on(run.close()), CleanupOutcome::Clean);
    assert!(task.joined());
    assert!(!Path::new(&mcp_path).exists());
}

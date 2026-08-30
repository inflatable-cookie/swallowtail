const STOP_REENTRY_TURN: &str = "claude-code-headless:claude-code-watchers-stop-reentry";

fn tools_list(endpoint: &str, bearer: &str, id: u64) {
    let body = serde_json::json!({
        "jsonrpc": WATCHER_BRIDGE_JSONRPC_VERSION,
        "id": id,
        "method": WATCHER_BRIDGE_TOOLS_LIST_METHOD,
        "params": {}
    })
    .to_string();
    let (status, response) = post_json(endpoint, bearer, &body);
    assert_eq!(status, 200, "{response}");
}

fn catch_up(
    events: &mut (impl futures_util::Stream<Item = Result<RuntimeEvent, swallowtail_runtime::RuntimeFailure>>
          + Unpin),
    recorder: &mut WatcherProofRecorder,
    local: &swallowtail_host_local::LocalHostServices,
    turn: &RuntimeTurnId,
) {
    recorder.ingest_bridge(&local.watcher_bridge_proof(turn));
    while let Some(next) = events.next().now_or_never() {
        match next {
            Some(Ok(event)) => {
                recorder.ingest_event(&event);
                recorder.ingest_bridge(&local.watcher_bridge_proof(turn));
            }
            Some(Err(error)) => panic!("event failed: {error}"),
            None => break,
        }
    }
}

fn wait_for_fact(
    events: &mut (impl futures_util::Stream<Item = Result<RuntimeEvent, swallowtail_runtime::RuntimeFailure>>
          + Unpin),
    recorder: &mut WatcherProofRecorder,
    local: &swallowtail_host_local::LocalHostServices,
    turn: &RuntimeTurnId,
    predicate: impl Fn(&WatcherProofFact) -> bool,
) {
    let started = std::time::Instant::now();
    while started.elapsed() < Duration::from_secs(2) {
        catch_up(events, recorder, local, turn);
        if recorder.facts().iter().any(&predicate) {
            return;
        }
        std::thread::sleep(Duration::from_millis(10));
    }
    panic!("missing required proof fact: {:?}", recorder.facts());
}

#[test]
fn fake_provider_stop_reentry_records_the_required_conjunction() {
    let topology = ExecutionTopologyFixture::local();
    let (prepared, local) =
        prepared_at_with_watchers(topology.execution_host_id().clone(), "2.1.251");
    let profile = watcher_profile(
        &prepared,
        topology.working_resource().clone(),
        "watchers-stop-reentry",
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
    let turn = RuntimeTurnId::new(STOP_REENTRY_TURN).expect("turn");
    let mut recorder = WatcherProofRecorder::new(STOP_REENTRY_TURN);
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
    completer.push_stdout(concat!(
        "{\"type\":\"system\",\"subtype\":\"init\",\"session_id\":\"fixture-session\",\"model\":\"claude-opus-5\",\"permissionMode\":\"plan\",\"claude_code_version\":\"2.1.251\",\"cwd\":\"/fixture\",\"tools\":[\"Read\"],\"mcp_servers\":[]}\n",
        "{\"type\":\"system\",\"subtype\":\"hook_started\",\"session_id\":\"fixture-session\",\"uuid\":\"stop-hook\"}\n",
    ));
    wait_for_fact(&mut events, &mut recorder, &local, &turn, |fact| {
        matches!(fact, WatcherProofFact::StopHookStarted { .. })
    });
    let (_, blocked) = stop_continuation(&endpoint, &bearer, 4);
    assert_eq!(blocked["allows_successful_completion"], false);
    wait_for_fact(&mut events, &mut recorder, &local, &turn, |fact| {
        matches!(fact, WatcherProofFact::StopGateActive { .. })
    });
    completer.push_stdout(concat!(
        "{\"type\":\"system\",\"subtype\":\"hook_response\",\"session_id\":\"fixture-session\",\"uuid\":\"stop-hook-response\"}\n",
        "{\"type\":\"assistant\",\"message\":{\"id\":\"msg_continue\",\"type\":\"message\",\"role\":\"assistant\",\"model\":\"claude-opus-5\",\"content\":[{\"type\":\"text\",\"text\":\"continuing after Stop\"}],\"stop_reason\":\"end_turn\",\"usage\":{\"input_tokens\":12,\"output_tokens\":3}},\"parent_tool_use_id\":null,\"uuid\":\"assistant-continue\",\"session_id\":\"fixture-session\"}\n",
    ));
    wait_for_fact(&mut events, &mut recorder, &local, &turn, |fact| {
        matches!(fact, WatcherProofFact::SameSessionContinuation { .. })
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
    wait_for_fact(&mut events, &mut recorder, &local, &turn, |fact| {
        matches!(fact, WatcherProofFact::WaitOrStop { .. })
    });
    completer.complete(
        "{\"type\":\"result\",\"subtype\":\"success\",\"is_error\":false,\"result\":\"WATCHER_LIVE_OK\",\"usage\":{\"input_tokens\":12,\"output_tokens\":3,\"cache_read_input_tokens\":4,\"cache_creation_input_tokens\":1},\"session_id\":\"fixture-session\"}\n",
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
    assert_eq!(outcome.status(), &TerminalStatus::Completed);
    assert_eq!(block_on(run.close()), CleanupOutcome::Clean);
    assert!(
        assert_stop_reentry_proof(recorder.facts()).is_ok(),
        "{:?}",
        recorder.facts()
    );
    assert!(!format!("{:?}", local.watcher_bridge_proof(&turn)).contains(&bearer));
    assert!(task.joined());
    assert!(!Path::new(&mcp_path).exists());
}

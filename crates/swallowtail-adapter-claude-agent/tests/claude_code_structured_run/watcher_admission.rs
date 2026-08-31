#[derive(Debug, Eq, PartialEq)]
enum FixtureBuiltInAdmission {
    Selected(Vec<String>),
    DefaultSet,
}

#[derive(Debug, Eq, PartialEq)]
struct FixtureToolAdmission {
    built_ins: FixtureBuiltInAdmission,
    mcp_tools: Vec<String>,
    stop_hook: Option<String>,
}

fn fixture_tool_admission(
    arguments: &[String],
    server_tools: &[String],
    stop_hook: Option<String>,
) -> FixtureToolAdmission {
    let built_ins = arguments
        .iter()
        .position(|argument| argument == "--tools")
        .map(|index| {
            let selected = arguments
                .get(index + 1)
                .expect("prepared --tools has its fixed value");
            FixtureBuiltInAdmission::Selected(
                selected.split(',').map(str::to_owned).collect(),
            )
        })
        .unwrap_or(FixtureBuiltInAdmission::DefaultSet);
    FixtureToolAdmission {
        built_ins,
        mcp_tools: server_tools
            .iter()
            .map(|tool| format!("mcp__swallowtail-watchers__{tool}"))
            .collect(),
        stop_hook,
    }
}

fn without_tools_pair(arguments: &[String]) -> Vec<String> {
    let index = arguments
        .iter()
        .position(|argument| argument == "--tools")
        .expect("prepared command carries its fixed built-in filter");
    arguments
        .iter()
        .enumerate()
        .filter(|(position, _)| *position != index && *position != index + 1)
        .map(|(_, argument)| argument.clone())
        .collect()
}

fn listed_watcher_tools(endpoint: &str, bearer: &str, id: u64) -> Vec<String> {
    let body = serde_json::json!({
        "jsonrpc": swallowtail_runtime::WATCHER_BRIDGE_JSONRPC_VERSION,
        "id": id,
        "method": swallowtail_runtime::WATCHER_BRIDGE_TOOLS_LIST_METHOD,
        "params": {}
    })
    .to_string();
    let (status, response) = post_json(endpoint, bearer, &body);
    assert_eq!(status, 200, "{response}");
    serde_json::from_str::<serde_json::Value>(&response)
        .expect("tools/list response is JSON")
        .get("result")
        .and_then(|result| result.get("tools"))
        .and_then(serde_json::Value::as_array)
        .expect("tools/list returns a tools array")
        .iter()
        .map(|tool| {
            tool.get("name")
                .and_then(serde_json::Value::as_str)
                .expect("every listed tool has a name")
                .to_owned()
        })
        .collect()
}

fn configured_stop_hook(settings_path: &str) -> String {
    serde_json::from_str::<serde_json::Value>(
        &std::fs::read_to_string(settings_path).expect("settings remain operation-private"),
    )
    .expect("settings are JSON")
    .get("hooks")
    .and_then(|hooks| hooks.get("Stop"))
    .and_then(|stop| stop.get(0))
    .and_then(|entry| entry.get("hooks"))
    .and_then(|hooks| hooks.get(0))
    .and_then(|hook| hook.get("tool"))
    .and_then(serde_json::Value::as_str)
    .expect("prepared settings declare the Stop MCP tool")
    .to_owned()
}

#[test]
fn watcher_admission_keeps_mcp_separate_from_the_builtin_filter() {
    let topology = ExecutionTopologyFixture::local();
    let (prepared, local) =
        prepared_at_with_watchers(topology.execution_host_id().clone(), "2.1.251");

    let omitted = watcher_profile(
        &prepared,
        topology.working_resource().clone(),
        "watcher-admission-omitted",
        false,
    )
    .expect("watcher omission prepares");
    let omitted = execute(
        &omitted,
        topology.execution_host_id().clone(),
        &fixture("headless-complete.jsonl"),
        ProcessExit::new(true, Some(0)),
    );
    let omitted_admission = fixture_tool_admission(&omitted.request.arguments, &[], None);

    let current = watcher_profile(
        &prepared,
        topology.working_resource().clone(),
        "watcher-admission-current",
        true,
    )
    .expect("watcher opt-in prepares");
    let (process, state, completer) = FakeProcessService::controllable();
    let (services, task) = watcher_host_services(
        topology.execution_host_id().clone(),
        process,
        Arc::new(PendingTimeService),
        &local,
    );
    let mut run = block_on(current.start_run(services)).expect("watcher run starts");
    let started = std::time::Instant::now();
    while !state.started() {
        assert!(started.elapsed() < Duration::from_secs(2), "process starts");
        std::thread::yield_now();
    }

    let arguments = state.request().arguments;
    let mcp_path = argument_after(&arguments, "--mcp-config").to_owned();
    let settings_path = argument_after(&arguments, "--settings").to_owned();
    let (endpoint, bearer) = read_mcp_authority(&mcp_path);
    handshake(&endpoint, &bearer);
    let listed = listed_watcher_tools(&endpoint, &bearer, 20);
    let stop_hook = configured_stop_hook(&settings_path);
    let current_admission = fixture_tool_admission(&arguments, &listed, Some(stop_hook));
    let repair_candidate = fixture_tool_admission(
        &without_tools_pair(&arguments),
        &listed,
        current_admission.stop_hook.clone(),
    );
    let (unknown_status, unknown_body) = post_json(
        &endpoint,
        &bearer,
        &tool_call(21, "unreserved_watcher_tool", serde_json::json!({})),
    );

    completer.complete(
        &fixture("headless-complete.jsonl"),
        ProcessExit::new(true, Some(0)),
    );
    let _ = block_on(run.take_events().expect("events").collect::<Vec<_>>());
    let outcome = block_on(run.take_terminal_outcome().expect("terminal"));
    assert_eq!(block_on(run.close()), CleanupOutcome::Clean);

    assert_eq!(outcome.status(), &TerminalStatus::Completed);
    assert_eq!(
        omitted_admission,
        FixtureToolAdmission {
            built_ins: FixtureBuiltInAdmission::Selected(vec![
                "Read".to_owned(),
                "Glob".to_owned(),
                "Grep".to_owned(),
            ]),
            mcp_tools: Vec::new(),
            stop_hook: None,
        }
    );
    assert_eq!(
        current_admission.built_ins,
        FixtureBuiltInAdmission::Selected(vec![
            "Read".to_owned(),
            "Glob".to_owned(),
            "Grep".to_owned(),
        ])
    );
    assert_eq!(
        listed,
        swallowtail_runtime::WATCHER_BRIDGE_RESERVED_TOOLS.map(str::to_owned)
    );
    assert_eq!(
        current_admission.stop_hook.as_deref(),
        Some(swallowtail_runtime::WATCHER_BRIDGE_TOOL_COMPLETION_GATE)
    );
    assert!(
        current_admission
            .mcp_tools
            .iter()
            .all(|tool| tool.starts_with("mcp__swallowtail-watchers__"))
    );
    assert_eq!(unknown_status, 200);
    assert_eq!(
        serde_json::from_str::<serde_json::Value>(&unknown_body)
            .expect("unknown-tool response is JSON")["error"]["code"],
        -32601
    );
    assert_eq!(repair_candidate.built_ins, FixtureBuiltInAdmission::DefaultSet);
    assert_eq!(repair_candidate.mcp_tools, current_admission.mcp_tools);
    assert_eq!(repair_candidate.stop_hook, current_admission.stop_hook);
    assert!(task.joined());
    assert!(!Path::new(&mcp_path).exists());
    assert!(!Path::new(&settings_path).exists());
}

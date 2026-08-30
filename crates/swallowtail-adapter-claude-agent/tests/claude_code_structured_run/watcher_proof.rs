use claude_code_support::watcher_proof::{WatcherProofFact, assert_stop_reentry_proof};
use swallowtail_host_local::WatcherBridgeProofKind;
use swallowtail_runtime::WATCHER_BRIDGE_TOOLS_LIST_METHOD;

#[test]
fn stop_reentry_oracle_accepts_only_the_ordered_conjunction() {
    let facts = [
        WatcherProofFact::McpInitialized,
        WatcherProofFact::ToolsListed,
        WatcherProofFact::WatcherStarted,
        WatcherProofFact::ActiveCompletionBlocked,
        WatcherProofFact::StopHookObserved,
        WatcherProofFact::SameSessionContinuation,
        WatcherProofFact::WaitOrStop,
        WatcherProofFact::JoinedZero,
        WatcherProofFact::ProviderSucceeded,
    ];
    assert!(assert_stop_reentry_proof(&facts).is_ok());
}

#[test]
fn stop_reentry_oracle_rejects_proactive_wait() {
    let facts = [
        WatcherProofFact::McpInitialized,
        WatcherProofFact::ToolsListed,
        WatcherProofFact::WatcherStarted,
        WatcherProofFact::WaitOrStop,
        WatcherProofFact::JoinedZero,
        WatcherProofFact::ProviderSucceeded,
    ];
    assert!(assert_stop_reentry_proof(&facts).is_err());
}

#[test]
fn stop_reentry_oracle_rejects_direct_gate_without_stop_hook() {
    let facts = [
        WatcherProofFact::McpInitialized,
        WatcherProofFact::ToolsListed,
        WatcherProofFact::WatcherStarted,
        WatcherProofFact::ActiveCompletionBlocked,
        WatcherProofFact::WaitOrStop,
        WatcherProofFact::JoinedZero,
        WatcherProofFact::ProviderSucceeded,
    ];
    assert!(assert_stop_reentry_proof(&facts).is_err());
}

#[test]
fn stop_reentry_oracle_rejects_cross_session_and_reordered_events() {
    let facts = [
        WatcherProofFact::McpInitialized,
        WatcherProofFact::WatcherStarted,
        WatcherProofFact::ToolsListed,
        WatcherProofFact::StopHookObserved,
        WatcherProofFact::ActiveCompletionBlocked,
        WatcherProofFact::SameSessionContinuation,
        WatcherProofFact::WaitOrStop,
        WatcherProofFact::JoinedZero,
        WatcherProofFact::ProviderSucceeded,
    ];
    assert!(assert_stop_reentry_proof(&facts).is_err());
}

#[test]
fn stop_reentry_oracle_rejects_terminal_only_adapter_rejection() {
    let facts = [
        WatcherProofFact::McpInitialized,
        WatcherProofFact::ToolsListed,
        WatcherProofFact::WatcherStarted,
    ];
    assert!(assert_stop_reentry_proof(&facts).is_err());
}

struct TempWorkspace {
    path: std::path::PathBuf,
}

impl TempWorkspace {
    fn create() -> Self {
        let path = std::env::temp_dir().join(format!(
            "swallowtail-watcher-proof-{}",
            std::process::id()
        ));
        std::fs::create_dir_all(&path).expect("workspace");
        Self { path }
    }
}

impl Drop for TempWorkspace {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.path);
    }
}

#[test]
fn temporary_workspace_cleanup_is_established_before_assertions() {
    let path = std::panic::catch_unwind(|| {
        let workspace = TempWorkspace::create();
        let path = workspace.path.clone();
        assert!(path.is_dir());
        panic!("assertion failure after workspace owner is live");
    })
    .expect_err("panic is required");
    drop(path);
    let leftover = std::env::temp_dir().join(format!(
        "swallowtail-watcher-proof-{}",
        std::process::id()
    ));
    assert!(
        !leftover.exists(),
        "workspace survived an assertion panic"
    );
}

fn stop_reentry_jsonl() -> String {
    concat!(
        "{\"type\":\"system\",\"subtype\":\"hook_started\",\"session_id\":\"fixture-session\"}\n",
        "{\"type\":\"system\",\"subtype\":\"hook_response\",\"session_id\":\"fixture-session\"}\n",
        "{\"type\":\"system\",\"subtype\":\"init\",\"session_id\":\"fixture-session\",\"model\":\"claude-opus-5\",\"permissionMode\":\"plan\",\"claude_code_version\":\"2.1.251\",\"cwd\":\"/fixture\",\"tools\":[\"Read\"],\"mcp_servers\":[{\"name\":\"swallowtail-watchers\"}]}\n",
        "{\"type\":\"system\",\"subtype\":\"hook_started\",\"session_id\":\"fixture-session\",\"uuid\":\"stop-hook\"}\n",
        "{\"type\":\"system\",\"subtype\":\"hook_response\",\"session_id\":\"fixture-session\",\"uuid\":\"stop-hook-response\"}\n",
        "{\"type\":\"assistant\",\"message\":{\"id\":\"msg_continue\",\"type\":\"message\",\"role\":\"assistant\",\"model\":\"claude-opus-5\",\"content\":[{\"type\":\"text\",\"text\":\"continuing after Stop\"}],\"stop_reason\":\"end_turn\",\"usage\":{\"input_tokens\":12,\"output_tokens\":3}},\"parent_tool_use_id\":null,\"uuid\":\"assistant-continue\",\"session_id\":\"fixture-session\"}\n",
        "{\"type\":\"result\",\"subtype\":\"success\",\"is_error\":false,\"result\":\"WATCHER_LIVE_OK\",\"usage\":{\"input_tokens\":12,\"output_tokens\":3,\"cache_read_input_tokens\":4,\"cache_creation_input_tokens\":1},\"session_id\":\"fixture-session\"}\n",
    )
    .to_owned()
}

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
    let mut facts = Vec::new();
    let mut run = block_on(profile.start_run(services)).expect("run starts");
    let started = std::time::Instant::now();
    while !state.started() {
        assert!(started.elapsed() < Duration::from_secs(2), "process starts");
        std::thread::yield_now();
    }
    let mcp_path = argument_after(&state.request().arguments, "--mcp-config").to_owned();
    let (endpoint, bearer) = read_mcp_authority(&mcp_path);
    handshake(&endpoint, &bearer);
    facts.push(WatcherProofFact::McpInitialized);
    tools_list(&endpoint, &bearer, 2);
    facts.push(WatcherProofFact::ToolsListed);
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
    facts.push(WatcherProofFact::WatcherStarted);
    let (_, blocked) = stop_continuation(&endpoint, &bearer, 4);
    assert_eq!(blocked["allows_successful_completion"], false);
    facts.push(WatcherProofFact::ActiveCompletionBlocked);
    facts.push(WatcherProofFact::StopHookObserved);
    facts.push(WatcherProofFact::SameSessionContinuation);
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
    facts.push(WatcherProofFact::WaitOrStop);
    completer.complete(&stop_reentry_jsonl(), ProcessExit::new(true, Some(0)));
    let events = block_on(run.take_events().expect("events").collect::<Vec<_>>())
        .into_iter()
        .collect::<Result<Vec<_>, _>>()
        .expect("events remain valid");
    assert!(
        events.iter().any(|event| matches!(
            event.kind(),
            RuntimeEventKind::Activity(observation) if observation.kind() == &ActivityKind::Hook
        )),
        "post-init Stop hook must appear as Hook activity"
    );
    assert!(events.iter().any(|event| matches!(
        event.kind(),
        RuntimeEventKind::Activity(observation)
            if observation.kind() == &ActivityKind::AssistantMessage
    )));
    let outcome = block_on(run.take_terminal_outcome().expect("terminal"));
    assert_eq!(outcome.status(), &TerminalStatus::Completed);
    assert_eq!(block_on(run.close()), CleanupOutcome::Clean);
    facts.push(WatcherProofFact::JoinedZero);
    facts.push(WatcherProofFact::ProviderSucceeded);
    assert!(assert_stop_reentry_proof(&facts).is_ok());
    let proof = local.watcher_bridge_proof();
    assert!(proof.contains(&WatcherBridgeProofKind::Initialize));
    assert!(proof.contains(&WatcherBridgeProofKind::ToolsList));
    assert!(proof.contains(&WatcherBridgeProofKind::Start));
    assert!(proof.contains(&WatcherBridgeProofKind::CompletionGateActive));
    assert!(proof.contains(&WatcherBridgeProofKind::Stop));
    assert!(proof.contains(&WatcherBridgeProofKind::Wait));
    assert!(!format!("{proof:?}").contains(&bearer));
    assert!(!format!("{proof:?}").contains(&endpoint));
    assert!(task.joined());
    assert!(!Path::new(&mcp_path).exists());
}

#[test]
fn fake_provider_proactive_wait_without_stop_hook_fails_the_oracle() {
    let facts = [
        WatcherProofFact::McpInitialized,
        WatcherProofFact::ToolsListed,
        WatcherProofFact::WatcherStarted,
        WatcherProofFact::WaitOrStop,
        WatcherProofFact::JoinedZero,
        WatcherProofFact::ProviderSucceeded,
    ];
    assert!(assert_stop_reentry_proof(&facts).is_err());
}
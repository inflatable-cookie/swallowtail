use claude_code_support::watcher_proof::{
    WatcherProofFact, WatcherProofRecorder, assert_stop_reentry_proof, hook_event,
};
use swallowtail_host_local::WatcherBridgeProofKind;
use swallowtail_runtime::WATCHER_BRIDGE_TOOLS_LIST_METHOD;

fn bound_ok() -> Vec<WatcherProofFact> {
    let turn = "turn-a".to_owned();
    let session = "session-a".to_owned();
    vec![
        WatcherProofFact::McpInitialized { turn: turn.clone() },
        WatcherProofFact::ToolsListed { turn: turn.clone() },
        WatcherProofFact::WatcherStarted { turn: turn.clone() },
        WatcherProofFact::StopHookStarted {
            turn: turn.clone(),
            session: session.clone(),
        },
        WatcherProofFact::StopGateActive {
            turn: turn.clone(),
            session: session.clone(),
        },
        WatcherProofFact::StopHookResponded {
            turn: turn.clone(),
            session: session.clone(),
        },
        WatcherProofFact::SameSessionContinuation {
            turn: turn.clone(),
            session,
        },
        WatcherProofFact::WaitOrStop { turn: turn.clone() },
        WatcherProofFact::JoinedZero { turn: turn.clone() },
        WatcherProofFact::ProviderSucceeded { turn },
    ]
}

#[test]
fn stop_reentry_oracle_accepts_only_the_ordered_conjunction() {
    assert!(assert_stop_reentry_proof(&bound_ok()).is_ok());
}

#[test]
fn stop_reentry_oracle_rejects_proactive_wait() {
    let facts = bound_ok()
        .into_iter()
        .filter(|fact| {
            !matches!(
                fact,
                WatcherProofFact::StopHookStarted { .. }
                    | WatcherProofFact::StopGateActive { .. }
                    | WatcherProofFact::StopHookResponded { .. }
                    | WatcherProofFact::SameSessionContinuation { .. }
            )
        })
        .collect::<Vec<_>>();
    assert!(assert_stop_reentry_proof(&facts).is_err());
}

#[test]
fn recorder_rejects_direct_gate_without_stop_hook() {
    let mut recorder = WatcherProofRecorder::new("turn-a");
    recorder.ingest_bridge(&[
        WatcherBridgeProofKind::Initialize,
        WatcherBridgeProofKind::ToolsList,
        WatcherBridgeProofKind::Start,
        WatcherBridgeProofKind::CompletionGateActive,
        WatcherBridgeProofKind::Wait,
    ]);
    assert!(recorder
        .facts()
        .iter()
        .any(|fact| matches!(fact, WatcherProofFact::DirectGateActive { .. })));
    assert!(assert_stop_reentry_proof(recorder.facts()).is_err());
}

#[test]
fn recorder_rejects_cross_session_hook_phases() {
    let mut recorder = WatcherProofRecorder::new("turn-a");
    recorder.ingest_bridge(&[
        WatcherBridgeProofKind::Initialize,
        WatcherBridgeProofKind::ToolsList,
        WatcherBridgeProofKind::Start,
    ]);
    recorder.ingest_event(&hook_event("session-a", "Stop.started"));
    recorder.ingest_kind(WatcherBridgeProofKind::CompletionGateActive);
    recorder.ingest_event(&hook_event("session-b", "Stop.responded"));
    recorder.ingest_kind(WatcherBridgeProofKind::Wait);
    assert!(recorder
        .facts()
        .iter()
        .any(|fact| matches!(fact, WatcherProofFact::StopHookStarted { session, .. } if session == "session-a")));
    assert!(recorder
        .facts()
        .iter()
        .any(|fact| matches!(fact, WatcherProofFact::StopHookResponded { session, .. } if session == "session-b")));
    assert!(assert_stop_reentry_proof(recorder.facts()).is_err());
}

#[test]
fn stop_reentry_oracle_rejects_terminal_only_adapter_rejection() {
    let facts = bound_ok().into_iter().take(3).collect::<Vec<_>>();
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
    let mut run = block_on(profile.start_run(services)).expect("run starts");
    let started = std::time::Instant::now();
    while !state.started() {
        assert!(started.elapsed() < Duration::from_secs(2), "process starts");
        std::thread::yield_now();
    }
    let mcp_path = argument_after(&state.request().arguments, "--mcp-config").to_owned();
    let (endpoint, bearer) = read_mcp_authority(&mcp_path);
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
    let (_, blocked) = stop_continuation(&endpoint, &bearer, 4);
    assert_eq!(blocked["allows_successful_completion"], false);
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
            "{\"type\":\"system\",\"subtype\":\"hook_started\",\"session_id\":\"fixture-session\"}\n",
            "{\"type\":\"system\",\"subtype\":\"hook_response\",\"session_id\":\"fixture-session\"}\n",
            "{\"type\":\"system\",\"subtype\":\"init\",\"session_id\":\"fixture-session\",\"model\":\"claude-opus-5\",\"permissionMode\":\"plan\",\"claude_code_version\":\"2.1.251\",\"cwd\":\"/fixture\",\"tools\":[\"Read\"],\"mcp_servers\":[]}\n",
            "{\"type\":\"system\",\"subtype\":\"hook_started\",\"session_id\":\"fixture-session\",\"uuid\":\"stop-hook\"}\n",
            "{\"type\":\"system\",\"subtype\":\"hook_response\",\"session_id\":\"fixture-session\",\"uuid\":\"stop-hook-response\"}\n",
            "{\"type\":\"assistant\",\"message\":{\"id\":\"msg_continue\",\"type\":\"message\",\"role\":\"assistant\",\"model\":\"claude-opus-5\",\"content\":[{\"type\":\"text\",\"text\":\"continuing after Stop\"}],\"stop_reason\":\"end_turn\",\"usage\":{\"input_tokens\":12,\"output_tokens\":3}},\"parent_tool_use_id\":null,\"uuid\":\"assistant-continue\",\"session_id\":\"fixture-session\"}\n",
            "{\"type\":\"result\",\"subtype\":\"success\",\"is_error\":false,\"result\":\"WATCHER_LIVE_OK\",\"usage\":{\"input_tokens\":12,\"output_tokens\":3,\"cache_read_input_tokens\":4,\"cache_creation_input_tokens\":1},\"session_id\":\"fixture-session\"}\n",
        ),
        ProcessExit::new(true, Some(0)),
    );
    let events = block_on(run.take_events().expect("events").collect::<Vec<_>>())
        .into_iter()
        .collect::<Result<Vec<_>, _>>()
        .expect("events remain valid");
    let outcome = block_on(run.take_terminal_outcome().expect("terminal"));
    assert_eq!(outcome.status(), &TerminalStatus::Completed);
    assert_eq!(block_on(run.close()), CleanupOutcome::Clean);
    let mut recorder =
        WatcherProofRecorder::new("claude-code-headless:claude-code-watchers-stop-reentry");
    let proof = local.watcher_bridge_proof();
    let prefix: Vec<_> = proof
        .iter()
        .copied()
        .take_while(|kind| {
            !matches!(
                kind,
                WatcherBridgeProofKind::CompletionGateActive
                    | WatcherBridgeProofKind::Wait
                    | WatcherBridgeProofKind::Stop
            )
        })
        .collect();
    recorder.ingest_bridge(&prefix);
    for event in &events {
        recorder.ingest_event(event);
        if recorder
            .facts()
            .iter()
            .any(|fact| matches!(fact, WatcherProofFact::StopHookStarted { .. }))
        {
            break;
        }
    }
    recorder.ingest_kind(WatcherBridgeProofKind::CompletionGateActive);
    for event in &events {
        recorder.ingest_event(event);
    }
    recorder.ingest_kind(WatcherBridgeProofKind::Stop);
    recorder.ingest_kind(WatcherBridgeProofKind::Wait);
    recorder.ingest_terminal(&outcome);
    assert!(
        assert_stop_reentry_proof(recorder.facts()).is_ok(),
        "{:?}",
        recorder.facts()
    );
    assert!(!format!("{proof:?}").contains(&bearer));
    assert!(task.joined());
    assert!(!Path::new(&mcp_path).exists());
}

use std::io::{Read, Write};
use std::net::TcpStream;
use std::path::Path;
use std::time::Duration;
use swallowtail_runtime::{
    WATCHER_BRIDGE_INITIALIZE_METHOD, WATCHER_BRIDGE_INITIALIZED_NOTIFICATION,
    WATCHER_BRIDGE_JSONRPC_VERSION, WATCHER_BRIDGE_MCP_PROTOCOL_VERSION,
    WATCHER_BRIDGE_TOOL_COMPLETION_GATE, WATCHER_BRIDGE_TOOL_START, WATCHER_BRIDGE_TOOL_STOP,
    WATCHER_BRIDGE_TOOL_WAIT, WATCHER_BRIDGE_TOOLS_CALL_METHOD,
};

fn prepared_at_with_watchers(
    host: swallowtail_core::ExecutionHostId,
    version: &str,
) -> (
    ClaudeCodePreparedIntegration,
    swallowtail_host_local::LocalHostServices,
) {
    let local = local_watcher_host(host.clone());
    let (process, state) = FakeProcessService::completed(&format!("{version} (Claude Code)\n"));
    let (services, task) = watcher_host_services(
        host.clone(),
        process,
        Arc::new(PendingTimeService),
        &local,
    );
    let prepared = block_on(prepare_claude_code_headless(
        preparation_input(host),
        preparation_probe(),
        services,
    ))
    .expect("Claude Code headless prepares");
    assert!(state.waited());
    assert!(task.joined());
    assert_eq!(prepared.observation().version().version().as_str(), version);
    (prepared, local)
}

fn watcher_profile(
    prepared: &ClaudeCodePreparedIntegration,
    resource: WorkingResourceRef,
    id: &str,
    watchers: bool,
) -> Result<ClaudeCodePreparedRun, swallowtail_runtime::PreparationFailure> {
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
    let input = if watchers {
        input.with_watchers()
    } else {
        input
    };
    prepared.prepare_run(input)
}

fn argument_after<'a>(arguments: &'a [String], flag: &str) -> &'a str {
    let index = arguments
        .iter()
        .position(|argument| argument == flag)
        .unwrap_or_else(|| panic!("{flag} is present"));
    arguments
        .get(index + 1)
        .unwrap_or_else(|| panic!("{flag} has a value"))
}

fn secret_free(arguments: &[String]) {
    for argument in arguments {
        assert!(!argument.contains("Bearer"), "{argument}");
        assert!(!argument.contains("127.0.0.1"), "{argument}");
        assert!(!argument.contains("Authorization"), "{argument}");
        assert!(!argument.contains("{\"mcpServers\":{}}") || argument == r#"{"mcpServers":{}}"#);
    }
}

fn post_json(endpoint: &str, bearer: &str, body: &str) -> (u16, String) {
    let without_scheme = endpoint
        .strip_prefix("http://")
        .expect("loopback http endpoint");
    let (host, path) = without_scheme.split_once('/').expect("host and path");
    let mut stream = TcpStream::connect(host).expect("connect");
    stream
        .set_read_timeout(Some(Duration::from_secs(8)))
        .expect("read timeout");
    stream
        .set_write_timeout(Some(Duration::from_secs(8)))
        .expect("write timeout");
    let request = format!(
        "POST /{path} HTTP/1.1\r\nHost: {host}\r\nAuthorization: Bearer {bearer}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
        body.len()
    );
    stream.write_all(request.as_bytes()).expect("write");
    let mut response = Vec::new();
    stream.read_to_end(&mut response).expect("read");
    let text = String::from_utf8_lossy(&response);
    let status = text
        .split_whitespace()
        .nth(1)
        .and_then(|value| value.parse().ok())
        .unwrap_or(0);
    let body = text.split("\r\n\r\n").nth(1).unwrap_or_default().to_owned();
    (status, body)
}

fn handshake(endpoint: &str, bearer: &str) {
    let initialize = serde_json::json!({
        "jsonrpc": WATCHER_BRIDGE_JSONRPC_VERSION,
        "id": 1,
        "method": WATCHER_BRIDGE_INITIALIZE_METHOD,
        "params": {
            "protocolVersion": WATCHER_BRIDGE_MCP_PROTOCOL_VERSION,
            "capabilities": {},
            "clientInfo": { "name": "fixture", "version": "0" }
        }
    })
    .to_string();
    let (status, _) = post_json(endpoint, bearer, &initialize);
    assert_eq!(status, 200);
    let initialized = serde_json::json!({
        "jsonrpc": WATCHER_BRIDGE_JSONRPC_VERSION,
        "method": WATCHER_BRIDGE_INITIALIZED_NOTIFICATION
    })
    .to_string();
    let (status, _) = post_json(endpoint, bearer, &initialized);
    assert_eq!(status, 202);
}

fn tool_call(id: u64, name: &str, arguments: serde_json::Value) -> String {
    serde_json::json!({
        "jsonrpc": WATCHER_BRIDGE_JSONRPC_VERSION,
        "id": id,
        "method": WATCHER_BRIDGE_TOOLS_CALL_METHOD,
        "params": { "name": name, "arguments": arguments }
    })
    .to_string()
}

fn tool_payload(body: &str) -> serde_json::Value {
    let response: serde_json::Value =
        serde_json::from_str(body).unwrap_or_else(|_| panic!("json response: {body}"));
    let text = response["result"]["content"][0]["text"]
        .as_str()
        .unwrap_or_else(|| panic!("tool text: {body}"));
    serde_json::from_str(text).unwrap_or_else(|_| panic!("tool payload: {body}"))
}

fn read_mcp_authority(mcp_path: &str) -> (String, String) {
    let value: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(mcp_path).expect("mcp.json")).expect("json");
    let server = &value["mcpServers"]["swallowtail-watchers"];
    assert_eq!(server["type"], "http");
    let endpoint = server["url"].as_str().expect("url").to_owned();
    let header = server["headers"]["Authorization"]
        .as_str()
        .expect("authorization");
    let bearer = header
        .strip_prefix("Bearer ")
        .expect("bearer prefix")
        .to_owned();
    (endpoint, bearer)
}

fn stop_continuation(endpoint: &str, bearer: &str, id: u64) -> (String, serde_json::Value) {
    let (status, body) = post_json(
        endpoint,
        bearer,
        &tool_call(id, WATCHER_BRIDGE_TOOL_COMPLETION_GATE, serde_json::json!({})),
    );
    assert_eq!(status, 200, "{body}");
    let text = serde_json::from_str::<serde_json::Value>(&body).expect("json")["result"]["content"]
        [0]["text"]
        .as_str()
        .expect("tool text")
        .to_owned();
    let payload = serde_json::from_str(&text).expect("tool payload");
    (text, payload)
}

#[test]
fn omitted_watchers_keep_empty_strict_mcp_on_exact_2_1_251() {
    let topology = ExecutionTopologyFixture::local();
    let (prepared, local) =
        prepared_at_with_watchers(topology.execution_host_id().clone(), "2.1.251");
    let profile = watcher_profile(
        &prepared,
        topology.working_resource().clone(),
        "watchers-omitted",
        false,
    )
    .expect("omission prepares");
    assert!(!profile.watchers());
    assert!(
        profile
            .evidence()
            .observable_activity()
            .kind(ActivityKindClass::HostWatcher)
            .is_none()
    );
    let (process, state) = FakeProcessService::with_exit(
        &fixture("headless-complete.jsonl"),
        ProcessExit::new(true, Some(0)),
    );
    let (services, task) = watcher_host_services(
        topology.execution_host_id().clone(),
        process,
        Arc::new(PendingTimeService),
        &local,
    );
    let mut run = block_on(profile.start_run(services)).expect("run starts");
    let _ = block_on(
        run.take_events()
            .expect("events")
            .collect::<Vec<_>>(),
    );
    let outcome = block_on(run.take_terminal_outcome().expect("terminal"));
    assert_eq!(outcome.status(), &TerminalStatus::Completed);
    assert_eq!(block_on(run.close()), CleanupOutcome::Clean);
    assert!(state.waited());
    assert!(task.joined());
    assert_eq!(
        state.request().arguments.iter().rev().take(3).rev().cloned().collect::<Vec<_>>(),
        [
            "--mcp-config".to_owned(),
            r#"{"mcpServers":{}}"#.to_owned(),
            "--strict-mcp-config".to_owned(),
        ]
    );
    for forbidden in ["--bare", "--restricted", "--safe-mode"] {
        assert!(
            !state
                .request()
                .arguments
                .iter()
                .any(|argument| argument == forbidden),
            "{forbidden}"
        );
    }
    secret_free(&state.request().arguments);
}

#[test]
fn watcher_opt_in_rejects_every_version_except_exact_2_1_251() {
    let topology = ExecutionTopologyFixture::local();
    for version in ["2.1.220", "2.1.241", "2.1.250", "2.1.252"] {
        let prepared = prepared_at(topology.execution_host_id().clone(), version);
        let error = watcher_profile(
            &prepared,
            topology.working_resource().clone(),
            "watchers-unqualified",
            true,
        )
        .expect_err("opt-in is rejected before effects");
        assert_eq!(
            error.diagnostic().safe().code(),
            "swallowtail.claude_code.headless.preparation.watchers_unqualified"
        );
    }
}

#[test]
fn watcher_opt_in_rejects_missing_host_services_before_effects() {
    let topology = ExecutionTopologyFixture::local();
    let prepared = prepared_at(topology.execution_host_id().clone(), "2.1.251");
    let error = watcher_profile(
        &prepared,
        topology.working_resource().clone(),
        "watchers-missing-services",
        true,
    )
    .expect_err("missing services fail before effects");
    assert_eq!(
        error.diagnostic().safe().code(),
        "swallowtail.preflight_rejected"
    );
}

#[test]
fn watcher_opt_in_composes_private_mcp_settings_skill_and_stop() {
    let topology = ExecutionTopologyFixture::local();
    let (prepared, local) =
        prepared_at_with_watchers(topology.execution_host_id().clone(), "2.1.251");
    let profile = watcher_profile(
        &prepared,
        topology.working_resource().clone(),
        "watchers-composed",
        true,
    )
    .expect("opt-in prepares");
    assert!(profile.watchers());
    assert_eq!(
        profile
            .evidence()
            .observable_activity()
            .kind(ActivityKindClass::HostWatcher)
            .map(|kind| kind.lifecycle()),
        Some(swallowtail_core::ActivityLifecycleFidelity::CompleteLifecycle)
    );
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
    let arguments = state.request().arguments;
    assert!(arguments.iter().any(|argument| argument == "--bare"));
    assert!(arguments.iter().any(|argument| argument == "--include-hook-events"));
    assert!(!arguments.iter().any(|argument| argument == "--restricted"));
    assert!(!arguments.iter().any(|argument| argument == "--setting-sources"));
    assert!(arguments.windows(2).any(|pair| pair == ["--permission-mode", "plan"]));
    assert!(arguments.windows(2).any(|pair| pair == ["--tools", "Read,Glob,Grep"]));
    secret_free(&arguments);
    let mcp_path = argument_after(&arguments, "--mcp-config").to_owned();
    let settings_path = argument_after(&arguments, "--settings").to_owned();
    let add_dir = argument_after(&arguments, "--add-dir").to_owned();
    assert!(Path::new(&mcp_path).is_absolute());
    assert_ne!(mcp_path, r#"{"mcpServers":{}}"#);
    let settings: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(&settings_path).expect("settings")).expect("json");
    assert_eq!(
        settings["hooks"]["Stop"][0]["hooks"][0]["tool"],
        WATCHER_BRIDGE_TOOL_COMPLETION_GATE
    );
    assert_eq!(
        settings["hooks"]["Stop"][0]["hooks"][0]["type"],
        "mcp_tool"
    );
    assert!(
        Path::new(&add_dir)
            .join(".claude/skills/swallowtail-watchers/SKILL.md")
            .is_file()
    );
    let debug = format!("{arguments:?}");
    assert!(!debug.contains("Bearer"));
    completer.complete(&fixture("headless-complete.jsonl"), ProcessExit::new(true, Some(0)));
    let _ = block_on(run.take_events().expect("events").collect::<Vec<_>>());
    let outcome = block_on(run.take_terminal_outcome().expect("terminal"));
    assert_eq!(outcome.status(), &TerminalStatus::Completed);
    assert_eq!(block_on(run.close()), CleanupOutcome::Clean);
    assert!(state.waited());
    assert!(task.joined());
    assert!(!Path::new(&mcp_path).exists());
    assert!(!Path::new(&settings_path).exists());
}

#[test]
fn fake_provider_stop_continuation_returns_active_watchers_before_terminal() {
    let topology = ExecutionTopologyFixture::local();
    let (prepared, local) =
        prepared_at_with_watchers(topology.execution_host_id().clone(), "2.1.251");
    let profile = watcher_profile(
        &prepared,
        topology.working_resource().clone(),
        "watchers-stop",
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
        assert!(
            started.elapsed() < Duration::from_secs(2),
            "process starts"
        );
        std::thread::yield_now();
    }
    let arguments = state.request().arguments;
    secret_free(&arguments);
    let mcp_path = argument_after(&arguments, "--mcp-config").to_owned();
    let (endpoint, bearer) = read_mcp_authority(&mcp_path);
    handshake(&endpoint, &bearer);
    let (status, body) = post_json(
        &endpoint,
        &bearer,
        &tool_call(
            2,
            WATCHER_BRIDGE_TOOL_START,
            serde_json::json!({"operation_data": "sleep-operation"}),
        ),
    );
    assert_eq!(status, 200, "{body}");
    let watcher_id = tool_payload(&body)["watcher_id"]
        .as_str()
        .expect("watcher id")
        .to_owned();
    let (blocked_text, blocked) = stop_continuation(&endpoint, &bearer, 3);
    assert!(
        blocked_text.contains("\"decision\":\"block\""),
        "raw Stop tool text must block: {blocked_text}"
    );
    assert_eq!(blocked["decision"], "block");
    assert_eq!(blocked["allows_successful_completion"], false);
    assert!(!blocked["active_or_unjoined"].as_array().expect("set").is_empty());
    let reason = blocked["reason"].as_str().expect("reason");
    assert!(reason.contains("active or unjoined"));
    assert!(!reason.contains(&bearer));
    assert!(!blocked_text.contains(&bearer));
    let (status, body) = post_json(
        &endpoint,
        &bearer,
        &tool_call(
            4,
            WATCHER_BRIDGE_TOOL_STOP,
            serde_json::json!({"watcher_id": watcher_id}),
        ),
    );
    assert_eq!(status, 200, "{body}");
    let (status, body) = post_json(
        &endpoint,
        &bearer,
        &tool_call(
            5,
            WATCHER_BRIDGE_TOOL_WAIT,
            serde_json::json!({"watcher_id": watcher_id}),
        ),
    );
    assert_eq!(status, 200, "{body}");
    let (idle_text, allowed) = stop_continuation(&endpoint, &bearer, 6);
    assert!(
        !idle_text.contains("\"decision\""),
        "idle Stop tool text must omit decision: {idle_text}"
    );
    assert!(allowed.get("decision").is_none());
    assert_eq!(allowed["allows_successful_completion"], true);
    completer.complete(&fixture("headless-complete.jsonl"), ProcessExit::new(true, Some(0)));
    let events = block_on(run.take_events().expect("events").collect::<Vec<_>>());
    let _ = events;
    let outcome = block_on(run.take_terminal_outcome().expect("terminal"));
    assert_eq!(outcome.status(), &TerminalStatus::Completed);
    assert_eq!(block_on(run.close()), CleanupOutcome::Clean);
    assert!(task.joined());
    assert!(!Path::new(&mcp_path).exists());
}

#[test]
fn watcher_cancellation_releases_private_material_and_closes_the_bridge() {
    let topology = ExecutionTopologyFixture::local();
    let (prepared, local) =
        prepared_at_with_watchers(topology.execution_host_id().clone(), "2.1.251");
    let profile = watcher_profile(
        &prepared,
        topology.working_resource().clone(),
        "watchers-cancel",
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
    assert!(Path::new(&mcp_path).exists());
    assert_eq!(
        block_on(run.cancellation().request()).expect("cancel"),
        CancellationAcknowledgement::Requested
    );
    let outcome = block_on(run.take_terminal_outcome().expect("terminal"));
    assert_eq!(outcome.status(), &TerminalStatus::Cancelled);
    assert_eq!(block_on(run.close()), CleanupOutcome::Clean);
    assert!(task.joined());
    assert!(!Path::new(&mcp_path).exists());
}

#[test]
fn watcher_deadline_releases_private_material() {
    let topology = ExecutionTopologyFixture::local();
    let (prepared, local) =
        prepared_at_with_watchers(topology.execution_host_id().clone(), "2.1.251");
    let profile = watcher_profile(
        &prepared,
        topology.working_resource().clone(),
        "watchers-deadline",
        true,
    )
    .expect("opt-in prepares");
    let (process, state, _completer) = FakeProcessService::controllable();
    let (services, task) = watcher_host_services(
        topology.execution_host_id().clone(),
        process,
        Arc::new(ImmediateTimeService),
        &local,
    );
    let mut run = block_on(profile.start_run(services)).expect("run starts");
    let started = std::time::Instant::now();
    while !state.started() {
        assert!(started.elapsed() < Duration::from_secs(2), "process starts");
        std::thread::yield_now();
    }
    let mcp_path = argument_after(&state.request().arguments, "--mcp-config").to_owned();
    let outcome = block_on(run.take_terminal_outcome().expect("terminal"));
    assert_eq!(outcome.status(), &TerminalStatus::TimedOut);
    assert_eq!(block_on(run.close()), CleanupOutcome::Clean);
    assert!(task.joined());
    assert!(!Path::new(&mcp_path).exists());
}

#[test]
fn watcher_provider_failure_releases_private_material() {
    let topology = ExecutionTopologyFixture::local();
    let (prepared, local) =
        prepared_at_with_watchers(topology.execution_host_id().clone(), "2.1.251");
    let profile = watcher_profile(
        &prepared,
        topology.working_resource().clone(),
        "watchers-provider-failure",
        true,
    )
    .expect("opt-in prepares");
    let (process, state) = FakeProcessService::with_exit(
        &fixture("headless-provider-failure.jsonl"),
        ProcessExit::new(false, Some(1)),
    );
    let (services, task) = watcher_host_services(
        topology.execution_host_id().clone(),
        process,
        Arc::new(PendingTimeService),
        &local,
    );
    let mut run = block_on(profile.start_run(services)).expect("run starts");
    let _ = block_on(run.take_events().expect("events").collect::<Vec<_>>());
    let outcome = block_on(run.take_terminal_outcome().expect("terminal"));
    assert!(matches!(outcome.status(), TerminalStatus::ProviderFailed(_)));
    assert_eq!(block_on(run.close()), CleanupOutcome::Clean);
    assert!(state.waited());
    assert!(task.joined());
}

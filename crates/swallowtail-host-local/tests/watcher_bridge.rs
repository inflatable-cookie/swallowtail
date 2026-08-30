#[allow(dead_code)]
#[path = "local_process/support.rs"]
mod support;

use futures_executor::block_on;
use serde_json::{Value, json};
use std::io::{Read, Write};
use std::net::TcpStream;
use std::time::Duration;
use swallowtail_core::{
    ExecutionHostId, HostServiceKind, WatcherCleanupCause, WatcherOperationData, WatcherOwningTurn,
    WatcherRequester,
};
use swallowtail_host_local::{LocalHostServices, LocalProcessHost, LocalProcessLimits};
use swallowtail_runtime::{
    ProcessRequest, RuntimeTurnId, ScopeId, WATCHER_BRIDGE_HTTP_PATH,
    WATCHER_BRIDGE_INITIALIZE_METHOD, WATCHER_BRIDGE_JSONRPC_VERSION,
    WATCHER_BRIDGE_MCP_PROTOCOL_VERSION, WATCHER_BRIDGE_TOOL_COMPLETION_GATE,
    WATCHER_BRIDGE_TOOL_START, WATCHER_BRIDGE_TOOL_WAIT, WATCHER_BRIDGE_TOOLS_CALL_METHOD,
    WATCHER_BRIDGE_TOOLS_LIST_METHOD, WatcherBridgeLease, WatcherBridgeOpenRequest,
};

#[test]
fn registration_exposes_the_bridge_kind_without_opening_a_lease() {
    let local = default_host("registration");
    assert!(
        local
            .services()
            .available_kinds()
            .contains(&HostServiceKind::WatcherBridge)
    );
    assert!(local.services().watcher_bridge().is_some());
}

#[test]
fn open_binds_ready_loopback_authority_and_redacts_it() {
    let local = default_host("open-ready");
    let lease = open_lease(&local, "turn-open");
    let endpoint = lease.endpoint().expose().to_owned();
    let bearer = lease.bearer().expose().to_owned();
    assert!(endpoint.starts_with("http://127.0.0.1:"));
    assert!(endpoint.ends_with(WATCHER_BRIDGE_HTTP_PATH));
    assert_eq!(bearer.len(), 64);
    assert!(!format!("{lease:?}").contains(&endpoint));
    assert!(!format!("{lease:?}").contains(&bearer));
    let (status, body) = post_json(&endpoint, Some(&bearer), &initialize_body(1));
    assert_eq!(status, 200);
    assert!(body.contains(WATCHER_BRIDGE_MCP_PROTOCOL_VERSION));
    close_lease(&local, lease);
}

#[test]
fn missing_wrong_and_duplicate_requests_fail_before_watcher_work() {
    let local = watcher_host("fail-closed", "sleep");
    let lease = open_lease(&local, "turn-fail");
    let endpoint = lease.endpoint().expose().to_owned();
    let bearer = lease.bearer().expose().to_owned();

    let (status, _) = post_json(&endpoint, None, &initialize_body(1));
    assert_eq!(status, 401);
    let (status, _) = post_json(&endpoint, Some("deadbeef"), &initialize_body(1));
    assert_eq!(status, 401);
    let start = tool_call(
        2,
        WATCHER_BRIDGE_TOOL_START,
        json!({"operation_data": "sleep-operation"}),
    );
    let (status, _) = post_json(&endpoint, Some(&bearer), &start);
    assert_eq!(status, 200);
    let (status, body) = post_json(&endpoint, Some(&bearer), &start);
    assert_eq!(status, 200);
    assert!(body.contains("correlation"));

    let listed = block_on(
        local
            .services()
            .watcher()
            .expect("watcher")
            .list(WatcherOwningTurn::new("turn-fail").expect("owning turn")),
    )
    .expect("list");
    assert_eq!(listed.len(), 1);
    assert_eq!(listed[0].accepted_by(), WatcherRequester::Model);
    close_lease(&local, lease);
}

#[test]
fn malformed_oversized_and_unknown_protocol_fail_closed() {
    let local = default_host("malformed");
    let lease = open_lease(&local, "turn-malformed");
    let endpoint = lease.endpoint().expose().to_owned();
    let bearer = lease.bearer().expose().to_owned();

    let (status, _) = post_json(&endpoint, Some(&bearer), "{not-json");
    assert_eq!(status, 400);
    let (status, _) = post_json(
        &endpoint,
        Some(&bearer),
        &json!({
            "jsonrpc": WATCHER_BRIDGE_JSONRPC_VERSION,
            "id": 1,
            "method": "tools/call",
            "params": { "name": "shell", "arguments": {} }
        })
        .to_string(),
    );
    assert_eq!(status, 200);
    let oversized = format!(
        "{{\"jsonrpc\":\"2.0\",\"id\":1,\"method\":\"initialize\",\"params\":{{\"protocolVersion\":\"{}\",\"pad\":\"{}\"}}}}",
        WATCHER_BRIDGE_MCP_PROTOCOL_VERSION,
        "x".repeat(20_000)
    );
    let (status, _) = post_json(&endpoint, Some(&bearer), &oversized);
    assert_eq!(status, 413);
    close_lease(&local, lease);
}

#[test]
fn reserved_family_reaches_the_same_registry_and_idle_gate_freezes() {
    let local = watcher_host("same-registry", "exit-zero");
    let lease = open_lease(&local, "turn-registry");
    let endpoint = lease.endpoint().expose().to_owned();
    let bearer = lease.bearer().expose().to_owned();

    let (status, body) = post_json(&endpoint, Some(&bearer), &tools_list_body(1));
    assert_eq!(status, 200);
    assert!(body.contains(WATCHER_BRIDGE_TOOL_START));
    assert!(body.contains(WATCHER_BRIDGE_TOOL_COMPLETION_GATE));

    let (status, body) = post_json(
        &endpoint,
        Some(&bearer),
        &tool_call(
            2,
            WATCHER_BRIDGE_TOOL_START,
            json!({"operation_data": "exit-zero-operation"}),
        ),
    );
    assert_eq!(status, 200);
    let watcher_id = tool_text_field(&body, "watcher_id");
    let (status, inspect) = post_json(
        &endpoint,
        Some(&bearer),
        &tool_call(
            3,
            swallowtail_runtime::WATCHER_BRIDGE_TOOL_INSPECT,
            json!({"watcher_id": watcher_id}),
        ),
    );
    assert_eq!(status, 200);
    assert!(inspect.contains(&watcher_id));

    let (status, body) = post_json(
        &endpoint,
        Some(&bearer),
        &tool_call(
            4,
            WATCHER_BRIDGE_TOOL_WAIT,
            json!({"watcher_id": watcher_id}),
        ),
    );
    assert_eq!(status, 200);
    assert!(body.contains("satisfied") || body.contains("terminal"));

    let (status, body) = post_json(
        &endpoint,
        Some(&bearer),
        &tool_call(5, WATCHER_BRIDGE_TOOL_COMPLETION_GATE, json!({})),
    );
    assert_eq!(status, 200);
    assert!(body.contains("frozen"));

    let (status, body) = post_json(
        &endpoint,
        Some(&bearer),
        &tool_call(
            6,
            WATCHER_BRIDGE_TOOL_START,
            json!({"operation_data": "exit-zero-operation"}),
        ),
    );
    assert_eq!(status, 200);
    assert!(body.contains("frozen"));
    close_lease(&local, lease);
}

#[test]
fn cross_lease_bearer_fails_and_close_releases_the_listener() {
    let local = default_host("cross-lease");
    let first = open_lease(&local, "turn-a");
    let second = open_lease(&local, "turn-b");
    let first_endpoint = first.endpoint().expose().to_owned();
    let first_bearer = first.bearer().expose().to_owned();
    let second_bearer = second.bearer().expose().to_owned();
    let (status, _) = post_json(&first_endpoint, Some(&second_bearer), &initialize_body(1));
    assert_eq!(status, 401);
    close_lease(&local, first);
    let closed = post_json_result(&first_endpoint, Some(&first_bearer), &initialize_body(2));
    assert!(closed.is_err() || closed.ok().is_some_and(|(status, _)| status != 200));
    close_lease(&local, second);
}

#[test]
fn dropped_lease_joins_without_leaving_a_listener() {
    let local = default_host("drop-join");
    let lease = open_lease(&local, "turn-drop");
    let endpoint = lease.endpoint().expose().to_owned();
    let bearer = lease.bearer().expose().to_owned();
    drop(lease);
    let closed = post_json_result(&endpoint, Some(&bearer), &initialize_body(1));
    assert!(closed.is_err() || closed.ok().is_some_and(|(status, _)| status != 200));
}

fn default_host(label: &str) -> LocalHostServices {
    LocalProcessHost::builder(LocalProcessLimits::default()).build_services(
        ExecutionHostId::new(format!("fixture.host.bridge.{label}")).expect("host id"),
    )
}

fn watcher_host(label: &str, mode: &str) -> LocalHostServices {
    let executable = support::executable_ref();
    let environment = support::environment_ref();
    let operation = WatcherOperationData::new(format!("{mode}-operation")).expect("operation");
    let request = ProcessRequest::new(executable.clone())
        .with_arguments(support::fixture_arguments())
        .with_environment([environment.clone()]);
    LocalProcessHost::builder(LocalProcessLimits::default())
        .approve_executable(
            executable,
            std::env::current_exe().expect("watcher fixture executable"),
        )
        .approve_environment(environment, support::fixture_environment(mode))
        .approve_watcher_operation(operation, request)
        .build_services(
            ExecutionHostId::new(format!("fixture.host.bridge.{label}")).expect("host id"),
        )
}

fn open_lease(local: &LocalHostServices, turn: &str) -> WatcherBridgeLease {
    let bridge = local
        .services()
        .watcher_bridge()
        .expect("bridge is registered")
        .clone();
    block_on(bridge.open(WatcherBridgeOpenRequest::new(
        ScopeId::new(format!("scope-{turn}")).expect("scope"),
        RuntimeTurnId::new(turn).expect("turn"),
    )))
    .expect("open succeeds")
}

fn close_lease(local: &LocalHostServices, lease: WatcherBridgeLease) {
    let bridge = local
        .services()
        .watcher_bridge()
        .expect("bridge is registered")
        .clone();
    block_on(bridge.close(lease, WatcherCleanupCause::Cancelled)).expect("close joins");
}

fn initialize_body(id: u64) -> String {
    json!({
        "jsonrpc": WATCHER_BRIDGE_JSONRPC_VERSION,
        "id": id,
        "method": WATCHER_BRIDGE_INITIALIZE_METHOD,
        "params": {
            "protocolVersion": WATCHER_BRIDGE_MCP_PROTOCOL_VERSION,
            "capabilities": {},
            "clientInfo": { "name": "fixture", "version": "0" }
        }
    })
    .to_string()
}

fn tools_list_body(id: u64) -> String {
    json!({
        "jsonrpc": WATCHER_BRIDGE_JSONRPC_VERSION,
        "id": id,
        "method": WATCHER_BRIDGE_TOOLS_LIST_METHOD
    })
    .to_string()
}

fn tool_call(id: u64, name: &str, arguments: Value) -> String {
    json!({
        "jsonrpc": WATCHER_BRIDGE_JSONRPC_VERSION,
        "id": id,
        "method": WATCHER_BRIDGE_TOOLS_CALL_METHOD,
        "params": { "name": name, "arguments": arguments }
    })
    .to_string()
}

fn tool_text_field(body: &str, field: &str) -> String {
    let response: Value = serde_json::from_str(body).expect("json response");
    let text = response["result"]["content"][0]["text"]
        .as_str()
        .expect("tool text");
    let payload: Value = serde_json::from_str(text).expect("tool payload");
    payload[field].as_str().expect("field").to_owned()
}

fn post_json(endpoint: &str, bearer: Option<&str>, body: &str) -> (u16, String) {
    post_json_result(endpoint, bearer, body).expect("http round-trip")
}

fn post_json_result(
    endpoint: &str,
    bearer: Option<&str>,
    body: &str,
) -> Result<(u16, String), std::io::Error> {
    let without_scheme = endpoint
        .strip_prefix("http://")
        .expect("loopback http endpoint");
    let (host, path) = without_scheme.split_once('/').expect("host and path");
    let mut stream = TcpStream::connect(host)?;
    stream.set_read_timeout(Some(Duration::from_secs(8)))?;
    stream.set_write_timeout(Some(Duration::from_secs(8)))?;
    let authorization = bearer
        .map(|value| format!("Authorization: Bearer {value}\r\n"))
        .unwrap_or_default();
    let request = format!(
        "POST /{path} HTTP/1.1\r\nHost: {host}\r\n{authorization}Content-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
        body.len()
    );
    stream.write_all(request.as_bytes())?;
    let mut response = Vec::new();
    stream.read_to_end(&mut response)?;
    let text = String::from_utf8_lossy(&response);
    let status = text
        .split_whitespace()
        .nth(1)
        .and_then(|value| value.parse().ok())
        .unwrap_or(0);
    let body = text.split("\r\n\r\n").nth(1).unwrap_or_default().to_owned();
    Ok((status, body))
}

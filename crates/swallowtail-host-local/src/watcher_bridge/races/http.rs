use super::super::LocalWatcherBridgeHostService;
use futures_executor::block_on;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::{Duration, Instant};
use swallowtail_runtime::{
    RuntimeTurnId, ScopeId, WATCHER_BRIDGE_INITIALIZE_METHOD,
    WATCHER_BRIDGE_INITIALIZED_NOTIFICATION, WATCHER_BRIDGE_JSONRPC_VERSION,
    WATCHER_BRIDGE_MCP_PROTOCOL_VERSION, WATCHER_BRIDGE_TOOL_START, WATCHER_BRIDGE_TOOL_WAIT,
    WATCHER_BRIDGE_TOOLS_CALL_METHOD, WatcherBridgeHostService, WatcherBridgeLease,
    WatcherBridgeOpenRequest,
};

pub(super) fn open(bridge: &LocalWatcherBridgeHostService, turn: &str) -> WatcherBridgeLease {
    block_on(bridge.open(WatcherBridgeOpenRequest::new(
        ScopeId::new(format!("scope-{turn}")).expect("scope"),
        RuntimeTurnId::new(turn).expect("turn"),
    )))
    .expect("open")
}

pub(super) fn handshake(endpoint: &str, bearer: &str) {
    let (status, _) = post(endpoint, bearer, initialize());
    assert_eq!(status, 200);
    let (status, _) = post(endpoint, bearer, initialized());
    assert_eq!(status, 202);
}

pub(super) fn spin_until(flag: &AtomicBool) {
    let started = Instant::now();
    while !flag.load(Ordering::SeqCst) {
        assert!(
            started.elapsed() < Duration::from_secs(5),
            "timed out waiting for watcher work to start"
        );
        thread::sleep(Duration::from_millis(5));
    }
}

pub(super) fn post(endpoint: &str, bearer: &str, body: String) -> (u16, String) {
    let without_scheme = endpoint.strip_prefix("http://").expect("http");
    let (host, path) = without_scheme.split_once('/').expect("path");
    let mut stream = TcpStream::connect(host).expect("connect");
    stream
        .set_read_timeout(Some(Duration::from_secs(8)))
        .expect("timeout");
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
    (
        status,
        text.split("\r\n\r\n").nth(1).unwrap_or_default().to_owned(),
    )
}

pub(super) fn tool_watcher_id(body: &str) -> String {
    let response: serde_json::Value = serde_json::from_str(body).expect("json");
    let text = response["result"]["content"][0]["text"]
        .as_str()
        .expect("tool text");
    serde_json::from_str::<serde_json::Value>(text).expect("payload")["watcher_id"]
        .as_str()
        .expect("id")
        .to_owned()
}

fn initialize() -> String {
    serde_json::json!({
        "jsonrpc": WATCHER_BRIDGE_JSONRPC_VERSION,
        "id": 1,
        "method": WATCHER_BRIDGE_INITIALIZE_METHOD,
        "params": {
            "protocolVersion": WATCHER_BRIDGE_MCP_PROTOCOL_VERSION,
            "capabilities": {},
            "clientInfo": { "name": "race", "version": "0" }
        }
    })
    .to_string()
}

fn initialized() -> String {
    serde_json::json!({
        "jsonrpc": WATCHER_BRIDGE_JSONRPC_VERSION,
        "method": WATCHER_BRIDGE_INITIALIZED_NOTIFICATION
    })
    .to_string()
}

pub(super) fn start_body(id: u64) -> String {
    serde_json::json!({
        "jsonrpc": WATCHER_BRIDGE_JSONRPC_VERSION,
        "id": id,
        "method": WATCHER_BRIDGE_TOOLS_CALL_METHOD,
        "params": { "name": WATCHER_BRIDGE_TOOL_START, "arguments": { "operation_data": "sleep-operation" } }
    })
    .to_string()
}

pub(super) fn wait_body(id: u64, watcher_id: &str) -> String {
    serde_json::json!({
        "jsonrpc": WATCHER_BRIDGE_JSONRPC_VERSION,
        "id": id,
        "method": WATCHER_BRIDGE_TOOLS_CALL_METHOD,
        "params": { "name": WATCHER_BRIDGE_TOOL_WAIT, "arguments": { "watcher_id": watcher_id } }
    })
    .to_string()
}

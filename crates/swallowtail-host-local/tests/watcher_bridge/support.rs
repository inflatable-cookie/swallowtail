#![allow(dead_code)]

#[path = "../local_process/support.rs"]
mod process;

use futures_executor::block_on;
use serde_json::{Value, json};
use swallowtail_core::{ExecutionHostId, WatcherCleanupCause, WatcherOperationData};
use swallowtail_host_local::{LocalHostServices, LocalProcessHost, LocalProcessLimits};
use swallowtail_runtime::{
    ProcessRequest, RuntimeTurnId, ScopeId, WATCHER_BRIDGE_INITIALIZE_METHOD,
    WATCHER_BRIDGE_INITIALIZED_NOTIFICATION, WATCHER_BRIDGE_JSONRPC_VERSION,
    WATCHER_BRIDGE_MCP_PROTOCOL_VERSION, WATCHER_BRIDGE_TOOLS_CALL_METHOD,
    WATCHER_BRIDGE_TOOLS_LIST_METHOD, WatcherBridgeLease, WatcherBridgeOpenRequest,
};

pub(super) fn default_host(label: &str) -> LocalHostServices {
    LocalProcessHost::builder(LocalProcessLimits::default()).build_services(
        ExecutionHostId::new(format!("fixture.host.bridge.{label}")).expect("host id"),
    )
}

pub(super) fn watcher_host(label: &str, mode: &str) -> LocalHostServices {
    let executable = process::executable_ref();
    let environment = process::environment_ref();
    let operation = WatcherOperationData::new(format!("{mode}-operation")).expect("operation");
    let request = ProcessRequest::new(executable.clone())
        .with_arguments(process::fixture_arguments())
        .with_environment([environment.clone()]);
    LocalProcessHost::builder(LocalProcessLimits::default())
        .approve_executable(
            executable,
            std::env::current_exe().expect("watcher fixture executable"),
        )
        .approve_environment(environment, process::fixture_environment(mode))
        .approve_watcher_operation(operation, request)
        .build_services(
            ExecutionHostId::new(format!("fixture.host.bridge.{label}")).expect("host id"),
        )
}

pub(super) fn open_lease(local: &LocalHostServices, turn: &str) -> WatcherBridgeLease {
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

pub(super) fn close_lease(local: &LocalHostServices, lease: WatcherBridgeLease) {
    let bridge = local
        .services()
        .watcher_bridge()
        .expect("bridge is registered")
        .clone();
    block_on(bridge.close(lease, WatcherCleanupCause::Cancelled)).expect("close joins");
}

pub(super) fn handshake(endpoint: &str, bearer: &str) {
    let (status, _) = super::http::post_json(endpoint, Some(bearer), &initialize_body(1));
    assert_eq!(status, 200);
    let (status, _) = super::http::post_json(endpoint, Some(bearer), &initialized_body());
    assert_eq!(status, 202);
}

pub(super) fn initialize_body(id: u64) -> String {
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

pub(super) fn initialized_body() -> String {
    json!({
        "jsonrpc": WATCHER_BRIDGE_JSONRPC_VERSION,
        "method": WATCHER_BRIDGE_INITIALIZED_NOTIFICATION
    })
    .to_string()
}

pub(super) fn tools_list_body(id: u64) -> String {
    json!({
        "jsonrpc": WATCHER_BRIDGE_JSONRPC_VERSION,
        "id": id,
        "method": WATCHER_BRIDGE_TOOLS_LIST_METHOD
    })
    .to_string()
}

pub(super) fn tool_call(id: Value, name: &str, arguments: Value) -> String {
    json!({
        "jsonrpc": WATCHER_BRIDGE_JSONRPC_VERSION,
        "id": id,
        "method": WATCHER_BRIDGE_TOOLS_CALL_METHOD,
        "params": { "name": name, "arguments": arguments }
    })
    .to_string()
}

pub(super) fn tool_payload(body: &str) -> Value {
    let response: Value = serde_json::from_str(body).expect("json response");
    let text = response["result"]["content"][0]["text"]
        .as_str()
        .expect("tool text");
    serde_json::from_str(text).expect("tool payload")
}

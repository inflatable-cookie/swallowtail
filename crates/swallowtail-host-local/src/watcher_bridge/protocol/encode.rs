use serde_json::Value;
use swallowtail_core::WatcherLifecyclePhase;
use swallowtail_runtime::{
    WATCHER_BRIDGE_JSONRPC_VERSION, WATCHER_BRIDGE_MCP_PROTOCOL_VERSION,
    WATCHER_BRIDGE_TOOL_COMPLETION_GATE, WATCHER_BRIDGE_TOOL_INSPECT, WATCHER_BRIDGE_TOOL_LIST,
    WATCHER_BRIDGE_TOOL_START, WATCHER_BRIDGE_TOOL_STOP, WATCHER_BRIDGE_TOOL_WAIT,
    WatcherBridgeCompletionState, WatcherSnapshot, WatcherStopAcknowledgement,
    WatcherWaitRepresentation,
};

pub(crate) fn jsonrpc_error(id: Option<Value>, code: i64, message: &str) -> String {
    serde_json::json!({
        "jsonrpc": WATCHER_BRIDGE_JSONRPC_VERSION,
        "id": id.unwrap_or(Value::Null),
        "error": { "code": code, "message": message }
    })
    .to_string()
}

pub(crate) fn jsonrpc_result(id: Value, result: Value) -> String {
    serde_json::json!({
        "jsonrpc": WATCHER_BRIDGE_JSONRPC_VERSION,
        "id": id,
        "result": result
    })
    .to_string()
}

pub(crate) fn initialize_result() -> Value {
    serde_json::json!({
        "protocolVersion": WATCHER_BRIDGE_MCP_PROTOCOL_VERSION,
        "capabilities": { "tools": { "listChanged": false } },
        "serverInfo": {
            "name": "swallowtail-watcher-bridge",
            "version": env!("CARGO_PKG_VERSION"),
        }
    })
}

pub(crate) fn tools_list_result() -> Value {
    serde_json::json!({
        "tools": [
            tool_schema(
                WATCHER_BRIDGE_TOOL_START,
                "Start one host-owned watcher for this turn",
                &["operation_data"],
            ),
            tool_schema(
                WATCHER_BRIDGE_TOOL_INSPECT,
                "Inspect one turn-owned watcher",
                &["watcher_id"],
            ),
            tool_schema(
                WATCHER_BRIDGE_TOOL_LIST,
                "List watchers owned by this turn",
                &[],
            ),
            tool_schema(
                WATCHER_BRIDGE_TOOL_WAIT,
                "Wait until one watcher is terminal and joined",
                &["watcher_id"],
            ),
            tool_schema(
                WATCHER_BRIDGE_TOOL_STOP,
                "Request stop for one turn-owned watcher",
                &["watcher_id"],
            ),
            tool_schema(
                WATCHER_BRIDGE_TOOL_COMPLETION_GATE,
                "Observe remaining work and freeze idle admission",
                &[],
            ),
        ]
    })
}

pub(crate) fn tool_text(payload: Value) -> Value {
    serde_json::json!({
        "content": [{ "type": "text", "text": payload.to_string() }],
        "isError": false
    })
}

pub(crate) fn snapshot_payload(snapshot: &WatcherSnapshot) -> Value {
    serde_json::json!({
        "watcher_id": snapshot.watcher_id().as_str(),
        "phase": phase_label(snapshot.phase()),
        "revision": snapshot.revision().get(),
        "accepted_by": snapshot.accepted_by().as_str(),
        "terminal_cause": snapshot.terminal_cause().map(|cause| cause.as_str()),
        "summary": snapshot.summary().map(swallowtail_core::WatcherSummary::as_str),
    })
}

pub(crate) fn list_payload(snapshots: &[WatcherSnapshot]) -> Value {
    serde_json::json!({
        "watchers": snapshots.iter().map(snapshot_payload).collect::<Vec<_>>()
    })
}

pub(crate) fn completion_payload(state: &WatcherBridgeCompletionState) -> Value {
    serde_json::json!({
        "admission": state.admission().as_str(),
        "active_or_unjoined": state.active_or_unjoined().iter().map(snapshot_payload).collect::<Vec<_>>(),
        "allows_successful_completion": state.allows_successful_completion(),
    })
}

pub(crate) fn wait_payload(
    representation: WatcherWaitRepresentation,
    snapshot: &WatcherSnapshot,
) -> Value {
    serde_json::json!({
        "wait": wait_label(representation),
        "terminal_cause": wait_cause(representation),
        "watcher": snapshot_payload(snapshot),
    })
}

pub(crate) fn stop_payload(
    acknowledgement: WatcherStopAcknowledgement,
    snapshot: &WatcherSnapshot,
) -> Value {
    serde_json::json!({
        "stop": match acknowledgement {
            WatcherStopAcknowledgement::Stopped => "stopped",
            WatcherStopAcknowledgement::AlreadyTerminal(_) => "already_terminal",
        },
        "watcher": snapshot_payload(snapshot),
    })
}

pub(crate) fn error_http_status(
    error: &swallowtail_runtime::RuntimeFailure,
) -> (u16, &'static str, i64) {
    match error.diagnostic().code() {
        "swallowtail.watcher_bridge.unauthorized" => (401, "Unauthorized", -32001),
        "swallowtail.watcher_bridge.oversized" => (413, "Payload Too Large", -32600),
        "swallowtail.watcher_bridge.unknown" => (200, "OK", -32601),
        "swallowtail.watcher_bridge.malformed" => (400, "Bad Request", -32600),
        "swallowtail.watcher_bridge.duplicate_correlation" => (200, "OK", -32002),
        "swallowtail.watcher_bridge.admission_frozen" => (200, "OK", -32003),
        "swallowtail.watcher_bridge.closed" => (200, "OK", -32004),
        "swallowtail.watcher_bridge.busy" => (429, "Too Many Requests", -32005),
        "swallowtail.watcher_bridge.handshake_required" => (200, "OK", -32006),
        _ => (200, "OK", -32000),
    }
}

pub(crate) fn error_message(error: &swallowtail_runtime::RuntimeFailure) -> &str {
    error.diagnostic().message()
}

fn tool_schema(name: &str, description: &str, required: &[&str]) -> Value {
    let mut properties = serde_json::Map::new();
    for field in required {
        properties.insert((*field).to_owned(), serde_json::json!({ "type": "string" }));
    }
    serde_json::json!({
        "name": name,
        "description": description,
        "inputSchema": {
            "type": "object",
            "properties": properties,
            "required": required,
            "additionalProperties": false
        }
    })
}

fn phase_label(phase: WatcherLifecyclePhase) -> &'static str {
    match phase {
        WatcherLifecyclePhase::Accepted => "accepted",
        WatcherLifecyclePhase::Running => "running",
        WatcherLifecyclePhase::Terminal => "terminal",
        WatcherLifecyclePhase::Joined => "joined",
    }
}

fn wait_label(representation: WatcherWaitRepresentation) -> &'static str {
    match representation {
        WatcherWaitRepresentation::Pending => "pending",
        WatcherWaitRepresentation::TerminalUnjoined(_) => "terminal_unjoined",
        WatcherWaitRepresentation::Satisfied(_) => "satisfied",
        WatcherWaitRepresentation::Cancelled => "cancelled",
        WatcherWaitRepresentation::DeadlineExceeded => "deadline_exceeded",
    }
}

fn wait_cause(representation: WatcherWaitRepresentation) -> Option<&'static str> {
    match representation {
        WatcherWaitRepresentation::TerminalUnjoined(cause)
        | WatcherWaitRepresentation::Satisfied(cause) => Some(cause.as_str()),
        WatcherWaitRepresentation::Pending
        | WatcherWaitRepresentation::Cancelled
        | WatcherWaitRepresentation::DeadlineExceeded => None,
    }
}

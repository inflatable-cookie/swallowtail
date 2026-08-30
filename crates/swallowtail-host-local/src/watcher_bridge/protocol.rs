use super::state::{LiveLease, malformed_failure, unauthorized_failure, unknown_failure};
use crate::output::failure;
use serde_json::{Map, Value};
use std::sync::Arc;
use swallowtail_core::{
    WatcherId, WatcherLifecyclePhase, WatcherOperationData, WatcherOwningTurn, WatcherRequester,
};
use swallowtail_runtime::{
    RuntimeFailure, WATCHER_BRIDGE_INITIALIZE_METHOD, WATCHER_BRIDGE_INITIALIZED_NOTIFICATION,
    WATCHER_BRIDGE_JSONRPC_VERSION, WATCHER_BRIDGE_MCP_PROTOCOL_VERSION,
    WATCHER_BRIDGE_RESERVED_TOOLS, WATCHER_BRIDGE_TOOL_COMPLETION_GATE,
    WATCHER_BRIDGE_TOOL_INSPECT, WATCHER_BRIDGE_TOOL_LIST, WATCHER_BRIDGE_TOOL_START,
    WATCHER_BRIDGE_TOOL_STOP, WATCHER_BRIDGE_TOOL_WAIT, WATCHER_BRIDGE_TOOLS_CALL_METHOD,
    WATCHER_BRIDGE_TOOLS_LIST_METHOD, WatcherBridgeCompletionState, WatcherSnapshot,
    WatcherWaitOptions,
};

pub(super) enum DecodedRequest {
    Initialize {
        id: Value,
    },
    Initialized,
    ToolsList {
        id: Value,
    },
    ToolsCall {
        id: Value,
        name: String,
        arguments: Map<String, Value>,
    },
}

pub(super) fn decode_request(body: &[u8]) -> Result<DecodedRequest, RuntimeFailure> {
    let value: Value = serde_json::from_slice(body).map_err(|_| malformed_failure())?;
    let object = value.as_object().ok_or_else(malformed_failure)?;
    for key in object.keys() {
        if !matches!(key.as_str(), "jsonrpc" | "id" | "method" | "params") {
            return Err(unknown_failure());
        }
    }
    let jsonrpc = object
        .get("jsonrpc")
        .and_then(Value::as_str)
        .ok_or_else(malformed_failure)?;
    if jsonrpc != WATCHER_BRIDGE_JSONRPC_VERSION {
        return Err(unknown_failure());
    }
    let method = object
        .get("method")
        .and_then(Value::as_str)
        .ok_or_else(malformed_failure)?;
    match method {
        WATCHER_BRIDGE_INITIALIZED_NOTIFICATION => {
            if object.contains_key("id") {
                return Err(malformed_failure());
            }
            Ok(DecodedRequest::Initialized)
        }
        WATCHER_BRIDGE_INITIALIZE_METHOD => {
            let id = object.get("id").cloned().ok_or_else(malformed_failure)?;
            require_initialize_params(object.get("params"))?;
            Ok(DecodedRequest::Initialize { id })
        }
        WATCHER_BRIDGE_TOOLS_LIST_METHOD => {
            let id = object.get("id").cloned().ok_or_else(malformed_failure)?;
            require_empty_params(object.get("params"))?;
            Ok(DecodedRequest::ToolsList { id })
        }
        WATCHER_BRIDGE_TOOLS_CALL_METHOD => {
            let id = object.get("id").cloned().ok_or_else(malformed_failure)?;
            let (name, arguments) = require_tool_call_params(object.get("params"))?;
            Ok(DecodedRequest::ToolsCall {
                id,
                name,
                arguments,
            })
        }
        _ => Err(unknown_failure()),
    }
}

pub(super) fn correlation_id(id: &Value) -> Result<String, RuntimeFailure> {
    match id {
        Value::String(value) if !value.is_empty() => Ok(value.clone()),
        Value::Number(value) => Ok(value.to_string()),
        _ => Err(malformed_failure()),
    }
}

pub(super) fn dispatch(
    live: &Arc<LiveLease>,
    request: DecodedRequest,
) -> Result<Option<String>, RuntimeFailure> {
    match request {
        DecodedRequest::Initialized => Ok(None),
        DecodedRequest::Initialize { id } => Ok(Some(jsonrpc_result(
            id,
            serde_json::json!({
                "protocolVersion": WATCHER_BRIDGE_MCP_PROTOCOL_VERSION,
                "capabilities": { "tools": { "listChanged": false } },
                "serverInfo": {
                    "name": "swallowtail-watcher-bridge",
                    "version": env!("CARGO_PKG_VERSION"),
                }
            }),
        ))),
        DecodedRequest::ToolsList { id } => Ok(Some(jsonrpc_result(id, tools_list_result()))),
        DecodedRequest::ToolsCall {
            id,
            name,
            arguments,
        } => {
            let result = dispatch_tool(live, &name, arguments)?;
            Ok(Some(jsonrpc_result(id, result)))
        }
    }
}

pub(super) fn jsonrpc_error(id: Option<Value>, code: i64, message: &str) -> String {
    serde_json::json!({
        "jsonrpc": WATCHER_BRIDGE_JSONRPC_VERSION,
        "id": id.unwrap_or(Value::Null),
        "error": { "code": code, "message": message }
    })
    .to_string()
}

pub(super) fn authenticate(
    live: &LiveLease,
    presented: Option<&str>,
) -> Result<(), RuntimeFailure> {
    let presented = presented.ok_or_else(unauthorized_failure)?;
    if live_bearer_matches(live, presented) {
        Ok(())
    } else {
        Err(unauthorized_failure())
    }
}

fn live_bearer_matches(live: &LiveLease, presented: &str) -> bool {
    let expected = live.bearer.as_bytes();
    let presented = presented.as_bytes();
    expected.len() == presented.len()
        && expected
            .iter()
            .zip(presented)
            .fold(0_u8, |acc, (left, right)| acc | (left ^ right))
            == 0
}

fn dispatch_tool(
    live: &Arc<LiveLease>,
    name: &str,
    arguments: Map<String, Value>,
) -> Result<Value, RuntimeFailure> {
    match name {
        WATCHER_BRIDGE_TOOL_START => call_start(live, arguments),
        WATCHER_BRIDGE_TOOL_INSPECT => call_inspect(live, arguments),
        WATCHER_BRIDGE_TOOL_LIST => {
            require_empty_object(&arguments)?;
            live.require_not_closed()?;
            let owning_turn = owning_turn(&live.turn)?;
            let snapshots = super::state::ready(live.watcher.list(owning_turn))?;
            Ok(tool_text(list_payload(&snapshots)))
        }
        WATCHER_BRIDGE_TOOL_WAIT => call_wait(live, arguments),
        WATCHER_BRIDGE_TOOL_STOP => call_stop(live, arguments),
        WATCHER_BRIDGE_TOOL_COMPLETION_GATE => {
            require_empty_object(&arguments)?;
            let state = live.completion_gate()?;
            Ok(tool_text(completion_payload(&state)))
        }
        _ => Err(unknown_failure()),
    }
}

fn call_start(
    live: &Arc<LiveLease>,
    arguments: Map<String, Value>,
) -> Result<Value, RuntimeFailure> {
    let operation_data = WatcherOperationData::new(require_string(&arguments, "operation_data")?)
        .map_err(|_| malformed_failure())?;
    require_only_keys(&arguments, &["operation_data"])?;
    live.begin_create()?;
    let result =
        live.watcher
            .accept_start(live.turn.clone(), WatcherRequester::Model, operation_data);
    live.end_create();
    Ok(tool_text(snapshot_payload(&super::state::ready(result)?)))
}

fn call_inspect(
    live: &Arc<LiveLease>,
    arguments: Map<String, Value>,
) -> Result<Value, RuntimeFailure> {
    let watcher_id = watcher_id(&arguments)?;
    require_only_keys(&arguments, &["watcher_id"])?;
    live.require_not_closed()?;
    let snapshot = super::state::ready(live.watcher.inspect(owning_turn(&live.turn)?, watcher_id))?;
    Ok(tool_text(snapshot_payload(&snapshot)))
}

fn call_wait(
    live: &Arc<LiveLease>,
    arguments: Map<String, Value>,
) -> Result<Value, RuntimeFailure> {
    let watcher_id = watcher_id(&arguments)?;
    require_only_keys(&arguments, &["watcher_id"])?;
    live.require_not_closed()?;
    let representation = futures_executor::block_on(live.watcher.wait(
        owning_turn(&live.turn)?,
        watcher_id,
        WatcherWaitOptions::new().with_cancellation(live.cancel.wait_requested()),
    ))?;
    Ok(tool_text(serde_json::json!({
        "wait": wait_label(representation),
    })))
}

fn call_stop(
    live: &Arc<LiveLease>,
    arguments: Map<String, Value>,
) -> Result<Value, RuntimeFailure> {
    let watcher_id = watcher_id(&arguments)?;
    require_only_keys(&arguments, &["watcher_id"])?;
    live.require_not_closed()?;
    let (acknowledgement, snapshot) = super::state::ready(
        live.watcher
            .request_stop(owning_turn(&live.turn)?, watcher_id),
    )?;
    Ok(tool_text(serde_json::json!({
        "stop": match acknowledgement {
            swallowtail_runtime::WatcherStopAcknowledgement::Stopped => "stopped",
            swallowtail_runtime::WatcherStopAcknowledgement::AlreadyTerminal(_) => {
                "already_terminal"
            }
        },
        "watcher": snapshot_payload(&snapshot),
    })))
}

fn require_initialize_params(params: Option<&Value>) -> Result<(), RuntimeFailure> {
    let object = params
        .and_then(Value::as_object)
        .ok_or_else(malformed_failure)?;
    for key in object.keys() {
        if !matches!(
            key.as_str(),
            "protocolVersion" | "capabilities" | "clientInfo"
        ) {
            return Err(unknown_failure());
        }
    }
    let version = object
        .get("protocolVersion")
        .and_then(Value::as_str)
        .ok_or_else(malformed_failure)?;
    if version != WATCHER_BRIDGE_MCP_PROTOCOL_VERSION {
        return Err(unknown_failure());
    }
    Ok(())
}

fn require_empty_params(params: Option<&Value>) -> Result<(), RuntimeFailure> {
    match params {
        None => Ok(()),
        Some(Value::Object(object)) if object.is_empty() => Ok(()),
        Some(Value::Null) => Ok(()),
        _ => Err(unknown_failure()),
    }
}

fn require_tool_call_params(
    params: Option<&Value>,
) -> Result<(String, Map<String, Value>), RuntimeFailure> {
    let object = params
        .and_then(Value::as_object)
        .ok_or_else(malformed_failure)?;
    for key in object.keys() {
        if !matches!(key.as_str(), "name" | "arguments") {
            return Err(unknown_failure());
        }
    }
    let name = object
        .get("name")
        .and_then(Value::as_str)
        .ok_or_else(malformed_failure)?
        .to_owned();
    if !WATCHER_BRIDGE_RESERVED_TOOLS.contains(&name.as_str()) {
        return Err(unknown_failure());
    }
    let arguments = match object.get("arguments") {
        None | Some(Value::Null) => Map::new(),
        Some(Value::Object(arguments)) => arguments.clone(),
        Some(_) => return Err(malformed_failure()),
    };
    Ok((name, arguments))
}

fn require_empty_object(arguments: &Map<String, Value>) -> Result<(), RuntimeFailure> {
    if arguments.is_empty() {
        Ok(())
    } else {
        Err(unknown_failure())
    }
}

fn require_only_keys(
    arguments: &Map<String, Value>,
    allowed: &[&str],
) -> Result<(), RuntimeFailure> {
    if arguments.len() == allowed.len() && allowed.iter().all(|key| arguments.contains_key(*key)) {
        Ok(())
    } else {
        Err(unknown_failure())
    }
}

fn require_string(arguments: &Map<String, Value>, key: &str) -> Result<String, RuntimeFailure> {
    arguments
        .get(key)
        .and_then(Value::as_str)
        .filter(|value| !value.is_empty())
        .map(str::to_owned)
        .ok_or_else(malformed_failure)
}

fn watcher_id(arguments: &Map<String, Value>) -> Result<WatcherId, RuntimeFailure> {
    WatcherId::new(require_string(arguments, "watcher_id")?).map_err(|_| malformed_failure())
}

fn owning_turn(
    turn: &swallowtail_runtime::RuntimeTurnId,
) -> Result<WatcherOwningTurn, RuntimeFailure> {
    WatcherOwningTurn::new(turn.as_str().to_owned()).map_err(|_| {
        failure(
            "swallowtail.watcher_bridge.identity_rejected",
            "Watcher bridge rejected the bound turn identity",
        )
    })
}

fn tools_list_result() -> Value {
    serde_json::json!({
        "tools": WATCHER_BRIDGE_RESERVED_TOOLS.iter().map(|name| {
            serde_json::json!({
                "name": name,
                "description": "Reserved operation-scoped watcher control",
                "inputSchema": { "type": "object", "additionalProperties": false }
            })
        }).collect::<Vec<_>>()
    })
}

fn tool_text(payload: Value) -> Value {
    serde_json::json!({
        "content": [{ "type": "text", "text": payload.to_string() }],
        "isError": false
    })
}

fn snapshot_payload(snapshot: &WatcherSnapshot) -> Value {
    serde_json::json!({
        "watcher_id": snapshot.watcher_id().as_str(),
        "phase": phase_label(snapshot.phase()),
        "revision": snapshot.revision().get(),
        "accepted_by": snapshot.accepted_by().as_str(),
        "terminal_cause": snapshot.terminal_cause().map(|cause| cause.as_str()),
        "summary": snapshot.summary().map(swallowtail_core::WatcherSummary::as_str),
    })
}

fn list_payload(snapshots: &[WatcherSnapshot]) -> Value {
    serde_json::json!({
        "watchers": snapshots.iter().map(snapshot_payload).collect::<Vec<_>>()
    })
}

fn completion_payload(state: &WatcherBridgeCompletionState) -> Value {
    serde_json::json!({
        "admission": state.admission().as_str(),
        "active_or_unjoined": state.active_or_unjoined().iter().map(snapshot_payload).collect::<Vec<_>>(),
        "allows_successful_completion": state.allows_successful_completion(),
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

fn wait_label(representation: swallowtail_runtime::WatcherWaitRepresentation) -> &'static str {
    match representation {
        swallowtail_runtime::WatcherWaitRepresentation::Pending => "pending",
        swallowtail_runtime::WatcherWaitRepresentation::TerminalUnjoined(_) => "terminal_unjoined",
        swallowtail_runtime::WatcherWaitRepresentation::Satisfied(_) => "satisfied",
        swallowtail_runtime::WatcherWaitRepresentation::Cancelled => "cancelled",
        swallowtail_runtime::WatcherWaitRepresentation::DeadlineExceeded => "deadline_exceeded",
    }
}

fn jsonrpc_result(id: Value, result: Value) -> String {
    serde_json::json!({
        "jsonrpc": WATCHER_BRIDGE_JSONRPC_VERSION,
        "id": id,
        "result": result
    })
    .to_string()
}

pub(super) fn error_http_status(error: &RuntimeFailure) -> (u16, &'static str, i64) {
    match error.diagnostic().code() {
        "swallowtail.watcher_bridge.unauthorized" => (401, "Unauthorized", -32001),
        "swallowtail.watcher_bridge.oversized" => (413, "Payload Too Large", -32600),
        "swallowtail.watcher_bridge.unknown" => (200, "OK", -32601),
        "swallowtail.watcher_bridge.malformed" => (400, "Bad Request", -32600),
        "swallowtail.watcher_bridge.duplicate_correlation" => (200, "OK", -32002),
        "swallowtail.watcher_bridge.admission_frozen" => (200, "OK", -32003),
        "swallowtail.watcher_bridge.closed" => (200, "OK", -32004),
        "swallowtail.watcher_bridge.busy" => (429, "Too Many Requests", -32005),
        _ => (200, "OK", -32000),
    }
}

pub(super) fn error_message(error: &RuntimeFailure) -> &str {
    error.diagnostic().message()
}

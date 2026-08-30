use super::super::failure::{malformed_failure, unknown_failure};
use serde_json::{Map, Value};
use swallowtail_runtime::{
    RuntimeFailure, WATCHER_BRIDGE_INITIALIZE_METHOD, WATCHER_BRIDGE_INITIALIZED_NOTIFICATION,
    WATCHER_BRIDGE_JSONRPC_VERSION, WATCHER_BRIDGE_MCP_PROTOCOL_VERSION,
    WATCHER_BRIDGE_RESERVED_TOOLS, WATCHER_BRIDGE_TOOLS_CALL_METHOD,
    WATCHER_BRIDGE_TOOLS_LIST_METHOD,
};

pub(crate) enum DecodedRequest {
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

pub(crate) fn decode_request(body: &[u8]) -> Result<DecodedRequest, RuntimeFailure> {
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
            let id = required_request_id(object)?;
            require_initialize_params(object.get("params"))?;
            Ok(DecodedRequest::Initialize { id })
        }
        WATCHER_BRIDGE_TOOLS_LIST_METHOD => {
            let id = required_request_id(object)?;
            require_empty_params(object.get("params"))?;
            Ok(DecodedRequest::ToolsList { id })
        }
        WATCHER_BRIDGE_TOOLS_CALL_METHOD => {
            let id = required_request_id(object)?;
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

pub(crate) fn correlation_id(id: &Value) -> Result<String, RuntimeFailure> {
    match id {
        Value::String(value) if !value.is_empty() => Ok(value.clone()),
        Value::Number(value) => Ok(value.to_string()),
        _ => Err(malformed_failure()),
    }
}

pub(crate) fn decoded_request_id(request: &DecodedRequest) -> Option<Value> {
    match request {
        DecodedRequest::Initialized => None,
        DecodedRequest::Initialize { id }
        | DecodedRequest::ToolsList { id }
        | DecodedRequest::ToolsCall { id, .. } => Some(id.clone()),
    }
}

pub(crate) fn recoverable_request_id(body: &[u8]) -> Option<Value> {
    let value: Value = serde_json::from_slice(body).ok()?;
    let id = value.get("id")?.clone();
    correlation_id(&id).ok()?;
    Some(id)
}

fn required_request_id(object: &Map<String, Value>) -> Result<Value, RuntimeFailure> {
    let id = object.get("id").cloned().ok_or_else(malformed_failure)?;
    let _ = correlation_id(&id)?;
    Ok(id)
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

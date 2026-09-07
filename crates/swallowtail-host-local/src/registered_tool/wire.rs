//! Swallowtail-owned mediated-stdio wire specification.
//!
//! This module depends only on the portable runtime vocabulary and `serde_json`.
//! It deliberately does not depend on the local listener, lease, process, or
//! materialization implementations, so a future courier crate can reuse it
//! without changing the wire.

use serde_json::{Map, Value, json};
use std::fmt;
use swallowtail_runtime::{
    REGISTERED_TOOL_PROXY_WIRE_TAG as RUNTIME_REGISTERED_TOOL_PROXY_WIRE_TAG,
    RegisteredToolProtocolVersion, RegisteredToolTransport,
};

/// Exact fixed argv tag for the reference courier.
pub const REGISTERED_TOOL_PROXY_WIRE_TAG: &str = RUNTIME_REGISTERED_TOOL_PROXY_WIRE_TAG;

/// Exact JSON-RPC version carried by the reference courier.
pub const REGISTERED_TOOL_PROXY_JSONRPC_VERSION: &str = "2.0";
/// Exact MCP protocol version carried by the pinned SDK corpus.
pub const REGISTERED_TOOL_PROXY_MCP_PROTOCOL_VERSION: &str = "2025-11-25";
/// Reserved server name for this attachment.
pub const REGISTERED_TOOL_PROXY_SERVER_NAME: &str = "swallowtail-registered-tools";
/// Private loopback path used by the registered-tool HTTP profile.
pub const REGISTERED_TOOL_PROXY_HTTP_PATH: &str = "/mcp";
/// Maximum one JSON-RPC record on either wire.
pub const REGISTERED_TOOL_PROXY_MAX_RECORD_BYTES: usize = 256 * 1024;

/// One provider-facing MCP JSON-RPC request admitted by the courier.
#[derive(Clone, Eq, PartialEq)]
pub enum RegisteredToolProxyRequest {
    /// Provider handshake.
    Initialize {
        /// JSON-RPC correlation id.
        id: Value,
        /// Provider-advertised MCP protocol version.
        protocol_version: String,
    },
    /// Provider handshake notification.
    Initialized,
    /// Provider tool discovery.
    ToolsList {
        /// JSON-RPC correlation id.
        id: Value,
    },
    /// One provider tool call.
    ToolsCall {
        /// JSON-RPC correlation id.
        id: Value,
        /// Exact selected registered-tool name.
        name: String,
        /// JSON object passed to the host dispatcher.
        arguments: Map<String, Value>,
    },
}

impl fmt::Debug for RegisteredToolProxyRequest {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Initialize {
                id,
                protocol_version,
            } => formatter
                .debug_struct("Initialize")
                .field("id", id)
                .field("protocol_version", protocol_version)
                .finish(),
            Self::Initialized => formatter.write_str("Initialized"),
            Self::ToolsList { id } => formatter.debug_struct("ToolsList").field("id", id).finish(),
            Self::ToolsCall { id, name, .. } => formatter
                .debug_struct("ToolsCall")
                .field("id", id)
                .field("name", name)
                .field("arguments", &"<redacted>")
                .finish(),
        }
    }
}

/// Safe wire decode error. Payloads are intentionally not retained.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RegisteredToolProxyWireError {
    /// The record was not valid bounded JSON-RPC.
    Malformed,
    /// The request used a method or field outside this frozen wire.
    Unsupported,
    /// The request omitted a required correlation or parameter.
    MissingField,
}

impl fmt::Display for RegisteredToolProxyWireError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::Malformed => "malformed registered-tool proxy wire record",
            Self::Unsupported => "unsupported registered-tool proxy wire value",
            Self::MissingField => "missing registered-tool proxy wire field",
        })
    }
}

impl std::error::Error for RegisteredToolProxyWireError {}

/// Rendezvous contents read once by a courier.
#[derive(Clone, Eq, PartialEq)]
pub struct RegisteredToolProxyRendezvousDocument {
    /// Fixed courier wire tag.
    pub wire_tag: String,
    /// Host-private loopback endpoint.
    pub endpoint: String,
    /// Operation-private bearer.
    pub bearer: String,
    /// Shared operation lease generation.
    pub lease_generation: u64,
    /// Registered-tool transport generation.
    pub transport_generation: u64,
    /// Exact selected Contract 063 protocol version.
    pub protocol_version: String,
    /// Reserved server name.
    pub server_name: String,
}

impl fmt::Debug for RegisteredToolProxyRendezvousDocument {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("RegisteredToolProxyRendezvousDocument")
            .field("wire_tag", &self.wire_tag)
            .field("endpoint", &"<redacted>")
            .field("bearer", &"<redacted>")
            .field("lease_generation", &self.lease_generation)
            .field("transport_generation", &self.transport_generation)
            .field("protocol_version", &self.protocol_version)
            .field("server_name", &self.server_name)
            .finish()
    }
}

impl RegisteredToolProxyRendezvousDocument {
    /// Creates a rendezvous document from one exact opened binding.
    #[must_use]
    pub fn new(
        endpoint: String,
        bearer: String,
        lease_generation: u64,
        transport_generation: u64,
        protocol_version: &RegisteredToolProtocolVersion,
    ) -> Self {
        Self {
            wire_tag: REGISTERED_TOOL_PROXY_WIRE_TAG.to_owned(),
            endpoint,
            bearer,
            lease_generation,
            transport_generation,
            protocol_version: protocol_version.as_str().to_owned(),
            server_name: REGISTERED_TOOL_PROXY_SERVER_NAME.to_owned(),
        }
    }

    /// Encodes the exact bounded rendezvous file body.
    pub fn encode(&self) -> Result<Vec<u8>, RegisteredToolProxyWireError> {
        serde_json::to_vec(&json!({
            "wire_tag": self.wire_tag,
            "endpoint": self.endpoint,
            "bearer": self.bearer,
            "lease_generation": self.lease_generation,
            "transport_generation": self.transport_generation,
            "protocol_version": self.protocol_version,
            "server_name": self.server_name,
        }))
        .map_err(|_| RegisteredToolProxyWireError::Malformed)
    }

    /// Decodes and validates a rendezvous file body.
    pub fn decode(body: &[u8]) -> Result<Self, RegisteredToolProxyWireError> {
        if body.len() > REGISTERED_TOOL_PROXY_MAX_RECORD_BYTES {
            return Err(RegisteredToolProxyWireError::Malformed);
        }
        let object = serde_json::from_slice::<Value>(body)
            .map_err(|_| RegisteredToolProxyWireError::Malformed)?
            .as_object()
            .cloned()
            .ok_or(RegisteredToolProxyWireError::Malformed)?;
        let allowed = [
            "wire_tag",
            "endpoint",
            "bearer",
            "lease_generation",
            "transport_generation",
            "protocol_version",
            "server_name",
        ];
        if object.keys().any(|key| !allowed.contains(&key.as_str())) {
            return Err(RegisteredToolProxyWireError::Unsupported);
        }
        let text = |key: &str| {
            object
                .get(key)
                .and_then(Value::as_str)
                .filter(|value| !value.is_empty())
                .map(str::to_owned)
                .ok_or(RegisteredToolProxyWireError::MissingField)
        };
        let lease_generation = object
            .get("lease_generation")
            .and_then(Value::as_u64)
            .filter(|value| *value != 0)
            .ok_or(RegisteredToolProxyWireError::MissingField)?;
        let transport_generation = object
            .get("transport_generation")
            .and_then(Value::as_u64)
            .filter(|value| *value != 0)
            .ok_or(RegisteredToolProxyWireError::MissingField)?;
        let document = Self {
            wire_tag: text("wire_tag")?,
            endpoint: text("endpoint")?,
            bearer: text("bearer")?,
            lease_generation,
            transport_generation,
            protocol_version: text("protocol_version")?,
            server_name: text("server_name")?,
        };
        if document.wire_tag != REGISTERED_TOOL_PROXY_WIRE_TAG
            || document.server_name != REGISTERED_TOOL_PROXY_SERVER_NAME
        {
            return Err(RegisteredToolProxyWireError::Unsupported);
        }
        Ok(document)
    }
}

/// Decodes one newline-delimited provider record.
pub fn decode_request(
    record: &[u8],
) -> Result<RegisteredToolProxyRequest, RegisteredToolProxyWireError> {
    if record.is_empty() || record.len() > REGISTERED_TOOL_PROXY_MAX_RECORD_BYTES {
        return Err(RegisteredToolProxyWireError::Malformed);
    }
    let object = serde_json::from_slice::<Value>(record)
        .map_err(|_| RegisteredToolProxyWireError::Malformed)?
        .as_object()
        .cloned()
        .ok_or(RegisteredToolProxyWireError::Malformed)?;
    if object
        .keys()
        .any(|key| !matches!(key.as_str(), "jsonrpc" | "id" | "method" | "params"))
    {
        return Err(RegisteredToolProxyWireError::Unsupported);
    }
    if object.get("jsonrpc").and_then(Value::as_str) != Some(REGISTERED_TOOL_PROXY_JSONRPC_VERSION)
    {
        return Err(RegisteredToolProxyWireError::Unsupported);
    }
    let method = object
        .get("method")
        .and_then(Value::as_str)
        .ok_or(RegisteredToolProxyWireError::MissingField)?;
    match method {
        "initialize" => {
            let id = request_id(&object)?;
            let params = object
                .get("params")
                .and_then(Value::as_object)
                .ok_or(RegisteredToolProxyWireError::MissingField)?;
            let protocol_version = params
                .get("protocolVersion")
                .and_then(Value::as_str)
                .ok_or(RegisteredToolProxyWireError::MissingField)?;
            Ok(RegisteredToolProxyRequest::Initialize {
                id,
                protocol_version: protocol_version.to_owned(),
            })
        }
        "notifications/initialized" => {
            if object.contains_key("id") {
                return Err(RegisteredToolProxyWireError::Unsupported);
            }
            Ok(RegisteredToolProxyRequest::Initialized)
        }
        "tools/list" => Ok(RegisteredToolProxyRequest::ToolsList {
            id: request_id(&object)?,
        }),
        "tools/call" => {
            let id = request_id(&object)?;
            let params = object
                .get("params")
                .and_then(Value::as_object)
                .ok_or(RegisteredToolProxyWireError::MissingField)?;
            let name = params
                .get("name")
                .and_then(Value::as_str)
                .filter(|value| !value.is_empty())
                .ok_or(RegisteredToolProxyWireError::MissingField)?
                .to_owned();
            let arguments = match params.get("arguments") {
                None | Some(Value::Null) => Map::new(),
                Some(Value::Object(arguments)) => arguments.clone(),
                Some(_) => return Err(RegisteredToolProxyWireError::Malformed),
            };
            Ok(RegisteredToolProxyRequest::ToolsCall {
                id,
                name,
                arguments,
            })
        }
        _ => Err(RegisteredToolProxyWireError::Unsupported),
    }
}

/// Encodes one successful JSON-RPC response.
#[must_use]
pub fn result(id: Value, value: Value) -> Vec<u8> {
    serde_json::to_vec(&json!({
        "jsonrpc": REGISTERED_TOOL_PROXY_JSONRPC_VERSION,
        "id": id,
        "result": value,
    }))
    .unwrap_or_default()
}

/// Encodes one safe JSON-RPC error response.
#[must_use]
pub fn error(id: Option<Value>, code: i64, message: &'static str) -> Vec<u8> {
    serde_json::to_vec(&json!({
        "jsonrpc": REGISTERED_TOOL_PROXY_JSONRPC_VERSION,
        "id": id.unwrap_or(Value::Null),
        "error": { "code": code, "message": message },
    }))
    .unwrap_or_default()
}

/// Encodes the exact initialized response.
#[must_use]
pub fn initialize_result() -> Value {
    json!({
        "protocolVersion": REGISTERED_TOOL_PROXY_MCP_PROTOCOL_VERSION,
        "capabilities": { "tools": { "listChanged": false } },
        "serverInfo": {
            "name": REGISTERED_TOOL_PROXY_SERVER_NAME,
            "version": REGISTERED_TOOL_PROXY_WIRE_TAG,
        },
    })
}

/// Encodes one tool-discovery result from bounded schema values.
#[must_use]
pub fn tools_list_result(tools: Vec<Value>) -> Value {
    json!({ "tools": tools })
}

/// Encodes one MCP tool result from the dispatcher body.
#[must_use]
pub fn tool_result(body: &[u8], is_error: bool) -> Value {
    let text = String::from_utf8_lossy(body).into_owned();
    json!({
        "content": [{ "type": "text", "text": text }],
        "isError": is_error,
    })
}

fn request_id(object: &Map<String, Value>) -> Result<Value, RegisteredToolProxyWireError> {
    match object.get("id") {
        Some(Value::String(value)) if !value.is_empty() => Ok(Value::String(value.clone())),
        Some(Value::Number(value)) => Ok(Value::Number(value.clone())),
        _ => Err(RegisteredToolProxyWireError::MissingField),
    }
}

/// Returns the fixed wire's transport identity for safe fixture assertions.
#[must_use]
pub const fn transport() -> RegisteredToolTransport {
    RegisteredToolTransport::PrivateLoopbackHttp
}

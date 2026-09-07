//! Bounded MCP JSON-RPC envelopes for the registered-tool carrier.
//!
//! Only the exact record shapes the frozen transcript in
//! `tests/fixtures/claude-agent-sdk-v1/registered-tool-mcp.jsonl` contains are
//! representable. Anything else — a wrong `jsonrpc` value, an oversized record,
//! an unknown method, a missing or repeated id — fails closed before it can
//! reach the Contract 063 kernel.
//!
//! Nothing here interprets tool arguments or results. Bodies stay opaque bytes
//! that only the kernel's bounded payload type carries.

use super::super::failure::failure;
use serde_json::{Map, Value, json};
use swallowtail_runtime::RuntimeFailure;

/// Exact JSON-RPC version every carrier record must carry.
pub const MCP_JSONRPC_VERSION: &str = "2.0";

/// Maximum bytes of one carrier record, matching the private wire bound.
pub const MAXIMUM_MCP_RECORD_BYTES: usize = 1024 * 1024;

/// JSON-RPC error code for a malformed request object.
pub const MCP_ERROR_INVALID_REQUEST: i64 = -32_600;
/// JSON-RPC error code for an unknown method.
pub const MCP_ERROR_METHOD_NOT_FOUND: i64 = -32_601;
/// JSON-RPC error code for invalid parameters, including an unknown tool.
pub const MCP_ERROR_INVALID_PARAMS: i64 = -32_602;
/// JSON-RPC error code for a carrier-internal failure.
pub const MCP_ERROR_INTERNAL: i64 = -32_603;

/// Exact carrier methods this route admits.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ClaudeAgentSdkMcpMethod {
    /// Version and capability handshake.
    Initialize,
    /// Post-handshake notification.
    Initialized,
    /// Bounded catalogue of the selected registered tools.
    ToolsList,
    /// One registered-tool call.
    ToolsCall,
    /// Client-initiated cancellation of one in-flight request.
    Cancelled,
}

impl ClaudeAgentSdkMcpMethod {
    /// Returns the exact wire spelling of this method.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Initialize => "initialize",
            Self::Initialized => "notifications/initialized",
            Self::ToolsList => "tools/list",
            Self::ToolsCall => "tools/call",
            Self::Cancelled => "notifications/cancelled",
        }
    }

    /// Reports whether this method is a notification, which carries no id.
    #[must_use]
    pub const fn is_notification(self) -> bool {
        matches!(self, Self::Initialized | Self::Cancelled)
    }

    fn parse(value: &str) -> Option<Self> {
        [
            Self::Initialize,
            Self::Initialized,
            Self::ToolsList,
            Self::ToolsCall,
            Self::Cancelled,
        ]
        .into_iter()
        .find(|method| method.as_str() == value)
    }
}

/// Bounded JSON-RPC request identity.
///
/// The MCP schema admits a string or a number. Both are preserved exactly so a
/// response can echo the caller's own spelling rather than a normalized one.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ClaudeAgentSdkMcpRequestId {
    /// Numeric request identity.
    Number(i64),
    /// Bounded textual request identity.
    Text(String),
}

impl ClaudeAgentSdkMcpRequestId {
    /// Returns the exact JSON value this identity was decoded from.
    #[must_use]
    pub fn to_value(&self) -> Value {
        match self {
            Self::Number(value) => json!(value),
            Self::Text(value) => json!(value),
        }
    }

    /// Returns a bounded stable rendering for correlation identities.
    #[must_use]
    pub fn as_correlation_text(&self) -> String {
        match self {
            Self::Number(value) => format!("n{value}"),
            Self::Text(value) => format!("s{value}"),
        }
    }

    fn decode(value: &Value) -> Option<Self> {
        match value {
            Value::Number(number) => number.as_i64().map(Self::Number),
            Value::String(text) if !text.is_empty() && text.len() <= 128 => {
                Some(Self::Text(text.clone()))
            }
            _ => None,
        }
    }
}

/// One decoded, bounded carrier request or notification.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ClaudeAgentSdkMcpRequest {
    id: Option<ClaudeAgentSdkMcpRequestId>,
    method: ClaudeAgentSdkMcpMethod,
    params: Map<String, Value>,
}

impl ClaudeAgentSdkMcpRequest {
    /// Decodes one bounded carrier record.
    ///
    /// A record that is oversized, not JSON, not a JSON-RPC 2.0 object, carries
    /// an unknown method, or mismatches the notification/request id rule is
    /// rejected here rather than anywhere closer to the kernel.
    pub fn decode(record: &[u8]) -> Result<Self, RuntimeFailure> {
        if record.is_empty() || record.len() > MAXIMUM_MCP_RECORD_BYTES {
            return Err(wire_failure("record was outside its byte bound"));
        }
        let value: Value =
            serde_json::from_slice(record).map_err(|_| wire_failure("record was not JSON"))?;
        let object = value
            .as_object()
            .ok_or_else(|| wire_failure("record was not a JSON-RPC object"))?;
        if object.get("jsonrpc").and_then(Value::as_str) != Some(MCP_JSONRPC_VERSION) {
            return Err(wire_failure("record did not carry JSON-RPC 2.0"));
        }
        let method = object
            .get("method")
            .and_then(Value::as_str)
            .and_then(ClaudeAgentSdkMcpMethod::parse)
            .ok_or_else(|| wire_failure("method was outside the admitted set"))?;
        let id = match object.get("id") {
            None | Some(Value::Null) => None,
            Some(value) => Some(
                ClaudeAgentSdkMcpRequestId::decode(value)
                    .ok_or_else(|| wire_failure("request id was invalid"))?,
            ),
        };
        if method.is_notification() != id.is_none() {
            return Err(wire_failure("request id did not match the method shape"));
        }
        let params = match object.get("params") {
            None | Some(Value::Null) => Map::new(),
            Some(Value::Object(params)) => params.clone(),
            Some(_) => return Err(wire_failure("params were not an object")),
        };
        Ok(Self { id, method, params })
    }

    /// Returns the request identity, absent for a notification.
    #[must_use]
    pub const fn id(&self) -> Option<&ClaudeAgentSdkMcpRequestId> {
        self.id.as_ref()
    }

    /// Returns the admitted method.
    #[must_use]
    pub const fn method(&self) -> ClaudeAgentSdkMcpMethod {
        self.method
    }

    /// Returns the decoded params object.
    #[must_use]
    pub const fn params(&self) -> &Map<String, Value> {
        &self.params
    }

    /// Returns one bounded string parameter.
    #[must_use]
    pub fn string_param(&self, name: &str) -> Option<&str> {
        self.params.get(name).and_then(Value::as_str)
    }
}

/// Builds one JSON-RPC success response for an exact request identity.
#[must_use]
pub fn mcp_result(id: &ClaudeAgentSdkMcpRequestId, result: Value) -> Value {
    json!({"jsonrpc": MCP_JSONRPC_VERSION, "id": id.to_value(), "result": result})
}

/// Builds one JSON-RPC error response for an exact request identity.
///
/// The message is a bounded safe reason code. Tool arguments, results, host
/// paths, endpoints, and credential material never appear in it.
#[must_use]
pub fn mcp_error(id: &ClaudeAgentSdkMcpRequestId, code: i64, safe_code: &str) -> Value {
    json!({
        "jsonrpc": MCP_JSONRPC_VERSION,
        "id": id.to_value(),
        "error": {"code": code, "message": safe_code},
    })
}

pub(super) fn wire_failure(detail: &str) -> RuntimeFailure {
    failure(
        "swallowtail.claude-agent.sdk.registered_tool.record_invalid",
        format!("Claude Agent SDK registered-tool carrier {detail}"),
    )
}

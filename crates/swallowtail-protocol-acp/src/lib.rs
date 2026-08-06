//! Provider-neutral Agent Client Protocol v1 transport support.

#![forbid(unsafe_code)]
#![deny(missing_docs)]

use serde_json::{Value, json};
use std::error::Error;
use std::fmt;

mod activity;
mod message;
mod session_list;

pub use activity::{
    AcpBoundedText, AcpCommand, AcpConfigCategory, AcpConfigChoice, AcpConfigChoices,
    AcpConfigGroup, AcpConfigKind, AcpConfigOption, AcpContentBlock, AcpCost, AcpMessageChunk,
    AcpMessageRole, AcpOptionalUpdate, AcpPlanEntry, AcpPlanEntryPriority, AcpPlanEntryStatus,
    AcpSessionUpdate, AcpSessionUpdateSemantics, AcpToolCall, AcpToolCallContent,
    AcpToolCallLocation, AcpToolCallStatus, AcpToolCallUpdate, AcpToolKind, AcpUsage,
    ActivityDecodeError, ActivityDecodeErrorKind, ActivityDecodeLimits, DecodedSessionUpdate,
    decode_session_update, decode_session_update_with_limits,
};
pub use message::{decode_message, encode_message};
pub use session_list::{
    ACP_SESSION_LIST_METHOD, AcpOpaqueExtensions, AcpSessionInfo, AcpSessionListCapabilities,
    AcpSessionListDecodeError, AcpSessionListDecodeErrorKind, AcpSessionListLimits,
    AcpSessionListPage, AcpSessionListRequest, decode_session_list_capabilities,
};

/// Stable ACP wire protocol version supported by this transport.
pub const ACP_PROTOCOL_VERSION: u64 = 1;
/// Default maximum bytes accepted for one newline-delimited JSON frame.
pub const DEFAULT_MAX_FRAME_BYTES: usize = 64 * 1024;
/// Default maximum bytes retained across incomplete input frames.
pub const DEFAULT_MAX_BUFFER_BYTES: usize = 256 * 1024;

/// Returns whether an ACP session-update kind carries session-scoped metadata.
#[must_use]
pub fn is_session_scoped_metadata_update_kind(kind: &str) -> bool {
    matches!(
        kind,
        "available_commands_update" | "config_option_update" | "current_mode_update"
    )
}

/// Returns whether ACP session-update parameters carry session-scoped metadata.
#[must_use]
pub fn is_session_scoped_metadata_update(params: &Value) -> bool {
    params
        .get("update")
        .and_then(|update| update.get("sessionUpdate"))
        .and_then(Value::as_str)
        .is_some_and(is_session_scoped_metadata_update_kind)
}

/// Independent frame and accumulated-buffer limits for ACP NDJSON input.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FramingLimits {
    maximum_frame_bytes: usize,
    maximum_buffer_bytes: usize,
}

impl FramingLimits {
    /// Creates explicit per-frame and accumulated-buffer limits.
    #[must_use]
    pub const fn new(maximum_frame_bytes: usize, maximum_buffer_bytes: usize) -> Self {
        Self {
            maximum_frame_bytes,
            maximum_buffer_bytes,
        }
    }

    /// Returns the maximum bytes accepted before one newline delimiter.
    #[must_use]
    pub const fn maximum_frame_bytes(self) -> usize {
        self.maximum_frame_bytes
    }

    /// Returns the maximum bytes retained across incomplete frames.
    #[must_use]
    pub const fn maximum_buffer_bytes(self) -> usize {
        self.maximum_buffer_bytes
    }
}

impl Default for FramingLimits {
    fn default() -> Self {
        Self::new(DEFAULT_MAX_FRAME_BYTES, DEFAULT_MAX_BUFFER_BYTES)
    }
}

/// Stable classification of an ACP framing or JSON-RPC codec failure.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProtocolErrorKind {
    /// Accumulated undecoded input exceeded the configured buffer bound.
    BufferLimitExceeded,
    /// One complete or partial frame exceeded the configured frame bound.
    FrameLimitExceeded,
    /// Input ended before the final frame delimiter.
    IncompleteFrame,
    /// A frame was not valid JSON.
    InvalidJson,
    /// JSON did not match an admitted JSON-RPC message shape.
    InvalidMessage,
    /// The JSON-RPC version was absent or unsupported.
    InvalidVersion,
    /// A validated message could not be serialized.
    SerializationFailed,
}

/// Bounded ACP protocol error without provider payload content.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ProtocolError {
    kind: ProtocolErrorKind,
}

impl ProtocolError {
    const fn new(kind: ProtocolErrorKind) -> Self {
        Self { kind }
    }

    /// Returns the stable protocol failure classification.
    #[must_use]
    pub const fn kind(self) -> ProtocolErrorKind {
        self.kind
    }
}

impl fmt::Display for ProtocolError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self.kind {
            ProtocolErrorKind::BufferLimitExceeded => "ACP input buffer limit exceeded",
            ProtocolErrorKind::FrameLimitExceeded => "ACP frame limit exceeded",
            ProtocolErrorKind::IncompleteFrame => "ACP input ended with an incomplete frame",
            ProtocolErrorKind::InvalidJson => "ACP frame is not valid JSON",
            ProtocolErrorKind::InvalidMessage => "ACP JSON-RPC message shape is invalid",
            ProtocolErrorKind::InvalidVersion => "ACP JSON-RPC version is invalid",
            ProtocolErrorKind::SerializationFailed => "ACP message could not be serialized",
        })
    }
}

impl Error for ProtocolError {}

/// One decoded ACP JSON-RPC message.
#[derive(Clone, Debug, PartialEq)]
pub enum Message {
    /// A request with a scalar correlation identity.
    Request {
        /// Provider- or client-supplied string or integer request identity.
        id: Value,
        /// Non-empty JSON-RPC method name.
        method: String,
        /// Request parameters, or JSON `null` when omitted.
        params: Value,
    },
    /// A notification without a response identity.
    Notification {
        /// Non-empty JSON-RPC method name.
        method: String,
        /// Notification parameters, or JSON `null` when omitted.
        params: Value,
    },
    /// A successful or failed response correlated to an exact request identity.
    Response {
        /// Provider- or client-supplied string or integer request identity.
        id: Value,
        /// Result value or decoded JSON-RPC error.
        result: Result<Value, RpcError>,
    },
}

/// JSON-RPC error code and provider message; message text is debug-redacted.
#[derive(Clone, PartialEq)]
pub struct RpcError {
    code: i64,
    message: String,
}

impl RpcError {
    /// Returns the JSON-RPC error code.
    #[must_use]
    pub const fn code(&self) -> i64 {
        self.code
    }

    /// Returns the provider-originated error message.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }
}

impl fmt::Debug for RpcError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("RpcError")
            .field("code", &self.code)
            .field("message", &"<redacted>")
            .finish()
    }
}

/// Incremental bounded decoder for newline-delimited ACP JSON-RPC messages.
pub struct NdjsonDecoder {
    limits: FramingLimits,
    pending: Vec<u8>,
}

impl NdjsonDecoder {
    /// Creates an empty decoder with explicit framing limits.
    #[must_use]
    pub fn new(limits: FramingLimits) -> Self {
        Self {
            limits,
            pending: Vec::new(),
        }
    }

    /// Adds input bytes and returns every complete message now available.
    pub fn push(&mut self, input: &[u8]) -> Result<Vec<Message>, ProtocolError> {
        if self.pending.len().saturating_add(input.len()) > self.limits.maximum_buffer_bytes {
            return Err(ProtocolError::new(ProtocolErrorKind::BufferLimitExceeded));
        }
        self.pending.extend_from_slice(input);
        let mut messages = Vec::new();
        while let Some(position) = self.pending.iter().position(|byte| *byte == b'\n') {
            if position > self.limits.maximum_frame_bytes {
                return Err(ProtocolError::new(ProtocolErrorKind::FrameLimitExceeded));
            }
            let frame: Vec<_> = self.pending.drain(..=position).collect();
            messages.push(decode_frame(&frame[..position])?);
        }
        if self.pending.len() > self.limits.maximum_frame_bytes {
            return Err(ProtocolError::new(ProtocolErrorKind::FrameLimitExceeded));
        }
        Ok(messages)
    }

    /// Verifies that input ended after a complete newline-delimited frame.
    pub fn finish(self) -> Result<(), ProtocolError> {
        if self.pending.is_empty() {
            Ok(())
        } else {
            Err(ProtocolError::new(ProtocolErrorKind::IncompleteFrame))
        }
    }
}

impl Default for NdjsonDecoder {
    fn default() -> Self {
        Self::new(FramingLimits::default())
    }
}

/// Encodes a numeric-ID JSON-RPC request as one bounded NDJSON frame.
pub fn encode_request(id: u64, method: &str, params: Value) -> Result<Vec<u8>, ProtocolError> {
    encode(json!({"jsonrpc": "2.0", "id": id, "method": method, "params": params}))
}

/// Encodes a JSON-RPC notification as one bounded NDJSON frame.
pub fn encode_notification(method: &str, params: Value) -> Result<Vec<u8>, ProtocolError> {
    encode(json!({"jsonrpc": "2.0", "method": method, "params": params}))
}

/// Encodes a successful response for an exact string or integer request ID.
pub fn encode_result(id: Value, result: Value) -> Result<Vec<u8>, ProtocolError> {
    encode(json!({"jsonrpc": "2.0", "id": id, "result": result}))
}

/// Encodes a safe static JSON-RPC error for an exact request ID.
pub fn encode_error(id: Value, code: i64, message: &'static str) -> Result<Vec<u8>, ProtocolError> {
    encode(json!({
        "jsonrpc": "2.0",
        "id": id,
        "error": {"code": code, "message": message}
    }))
}

pub(crate) fn encode(value: Value) -> Result<Vec<u8>, ProtocolError> {
    let mut bytes = serde_json::to_vec(&value)
        .map_err(|_| ProtocolError::new(ProtocolErrorKind::SerializationFailed))?;
    if bytes.len() > DEFAULT_MAX_FRAME_BYTES {
        return Err(ProtocolError::new(ProtocolErrorKind::FrameLimitExceeded));
    }
    bytes.push(b'\n');
    Ok(bytes)
}

pub(crate) fn decode_frame(frame: &[u8]) -> Result<Message, ProtocolError> {
    if frame.is_empty() {
        return Err(ProtocolError::new(ProtocolErrorKind::InvalidMessage));
    }
    let value: Value = serde_json::from_slice(frame)
        .map_err(|_| ProtocolError::new(ProtocolErrorKind::InvalidJson))?;
    if value.get("jsonrpc").and_then(Value::as_str) != Some("2.0") {
        return Err(ProtocolError::new(ProtocolErrorKind::InvalidVersion));
    }
    let id = value.get("id").cloned();
    if let Some(method) = value.get("method").and_then(Value::as_str) {
        if method.is_empty() {
            return Err(ProtocolError::new(ProtocolErrorKind::InvalidMessage));
        }
        let params = value.get("params").cloned().unwrap_or(Value::Null);
        return match id {
            Some(id) if valid_id(&id) => Ok(Message::Request {
                id,
                method: method.to_owned(),
                params,
            }),
            None => Ok(Message::Notification {
                method: method.to_owned(),
                params,
            }),
            _ => Err(ProtocolError::new(ProtocolErrorKind::InvalidMessage)),
        };
    }
    let id = id
        .filter(valid_id)
        .ok_or_else(|| ProtocolError::new(ProtocolErrorKind::InvalidMessage))?;
    match (value.get("result"), value.get("error")) {
        (Some(result), None) => Ok(Message::Response {
            id,
            result: Ok(result.clone()),
        }),
        (None, Some(error)) => {
            let code = error
                .get("code")
                .and_then(Value::as_i64)
                .ok_or_else(|| ProtocolError::new(ProtocolErrorKind::InvalidMessage))?;
            let message = error
                .get("message")
                .and_then(Value::as_str)
                .ok_or_else(|| ProtocolError::new(ProtocolErrorKind::InvalidMessage))?;
            Ok(Message::Response {
                id,
                result: Err(RpcError {
                    code,
                    message: message.to_owned(),
                }),
            })
        }
        _ => Err(ProtocolError::new(ProtocolErrorKind::InvalidMessage)),
    }
}

fn valid_id(id: &Value) -> bool {
    id.is_string() || id.is_u64() || id.is_i64()
}

#[cfg(test)]
mod tests;

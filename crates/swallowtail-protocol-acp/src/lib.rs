//! Provider-neutral Agent Client Protocol v1 transport support.

#![forbid(unsafe_code)]

use serde_json::{Value, json};
use std::error::Error;
use std::fmt;

/// Stable ACP wire protocol version supported by this transport.
pub const ACP_PROTOCOL_VERSION: u64 = 1;
pub const DEFAULT_MAX_FRAME_BYTES: usize = 64 * 1024;
pub const DEFAULT_MAX_BUFFER_BYTES: usize = 256 * 1024;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FramingLimits {
    maximum_frame_bytes: usize,
    maximum_buffer_bytes: usize,
}

impl FramingLimits {
    #[must_use]
    pub const fn new(maximum_frame_bytes: usize, maximum_buffer_bytes: usize) -> Self {
        Self {
            maximum_frame_bytes,
            maximum_buffer_bytes,
        }
    }

    #[must_use]
    pub const fn maximum_frame_bytes(self) -> usize {
        self.maximum_frame_bytes
    }

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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProtocolErrorKind {
    BufferLimitExceeded,
    FrameLimitExceeded,
    IncompleteFrame,
    InvalidJson,
    InvalidMessage,
    InvalidVersion,
    SerializationFailed,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ProtocolError {
    kind: ProtocolErrorKind,
}

impl ProtocolError {
    const fn new(kind: ProtocolErrorKind) -> Self {
        Self { kind }
    }

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

#[derive(Clone, Debug, PartialEq)]
pub enum Message {
    Request {
        id: Value,
        method: String,
        params: Value,
    },
    Notification {
        method: String,
        params: Value,
    },
    Response {
        id: Value,
        result: Result<Value, RpcError>,
    },
}

#[derive(Clone, PartialEq)]
pub struct RpcError {
    code: i64,
    message: String,
}

impl RpcError {
    #[must_use]
    pub const fn code(&self) -> i64 {
        self.code
    }

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

pub struct NdjsonDecoder {
    limits: FramingLimits,
    pending: Vec<u8>,
}

impl NdjsonDecoder {
    #[must_use]
    pub fn new(limits: FramingLimits) -> Self {
        Self {
            limits,
            pending: Vec::new(),
        }
    }

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

pub fn encode_request(id: u64, method: &str, params: Value) -> Result<Vec<u8>, ProtocolError> {
    encode(json!({"jsonrpc": "2.0", "id": id, "method": method, "params": params}))
}

pub fn encode_notification(method: &str, params: Value) -> Result<Vec<u8>, ProtocolError> {
    encode(json!({"jsonrpc": "2.0", "method": method, "params": params}))
}

pub fn encode_result(id: Value, result: Value) -> Result<Vec<u8>, ProtocolError> {
    encode(json!({"jsonrpc": "2.0", "id": id, "result": result}))
}

pub fn encode_error(id: Value, code: i64, message: &'static str) -> Result<Vec<u8>, ProtocolError> {
    encode(json!({
        "jsonrpc": "2.0",
        "id": id,
        "error": {"code": code, "message": message}
    }))
}

fn encode(value: Value) -> Result<Vec<u8>, ProtocolError> {
    let mut bytes = serde_json::to_vec(&value)
        .map_err(|_| ProtocolError::new(ProtocolErrorKind::SerializationFailed))?;
    if bytes.len() > DEFAULT_MAX_FRAME_BYTES {
        return Err(ProtocolError::new(ProtocolErrorKind::FrameLimitExceeded));
    }
    bytes.push(b'\n');
    Ok(bytes)
}

fn decode_frame(frame: &[u8]) -> Result<Message, ProtocolError> {
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

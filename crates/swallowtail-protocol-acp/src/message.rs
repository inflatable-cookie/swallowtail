use super::{
    FramingLimits, Message, ProtocolError, ProtocolErrorKind, decode_frame, encode_with_limit,
};
use serde_json::json;

/// Encodes one typed ACP message as a bounded NDJSON frame using default limits.
pub fn encode_message(message: &Message) -> Result<Vec<u8>, ProtocolError> {
    encode_message_with_limits(message, FramingLimits::default())
}

/// Encodes one typed ACP message as an NDJSON frame bounded by explicit limits.
pub fn encode_message_with_limits(
    message: &Message,
    limits: FramingLimits,
) -> Result<Vec<u8>, ProtocolError> {
    let value = match message {
        Message::Request { id, method, params } => {
            json!({"jsonrpc": "2.0", "id": id, "method": method, "params": params})
        }
        Message::Notification { method, params } => {
            json!({"jsonrpc": "2.0", "method": method, "params": params})
        }
        Message::Response { id, result } => match result {
            Ok(result) => json!({"jsonrpc": "2.0", "id": id, "result": result}),
            Err(error) => json!({
                "jsonrpc": "2.0",
                "id": id,
                "error": {"code": error.code(), "message": error.message()}
            }),
        },
    };
    encode_with_limit(value, limits.maximum_frame_bytes())
}

/// Decodes one complete ACP JSON-RPC frame without a newline delimiter using default limits.
pub fn decode_message(frame: &[u8]) -> Result<Message, ProtocolError> {
    decode_message_with_limits(frame, FramingLimits::default())
}

/// Decodes one complete ACP JSON-RPC frame without a newline delimiter under explicit limits.
pub fn decode_message_with_limits(
    frame: &[u8],
    limits: FramingLimits,
) -> Result<Message, ProtocolError> {
    if frame.len() > limits.maximum_frame_bytes() {
        return Err(ProtocolError::new(ProtocolErrorKind::FrameLimitExceeded));
    }
    decode_frame(frame)
}

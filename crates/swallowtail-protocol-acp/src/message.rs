use super::{
    DEFAULT_MAX_FRAME_BYTES, Message, ProtocolError, ProtocolErrorKind, decode_frame, encode,
};
use serde_json::json;

pub fn encode_message(message: &Message) -> Result<Vec<u8>, ProtocolError> {
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
    encode(value)
}

pub fn decode_message(frame: &[u8]) -> Result<Message, ProtocolError> {
    if frame.len() > DEFAULT_MAX_FRAME_BYTES {
        return Err(ProtocolError::new(ProtocolErrorKind::FrameLimitExceeded));
    }
    decode_frame(frame)
}

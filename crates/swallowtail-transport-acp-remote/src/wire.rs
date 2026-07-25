use crate::error::{RemoteAcpError, capacity_error, protocol_error};
use agent_client_protocol::TransportFrame;
use swallowtail_protocol_acp::{Message, decode_message, encode_message};

pub(crate) fn encode(
    message: &Message,
    maximum_frame_bytes: usize,
) -> Result<String, RemoteAcpError> {
    let mut encoded = encode_message(message).map_err(|_| protocol_error())?;
    encoded.pop();
    if encoded.len() > maximum_frame_bytes {
        return Err(capacity_error());
    }
    let text = String::from_utf8(encoded).map_err(|_| protocol_error())?;
    match TransportFrame::parse_json(&text) {
        TransportFrame::Single(frame) => TransportFrame::Single(frame)
            .to_json()
            .map_err(|_| protocol_error()),
        TransportFrame::Malformed { .. } | TransportFrame::Batch(_) => Err(protocol_error()),
    }
}

pub(crate) fn decode(text: &str, maximum_frame_bytes: usize) -> Result<Message, RemoteAcpError> {
    if text.len() > maximum_frame_bytes {
        return Err(capacity_error());
    }
    let normalized = match TransportFrame::parse_json(text) {
        TransportFrame::Single(frame) => TransportFrame::Single(frame)
            .to_json()
            .map_err(|_| protocol_error())?,
        TransportFrame::Malformed { .. } | TransportFrame::Batch(_) => {
            return Err(protocol_error());
        }
    };
    decode_message(normalized.as_bytes()).map_err(|_| protocol_error())
}

#[cfg(test)]
mod tests {
    use super::{decode, encode};
    use serde_json::json;
    use swallowtail_protocol_acp::Message;

    #[test]
    fn exact_sdk_schema_is_private_and_rejects_batches() {
        let message = Message::Request {
            id: json!(1),
            method: "initialize".to_owned(),
            params: json!({"protocolVersion": 1}),
        };
        let encoded = encode(&message, 1024).expect("message encodes");
        assert_eq!(decode(&encoded, 1024).expect("message decodes"), message);
        assert!(decode("[{\"jsonrpc\":\"2.0\",\"id\":1,\"result\":{}}]", 1024).is_err());
    }
}

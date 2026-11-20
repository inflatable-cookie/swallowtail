use crate::error::{RemoteAcpError, capacity_error, protocol_error};
use agent_client_protocol::TransportFrame;
use swallowtail_protocol_acp::{
    FramingLimits, Message, ProtocolErrorKind, decode_message_with_limits,
    encode_message_with_limits,
};

pub(crate) fn encode(
    message: &Message,
    maximum_frame_bytes: usize,
) -> Result<String, RemoteAcpError> {
    let limits = FramingLimits::new(maximum_frame_bytes, maximum_frame_bytes);
    let mut encoded = encode_message_with_limits(message, limits).map_err(|error| {
        if error.kind() == ProtocolErrorKind::FrameLimitExceeded {
            capacity_error()
        } else {
            protocol_error()
        }
    })?;
    encoded.pop();
    if encoded.len() > maximum_frame_bytes {
        return Err(capacity_error());
    }
    let text = String::from_utf8(encoded).map_err(|_| protocol_error())?;
    let normalized = match TransportFrame::parse_json(&text) {
        TransportFrame::Single(frame) => TransportFrame::Single(frame)
            .to_json()
            .map_err(|_| protocol_error())?,
        TransportFrame::Malformed { .. } | TransportFrame::Batch(_) => {
            return Err(protocol_error());
        }
    };
    if normalized.len() > maximum_frame_bytes {
        return Err(capacity_error());
    }
    Ok(normalized)
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
    if normalized.len() > maximum_frame_bytes {
        return Err(capacity_error());
    }
    let limits = FramingLimits::new(maximum_frame_bytes, maximum_frame_bytes);
    decode_message_with_limits(normalized.as_bytes(), limits).map_err(|error| {
        if error.kind() == ProtocolErrorKind::FrameLimitExceeded {
            capacity_error()
        } else {
            protocol_error()
        }
    })
}

#[cfg(test)]
mod tests {
    use super::{decode, encode};
    use crate::RemoteAcpErrorKind;
    use serde_json::json;
    use swallowtail_protocol_acp::{DEFAULT_MAX_FRAME_BYTES, Message};

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

    #[test]
    fn honors_configured_frame_bounds_above_64kib_and_classifies_oversize_as_capacity_exceeded() {
        let large = Message::Notification {
            method: "session/update".to_owned(),
            params: json!({"payload": "x".repeat(DEFAULT_MAX_FRAME_BYTES + 1024)}),
        };
        let limit = DEFAULT_MAX_FRAME_BYTES * 2;
        let encoded = encode(&large, limit).expect("configured bound above 64 KiB encodes");
        assert!(encoded.len() > DEFAULT_MAX_FRAME_BYTES);
        assert_eq!(
            decode(&encoded, limit).expect("configured bound above 64 KiB decodes"),
            large
        );

        let too_small = encoded.len() - 1;
        assert_eq!(
            encode(&large, too_small)
                .expect_err("oversized outbound frame fails")
                .kind(),
            RemoteAcpErrorKind::CapacityExceeded
        );
        assert_eq!(
            decode(&encoded, too_small)
                .expect_err("oversized inbound frame fails")
                .kind(),
            RemoteAcpErrorKind::CapacityExceeded
        );
    }
}

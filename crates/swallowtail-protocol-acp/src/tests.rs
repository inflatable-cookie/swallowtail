use super::{
    AcpSessionUpdate, DEFAULT_MAX_BUFFER_BYTES, DEFAULT_MAX_FRAME_BYTES, FramingLimits,
    MeasureJsonError, Message, NdjsonDecoder, ProtocolErrorKind, decode_message,
    decode_message_with_limits, decode_session_update, encode_message, encode_message_with_limits,
    encode_request, is_session_scoped_metadata_update, is_session_scoped_metadata_update_kind,
    measure_json_bytes,
};
use serde_json::json;

#[test]
fn decoder_correlates_split_response_frames() {
    let mut decoder = NdjsonDecoder::default();
    assert!(
        decoder
            .push(br#"{"jsonrpc":"2.0","id":7,"res"#)
            .unwrap()
            .is_empty()
    );
    let messages = decoder
        .push(b"ult\":{\"ok\":true}}\n")
        .expect("valid frame");
    assert_eq!(
        messages,
        [Message::Response {
            id: json!(7),
            result: Ok(json!({"ok": true})),
        }]
    );
    decoder.finish().expect("complete input");
}

#[test]
fn decoder_rejects_incomplete_and_oversized_frames() {
    let mut decoder = NdjsonDecoder::default();
    decoder.push(b"{").expect("partial frame is buffered");
    assert_eq!(
        decoder.finish().expect_err("partial frame fails").kind(),
        ProtocolErrorKind::IncompleteFrame
    );
    assert!(encode_request(1, "session/prompt", json!({"text": "x"})).is_ok());
}

#[test]
fn configured_decoder_accepts_a_frame_above_the_shared_default() {
    let mut frame = serde_json::to_vec(&json!({
        "jsonrpc": "2.0",
        "method": "session/update",
        "params": {"payload": "x".repeat(DEFAULT_MAX_FRAME_BYTES)}
    }))
    .expect("fixture frame serializes");
    frame.push(b'\n');

    let mut default_decoder = NdjsonDecoder::default();
    assert_eq!(
        default_decoder
            .push(&frame)
            .expect_err("shared default remains bounded")
            .kind(),
        ProtocolErrorKind::FrameLimitExceeded
    );

    let mut configured_decoder = NdjsonDecoder::new(FramingLimits::new(
        DEFAULT_MAX_BUFFER_BYTES,
        DEFAULT_MAX_BUFFER_BYTES,
    ));
    assert_eq!(
        configured_decoder
            .push(&frame)
            .expect("route-specific decoder accepts the frame")
            .len(),
        1
    );
    configured_decoder.finish().expect("complete input");
}

#[test]
fn session_scoped_metadata_updates_are_classified_without_flattening_unknown_kinds() {
    for kind in [
        "available_commands_update",
        "config_option_update",
        "current_mode_update",
    ] {
        assert!(is_session_scoped_metadata_update_kind(kind));
        assert!(is_session_scoped_metadata_update(&json!({
            "sessionId": "fixture-session",
            "update": {"sessionUpdate": kind}
        })));
    }

    assert!(!is_session_scoped_metadata_update_kind(
        "agent_message_chunk"
    ));
    assert!(!is_session_scoped_metadata_update(&json!({
        "update": {"sessionUpdate": "unknown_update"}
    })));
    assert!(!is_session_scoped_metadata_update(&json!({
        "update": {"sessionUpdate": 1}
    })));
    assert!(!is_session_scoped_metadata_update(&json!({})));
}

#[test]
fn complete_message_codec_preserves_protocol_shapes() {
    let messages = [
        Message::Request {
            id: json!(7),
            method: "session/prompt".to_owned(),
            params: json!({"sessionId": "session-1"}),
        },
        Message::Notification {
            method: "session/cancel".to_owned(),
            params: json!({"sessionId": "session-1"}),
        },
        Message::Response {
            id: json!("callback-1"),
            result: Ok(json!({"outcome": "cancelled"})),
        },
    ];
    for message in messages {
        let encoded = encode_message(&message).expect("message encodes");
        let decoded = decode_message(&encoded[..encoded.len() - 1]).expect("message decodes");
        assert_eq!(decoded, message);
    }
}

#[test]
fn message_codec_with_limits_supports_frames_above_shared_default() {
    let message = Message::Notification {
        method: "session/update".to_owned(),
        params: json!({"payload": "x".repeat(DEFAULT_MAX_FRAME_BYTES)}),
    };
    assert_eq!(
        encode_message(&message)
            .expect_err("default encoder rejects frames above 64 KiB")
            .kind(),
        ProtocolErrorKind::FrameLimitExceeded
    );

    let limits = FramingLimits::new(DEFAULT_MAX_BUFFER_BYTES, DEFAULT_MAX_BUFFER_BYTES);
    let encoded = encode_message_with_limits(&message, limits)
        .expect("configured limits encode frames above 64 KiB");
    let frame = &encoded[..encoded.len() - 1];
    assert!(frame.len() > DEFAULT_MAX_FRAME_BYTES);
    assert_eq!(
        decode_message(frame)
            .expect_err("default decoder rejects frames above 64 KiB")
            .kind(),
        ProtocolErrorKind::FrameLimitExceeded
    );
    assert_eq!(
        decode_message_with_limits(frame, limits).expect("configured limits decode frame"),
        message
    );
}

#[test]
fn measure_json_bytes_matches_encoded_len_and_stops_at_limit() {
    let value = json!({"a": "hello", "b": [1, 2, 3]});
    let expected = serde_json::to_vec(&value).expect("serializes").len();
    assert_eq!(measure_json_bytes(&value, expected).ok(), Some(expected));
    assert!(matches!(
        measure_json_bytes(&value, expected - 1),
        Err(MeasureJsonError::LimitExceeded)
    ));
}

#[test]
fn tool_call_locations_treat_null_line_as_none() {
    let decoded = decode_session_update(&json!({
        "sessionId": "session-1",
        "update": {
            "sessionUpdate": "tool_call",
            "toolCallId": "tool-1",
            "title": "Read file",
            "locations": [
                {"path": "src/lib.rs", "line": null},
                {"path": "src/main.rs", "line": 42}
            ]
        }
    }))
    .expect("null line decodes as None");
    let AcpSessionUpdate::ToolCall(call) = decoded.update else {
        panic!("expected tool_call update");
    };
    assert_eq!(call.locations.len(), 2);
    assert_eq!(call.locations[0].path.as_str(), "src/lib.rs");
    assert_eq!(call.locations[0].line, None);
    assert_eq!(call.locations[1].path.as_str(), "src/main.rs");
    assert_eq!(call.locations[1].line, Some(42));
}

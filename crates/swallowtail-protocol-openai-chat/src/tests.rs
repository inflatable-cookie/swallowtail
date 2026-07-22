use super::*;
use serde_json::{Value, json};

#[test]
fn decoder_handles_fragmentation_comments_multiline_data_and_done() {
    let mut decoder = SseDecoder::default();
    assert!(
        decoder
            .push(b": keepalive\n\nda")
            .expect("prefix decodes")
            .is_empty()
    );
    let records = decoder
        .push(b"ta: {\"choices\":\ndata: []}\r\n\r\ndata: [DONE]\n\n")
        .expect("records decode");
    assert_eq!(records.len(), 2);
    assert_eq!(records[0], SseRecord::Data(b"{\"choices\":\n[]}".to_vec()));
    assert_eq!(records[1], SseRecord::Done);
    decoder.finish().expect("stream finishes");
}

#[test]
fn decoder_rejects_unsupported_fields_disconnect_and_limits() {
    let mut decoder = SseDecoder::default();
    assert_eq!(
        decoder
            .push(b"event: message\n\n")
            .expect_err("field fails")
            .kind(),
        ProtocolErrorKind::UnsupportedSseField
    );

    let mut decoder = SseDecoder::default();
    decoder
        .push(b"data: {\"choices\":")
        .expect("partial accepted");
    assert_eq!(
        decoder.finish().expect_err("disconnect fails").kind(),
        ProtocolErrorKind::IncompleteRecord
    );

    let limits = CodecLimits::new(8, 4, 1, 1, 8);
    assert_eq!(
        SseDecoder::new(limits)
            .push(b"123456789")
            .expect_err("bound fails")
            .kind(),
        ProtocolErrorKind::BufferLimitExceeded
    );
}

#[test]
fn request_encoding_is_explicit_and_extension_names_do_not_collide() {
    let mut request = ChatRequest::new(
        "fixture-model",
        vec![ChatMessage::new("user", "hello")],
        true,
        true,
    );
    request
        .insert_extension("max_completion_tokens", json!(32))
        .expect("extension accepted");
    assert_eq!(
        request
            .insert_extension("model", json!("fallback"))
            .expect_err("known field rejected")
            .kind(),
        ProtocolErrorKind::InvalidStructure
    );
    let value: Value = serde_json::from_slice(
        &encode_request(&request, CodecLimits::default()).expect("request encodes"),
    )
    .expect("encoded request parses");
    assert_eq!(value["model"], "fixture-model");
    assert_eq!(value["stream"], true);
    assert_eq!(value["stream_options"]["include_usage"], true);
    assert_eq!(value["max_completion_tokens"], 32);
}

#[test]
fn structural_unknowns_are_returned_and_error_messages_are_redacted_in_debug() {
    let payload = decode_payload(
        br#"{"id":"one","model":"fixture","choices":[{"index":0,"delta":{"reasoning_content":"private"},"finish_reason":null}],"future":"bounded"}"#,
        CodecLimits::default(),
    )
    .expect("chunk decodes");
    let Payload::Chunk(chunk) = payload else {
        panic!("expected chunk")
    };
    assert_eq!(chunk.unknown_fields[0].name(), "future");
    assert_eq!(
        chunk.choices[0].delta.unknown_fields[0].name(),
        "reasoning_content"
    );

    let Payload::Error(error) = decode_payload(
        br#"{"error":{"type":"server_error","message":"private payload","code":500}}"#,
        CodecLimits::default(),
    )
    .expect("error decodes") else {
        panic!("expected error")
    };
    assert!(!format!("{:?}", error.error).contains("private payload"));
}

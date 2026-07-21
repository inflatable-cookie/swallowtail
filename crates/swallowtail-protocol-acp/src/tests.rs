use super::{Message, NdjsonDecoder, ProtocolErrorKind, encode_request};
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

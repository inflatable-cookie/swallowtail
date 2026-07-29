use swallowtail_protocol_acp::{AcpContentBlock, AcpSessionUpdate, decode_session_update};

#[test]
fn every_stable_content_block_shape_retains_its_typed_payload() {
    let contents = [
        serde_json::json!({
            "type": "image",
            "data": "aW1hZ2U=",
            "mimeType": "image/png",
            "uri": "file:///fixture/image.png"
        }),
        serde_json::json!({
            "type": "audio",
            "data": "YXVkaW8=",
            "mimeType": "audio/wav"
        }),
        serde_json::json!({
            "type": "resource",
            "resource": {
                "text": "fixture text",
                "uri": "file:///fixture/text",
                "mimeType": "text/plain"
            }
        }),
        serde_json::json!({
            "type": "resource",
            "resource": {
                "blob": "YmxvYg==",
                "uri": "file:///fixture/blob",
                "mimeType": "application/octet-stream"
            }
        }),
    ];
    let decoded = contents.map(|content| {
        let record = decode_session_update(&serde_json::json!({
            "sessionId": "session-fixture",
            "update": {
                "sessionUpdate": "agent_message_chunk",
                "content": content
            }
        }))
        .expect("stable content block decodes");
        let AcpSessionUpdate::Message(message) = record.update else {
            panic!("content remains a message chunk");
        };
        message.content
    });

    assert!(matches!(decoded[0], AcpContentBlock::Image { .. }));
    assert!(matches!(decoded[1], AcpContentBlock::Audio { .. }));
    assert!(matches!(
        decoded[2],
        AcpContentBlock::EmbeddedTextResource { .. }
    ));
    assert!(matches!(
        decoded[3],
        AcpContentBlock::EmbeddedBlobResource { .. }
    ));
}

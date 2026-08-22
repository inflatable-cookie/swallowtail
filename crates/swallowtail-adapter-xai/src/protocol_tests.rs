use super::Request;
use serde_json::{Value, json};
use std::num::NonZeroU64;
use swallowtail_core::ReasoningMode;

#[test]
fn absent_controls_keep_the_existing_request_shape() {
    let request = Request::turn("grok-fixture-exact", "First request.", None, None, None)
        .expect("request encodes");
    assert_eq!(
        serde_json::from_str::<Value>(&request).expect("request parses"),
        json!({
            "type": "response.create",
            "model": "grok-fixture-exact",
            "store": false,
            "input": [{
                "type": "message",
                "role": "user",
                "content": [{"type": "input_text", "text": "First request."}]
            }],
            "tools": []
        })
    );
}

#[test]
fn controls_are_independent_and_continuation_is_preserved() {
    let reasoning = ReasoningMode::new("xhigh").expect("reasoning mode is valid");
    let request = Request::turn(
        "grok-4.6",
        "Second request.",
        Some("resp_fixture_first"),
        Some(&reasoning),
        Some(NonZeroU64::new(512).expect("maximum is positive")),
    )
    .expect("request encodes");
    let value = serde_json::from_str::<Value>(&request).expect("request parses");
    assert_eq!(value["reasoning"], json!({"effort": "xhigh"}));
    assert_eq!(value["max_output_tokens"], 512);
    assert_eq!(value["previous_response_id"], "resp_fixture_first");
    assert!(value.get("stream").is_none());
    assert!(value.get("background").is_none());
}

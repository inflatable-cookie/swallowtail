use serde_json::Value;

#[test]
fn qualified_range_fixture_keeps_native_controls_distinct() {
    let request: Value = serde_json::from_str(include_str!(
        "fixtures/ollama-native-v0.14.0-v0.32.1/generation-controls-chat-request.json"
    ))
    .expect("fixture parses");

    assert_eq!(request["options"]["num_predict"], 64);
    assert_eq!(request["think"], "high");
    assert_eq!(request["format"]["type"], "object");
    assert!(request.get("response_format").is_none());
}

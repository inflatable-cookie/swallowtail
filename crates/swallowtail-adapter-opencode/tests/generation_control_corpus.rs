use serde_json::Value;

#[test]
fn qualified_range_fixture_keeps_variant_and_harness_schema_distinct() {
    let request: Value = serde_json::from_str(include_str!(
        "fixtures/opencode-v1.14.48-v1.18.10/generation-controls-prompt-request.json"
    ))
    .expect("fixture parses");

    assert_eq!(request["variant"], "high");
    assert_eq!(request["format"]["type"], "json_schema");
    assert_eq!(request["format"]["retryCount"], 0);
    assert!(request.get("max_output_tokens").is_none());
}

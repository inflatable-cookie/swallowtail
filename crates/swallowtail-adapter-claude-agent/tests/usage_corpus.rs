use serde_json::Value;

const USAGE: &str = include_str!("fixtures/claude-agent-acp-v0.53.0-v0.61.0/usage.json");

#[test]
fn qualified_prompt_usage_is_distinct_from_context_occupancy() {
    let fixture: Value = serde_json::from_str(USAGE).expect("usage fixture parses");
    let versions = fixture["qualified_versions"]
        .as_array()
        .expect("qualified versions are present");
    assert_eq!(versions.len(), 9);
    assert_eq!(versions.first().and_then(Value::as_str), Some("0.53.0"));
    assert_eq!(versions.last().and_then(Value::as_str), Some("0.61.0"));
    assert!(!versions.iter().any(|version| version == "0.58.0"));

    let usage = &fixture["prompt_response"]["usage"];
    let components = [
        "inputTokens",
        "outputTokens",
        "cachedReadTokens",
        "cachedWriteTokens",
    ]
    .map(|field| {
        usage[field]
            .as_u64()
            .expect("usage component is an integer")
    });
    assert_eq!(
        usage["totalTokens"].as_u64(),
        Some(components.into_iter().sum())
    );

    let context = &fixture["context_update"];
    assert_eq!(context["sessionUpdate"], "usage_update");
    assert!(context["used"].is_u64());
    assert!(context["size"].is_u64());
    assert!(context.get("inputTokens").is_none());
}

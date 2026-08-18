use serde_json::Value;
use std::collections::BTreeSet;

const RANGE: &str = include_str!("fixtures/claude-agent-acp-v0.53.0-v0.61.0/activity-range.json");
const ACTIVITY: &str = include_str!("fixtures/claude-agent-acp-v0.53.0-v0.61.0/activity.jsonl");

#[test]
fn every_qualified_claude_segment_has_exact_activity_provenance() {
    let range: Value = serde_json::from_str(RANGE).expect("range fixture is valid JSON");
    let segments = range["qualified_segments"]
        .as_array()
        .expect("segments are an array");
    assert_eq!(segments.len(), 7);
    for segment in segments {
        assert!(segment["range"].is_string());
        assert!(segment["acp_sdk"].is_string());
        assert!(segment["stable_schema"].is_string());
        assert_sha(segment, "tag_commit", 40);
        assert_sha(segment, "source_sha256", 64);
    }
    assert_eq!(range["current_external_releases"][2]["version"], "0.64.0");
    assert_eq!(
        range["current_external_releases"][2]["classification"],
        "qualified"
    );
    assert_eq!(
        range["current_external_releases"][2]["profile"],
        "0.64.0-guarantee"
    );
    assert_eq!(range["current_external_releases"][3]["version"], "0.69.0");
    assert_eq!(
        range["current_external_releases"][3]["classification"],
        "qualified"
    );
    assert_eq!(
        range["current_external_releases"][3]["profile"],
        "0.69.0-guarantee"
    );
}

#[test]
fn claude_activity_keeps_display_tool_and_provider_metadata_boundaries() {
    let cases = json_lines(ACTIVITY);
    let names: BTreeSet<_> = cases
        .iter()
        .map(|case| case["case"].as_str().expect("case name is text"))
        .collect();
    for required in [
        "assistant-message",
        "thought-display",
        "plan-replacement",
        "tool-create",
        "tool-denied",
        "usage",
        "mode",
        "commands",
        "config",
        "permission",
        "completion",
        "unknown-safe",
        "malformed-tool",
    ] {
        assert!(names.contains(required), "missing case {required}");
    }
    assert_eq!(
        case(&cases, "thought-display")["expected"]["disclosure"],
        "provider_display_content"
    );
    assert_eq!(
        case(&cases, "tool-create")["expected"]["rawInput"],
        "excluded"
    );
    assert_eq!(
        case(&cases, "malformed-tool")["expected"]["semantics"],
        "fail_closed"
    );
}

fn assert_sha(value: &Value, field: &str, length: usize) {
    let hash = value[field].as_str().expect("hash is text");
    assert_eq!(hash.len(), length);
    assert!(hash.bytes().all(|byte| byte.is_ascii_hexdigit()));
}

fn case<'a>(cases: &'a [Value], name: &str) -> &'a Value {
    cases
        .iter()
        .find(|case| case["case"] == name)
        .expect("fixture case exists")
}

fn json_lines(value: &str) -> Vec<Value> {
    value
        .lines()
        .map(|line| serde_json::from_str(line).expect("fixture line is valid JSON"))
        .collect()
}

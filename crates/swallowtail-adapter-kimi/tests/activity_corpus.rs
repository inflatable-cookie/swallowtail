use serde_json::Value;
use std::collections::BTreeSet;

const RANGE: &str = include_str!("fixtures/kimi-code-acp-v0.28.1-v0.31.0/activity-range.json");
const ACTIVITY: &str = include_str!("fixtures/kimi-code-acp-v0.28.1-v0.31.0/activity.jsonl");
const RELEASE_0_31_1: &str = include_str!("fixtures/kimi-code-0.31.1/release.json");

#[test]
fn every_qualified_kimi_acp_segment_has_exact_activity_provenance() {
    let range: Value = serde_json::from_str(RANGE).expect("range fixture is valid JSON");
    let segments = range["qualified_segments"]
        .as_array()
        .expect("segments are an array");
    assert_eq!(segments.len(), 2);
    assert_eq!(segments[0]["range"], "0.28.1");
    assert_eq!(segments[1]["range"], "0.29.0..=0.31.0");
    for segment in segments {
        assert_eq!(segment["stable_schema"], "schema-v1.19.1");
        assert_sha(segment, "tag_commit", 40);
        assert_sha(segment, "events_map_sha256", 64);
    }
    assert_eq!(range["current_external_release"]["version"], "0.31.0");
    assert_eq!(
        range["current_external_release"]["classification"],
        "qualified"
    );
    assert_eq!(
        range["current_external_release"]["profile"],
        "0.31.0-guarantee"
    );
    assert_eq!(
        range["current_external_release"]["activity_source_delta"],
        "none"
    );
    assert_eq!(
        range["later_stable_posture"]["classification"],
        "unverified-newer"
    );

    let release: Value =
        serde_json::from_str(RELEASE_0_31_1).expect("0.31.1 release fixture is valid JSON");
    assert_eq!(release["release"]["version"], "0.31.1");
    assert_eq!(
        release["acp"]["events_map_sha256"],
        segments[1]["events_map_sha256"]
    );
    assert_eq!(
        release["acp"]["behavior"],
        "kimi.acp.reasoning.declared-effort-v2"
    );
}

#[test]
fn kimi_corpus_preserves_lazy_tool_lifecycle_plan_and_exclusions() {
    let cases = json_lines(ACTIVITY);
    let names: BTreeSet<_> = cases
        .iter()
        .map(|case| case["case"].as_str().expect("case name is text"))
        .collect();
    for required in [
        "assistant-message",
        "thought-display",
        "tool-lazy-create",
        "tool-start-upgrade",
        "tool-completed",
        "plan-replacement",
        "commands-empty",
        "config-replacement",
        "permission",
        "completion",
        "unknown-safe",
        "malformed-tool",
    ] {
        assert!(names.contains(required), "missing case {required}");
    }
    assert_eq!(
        case(&cases, "tool-start-upgrade")["expected"]["semantics"],
        "partial_update_with_content_replacement"
    );
    assert_eq!(
        case(&cases, "tool-completed")["expected"]["rawOutput"],
        "excluded"
    );
    assert_eq!(
        case(&cases, "plan-replacement")["expected"]["semantics"],
        "authoritative_full_replacement"
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

use serde_json::Value;
use std::collections::BTreeSet;

const RANGE: &str = include_str!("fixtures/gemini-cli-acp-v0.51.0/activity-range.json");
const ACTIVITY: &str = include_str!("fixtures/gemini-cli-acp-v0.51.0/activity.jsonl");

#[test]
fn exact_gemini_acp_release_has_activity_provenance_without_newer_widening() {
    let range: Value = serde_json::from_str(RANGE).expect("range fixture is valid JSON");
    let segment = &range["qualified_segments"][0];
    assert_eq!(segment["range"], "0.51.0");
    assert_eq!(segment["stable_schema"], "schema-v1.19.0");
    assert_eq!(segment["acp_sdk"], "0.16.1");
    for field in ["tag_commit", "session_source_sha256", "utils_source_sha256"] {
        let expected = if field == "tag_commit" { 40 } else { 64 };
        assert_sha(segment, field, expected);
    }
    assert_eq!(range["current_external_release"]["version"], "0.53.0");
    assert_eq!(
        range["current_external_release"]["classification"],
        "unverified-newer"
    );
    assert_eq!(
        range["current_external_release"]["activity_source_delta"],
        "none"
    );
    assert_eq!(
        range["current_external_release"]["full_harness_qualification"],
        false
    );
}

#[test]
fn gemini_corpus_separates_model_thoughts_warnings_and_tool_lifecycle() {
    let cases = json_lines(ACTIVITY);
    let names: BTreeSet<_> = cases
        .iter()
        .map(|case| case["case"].as_str().expect("case name is text"))
        .collect();
    for required in [
        "assistant-message",
        "model-thought",
        "operational-warning-thought",
        "tool-create",
        "tool-completed",
        "tool-failed",
        "commands",
        "mode-display",
        "permission",
        "completion",
        "unknown-safe",
        "malformed-tool",
    ] {
        assert!(names.contains(required), "missing case {required}");
    }
    assert_eq!(
        case(&cases, "model-thought")["expected"]["portable"],
        "reasoning_summary_candidate"
    );
    assert_eq!(
        case(&cases, "operational-warning-thought")["expected"]["portable"],
        "adapter_classified_warning"
    );
    assert_eq!(
        case(&cases, "mode-display")["expected"]["mode_evidence"],
        false
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

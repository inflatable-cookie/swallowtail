use serde_json::Value;
use std::collections::BTreeSet;

const RANGE: &str = include_str!("fixtures/gemini-cli-acp-v0.51.0/activity-range.json");
const ACTIVITY: &str = include_str!("fixtures/gemini-cli-acp-v0.51.0/activity.jsonl");
const IDENTITY: &str = include_str!("fixtures/gemini-cli-0.56.0/identity.json");
const PROTOCOL: &str = include_str!("fixtures/gemini-cli-0.56.0/protocol.json");

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

#[test]
fn gemini_cli_0_56_identity_corpus_freezes_currentness_decisions() {
    let identity: Value = serde_json::from_str(IDENTITY).expect("identity fixture is valid JSON");
    assert_eq!(identity["family"], "gemini-cli");
    assert_eq!(identity["official"]["version"], "0.56.0");
    assert_eq!(identity["host"]["version"], "0.53.0");
    assert_eq!(identity["host"]["host_install_changed"], false);
    assert_eq!(identity["published_stables_from_previous_ceilings"].as_array().map(Vec::len), Some(6));
    assert_eq!(identity["unpublished_later_stable"], "0.56.1");
    assert_eq!(identity["ignored_preview"], "0.57.0-preview.0");

    for axis in ["acp", "headless"] {
        assert_eq!(identity["identity_decision"][axis]["shape"], "compatible-extension");
        assert_eq!(identity["identity_decision"][axis]["raise_latest_qualified_to"], "0.56.0");
        assert_eq!(identity["identity_decision"][axis]["keep_baseline"], "0.51.0");
        assert_eq!(identity["identity_decision"][axis]["new_milestone"], false);
    }
    assert_eq!(identity["identity_decision"]["new_public_operation"], false);
    assert_eq!(identity["identity_decision"]["provider_prompt_sent"], false);
    assert_eq!(identity["identity_decision"]["live_session"], false);
}

#[test]
fn gemini_cli_0_56_protocol_corpus_keeps_axes_and_unmapped_deltas_explicit() {
    let protocol: Value = serde_json::from_str(PROTOCOL).expect("protocol fixture is valid JSON");
    assert_eq!(protocol["official_version"], "0.56.0");
    assert_eq!(protocol["acp"]["axis"], "gemini-cli.acp-agent");
    assert_eq!(protocol["headless"]["axis"], "gemini-cli.headless-stream-json");
    assert_eq!(protocol["acp"]["selected_external_shapes_unchanged_from_0.51.0"], true);
    assert_eq!(protocol["headless"]["selected_external_shapes_unchanged_through_0.56.0"], true);
    assert_eq!(protocol["acp"]["provider_prompt_sent"], false);
    assert_eq!(protocol["headless"]["provider_prompt_sent"], false);
    assert_eq!(protocol["acp"]["live_session"], false);
    assert_eq!(protocol["headless"]["live_session"], false);
    assert!(!protocol["acp"]["unmapped_additions"].as_array().unwrap().is_empty());
    assert!(!protocol["headless"]["unmapped_additions"].as_array().unwrap().is_empty());
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

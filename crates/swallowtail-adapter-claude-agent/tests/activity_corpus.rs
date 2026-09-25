use serde_json::Value;
use std::collections::BTreeSet;

const RANGE: &str = include_str!("fixtures/claude-agent-acp-v0.53.0-v0.61.0/activity-range.json");
const ACTIVITY: &str = include_str!("fixtures/claude-agent-acp-v0.53.0-v0.61.0/activity.jsonl");

const EXPECTED_EMITTED_UPDATES: &[&str] = &[
    "agent_message_chunk",
    "agent_thought_chunk",
    "async_task_progress",
    "async_task_spawned",
    "async_task_state_update",
    "available_commands_update",
    "config_option_update",
    "current_mode_update",
    "plan",
    "session_info_update",
    "subagent_spawned",
    "subagent_state_update",
    "tool_call",
    "tool_call_update",
    "usage_update",
    "user_message_chunk",
];

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
    assert_eq!(range["qualified_segments"][6]["range"], "0.66.0..=0.81.2");
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
    assert_eq!(range["current_external_releases"][4]["version"], "0.70.0");
    assert_eq!(
        range["current_external_releases"][4]["classification"],
        "qualified"
    );
    assert_eq!(
        range["current_external_releases"][4]["profile"],
        "0.70.0-guarantee"
    );
    assert_eq!(range["current_external_releases"][5]["version"], "0.71.0");
    assert_eq!(
        range["current_external_releases"][5]["classification"],
        "qualified"
    );
    assert_eq!(
        range["current_external_releases"][5]["profile"],
        "0.73.0-guarantee"
    );
    assert_eq!(
        range["current_external_releases"][5]["activity_delta"],
        "session-titles-subagents-modes-steering-unmapped"
    );
    assert_eq!(range["current_external_releases"][6]["version"], "0.72.0");
    assert_eq!(
        range["current_external_releases"][6]["classification"],
        "qualified"
    );
    assert_eq!(
        range["current_external_releases"][6]["profile"],
        "0.73.0-guarantee"
    );
    assert_eq!(
        range["current_external_releases"][6]["activity_delta"],
        "effort-result-attribution-model-switch-hooks-unmapped"
    );
    assert_eq!(range["current_external_releases"][7]["version"], "0.73.0");
    assert_eq!(
        range["current_external_releases"][7]["classification"],
        "qualified"
    );
    assert_eq!(
        range["current_external_releases"][7]["profile"],
        "0.73.0-guarantee"
    );
    assert_eq!(
        range["current_external_releases"][7]["activity_delta"],
        "agent-sdk-pin-unmapped"
    );
    assert_eq!(range["current_external_releases"][8]["version"], "0.74.0");
    assert_eq!(
        range["current_external_releases"][8]["classification"],
        "qualified"
    );
    assert_eq!(
        range["current_external_releases"][8]["profile"],
        "0.76.0-guarantee"
    );
    assert_eq!(
        range["current_external_releases"][8]["activity_delta"],
        "hide-claude-auth-guard-and-session-failure-unmapped"
    );
    assert_eq!(range["current_external_releases"][9]["version"], "0.75.0");
    assert_eq!(
        range["current_external_releases"][9]["classification"],
        "qualified"
    );
    assert_eq!(
        range["current_external_releases"][9]["profile"],
        "0.76.0-guarantee"
    );
    assert_eq!(
        range["current_external_releases"][9]["activity_delta"],
        "auth-status-compaction-and-usage-markdown-unmapped"
    );
    assert_eq!(range["current_external_releases"][10]["version"], "0.75.1");
    assert_eq!(
        range["current_external_releases"][10]["classification"],
        "qualified"
    );
    assert_eq!(
        range["current_external_releases"][10]["profile"],
        "0.76.0-guarantee"
    );
    assert_eq!(
        range["current_external_releases"][10]["activity_delta"],
        "resumed-transcript-and-fork-meta-unmapped"
    );
    assert_eq!(range["current_external_releases"][11]["version"], "0.76.0");
    assert_eq!(
        range["current_external_releases"][11]["classification"],
        "qualified"
    );
    assert_eq!(
        range["current_external_releases"][11]["profile"],
        "0.76.0-guarantee"
    );
    assert_eq!(
        range["current_external_releases"][11]["activity_delta"],
        "recommended-config-values-and-clear-context-unmapped"
    );
    assert_eq!(range["current_external_releases"][12]["version"], "0.77.0");
    assert_eq!(
        range["current_external_releases"][12]["classification"],
        "qualified"
    );
    assert_eq!(
        range["current_external_releases"][12]["profile"],
        "0.79.0-guarantee"
    );
    assert_eq!(
        range["current_external_releases"][12]["activity_delta"],
        "agent-config-removal-and-multi-select-other-description"
    );
    assert_eq!(range["current_external_releases"][13]["version"], "0.78.0");
    assert_eq!(
        range["current_external_releases"][13]["classification"],
        "qualified"
    );
    assert_eq!(
        range["current_external_releases"][13]["profile"],
        "0.79.0-guarantee"
    );
    assert_eq!(
        range["current_external_releases"][13]["activity_delta"],
        "compaction-update-and-single-select-other-description"
    );
    assert_eq!(range["current_external_releases"][14]["version"], "0.79.0");
    assert_eq!(
        range["current_external_releases"][14]["classification"],
        "qualified"
    );
    assert_eq!(
        range["current_external_releases"][14]["profile"],
        "0.79.0-guarantee"
    );
    assert_eq!(
        range["current_external_releases"][14]["activity_delta"],
        "agent-sdk-pin-and-shell-permission-title-unmapped"
    );
    assert_eq!(
        range["current_external_releases"]
            .as_array()
            .expect("releases")
            .len(),
        19
    );
    assert_eq!(range["current_external_releases"][18]["version"], "0.81.2");
    assert_eq!(
        range["current_external_releases"][18]["classification"],
        "qualified"
    );
    assert_eq!(
        range["current_external_releases"][18]["profile"],
        "0.81.2-guarantee"
    );
}

#[test]
fn claude_activity_emitted_updates_are_the_exact_known_set() {
    let expected: BTreeSet<_> = EXPECTED_EMITTED_UPDATES.iter().copied().collect();
    assert_eq!(expected.len(), 16);
    for kind in [
        "subagent_spawned",
        "subagent_state_update",
        "async_task_spawned",
        "async_task_progress",
        "async_task_state_update",
    ] {
        assert!(expected.contains(kind), "missing emitted update {kind}");
    }
    let range: Value = serde_json::from_str(RANGE).expect("range fixture is valid JSON");
    let actual: BTreeSet<_> = range["emitted_updates"]
        .as_array()
        .expect("emitted_updates is an array")
        .iter()
        .map(|value| value.as_str().expect("emitted update is text"))
        .collect();
    assert_eq!(actual, expected);
    assert_eq!(
        range["emitted_updates"],
        serde_json::json!(EXPECTED_EMITTED_UPDATES)
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

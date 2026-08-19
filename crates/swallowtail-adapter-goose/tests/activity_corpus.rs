use serde_json::Value;
use std::collections::BTreeSet;

const ACTIVITY: &str = include_str!("fixtures/goose-acp-1.46.0/activity.jsonl");

#[test]
fn goose_corpus_keeps_raw_payloads_usage_and_allow_always_out_of_stable_activity() {
    let cases = json_lines(ACTIVITY);
    let names: BTreeSet<_> = cases
        .iter()
        .map(|case| case["case"].as_str().expect("case name is text"))
        .collect();
    for required in [
        "assistant-message",
        "thought-display",
        "user-echo",
        "tool-create",
        "tool-complete",
        "tool-failed",
        "config",
        "session-info",
        "usage-update-unclaimed",
        "permission",
        "completion",
        "cancelled",
        "max-tokens",
        "unknown-safe",
        "custom-goose-notification",
        "malformed-tool",
    ] {
        assert!(names.contains(required), "missing case {required}");
    }
    assert_eq!(
        case(&cases, "tool-create")["expected"]["rawInput"],
        "excluded"
    );
    assert_eq!(
        case(&cases, "tool-complete")["expected"]["rawOutput"],
        "excluded"
    );
    assert_eq!(case(&cases, "config")["expected"]["agent_activity"], false);
    assert_eq!(
        case(&cases, "session-info")["expected"]["agent_activity"],
        false
    );
    assert_eq!(
        case(&cases, "usage-update-unclaimed")["expected"]["usage_evidence"],
        false
    );
    assert_eq!(
        case(&cases, "permission")["expected"]["allow_always"],
        "unselected"
    );
    assert_eq!(
        case(&cases, "custom-goose-notification")["expected"]["semantics"],
        "unmapped"
    );
    assert_eq!(
        case(&cases, "malformed-tool")["expected"]["semantics"],
        "fail_closed"
    );
}

fn case<'a>(cases: &'a [Value], name: &str) -> &'a Value {
    cases
        .iter()
        .find(|case| case["case"] == name)
        .unwrap_or_else(|| panic!("missing case {name}"))
}

fn json_lines(body: &str) -> Vec<Value> {
    body.lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| serde_json::from_str(line).expect("activity line is JSON"))
        .collect()
}

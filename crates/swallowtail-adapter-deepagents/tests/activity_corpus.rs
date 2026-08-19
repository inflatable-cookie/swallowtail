use serde_json::Value;
use std::collections::BTreeSet;

const ACTIVITY: &str = include_str!("fixtures/deepagents-acp-0.1.25/activity.jsonl");

#[test]
fn deepagents_corpus_keeps_raw_payloads_usage_and_allow_always_out_of_stable_activity() {
    let cases = json_lines(ACTIVITY);
    let names: BTreeSet<_> = cases
        .iter()
        .map(|case| case["case"].as_str().expect("case name is text"))
        .collect();
    for required in [
        "assistant-message",
        "tool-create",
        "tool-complete",
        "permission",
        "completion",
        "cancelled",
        "unknown-safe",
        "slash-command-unmapped",
        "content-field-rejected",
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
    assert_eq!(
        case(&cases, "permission")["expected"]["allow_always"],
        "unselected"
    );
    assert_eq!(
        case(&cases, "slash-command-unmapped")["expected"]["semantics"],
        "unmapped"
    );
    assert_eq!(
        case(&cases, "content-field-rejected")["expected"]["semantics"],
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

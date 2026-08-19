use serde_json::Value;
use std::collections::BTreeSet;

const ACTIVITY: &str = include_str!("fixtures/copilot-cli-acp-1.0.80/activity.jsonl");

#[test]
fn copilot_corpus_keeps_usage_slash_and_allow_always_out_of_stable_activity() {
    let cases = json_lines(ACTIVITY);
    let names: BTreeSet<_> = cases
        .iter()
        .map(|case| case["case"].as_str().expect("case name is text"))
        .collect();
    for required in [
        "assistant-message",
        "available-commands",
        "usage-update-unclaimed",
        "permission",
        "completion",
        "cancelled",
        "unknown-safe",
        "interactive-slash-unmapped",
    ] {
        assert!(names.contains(required), "missing case {required}");
    }
    assert_eq!(
        case(&cases, "available-commands")["expected"]["swallowtail_slash_operation"],
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
        case(&cases, "interactive-slash-unmapped")["expected"]["swallowtail_operation"],
        false
    );
    assert_eq!(
        case(&cases, "unknown-safe")["expected"]["semantics"],
        "bounded_namespaced_unknown"
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

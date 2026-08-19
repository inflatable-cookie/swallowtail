use serde_json::Value;
use std::collections::BTreeSet;

const ACTIVITY: &str = include_str!("fixtures/mistral-vibe-headless-2.24.2/activity.jsonl");

#[test]
fn vibe_corpus_keeps_raw_payloads_yolo_and_acp_out_of_stable_activity() {
    let cases = json_lines(ACTIVITY);
    let names: BTreeSet<_> = cases
        .iter()
        .map(|case| case["case"].as_str().expect("case name is text"))
        .collect();
    for required in [
        "assistant-text",
        "reasoning",
        "effect-tool",
        "callback-deny",
        "unknown-entry-type",
        "acp-jsonrpc-is-not-headless",
        "json-dump-array-wrong-wire",
        "limit-stderr",
        "host-abort",
    ] {
        assert!(names.contains(required), "missing case {required}");
    }
    assert_eq!(
        case(&cases, "effect-tool")["expected"]["rawInput"],
        "excluded"
    );
    assert_eq!(
        case(&cases, "effect-tool")["expected"]["rawOutput"],
        "excluded"
    );
    assert_eq!(case(&cases, "callback-deny")["expected"]["allow"], false);
    assert_eq!(
        case(&cases, "user-prompt")["expected"]["agent_activity"],
        false
    );
    assert_eq!(
        case(&cases, "notice-status")["expected"]["agent_activity"],
        false
    );
}

fn json_lines(body: &str) -> Vec<Value> {
    body.lines()
        .filter(|line| !line.is_empty())
        .map(|line| serde_json::from_str(line).expect("activity line"))
        .collect()
}

fn case<'a>(cases: &'a [Value], name: &str) -> &'a Value {
    cases
        .iter()
        .find(|row| row["case"] == name)
        .unwrap_or_else(|| panic!("missing case {name}"))
}

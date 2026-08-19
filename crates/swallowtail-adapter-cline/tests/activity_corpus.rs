use serde_json::Value;
use std::collections::BTreeSet;

const ACTIVITY: &str = include_str!("fixtures/cline-acp-3.0.55/activity.jsonl");
const HEADLESS_ACTIVITY: &str = include_str!("fixtures/cline-headless-3.0.55/activity.jsonl");

#[test]
fn cline_corpus_keeps_raw_payloads_and_allow_always_out_of_stable_activity() {
    let cases = json_lines(ACTIVITY);
    let names: BTreeSet<_> = cases
        .iter()
        .map(|case| case["case"].as_str().expect("case name is text"))
        .collect();
    for required in [
        "assistant-message",
        "thought-display",
        "tool-create",
        "tool-complete",
        "tool-failed",
        "mode",
        "config",
        "session-info",
        "permission",
        "completion",
        "cancelled",
        "unknown-safe",
        "malformed-tool",
        "usage-not-forwarded",
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
    assert_eq!(case(&cases, "mode")["expected"]["agent_activity"], false);
    assert_eq!(case(&cases, "config")["expected"]["agent_activity"], false);
    assert_eq!(
        case(&cases, "session-info")["expected"]["agent_activity"],
        false
    );
    assert_eq!(
        case(&cases, "permission")["expected"]["allow_always"],
        "unselected"
    );
    assert_eq!(
        case(&cases, "usage-not-forwarded")["expected"]["session_update"],
        false
    );
    assert_eq!(
        case(&cases, "malformed-tool")["expected"]["semantics"],
        "fail_closed"
    );
}

#[test]
fn headless_corpus_keeps_ask_say_usage_and_team_events_out_of_stable_activity() {
    let cases = json_lines(HEADLESS_ACTIVITY);
    let names: BTreeSet<_> = cases
        .iter()
        .map(|case| case["case"].as_str().expect("case name is text"))
        .collect();
    for required in [
        "assistant-text",
        "thought-display",
        "tool-start",
        "tool-end",
        "run-result-completed",
        "run-aborted",
        "usage-excluded",
        "team-event-unmapped",
        "unknown-envelope",
        "docs-ask-say-wrong-wire",
    ] {
        assert!(names.contains(required), "missing case {required}");
    }
    assert_eq!(
        case(&cases, "tool-start")["expected"]["rawInput"],
        "excluded"
    );
    assert_eq!(
        case(&cases, "tool-end")["expected"]["rawOutput"],
        "excluded"
    );
    assert_eq!(
        case(&cases, "usage-excluded")["expected"]["raw_payload"],
        "excluded"
    );
    assert_eq!(
        case(&cases, "team-event-unmapped")["expected"]["unmapped"],
        true
    );
    assert_eq!(
        case(&cases, "docs-ask-say-wrong-wire")["expected"]["semantics"],
        "fail_closed_wrong_wire"
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

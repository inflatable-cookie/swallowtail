use serde_json::Value;
use std::collections::BTreeSet;

const ACTIVITY: &str = include_str!("fixtures/qoder-headless-1.1.25/activity.jsonl");

#[test]
fn qoder_corpus_keeps_usage_acp_and_partial_stream_out_of_stable_activity() {
    let cases = json_lines(ACTIVITY);
    let names: BTreeSet<_> = cases
        .iter()
        .map(|case| case["case"].as_str().expect("case name is text"))
        .collect();
    for required in [
        "system-init",
        "assistant-text",
        "result-success",
        "result-max-turns",
        "result-abort",
        "usage-excluded",
        "stream-event-unselected",
        "hook-unmapped",
        "unknown-envelope",
        "acp-jsonrpc-is-not-headless",
        "host-abort",
    ] {
        assert!(names.contains(required), "missing case {required}");
    }
    assert_eq!(
        case(&cases, "usage-excluded")["expected"]["raw_payload"],
        "excluded"
    );
    assert_eq!(
        case(&cases, "system-init")["expected"]["agent_activity"],
        false
    );
    assert_eq!(
        case(&cases, "stream-event-unselected")["expected"]["semantics"],
        "not_emitted_on_selected_wire"
    );
    assert_eq!(
        case(&cases, "unknown-envelope")["expected"]["semantics"],
        "fail_closed_framing"
    );
    assert_eq!(
        case(&cases, "result-max-turns")["expected"]["stop_reason"],
        "bounded_limit"
    );
    assert_eq!(
        case(&cases, "result-abort")["expected"]["stop_reason"],
        "cancelled"
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

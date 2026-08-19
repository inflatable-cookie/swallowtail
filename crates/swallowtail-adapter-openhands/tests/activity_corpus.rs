use serde_json::Value;
use std::collections::BTreeSet;

const ACTIVITY: &str = include_str!("fixtures/openhands-agent-server-1.42.1/activity.jsonl");
const NEGATIVE: &str = include_str!("fixtures/openhands-agent-server-1.42.1/negative-cases.json");

#[test]
fn openhands_corpus_keeps_selected_kinds_and_fail_closed_negatives() {
    let events = json_lines(ACTIVITY);
    let kinds: BTreeSet<_> = events
        .iter()
        .map(|event| event["kind"].as_str().expect("kind is text"))
        .collect();
    for required in [
        "ConversationStateUpdateEvent",
        "MessageEvent",
        "StreamingDeltaEvent",
        "FinishAction",
    ] {
        assert!(kinds.contains(required), "missing kind {required}");
    }
    assert!(
        events
            .iter()
            .any(|event| event["value"] == "finished" && event["key"] == "execution_status")
    );
    assert!(events.iter().any(|event| {
        event["kind"] == "StreamingDeltaEvent" && event["content"] == "OpenHands display text."
    }));
    for event in &events {
        assert_ne!(event.get("jsonrpc"), Some(&Value::from("2.0")));
        assert_ne!(event.get("action"), Some(&Value::from("init")));
    }

    let negatives: Value = serde_json::from_str(NEGATIVE).expect("negatives");
    let ids: BTreeSet<_> = negatives["cases"]
        .as_array()
        .expect("cases")
        .iter()
        .map(|case| case["id"].as_str().expect("id"))
        .collect();
    for required in [
        "malformed-json",
        "unknown-kind-fail-closed",
        "acp-jsonrpc-is-not-this-route",
        "v0-socketio-init-session-unmapped",
        "never-confirm-not-swallowtail-authority",
        "omit-confirmation-policy-inherits-never-confirm",
        "omit-max-iterations-inherits-500",
        "wildcard-host-not-swallowtail-authority",
    ] {
        assert!(ids.contains(required), "missing negative {required}");
    }
    assert_eq!(
        negative(&negatives, "acp-jsonrpc-is-not-this-route")["expected"],
        "reject_as_contract_035_or_acp_not_this_route"
    );
    assert_eq!(
        negative(&negatives, "unknown-kind-fail-closed")["expected"],
        "fail_closed_unknown_kind"
    );
    assert_eq!(
        negative(&negatives, "never-confirm-not-swallowtail-authority")["expected"],
        "do_not_select_as_swallowtail_authority"
    );
}

fn json_lines(body: &str) -> Vec<Value> {
    body.lines()
        .filter(|line| !line.is_empty())
        .map(|line| serde_json::from_str(line).expect("activity line"))
        .collect()
}

fn negative<'a>(root: &'a Value, id: &str) -> &'a Value {
    root["cases"]
        .as_array()
        .expect("cases")
        .iter()
        .find(|case| case["id"] == id)
        .unwrap_or_else(|| panic!("missing case {id}"))
}

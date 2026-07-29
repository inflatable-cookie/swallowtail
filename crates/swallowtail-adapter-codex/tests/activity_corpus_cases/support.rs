use serde_json::Value;
use std::collections::BTreeSet;

pub(crate) const RANGE: &str = include_str!("../fixtures/activity/range.json");
pub(crate) const APP_SERVER: &str = include_str!("../fixtures/activity/app-server.jsonl");
pub(crate) const EXEC: &str = include_str!("../fixtures/activity/exec.jsonl");

pub(crate) fn assert_segments(value: &Value, revisions: &[&str]) {
    let segments = value.as_array().expect("segments are an array");
    assert!(!segments.is_empty());
    let observed: BTreeSet<_> = segments
        .iter()
        .map(|segment| {
            assert!(segment["range"].is_string());
            assert!(segment["introduced_by"].is_string());
            let commit = segment["tag_commit"].as_str().expect("commit is text");
            assert_eq!(commit.len(), 40);
            assert!(commit.bytes().all(|byte| byte.is_ascii_hexdigit()));
            segment["revision"].as_str().expect("revision is text")
        })
        .collect();
    for revision in revisions {
        assert!(observed.contains(revision), "missing revision {revision}");
    }
}

pub(crate) fn methods(case: &Value) -> BTreeSet<&str> {
    case["messages"]
        .as_array()
        .expect("messages are an array")
        .iter()
        .map(|message| message["method"].as_str().expect("method is text"))
        .collect()
}

pub(crate) fn case_names(cases: &[Value]) -> BTreeSet<&str> {
    cases
        .iter()
        .map(|case| case["case"].as_str().expect("case name is text"))
        .collect()
}

pub(crate) fn case<'a>(cases: &'a [Value], name: &str) -> &'a Value {
    cases
        .iter()
        .find(|case| case["case"] == name)
        .expect("fixture case exists")
}

pub(crate) fn json_lines(value: &str) -> Vec<Value> {
    value.lines().map(json).collect()
}

pub(crate) fn strings(value: &Value) -> BTreeSet<&str> {
    value
        .as_array()
        .expect("value is an array")
        .iter()
        .map(|value| value.as_str().expect("value is text"))
        .collect()
}

pub(crate) fn json(value: &str) -> Value {
    serde_json::from_str(value).expect("fixture JSON is valid")
}

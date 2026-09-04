use std::collections::BTreeSet;

use serde_json::{Map, Value};

#[path = "opencode_http_1_18_28_delta_ledger/identity.rs"]
mod identity;
#[path = "opencode_http_1_18_28_delta_ledger/inventory.rs"]
mod inventory;
#[path = "opencode_http_1_18_28_delta_ledger/protocol.rs"]
mod protocol;

const IDENTITY: &str = include_str!("fixtures/opencode-1.18.28/identity.json");
const PROTOCOL: &str = include_str!("fixtures/opencode-1.18.28/protocol.json");
const INVENTORY: &str = include_str!("fixtures/opencode-1.18.28/dist-inventory.json");

fn json(input: &str) -> Value {
    serde_json::from_str(input).expect("fixture is valid JSON")
}

fn assert_nonempty_string(value: &Value, key: &str) {
    assert!(
        value[key].as_str().is_some_and(|text| !text.is_empty()),
        "missing {key}"
    );
}

fn assert_exact_strings(actual: &Value, expected: &[&str]) {
    let actual = actual.as_array().expect("string array");
    assert_eq!(actual.len(), expected.len());
    let actual = actual
        .iter()
        .map(|value| value.as_str().unwrap())
        .collect::<BTreeSet<_>>();
    let expected = expected.iter().copied().collect::<BTreeSet<_>>();
    assert_eq!(actual, expected);
}

fn assert_exact_object_keys(actual: &Map<String, Value>, expected: &[&str]) {
    let actual = actual.keys().map(String::as_str).collect::<BTreeSet<_>>();
    let expected = expected.iter().copied().collect::<BTreeSet<_>>();
    assert_eq!(actual, expected);
}

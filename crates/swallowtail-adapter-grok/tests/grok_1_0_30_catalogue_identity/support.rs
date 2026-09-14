//! Shared fixture access for the Grok Build catalogue 1.0.30 identity ledger.

use serde_json::Value;

pub(super) const IDENTITY: &str = include_str!("../fixtures/grok-1.0.30-catalogue/identity.json");

pub(super) fn json(value: &str) -> Value {
    serde_json::from_str(value).expect("frozen corpus JSON is valid")
}

pub(super) fn strings(value: &Value) -> Vec<&str> {
    value
        .as_array()
        .expect("value is an array")
        .iter()
        .map(|value| value.as_str().expect("array value is text"))
        .collect()
}

pub(super) fn assert_exact_string_set(value: &Value, expected: &[&str]) {
    assert_eq!(strings(value), expected);
}

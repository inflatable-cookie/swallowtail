//! Shared fixture access for the Grok Build ACP 1.0.30 identity ledger.

use serde_json::Value;
use sha2::{Digest, Sha256};
use swallowtail_core::InterfaceVersion;

pub(super) const IDENTITY: &str = include_str!("../fixtures/grok-1.0.30/identity.json");
pub(super) const PROTOCOL: &str = include_str!("../fixtures/grok-1.0.30/protocol.json");
pub(super) const DIST_INVENTORY: &str = include_str!("../fixtures/grok-1.0.30/dist-inventory.json");

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

/// SHA-256 over the entries joined by a newline with a trailing newline.
pub(super) fn digest_lines(items: &[String]) -> String {
    let mut joined = items.join("\n");
    joined.push('\n');
    let mut hasher = Sha256::new();
    hasher.update(joined.as_bytes());
    format!("{:x}", hasher.finalize())
}

pub(super) fn version(value: &str) -> InterfaceVersion {
    InterfaceVersion::new(value).expect("fixture version is valid")
}

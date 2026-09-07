use std::collections::BTreeSet;

use serde_json::{Map, Value};

#[path = "opencode_http_1_18_29_delta_ledger/identity.rs"]
mod identity;
#[path = "opencode_http_1_18_29_delta_ledger/inventory.rs"]
mod inventory;
#[path = "opencode_http_1_18_29_delta_ledger/protocol.rs"]
mod protocol;

const IDENTITY: &str = include_str!("fixtures/opencode-1.18.29/identity.json");
const PROTOCOL: &str = include_str!("fixtures/opencode-1.18.29/protocol.json");
const INVENTORY: &str = include_str!("fixtures/opencode-1.18.29/dist-inventory.json");
const CLAIM: &str = include_str!("fixtures/opencode-1.18.29/claim.json");

#[test]
fn admitted_claim_fixture_records_intended_after_state() {
    let fixture = json(CLAIM);
    assert_eq!(fixture["claim_id"], "opencode.http.server-window-1");
    assert_eq!(fixture["baseline"], "1.14.48");
    assert_eq!(fixture["latest_qualified"], "1.18.29");
    assert_eq!(fixture["behavior_revision"], "opencode.http-sse.surface-19");
    assert_exact_strings(&fixture["newly_qualified"], &["1.18.29"]);
    assert_eq!(fixture["unverified_newer"], "1.18.30");
    assert_eq!(fixture["newer_version_posture"], "allow_unverified");
}

fn json(input: &str) -> Value {
    serde_json::from_str(input).expect("fixture is valid JSON")
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

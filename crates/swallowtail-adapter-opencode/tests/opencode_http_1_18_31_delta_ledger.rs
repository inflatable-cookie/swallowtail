use std::collections::BTreeSet;

use serde_json::{Map, Value};
use swallowtail_adapter_opencode::{
    OPENCODE_LATEST_QUALIFIED_VERSION, opencode_http_claim, opencode_server_binding,
};
use swallowtail_core::InterfaceCompatibilityAssessment;

#[path = "opencode_http_1_18_31_delta_ledger/identity.rs"]
mod identity;
#[path = "opencode_http_1_18_31_delta_ledger/inventory.rs"]
mod inventory;
#[path = "opencode_http_1_18_31_delta_ledger/protocol.rs"]
mod protocol;

const IDENTITY: &str = include_str!("fixtures/opencode-1.18.31/identity.json");
const PROTOCOL: &str = include_str!("fixtures/opencode-1.18.31/protocol.json");
const INVENTORY: &str = include_str!("fixtures/opencode-1.18.31/dist-inventory.json");
const CLAIM: &str = include_str!("fixtures/opencode-1.18.31/claim.json");

#[test]
fn identity_keeps_production_claim_at_the_previous_ceiling() {
    let identity = json(IDENTITY);
    let claim = opencode_http_claim();
    assert_eq!(OPENCODE_LATEST_QUALIFIED_VERSION, "1.18.30");
    assert_eq!(
        claim.latest_qualified().as_str(),
        identity["claim_at_observation"]["latest_qualified"]
    );
    assert_eq!(
        identity["identity_decision"]["claim_changed_in_identity_card"],
        false
    );
    let later = opencode_server_binding("1.18.31").expect("official hop is safe");
    assert!(matches!(
        claim.assess(later.version()),
        InterfaceCompatibilityAssessment::UnverifiedNewer(_)
    ));
}

#[test]
fn admitted_claim_fixture_names_the_intended_after_state() {
    let fixture = json(CLAIM);
    assert_eq!(fixture["claim_id"], "opencode.http.server-window-1");
    assert_eq!(fixture["baseline"], "1.14.48");
    assert_eq!(fixture["latest_qualified"], "1.18.31");
    assert_eq!(fixture["behavior_revision"], "opencode.http-sse.surface-19");
    assert_exact_strings(&fixture["newly_qualified"], &["1.18.31"]);
    assert_eq!(fixture["unverified_newer"], "1.18.32");
    assert_eq!(fixture["newer_version_posture"], "allow_unverified");
    assert_exact_strings(
        &fixture["historical_gaps_preserved"],
        &[
            "1.14.52", "1.15.8", "1.15.14", "1.16.1", "1.16.3", "1.17.21",
        ],
    );
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

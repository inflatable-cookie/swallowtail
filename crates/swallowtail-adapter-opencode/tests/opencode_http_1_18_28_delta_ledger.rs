use std::collections::BTreeSet;

use serde_json::{Map, Value};

#[path = "opencode_http_1_18_28_delta_ledger/identity.rs"]
mod identity;
#[path = "opencode_http_1_18_28_delta_ledger/inventory.rs"]
mod inventory;
#[path = "opencode_http_1_18_28_delta_ledger/protocol.rs"]
mod protocol;

use swallowtail_adapter_opencode::{opencode_http_claim, opencode_server_binding};
use swallowtail_core::InterfaceCompatibilityAssessment;

const IDENTITY: &str = include_str!("fixtures/opencode-1.18.28/identity.json");
const PROTOCOL: &str = include_str!("fixtures/opencode-1.18.28/protocol.json");
const INVENTORY: &str = include_str!("fixtures/opencode-1.18.28/dist-inventory.json");
const CLAIM: &str = include_str!("fixtures/opencode-1.18.28/claim.json");

#[test]
fn admitted_claim_fixture_matches_production_selection() {
    let fixture = json(CLAIM);
    let claim = opencode_http_claim();
    assert_eq!(claim.id().as_str(), fixture["claim_id"]);
    assert_eq!(claim.baseline().as_str(), fixture["baseline"]);
    assert_eq!(
        claim.latest_qualified().as_str(),
        fixture["latest_qualified"]
    );
    assert_exact_strings(
        &fixture["newly_qualified"],
        &[
            "1.18.21", "1.18.22", "1.18.23", "1.18.24", "1.18.25", "1.18.26", "1.18.27", "1.18.28",
        ],
    );
    for version in fixture["newly_qualified"]
        .as_array()
        .expect("version array")
    {
        let binding = opencode_server_binding(version.as_str().expect("version string"))
            .expect("qualified binding");
        let InterfaceCompatibilityAssessment::Qualified(matched) = claim.assess(binding.version())
        else {
            panic!("published hop is not qualified");
        };
        assert_eq!(
            matched.behavior_revision().as_str(),
            fixture["behavior_revision"]
        );
    }
    let later = opencode_server_binding(fixture["unverified_newer"].as_str().unwrap()).unwrap();
    assert!(matches!(
        claim.assess(later.version()),
        InterfaceCompatibilityAssessment::UnverifiedNewer(_)
    ));
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

use serde_json::Value;
use sha2::{Digest, Sha256};
use swallowtail_adapter_claude_agent::claude_agent_acp_claim;
use swallowtail_core::{InterfaceCompatibilityAssessment, InterfaceVersion};

const INVENTORY: &str = include_str!("fixtures/claude-agent-acp-0.109.1/artifact-inventory.json");

#[test]
fn frozen_hops_keep_their_exact_artifact_inventory() {
    assert_eq!(
        hex::encode(Sha256::digest(INVENTORY.as_bytes())),
        "384dcab73106e66854edfccfe25ac82ceda2a5fb5f92e1c8e6d30066813068f8"
    );
}

#[test]
fn frozen_newer_hops_still_require_guarded_qualification() {
    let inventory: Value = serde_json::from_str(INVENTORY).expect("ACP artifact inventory");
    let claim = claude_agent_acp_claim();
    assert_eq!(claim.axis().as_str(), inventory["axis"].as_str().unwrap());
    assert_eq!(claim.latest_qualified().as_str(), "0.106.0");

    let hops = inventory["published_stables_after_ceiling"]
        .as_array()
        .expect("published ACP hops");
    for version in hops.iter().chain(std::iter::once(
        &inventory["first_unpublished_stable_after_latest"],
    )) {
        let version = version.as_str().expect("ACP adapter version");
        let assessed = InterfaceVersion::new(version).expect("valid ACP adapter version");
        assert!(
            matches!(
                claim.assess(&assessed),
                InterfaceCompatibilityAssessment::UnverifiedNewer(_)
            ),
            "ACP {version} requires separate qualification"
        );
    }
}

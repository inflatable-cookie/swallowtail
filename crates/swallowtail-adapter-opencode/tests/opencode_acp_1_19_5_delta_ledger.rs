use std::collections::BTreeSet;

use serde_json::Value;
use swallowtail_adapter_opencode::{
    OPENCODE_ACP_LATEST_QUALIFIED_VERSION, opencode_acp_binding, opencode_acp_claim,
    opencode_http_claim,
};
use swallowtail_core::{InterfaceCompatibilityAssessment, InterfaceSupportStatus};

const IDENTITY: &str = include_str!("fixtures/opencode-acp-1.19.5/identity.json");
const INVENTORY: &str = include_str!("fixtures/opencode-acp-1.19.5/dist-inventory.json");
const PROTOCOL: &str = include_str!("fixtures/opencode-acp-1.19.5/protocol.json");

#[test]
fn acp_identity_extends_v2_without_using_the_registry_2_line() {
    let identity = json(IDENTITY);
    let protocol = json(PROTOCOL);
    let claim = opencode_acp_claim();
    let http = opencode_http_claim();
    let window = &identity["qualification_window"];
    assert_eq!(identity["family"], "opencode.acp");
    assert_eq!(identity["research"], 464);
    assert_eq!(
        identity["official_channels"]["npm_dist_tag_latest"],
        "1.19.5"
    );
    assert_eq!(
        identity["official_channels"]["github_latest_stable_tag"],
        "v1.19.5"
    );
    assert_eq!(identity["official_channels"]["channels_agree"], true);
    assert_eq!(
        identity["official_channels"]["next_stable_not_published"],
        "1.19.6"
    );
    assert_eq!(
        identity["official_channels"]["acp_registry_opencode_version"],
        "2.2.0"
    );
    assert_eq!(
        identity["official_channels"]["acp_registry_used_as_axis"],
        false
    );
    assert_eq!(window["previous_ceiling"], "1.19.4");
    assert_eq!(window["new_ceiling"], "1.19.5");
    assert_eq!(window["unpublished_gap"], "1.18.36");
    assert_eq!(window["v1_segment"], "1.18.18..=1.18.30");
    assert_exact_strings(
        &window["v2_segments"],
        &["1.18.31..=1.18.35", "1.19.0..=1.19.5"],
    );
    assert_eq!(identity["host"]["present"], false);
    assert_eq!(identity["host"]["on_path"], false);
    assert_eq!(
        identity["github_tag_sources"]["1.19.4"]["canonical_tree_sha256"],
        "ae0d6884639fda9deb79082e5b27483c9b6a6f45fb8868dadd89714722cbf287"
    );
    assert_eq!(
        identity["github_tag_sources"]["1.19.5"]["commit"],
        "810c0eba1a7348036f8bdf61e6632d0c36c6876c"
    );
    assert_eq!(
        identity["github_tag_sources"]["1.19.5"]["canonical_tree_sha256"],
        "98d156015c7d31198a689d93fbe18bdc9a11c833228473e8334ee36c9973a56b"
    );
    assert_eq!(
        identity["identity_decision"]["shape"],
        "compatible-extension"
    );
    assert_eq!(
        identity["identity_decision"]["behavior"],
        "opencode.acp-v1.client-mcp-servers-v2"
    );
    assert_eq!(
        identity["identity_decision"]["flatten_onto_opencode_http"],
        false
    );
    assert_eq!(protocol["acp_sdk_pin"], "0.21.0");
    assert_eq!(protocol["protocol_version_constant"], 1);
    assert_eq!(
        protocol["source_slice"]["route_behavior_files_byte_identical"],
        true
    );
    assert_eq!(
        protocol["http_mcp_live_evidence"]["qualified_exact_version"],
        "1.18.18"
    );
    assert_eq!(
        protocol["http_mcp_live_evidence"]["transfer_to_1_19_5"],
        false
    );
    assert_eq!(protocol["provider_prompt_sent"], false);
    assert_eq!(protocol["downloaded_artifact_executed"], false);
    assert_eq!(OPENCODE_ACP_LATEST_QUALIFIED_VERSION, "1.19.5");
    assert_eq!(claim.id().as_str(), "opencode.acp.executable-window-1");
    assert_eq!(claim.axis().as_str(), "opencode.executable");
    assert_ne!(claim.axis().as_str(), http.axis().as_str());
    assert_ne!(claim.id().as_str(), http.id().as_str());
    assert_eq!(claim.latest_qualified().as_str(), "1.19.5");

    let milestones: Vec<_> = claim.milestones().collect();
    assert_eq!(milestones.len(), 3);
    assert_eq!(milestones[0].minimum().as_str(), "1.18.18");
    assert_eq!(milestones[0].maximum().as_str(), "1.18.30");
    assert_eq!(
        milestones[0].support_status(),
        InterfaceSupportStatus::Deprecated
    );
    assert_eq!(milestones[1].minimum().as_str(), "1.18.31");
    assert_eq!(milestones[1].maximum().as_str(), "1.18.35");
    assert_eq!(milestones[2].minimum().as_str(), "1.19.0");
    assert_eq!(milestones[2].maximum().as_str(), "1.19.5");
    assert_eq!(
        milestones[1].behavior_revision().as_str(),
        milestones[2].behavior_revision().as_str()
    );
    assert_eq!(
        milestones[2].support_status(),
        InterfaceSupportStatus::Maintained
    );

    for version in ["1.19.4", "1.19.5"] {
        let binding = opencode_acp_binding(version).unwrap();
        assert!(matches!(
            claim.assess(binding.version()),
            InterfaceCompatibilityAssessment::Qualified(_)
        ));
    }
    let gap = opencode_acp_binding("1.18.36").unwrap();
    assert!(matches!(
        claim.assess(gap.version()),
        InterfaceCompatibilityAssessment::Incompatible
    ));
    let later = opencode_acp_binding("1.19.6").unwrap();
    assert!(matches!(
        claim.assess(later.version()),
        InterfaceCompatibilityAssessment::UnverifiedNewer(_)
    ));
}

#[test]
fn acp_behavior_sources_stay_byte_identical_on_the_1_19_5_hop() {
    let inventory = json(INVENTORY);
    let protocol = json(PROTOCOL);
    let hops = inventory["repository_hops"].as_array().unwrap();
    assert_eq!(hops.len(), 1);
    let hop = &hops[0];
    assert_eq!(hop["hop"], "1.19.4_to_1.19.5");
    assert_eq!(hop["repository_delta_counts"]["added"], 4);
    assert_eq!(hop["repository_delta_counts"]["removed"], 13);
    assert_eq!(hop["repository_delta_counts"]["changed"], 83);
    assert_eq!(hop["repository_delta_counts"]["identical"], 6540);
    assert_eq!(
        hop["repository_path_sha256"]["changed"],
        "4491c152d5e7302a381c25b020e351a15ae651133c37b5647469d76d0bc9f97f"
    );
    assert_exact_strings(
        &hop["changed_selected_files"],
        &["packages/opencode/package.json"],
    );

    let selected = inventory["selected_file_sha256"].as_object().unwrap();
    let behavior = protocol["source_slice"]["route_behavior_files"]
        .as_array()
        .unwrap();
    for path in behavior {
        let path = path.as_str().unwrap();
        let versions = selected[path].as_object().expect(path);
        assert_eq!(versions["1.19.4"], versions["1.19.5"], "{path}");
    }
    let manifest = &selected["packages/opencode/package.json"];
    assert_ne!(manifest["1.19.4"], manifest["1.19.5"]);
}

fn json(input: &str) -> Value {
    serde_json::from_str(input).expect("fixture is valid JSON")
}

fn assert_exact_strings(actual: &Value, expected: &[&str]) {
    let actual = actual
        .as_array()
        .expect("string array")
        .iter()
        .map(|value| value.as_str().expect("string"))
        .collect::<BTreeSet<_>>();
    assert_eq!(actual, expected.iter().copied().collect());
}

use std::collections::BTreeSet;

use serde_json::Value;
use sha2::{Digest, Sha256};
use swallowtail_adapter_opencode::{
    OPENCODE_ACP_LATEST_QUALIFIED_VERSION, opencode_acp_binding, opencode_acp_claim,
    opencode_http_claim,
};
use swallowtail_core::{InterfaceCompatibilityAssessment, InterfaceSupportStatus};

const IDENTITY: &str = include_str!("fixtures/opencode-acp-1.19.5/identity.json");
const INVENTORY: &str = include_str!("fixtures/opencode-acp-1.19.5/dist-inventory.json");
const PROTOCOL: &str = include_str!("fixtures/opencode-acp-1.19.5/protocol.json");
const ACP_1_19_6: &str = include_str!("fixtures/opencode-acp-1.19.6/source-inventory.json");
const SHARED_IDENTITY: &str = include_str!("fixtures/opencode-http-1.19.6/identity.json");
const SHARED_NATIVE: &str = include_str!("fixtures/opencode-http-1.19.6/native-inventory.json");
const SHARED_SOURCE: &str = include_str!("fixtures/opencode-http-1.19.6/source-inventory.json");

#[test]
fn acp_identity_retains_v2_without_using_the_registry_2_line() {
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
    assert_eq!(window["new_ceiling"], "1.19.4");
    assert_eq!(window["unpublished_gap"], "1.18.36");
    assert_eq!(window["v1_segment"], "1.18.18..=1.18.30");
    assert_exact_strings(
        &window["v2_segments"],
        &["1.18.31..=1.18.35", "1.19.0..=1.19.4"],
    );
    assert_eq!(identity["native_payload_correlated_to_tag"], false);
    assert_eq!(identity["attestation_observed"], false);
    assert_eq!(
        identity["native_inventory_scope"],
        "linux-x64 only; wrapper selects 12 native packages"
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
    assert_eq!(identity["identity_decision"]["shape"], "stop");
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
    assert_eq!(OPENCODE_ACP_LATEST_QUALIFIED_VERSION, "1.19.4");
    assert_eq!(claim.id().as_str(), "opencode.acp.executable-window-1");
    assert_eq!(claim.axis().as_str(), "opencode.executable");
    assert_ne!(claim.axis().as_str(), http.axis().as_str());
    assert_ne!(claim.id().as_str(), http.id().as_str());
    assert_eq!(claim.latest_qualified().as_str(), "1.19.4");

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
    assert_eq!(milestones[2].maximum().as_str(), "1.19.4");
    assert_eq!(
        milestones[1].behavior_revision().as_str(),
        milestones[2].behavior_revision().as_str()
    );
    assert_eq!(
        milestones[2].support_status(),
        InterfaceSupportStatus::Maintained
    );

    {
        let version = "1.19.4";
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
    let later = opencode_acp_binding("1.19.5").unwrap();
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

#[test]
fn acp_1_19_6_stop_binds_selected_deltas_to_shared_release_identity() {
    let inventory = json(ACP_1_19_6);
    let shared = json(SHARED_IDENTITY);
    let shared_source = json(SHARED_SOURCE);
    let versions = ["1.19.4", "1.19.5", "1.19.6"];
    assert_eq!(inventory["compared"], serde_json::json!(versions));
    assert_eq!(
        inventory["official_latest"]["npm"],
        shared["official"]["npm_latest"]
    );
    assert_eq!(
        inventory["official_latest"]["github"],
        shared["official"]["github_latest"]
    );
    for version in versions {
        assert_eq!(
            inventory["source_archive_sha256"][version],
            shared_source["source_trees"][version]["source_archive_sha256"]
        );
    }

    let ledger_hashes = &inventory["shared_ledger_sha256"];
    for (name, contents) in [
        ("identity.json", SHARED_IDENTITY),
        ("native-inventory.json", SHARED_NATIVE),
        ("source-inventory.json", SHARED_SOURCE),
    ] {
        assert_eq!(
            ledger_hashes[name],
            format!("{:x}", Sha256::digest(contents))
        );
    }

    let sources = inventory["selected_source_sha256"].as_object().unwrap();
    let acp_paths = sources
        .keys()
        .filter_map(|path| path.strip_prefix("packages/opencode/src/acp/"))
        .collect::<BTreeSet<_>>();
    assert_eq!(
        acp_paths,
        BTreeSet::from([
            "agent.ts",
            "config-option.ts",
            "content.ts",
            "directory.ts",
            "error.ts",
            "event.ts",
            "permission.ts",
            "profile.ts",
            "service.ts",
            "session.ts",
            "tool.ts",
            "usage.ts",
        ])
    );
    let expected_deltas: [&[&str]; 2] = [
        &[
            "bun.lock",
            "packages/core/package.json",
            "packages/opencode/package.json",
            "packages/opencode/src/config/config.ts",
            "packages/opencode/src/config/markdown.ts",
            "packages/opencode/src/session/instruction.ts",
            "packages/opencode/src/session/llm.ts",
            "packages/opencode/src/session/llm/request.ts",
            "packages/opencode/src/session/overflow.ts",
        ],
        &[
            "bun.lock",
            "packages/core/package.json",
            "packages/llm/src/provider-error.ts",
            "packages/opencode/package.json",
            "packages/opencode/src/session/message-v2.ts",
        ],
    ];
    for (before, expected) in expected_deltas.into_iter().enumerate() {
        let changed = sources
            .iter()
            .filter_map(|(path, hashes)| {
                (hashes[before] != hashes[before + 1]).then_some(path.as_str())
            })
            .collect::<BTreeSet<_>>();
        assert_eq!(changed, expected.iter().copied().collect());
    }
    let unchanged_feeders = sources
        .iter()
        .filter(|(path, hashes)| {
            !path.starts_with("packages/opencode/src/acp/")
                && hashes[0] == hashes[1]
                && hashes[1] == hashes[2]
        })
        .map(|(path, _)| path.as_str())
        .collect::<BTreeSet<_>>();
    assert_eq!(
        unchanged_feeders,
        BTreeSet::from([
            "packages/core/src/config/mcp.ts",
            "packages/core/src/plugin/provider/gitlab.ts",
            "packages/core/src/v1/config/mcp.ts",
            "packages/opencode/src/cli/cmd/acp.ts",
            "packages/opencode/src/index.ts",
            "packages/opencode/src/mcp/index.ts",
            "packages/opencode/src/plugin/index.ts",
            "packages/opencode/src/provider/provider.ts",
            "packages/opencode/src/server/routes/instance/httpapi/groups/mcp.ts",
            "packages/opencode/src/server/routes/instance/httpapi/handlers/config.ts",
            "packages/opencode/src/server/routes/instance/httpapi/handlers/mcp.ts",
            "packages/opencode/src/server/routes/instance/httpapi/handlers/session.ts",
            "packages/opencode/src/session/compaction.ts",
            "packages/opencode/src/session/processor.ts",
            "packages/opencode/src/session/prompt.ts",
        ])
    );
    assert_eq!(inventory["acp_sdk_pin"], "0.21.0");
    assert_eq!(
        inventory["gitlab_provider_pins"],
        serde_json::json!(["6.19.0", "6.23.1", "6.23.1"])
    );
}

#[test]
fn acp_keeps_both_unproved_hops_above_the_maintained_window() {
    let inventory = json(ACP_1_19_6);
    let claim = opencode_acp_claim();
    assert_eq!(inventory["family"], "opencode.acp");
    assert_eq!(inventory["axis"], claim.axis().as_str());
    assert_eq!(
        inventory["qualified_ceiling"],
        claim.latest_qualified().as_str()
    );
    assert_eq!(inventory["unpublished_gap"], "1.18.36");
    assert_eq!(inventory["first_unpublished_later_stable"], "1.19.7");
    assert_eq!(inventory["http_mcp_live_exact_version"], "1.18.18");
    assert_eq!(
        inventory["qualification_gates"],
        serde_json::json!({
            "1.19.5": ["gitlab_catalogue_and_turn"],
            "1.19.6": ["gitlab_catalogue_and_turn", "overflow_recovery_and_image_history"],
        })
    );
    for version in ["1.19.5", "1.19.6", "1.19.7"] {
        let binding = opencode_acp_binding(version).expect("valid 1.x version");
        assert!(
            matches!(
                claim.assess(binding.version()),
                InterfaceCompatibilityAssessment::UnverifiedNewer(_)
            ),
            "{version}"
        );
    }
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

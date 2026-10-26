use serde_json::Value;
use sha2::{Digest, Sha256};
use swallowtail_adapter_cursor::{
    cursor_acp_claim, cursor_agent_release_binding, cursor_catalogue_claim, cursor_headless_claim,
};
use swallowtail_core::InterfaceCompatibilityAssessment;

const IDENTITY: &str =
    include_str!("fixtures/cursor-agent-acp-headless-2026.10.22/identity.json");
const PROTOCOL: &str =
    include_str!("fixtures/cursor-agent-acp-headless-2026.10.22/protocol.json");
const DIST_INVENTORY: &str =
    include_str!("fixtures/cursor-agent-acp-headless-2026.10.22/dist-inventory.json");

fn json(source: &str) -> Value {
    serde_json::from_str(source).expect("Cursor 2026.10.22 fixture is valid JSON")
}

fn sha256(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

fn version(value: &str) -> swallowtail_core::InterfaceVersion {
    swallowtail_core::InterfaceVersion::new(value).expect("fixture version is valid")
}

#[test]
fn identity_freezes_registry_hops_and_keeps_the_routes_separate() {
    let identity = json(IDENTITY);
    let protocol = json(PROTOCOL);
    assert_eq!(identity["axis"], "cursor-agent.release-date");
    assert_eq!(identity["observed_at"], "2026-10-26");
    assert_eq!(identity["registry"]["current_version"], "2026.10.22");
    assert_eq!(identity["registry"]["build_revision"], "27dcf5f");
    assert_eq!(
        identity["previous_claim_at_observation"]["acp_latest_qualified_build"],
        "2026.10.01-14929f9"
    );
    assert_eq!(
        identity["previous_claim_at_observation"]["headless_latest_qualified_build"],
        "2026.10.01-14929f9"
    );
    assert_eq!(
        identity["published_registry_builds_after_ceiling"],
        serde_json::json!([
            "2026.10.14-0f9edde",
            "2026.10.14-941492e",
            "2026.10.16-e3cb207",
            "2026.10.18-ea085cb",
            "2026.10.19-6bbf2c4",
            "2026.10.22-27dcf5f"
        ])
    );
    assert_eq!(
        identity["qualified_exact_builds_added"],
        serde_json::json!([
            "2026.10.14-941492e",
            "2026.10.16-e3cb207",
            "2026.10.18-ea085cb",
            "2026.10.19-6bbf2c4",
            "2026.10.22-27dcf5f"
        ])
    );
    assert_eq!(
        identity["superseded_same_date_build_rejected"]["version"],
        "2026.10.14-0f9edde"
    );
    assert_eq!(
        identity["general_installer_channel"]["observed_build"],
        "2026.10.22-27dcf5f"
    );
    assert_eq!(
        identity["general_installer_channel"]["agrees_with_registry_target"],
        true
    );
    assert_eq!(identity["host_observation"]["cursor_agent_on_path"], false);
    assert_eq!(identity["host_observation"]["installation_changed"], false);

    let decision = &identity["identity_decision"];
    assert_eq!(decision["shape"], "compatible-extension");
    assert_eq!(
        decision["keep_acp_claim_id"],
        "cursor-agent.acp.release-window-3"
    );
    assert_eq!(
        decision["keep_headless_claim_id"],
        "cursor-agent.headless.release-window-3"
    );
    assert_eq!(
        decision["keep_acp_behavior_revision"],
        "cursor-agent.acp-v1.interactive-v1"
    );
    assert_eq!(
        decision["keep_headless_behavior_revision"],
        "cursor-agent.stream-json.structured-v1"
    );
    assert_eq!(decision["raise_acp_ceiling_to"], "2026.10.22-27dcf5f");
    assert_eq!(decision["raise_headless_ceiling_to"], "2026.10.22-27dcf5f");
    assert_eq!(decision["catalogue_claim_changed"], false);
    assert_eq!(
        decision["catalogue_ceiling_unchanged"],
        "2026.10.01-14929f9"
    );
    assert_eq!(decision["flatten_routes"], false);
    assert_eq!(decision["new_public_operation"], false);
    assert_eq!(decision["provider_prompt_sent"], false);
    assert_eq!(decision["downloaded_artifacts_executed"], false);
    assert_eq!(decision["synthetic_unverified_newer"], "2026-10-23");

    assert_eq!(
        protocol["acp"]["initialize_shape"]["protocol_version_constant"],
        1
    );
    assert_eq!(
        protocol["acp"]["initialize_shape"]["auth_methods"],
        serde_json::json!(["cursor_login"])
    );
    assert_eq!(protocol["acp"]["load_session_posture"], "advertised-without-proven-replay");
    assert_eq!(protocol["acp"]["continuation_recovery"], "blocked");
    assert_eq!(protocol["headless"]["mode_choices"], serde_json::json!(["plan", "ask"]));
    assert_eq!(protocol["headless"]["selected_wire_shape_unchanged"], true);
    assert_eq!(protocol["catalogue_or_cross_route_transfer"], false);
    assert_eq!(protocol["downloaded_artifacts_executed"], false);

    let hops = identity["hops"].as_array().expect("hops");
    assert_eq!(hops.len(), 7);
    assert_eq!(hops[0]["version"], "2026.10.01-14929f9");
    assert_eq!(
        hops[0]["darwin_arm64_archive_sha256"],
        "778d04e542adc5c8b6760fda3ebe0757f903b1764f2792c232ef9a35e6e2151b"
    );
    assert_eq!(
        hops[0]["linux_x64_archive_sha256"],
        "ba9a855f8f813c91b9f2707127572d2dc9ae5a62818e1c36719625d0fb8bd452"
    );
    assert_eq!(hops[6]["version"], "2026.10.22-27dcf5f");
    assert_eq!(
        hops[6]["darwin_arm64_archive_sha256"],
        "2f5b0a3dd5fb951cecffac813867d8a08d810e59837d59fd5f12d69b922398d7"
    );
    assert_eq!(
        hops[6]["linux_x64_archive_sha256"],
        "85296944e4600a83c4b764f578d1502bf9c416db200d33a593e8f3ecabfdc294"
    );
    assert_eq!(
        hops[6]["darwin_arm64_acp_command_chunk"],
        "dist-package/7096.index.js"
    );
    assert_eq!(
        hops[6]["linux_x64_headless_writer_chunk"],
        "dist-package/7000.index.js"
    );
    for hop in hops {
        assert_eq!(
            hop["launcher_sha256"],
            "2ccc9a8e167797641448b5e5c936f006ba137a2555f117f38c5eb76a5238a233"
        );
        for field in [
            "darwin_arm64_archive_sha256",
            "linux_x64_archive_sha256",
            "darwin_arm64_acp_command_chunk_sha256",
            "linux_x64_headless_writer_chunk_sha256",
            "darwin_arm64_runtime_index_sha256",
        ] {
            let digest = hop[field].as_str().expect(field);
            assert_eq!(digest.len(), 64, "{field}");
        }
    }
}

#[test]
fn hop_ledgers_are_mutation_sensitive_and_claims_follow_the_decision() {
    let inventory = json(DIST_INVENTORY);
    assert_eq!(
        inventory["compared"],
        serde_json::json!([
            "2026.10.01-14929f9",
            "2026.10.14-0f9edde",
            "2026.10.14-941492e",
            "2026.10.16-e3cb207",
            "2026.10.18-ea085cb",
            "2026.10.19-6bbf2c4",
            "2026.10.22-27dcf5f"
        ])
    );
    assert_eq!(inventory["package_file_counts"]["darwin-arm64"]["2026.10.01-14929f9"], 449);
    assert_eq!(inventory["package_file_counts"]["linux-x64"]["2026.10.22-27dcf5f"], 445);
    assert_eq!(inventory["not_a_complete_semantic_changelog"], true);

    for platform in ["darwin-arm64", "linux-x64"] {
        let hops = inventory["hop_deltas"][platform]
            .as_object()
            .expect("platform hop ledger");
        assert_eq!(hops.len(), 6, "{platform}");
        for (name, hop) in hops {
            for category in ["added", "removed", "changed", "identical"] {
                let paths = hop[category]["paths"]
                    .as_array()
                    .expect("path set");
                assert_eq!(
                    paths.len(),
                    usize::try_from(hop[category]["count"].as_u64().expect("count")).expect("count"),
                    "{platform} {name} {category}"
                );
                assert!(
                    paths.windows(2).all(|pair| pair[0].as_str().unwrap() < pair[1].as_str().unwrap()),
                    "{platform} {name} {category} sorted"
                );
                assert_eq!(
                    sha256(&serde_json::to_vec(paths).expect("path set encodes")),
                    hop[category]["sha256"],
                    "{platform} {name} {category}"
                );
            }
        }
    }

    let acp = cursor_acp_claim();
    let headless = cursor_headless_claim();
    let catalogue = cursor_catalogue_claim();
    assert_ne!(acp.id(), headless.id());
    assert_ne!(acp.id(), catalogue.id());
    for release in ["2026-10-14", "2026-10-16", "2026-10-18", "2026-10-19", "2026-10-22"] {
        assert!(acp.supports(&version(release)), "{release}");
        assert!(headless.supports(&version(release)), "{release}");
        assert!(!catalogue.supports(&version(release)), "{release}");
    }
    assert!(cursor_agent_release_binding("2026.10.22-27dcf5f").is_some());
    assert!(cursor_agent_release_binding("2026.10.14-941492e").is_some());
    assert!(cursor_agent_release_binding("2026.10.14-0f9edde").is_none());
    for gap in ["2026-10-02", "2026-10-15", "2026-10-21"] {
        assert!(!acp.permits(&version(gap)), "{gap}");
        assert!(!headless.permits(&version(gap)), "{gap}");
    }
    let InterfaceCompatibilityAssessment::UnverifiedNewer(newer) = acp.assess(&version("2026-10-23"))
    else {
        panic!("2026-10-23 stays unverified newer");
    };
    assert_eq!(newer.latest_qualified().as_str(), "2026-10-22");
    let InterfaceCompatibilityAssessment::UnverifiedNewer(newer) =
        catalogue.assess(&version("2026-10-22"))
    else {
        panic!("catalogue stays unverified newer above 2026-10-01");
    };
    assert_eq!(newer.latest_qualified().as_str(), "2026-10-01");
}

use serde_json::Value;
use std::collections::BTreeSet;
use swallowtail_adapter_claude_agent::{
    CLAUDE_AGENT_ACP_AXIS, CLAUDE_AGENT_ACP_BASELINE_VERSION,
    CLAUDE_AGENT_ACP_LATEST_QUALIFIED_VERSION, claude_agent_acp_claim,
};
use swallowtail_core::{
    InterfaceCompatibilityAssessment, InterfaceSupportStatus, InterfaceVersion,
};

const IDENTITY: &str = include_str!("fixtures/claude-agent-acp-0.96.0/identity.json");
const PROTOCOL: &str = include_str!("fixtures/claude-agent-acp-0.96.0/protocol.json");
const DIST_INVENTORY: &str = include_str!("fixtures/claude-agent-acp-0.96.0/dist-inventory.json");

#[test]
fn identity_and_claim_qualify_current_0_96_0_with_exact_hop_evidence() {
    let identity: Value = serde_json::from_str(IDENTITY).expect("identity is valid JSON");
    let protocol: Value = serde_json::from_str(PROTOCOL).expect("protocol is valid JSON");
    let inventory: Value = serde_json::from_str(DIST_INVENTORY).expect("inventory is valid JSON");

    assert_eq!(fingerprint(IDENTITY), 0x8534_21db_4a48_71f7);
    assert_eq!(fingerprint(PROTOCOL), 0x73a5_7e6f_b50b_1b6b);
    assert_eq!(fingerprint(DIST_INVENTORY), 0x0e04_ea1f_af81_94ab);
    assert_keys(
        &identity,
        &[
            "axis",
            "npm_package",
            "not_claude_code",
            "not_claude_agent_sdk",
            "observed_at",
            "official_channel_observation",
            "official_current",
            "previous_ceiling",
            "published_stable_hops_from_previous_ceiling",
            "preview_only_patch_lines",
            "stable_holes_preserved",
            "claim_at_observation",
            "identity_decision",
            "separate_gates",
            "inspection_boundary",
        ],
    );
    assert_keys(
        &protocol,
        &[
            "selected_routes",
            "selected_protocol_version",
            "swallowtail_initialize_request",
            "official_current_runtime",
            "stable_schema_and_lifecycle",
            "tool_exchange",
            "permissions_and_elicitation",
            "configuration_and_usage",
            "bounded_unmapped",
            "separate_live_gate",
            "selected_compatible_because",
            "provider_prompt_sent",
            "live_acp_initialize",
        ],
    );

    assert_eq!(identity["axis"], CLAUDE_AGENT_ACP_AXIS);
    assert_eq!(
        identity["npm_package"],
        "@agentclientprotocol/claude-agent-acp"
    );
    assert_eq!(identity["not_claude_code"], true);
    assert_eq!(identity["not_claude_agent_sdk"], true);
    assert_eq!(
        identity["official_channel_observation"]["npm_latest"],
        "0.96.0"
    );
    assert_eq!(
        identity["official_channel_observation"]["npm_preview"],
        "0.96.1-preview.7"
    );
    assert_eq!(
        identity["official_channel_observation"]["github_latest_stable_tag"],
        "v0.96.0"
    );
    assert_eq!(
        identity["official_channel_observation"]["host_binary"],
        "not-on-path"
    );
    assert_eq!(
        identity["official_channel_observation"]["channels_agree"],
        true
    );
    assert_eq!(identity["official_current"]["version"], "0.96.0");
    assert_eq!(
        identity["official_current"]["acp_registry_version"],
        "0.96.0"
    );
    assert_eq!(identity["official_current"]["gitHead_matches_tag"], true);
    assert_eq!(
        identity["official_current"]["tarball_sha256"],
        "cb70069d2538e481795ca1853888107fd53c0754dea78814467ba1e26f72b9d6"
    );
    assert_eq!(identity["official_current"]["acp_sdk"], "2.0.2");
    assert_eq!(identity["official_current"]["agent_sdk"], "0.3.310");
    assert_eq!(identity["previous_ceiling"], "0.87.0");
    assert_eq!(
        identity["claim_at_observation"]["latest_qualified"],
        "0.87.0"
    );
    assert_eq!(
        identity["published_stable_hops_from_previous_ceiling"],
        serde_json::json!([
            "0.88.0", "0.89.0", "0.89.1", "0.90.0", "0.91.0", "0.92.0", "0.93.0", "0.94.0",
            "0.95.0", "0.96.0"
        ])
    );
    assert_eq!(
        identity["preview_only_patch_lines"],
        serde_json::json!([
            "0.87.1", "0.88.1", "0.89.2", "0.90.1", "0.91.1", "0.92.1", "0.93.1", "0.94.1",
            "0.95.1"
        ])
    );
    assert_eq!(
        identity["stable_holes_preserved"],
        serde_json::json!(["0.52.0", "0.58.0"])
    );
    assert_eq!(
        identity["identity_decision"]["shape"],
        "compatible-extension"
    );
    assert_eq!(
        identity["identity_decision"]["extend_v7"],
        "0.66.0..=0.96.0"
    );
    assert_eq!(
        identity["identity_decision"]["raise_latest_qualified_to"],
        "0.96.0"
    );
    assert_eq!(
        identity["identity_decision"]["keep_claim_id"],
        "claude-agent.acp.window-2"
    );
    assert_eq!(
        identity["identity_decision"]["later_stable_unverified"],
        "0.97.0"
    );
    assert_eq!(identity["identity_decision"]["new_milestone"], false);
    assert_eq!(
        identity["identity_decision"]["new_public_mapped_operation"],
        false
    );
    assert_eq!(
        identity["separate_gates"]["http_mcp_honouring_exact_points"],
        serde_json::json!(["0.79.0", "0.81.2"])
    );
    assert_eq!(
        identity["separate_gates"]["http_mcp_honouring_extended"],
        false
    );
    assert_eq!(identity["separate_gates"]["dependency_pin_changed"], false);
    assert_eq!(
        identity["separate_gates"]["claude_code_claims_unchanged"],
        true
    );
    assert_eq!(
        identity["separate_gates"]["claude_agent_sdk_claim_unchanged"],
        true
    );
    assert_eq!(
        identity["inspection_boundary"]["official_binary_executed"],
        false
    );
    assert_eq!(
        identity["inspection_boundary"]["provider_prompt_sent"],
        false
    );
    assert_eq!(identity["inspection_boundary"]["credentials_used"], false);

    assert_eq!(protocol["selected_protocol_version"], 1);
    assert_eq!(
        protocol["swallowtail_initialize_request"]["advertises_air"],
        false
    );
    assert_eq!(
        protocol["swallowtail_initialize_request"]["advertises_async_tasks"],
        false
    );
    assert_eq!(
        protocol["swallowtail_initialize_request"]["advertises_session_index"],
        false
    );
    assert_eq!(
        protocol["official_current_runtime"]["auto_routes_protocol_version_1_to_v1"],
        true
    );
    assert_eq!(
        protocol["official_current_runtime"]["swallowtail_requests_v2"],
        false
    );
    assert_eq!(
        protocol["stable_schema_and_lifecycle"]["acp_sdk_major_at_0_94_0_is_import_path_not_wire_reset"],
        true
    );
    assert_eq!(
        protocol["stable_schema_and_lifecycle"]["missing_close_returns_empty_success_from"],
        "0.89.0"
    );
    assert_eq!(
        protocol["stable_schema_and_lifecycle"]["unstored_delete_throws_via_agent_sdk_deleteSession_through"],
        "0.90.0"
    );
    assert_eq!(
        protocol["stable_schema_and_lifecycle"]["unstored_delete_returns_empty_success_via_deleteSessionIfStored_from"],
        "0.91.0"
    );
    assert_eq!(
        protocol["stable_schema_and_lifecycle"]["no_public_lifecycle_break"],
        true
    );
    assert_eq!(
        protocol["stable_schema_and_lifecycle"]["cancel_stop_reason_stays_cancelled"],
        true
    );
    assert_eq!(
        protocol["permissions_and_elicitation"]["swallowtail_one_shot_allow_or_reject_subset_unchanged"],
        true
    );
    assert_eq!(
        protocol["configuration_and_usage"]["model_option_id_stays"],
        "model"
    );
    assert_eq!(
        protocol["bounded_unmapped"]["experimental_v2_unqualified"],
        true
    );
    assert_eq!(
        protocol["separate_live_gate"]["newer_http_mcp_live_evidence"],
        false
    );
    assert_eq!(protocol["provider_prompt_sent"], false);
    assert_eq!(protocol["live_acp_initialize"], false);

    assert_keys(
        &inventory,
        &[
            "compared",
            "inventory_scope",
            "package_artifacts",
            "tree_sha256",
            "file_deltas",
            "classification_rules",
            "no_removed_files",
            "source_review_basis",
        ],
    );
    assert_eq!(
        inventory["compared"],
        serde_json::json!([
            "0.87.0", "0.88.0", "0.89.0", "0.89.1", "0.90.0", "0.91.0", "0.92.0", "0.93.0",
            "0.94.0", "0.95.0", "0.96.0"
        ])
    );
    assert_eq!(inventory["no_removed_files"], true);
    assert_eq!(inventory["package_artifacts"].as_array().unwrap().len(), 11);
    assert_eq!(inventory["tree_sha256"].as_object().unwrap().len(), 11);
    assert_eq!(inventory["file_deltas"].as_array().unwrap().len(), 10);
    for (artifact, expected) in inventory["package_artifacts"]
        .as_array()
        .unwrap()
        .iter()
        .zip([
            ("0.87.0", 219),
            ("0.88.0", 219),
            ("0.89.0", 261),
            ("0.89.1", 261),
            ("0.90.0", 267),
            ("0.91.0", 270),
            ("0.92.0", 276),
            ("0.93.0", 276),
            ("0.94.0", 276),
            ("0.95.0", 282),
            ("0.96.0", 282),
        ])
    {
        assert_eq!(artifact["version"], expected.0);
        assert_eq!(artifact["file_count"], expected.1);
        assert_eq!(artifact["gitHead_matches_tag"], true);
        assert_sha256(artifact["tarball_sha256"].as_str().unwrap());
        assert_sha256(artifact["path_set_sha256"].as_str().unwrap());
    }

    let expected_hops = [
        ("0.87.0", "0.88.0", 0, 0, 16, 14, 2),
        ("0.88.0", "0.89.0", 42, 0, 34, 19, 57),
        ("0.89.0", "0.89.1", 0, 0, 16, 1, 15),
        ("0.89.1", "0.90.0", 6, 0, 51, 17, 40),
        ("0.90.0", "0.91.0", 3, 0, 44, 17, 30),
        ("0.91.0", "0.92.0", 6, 0, 32, 10, 28),
        ("0.92.0", "0.93.0", 0, 0, 15, 6, 9),
        ("0.93.0", "0.94.0", 0, 0, 129, 58, 71),
        ("0.94.0", "0.95.0", 6, 0, 53, 17, 42),
        ("0.95.0", "0.96.0", 0, 0, 15, 7, 8),
    ];
    for (delta, (from, to, added, removed, changed, mapped, internal)) in inventory["file_deltas"]
        .as_array()
        .unwrap()
        .iter()
        .zip(expected_hops)
    {
        assert_eq!(delta["from"], from);
        assert_eq!(delta["to"], to);
        assert_eq!(delta["added"].as_array().unwrap().len(), added);
        assert_eq!(delta["removed"].as_array().unwrap().len(), removed);
        assert_eq!(delta["changed"].as_array().unwrap().len(), changed);
        for file in delta["added"].as_array().unwrap() {
            assert_keys(file, &["path", "sha256", "classification", "reason"]);
            assert_sha256(file["sha256"].as_str().unwrap());
        }
        for file in delta["changed"].as_array().unwrap() {
            assert_keys(
                file,
                &[
                    "path",
                    "before_sha256",
                    "after_sha256",
                    "classification",
                    "reason",
                ],
            );
            assert_sha256(file["before_sha256"].as_str().unwrap());
            assert_sha256(file["after_sha256"].as_str().unwrap());
        }
        let classified = delta["added"]
            .as_array()
            .unwrap()
            .iter()
            .chain(delta["changed"].as_array().unwrap());
        let mut mapped_count = 0;
        let mut internal_count = 0;
        for file in classified {
            match file["classification"].as_str().unwrap() {
                "mapped_adjacent" => mapped_count += 1,
                "provider_internal" => internal_count += 1,
                other => panic!("unexpected file classification: {other}"),
            }
        }
        assert_eq!((mapped_count, internal_count), (mapped, internal));
    }

    assert_eq!(CLAUDE_AGENT_ACP_BASELINE_VERSION, "0.53.0");
    assert_eq!(CLAUDE_AGENT_ACP_LATEST_QUALIFIED_VERSION, "0.98.0");
    let claim = claude_agent_acp_claim();
    assert_eq!(claim.id().as_str(), "claude-agent.acp.window-2");
    for version in [
        "0.88.0", "0.89.0", "0.89.1", "0.90.0", "0.91.0", "0.92.0", "0.93.0", "0.94.0", "0.95.0",
        "0.96.0", "0.97.0", "0.98.0",
    ] {
        assert!(matches!(
            claim.assess(&version_value(version)),
            InterfaceCompatibilityAssessment::Qualified(matched)
                if matched.support_status() == InterfaceSupportStatus::Maintained
                    && matched.behavior_revision().as_str()
                        == "claude-agent.acp.initialize-meta-extensions-v7"
        ));
    }
    for excluded in ["0.52.0", "0.58.0", "0.96.1-preview.7"] {
        assert!(!claim.permits(&version_value(excluded)));
    }
    assert!(matches!(
        claim.assess(&version_value("0.99.0")),
        InterfaceCompatibilityAssessment::UnverifiedNewer(_)
    ));
}

fn assert_keys(value: &Value, expected: &[&str]) {
    let actual = value
        .as_object()
        .expect("expected JSON object")
        .keys()
        .map(String::as_str)
        .collect::<BTreeSet<_>>();
    let expected = expected.iter().copied().collect::<BTreeSet<_>>();
    assert_eq!(actual, expected);
}

fn assert_sha256(value: &str) {
    assert_eq!(value.len(), 64);
    assert!(value.bytes().all(|byte| byte.is_ascii_hexdigit()));
}

fn fingerprint(value: &str) -> u64 {
    value.bytes().fold(0xcbf2_9ce4_8422_2325, |hash, byte| {
        (hash ^ u64::from(byte)).wrapping_mul(0x0000_0100_0000_01b3)
    })
}

fn version_value(value: &str) -> InterfaceVersion {
    InterfaceVersion::new(value).expect("fixture version is valid")
}

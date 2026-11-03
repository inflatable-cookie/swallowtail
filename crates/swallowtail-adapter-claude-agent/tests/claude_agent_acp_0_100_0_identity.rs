use serde_json::Value;
use std::collections::BTreeSet;
use swallowtail_adapter_claude_agent::{
    CLAUDE_AGENT_ACP_AXIS, CLAUDE_AGENT_ACP_BASELINE_VERSION,
    CLAUDE_AGENT_ACP_LATEST_QUALIFIED_VERSION, claude_agent_acp_claim,
};
use swallowtail_core::{
    InterfaceCompatibilityAssessment, InterfaceSupportStatus, InterfaceVersion,
};

const IDENTITY: &str = include_str!("fixtures/claude-agent-acp-0.100.0/identity.json");
const PROTOCOL: &str = include_str!("fixtures/claude-agent-acp-0.100.0/protocol.json");
const DIST_INVENTORY: &str = include_str!("fixtures/claude-agent-acp-0.100.0/dist-inventory.json");

#[test]
fn identity_and_claim_qualify_current_0_100_0_with_exact_hop_evidence() {
    let identity: Value = serde_json::from_str(IDENTITY).expect("identity is valid JSON");
    let protocol: Value = serde_json::from_str(PROTOCOL).expect("protocol is valid JSON");
    let inventory: Value = serde_json::from_str(DIST_INVENTORY).expect("inventory is valid JSON");

    assert_eq!(fingerprint(IDENTITY), 0xdf65_7178_8c79_a45b);
    assert_eq!(fingerprint(PROTOCOL), 0xfb1c_6c45_b837_906e);
    assert_eq!(fingerprint(DIST_INVENTORY), 0x26cd_69df_e251_dd50);
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
            "previous_ceiling_tarball_sha256",
            "published_stable_hops_from_previous_ceiling",
            "published_stable_hops_note",
            "preview_only_patch_lines",
            "ignored_preview_tag",
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
            "byte_identical_selected_files_0_96_0_through_0_100_0",
            "byte_identical_selected_files_note",
            "delete_session_if_stored_sha256_unchanged",
            "delete_session_if_stored_body_unchanged_through",
            "stable_schema_and_lifecycle",
            "failure_codes",
            "tool_exchange",
            "permissions_and_elicitation",
            "configuration_and_usage",
            "available_commands",
            "bounded_unmapped",
            "separate_live_gate",
            "selected_compatible_because",
            "provider_prompt_sent",
            "live_acp_initialize",
        ],
    );
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

    assert_eq!(identity["axis"], CLAUDE_AGENT_ACP_AXIS);
    assert_eq!(
        identity["npm_package"],
        "@agentclientprotocol/claude-agent-acp"
    );
    assert_eq!(identity["not_claude_code"], true);
    assert_eq!(identity["not_claude_agent_sdk"], true);
    assert_eq!(identity["observed_at"], "2026-11-03");
    assert_eq!(
        identity["official_channel_observation"]["npm_latest"],
        "0.100.0"
    );
    assert_eq!(
        identity["official_channel_observation"]["npm_preview"],
        "0.100.1-preview.1"
    );
    assert_eq!(
        identity["official_channel_observation"]["github_latest_stable_tag"],
        "v0.100.0"
    );
    assert_eq!(
        identity["official_channel_observation"]["acp_registry_version"],
        "0.100.0"
    );
    assert_eq!(
        identity["official_channel_observation"]["host_binary"],
        "not-on-path-in-this-observation-environment"
    );
    assert_eq!(
        identity["official_channel_observation"]["channels_agree"],
        true
    );
    assert_eq!(identity["official_current"]["version"], "0.100.0");
    assert_eq!(identity["official_current"]["gitHead_matches_tag"], true);
    assert_eq!(
        identity["official_current"]["tarball_sha256"],
        "bb1de8a11cb585a4e8da2fd1a0252aa613ac9cd2cdc47a23247c9847c6d132b8"
    );
    assert_eq!(identity["official_current"]["acp_sdk"], "2.4.0");
    assert_eq!(identity["official_current"]["agent_sdk"], "0.3.316");
    assert_eq!(identity["previous_ceiling"], "0.98.0");
    assert_eq!(
        identity["claim_at_observation"]["latest_qualified"],
        "0.98.0"
    );
    assert_eq!(
        identity["identity_decision"]["shape"],
        "compatible-extension"
    );
    assert_eq!(
        identity["identity_decision"]["extend_v7"],
        "0.66.0..=0.100.0"
    );
    assert_eq!(
        identity["identity_decision"]["raise_latest_qualified_to"],
        "0.100.0"
    );
    assert_eq!(
        identity["identity_decision"]["keep_claim_id"],
        "claude-agent.acp.window-2"
    );
    assert_eq!(
        identity["identity_decision"]["later_stable_unverified"],
        "0.100.1"
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
        protocol["swallowtail_initialize_request"]["advertises_session_index"],
        false
    );
    assert_eq!(
        protocol["swallowtail_initialize_request"]["advertises_boolean_config_options"],
        false
    );
    assert_eq!(
        protocol["swallowtail_initialize_request"]["advertises_plan_operations"],
        false
    );
    assert_eq!(
        protocol["swallowtail_initialize_request"]["sends_additional_directories"],
        false
    );
    assert_eq!(
        protocol["swallowtail_initialize_request"]["prompt_content_types_sent"],
        serde_json::json!(["text"])
    );
    assert_eq!(
        protocol["official_current_runtime"]["auto_routes_protocol_version_1_to_v1"],
        true
    );
    assert_eq!(
        protocol["official_current_runtime"]["serve_js_byte_identical_from_0_96_0"],
        true
    );
    assert_eq!(
        protocol["byte_identical_selected_files_0_96_0_through_0_100_0"],
        serde_json::json!(["dist/index.js", "dist/serve.js", "dist/tools.js"])
    );
    assert_eq!(
        protocol["stable_schema_and_lifecycle"]["deleteSessionIfStored_body_unchanged_through"],
        "0.100.0"
    );
    assert_eq!(
        protocol["stable_schema_and_lifecycle"]["no_public_lifecycle_break"],
        true
    );
    assert_eq!(
        protocol["tool_exchange"]["v1_permission_subject_status_stays_pending"],
        true
    );
    assert_eq!(
        protocol["tool_exchange"]["cancelled_permission_outcome_never_sent_by_swallowtail"],
        true
    );
    assert_eq!(
        protocol["tool_exchange"]["v2_paused_tool_status_v2_only"],
        true
    );
    assert_eq!(
        protocol["permissions_and_elicitation"]["elicitation_create_request_and_response_shapes_unchanged"],
        true
    );
    assert_eq!(
        protocol["configuration_and_usage"]["model_option_id_stays"],
        "model"
    );
    assert_eq!(
        protocol["configuration_and_usage"]["ultracode_option_id"],
        "ultracode"
    );
    assert_eq!(
        protocol["configuration_and_usage"]["swallowtail_does_not_select_ultracode"],
        true
    );
    assert_eq!(
        protocol["configuration_and_usage"]["current_value_round_trip_of_listed_values_unchanged"],
        true
    );
    assert_eq!(
        protocol["configuration_and_usage"]["clear_context_usage_update_site_from"],
        "0.100.0"
    );
    assert_eq!(
        protocol["available_commands"]["name_filter_only_removes_entries"],
        true
    );
    assert_eq!(
        protocol["bounded_unmapped"]["experimental_v2_unqualified"],
        true
    );
    assert_eq!(
        protocol["bounded_unmapped"]["background_work_enabled_only_with_v2_background_option"],
        true
    );
    assert_eq!(
        protocol["separate_live_gate"]["newer_http_mcp_live_evidence"],
        false
    );
    assert_eq!(protocol["provider_prompt_sent"], false);
    assert_eq!(protocol["live_acp_initialize"], false);
    assert_sha256(
        protocol["official_current_runtime"]["index_js_sha256_unchanged_from_0_96_0"]
            .as_str()
            .unwrap(),
    );
    assert_sha256(
        protocol["delete_session_if_stored_sha256_unchanged"]
            .as_str()
            .unwrap(),
    );

    assert_eq!(
        inventory["compared"],
        serde_json::json!(["0.98.0", "0.99.0", "0.99.1", "0.100.0"])
    );
    assert_eq!(inventory["no_removed_files"], true);
    assert_eq!(inventory["package_artifacts"].as_array().unwrap().len(), 4);
    assert_eq!(inventory["tree_sha256"].as_object().unwrap().len(), 4);
    assert_eq!(inventory["file_deltas"].as_array().unwrap().len(), 3);
    for (artifact, expected) in inventory["package_artifacts"]
        .as_array()
        .unwrap()
        .iter()
        .zip([
            ("0.98.0", 285, "2.0.2", "0.3.310"),
            ("0.99.0", 288, "2.1.0", "0.3.312"),
            ("0.99.1", 288, "2.1.0", "0.3.312"),
            ("0.100.0", 297, "2.4.0", "0.3.316"),
        ])
    {
        assert_eq!(artifact["version"], expected.0);
        assert_eq!(artifact["file_count"], expected.1);
        assert_eq!(artifact["gitHead_matches_tag"], true);
        assert_eq!(artifact["acp_sdk"], expected.2);
        assert_eq!(artifact["agent_sdk"], expected.3);
        assert_sha256(artifact["tarball_sha256"].as_str().unwrap());
        assert_sha256(artifact["path_set_sha256"].as_str().unwrap());
    }

    let expected_hops = [
        ("0.98.0", "0.99.0", 3, 0, 37, 248),
        ("0.99.0", "0.99.1", 0, 0, 12, 276),
        ("0.99.1", "0.100.0", 9, 0, 53, 235),
    ];
    for (delta, (from, to, added, removed, changed, unchanged)) in inventory["file_deltas"]
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
        assert_eq!(delta["unchanged_file_count"], unchanged);
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
        for file in delta["added"]
            .as_array()
            .unwrap()
            .iter()
            .chain(delta["changed"].as_array().unwrap())
        {
            match file["classification"].as_str().unwrap() {
                "mapped_adjacent" | "provider_internal" => {}
                other => panic!("unexpected file classification: {other}"),
            }
        }
    }
    let tree = &inventory["tree_sha256"]["0.100.0"];
    assert_eq!(
        tree["dist/acp-agent.js"],
        "2170737a5531b0c08e5cc8b3190491fcf01d7e4e58b6d670e1049c6e5c27a09f"
    );
    assert_eq!(
        tree["dist/index.js"],
        "dac87af861bedf6ee1444e99832274d6f517f5e83006f3905094cfc4479b3b00"
    );
    assert_eq!(
        tree["dist/session-index/service.js"],
        "c088c7abc760431f1cec1952725423191cc29ac59e157969eb48ac9b79afb567"
    );

    assert_eq!(CLAUDE_AGENT_ACP_BASELINE_VERSION, "0.53.0");
    assert_eq!(CLAUDE_AGENT_ACP_LATEST_QUALIFIED_VERSION, "0.100.0");
    let claim = claude_agent_acp_claim();
    assert_eq!(claim.id().as_str(), "claude-agent.acp.window-2");
    for version in ["0.96.0", "0.97.0", "0.98.0", "0.99.0", "0.99.1", "0.100.0"] {
        assert!(matches!(
            claim.assess(&version_value(version)),
            InterfaceCompatibilityAssessment::Qualified(matched)
                if matched.support_status() == InterfaceSupportStatus::Maintained
                    && matched.behavior_revision().as_str()
                        == "claude-agent.acp.initialize-meta-extensions-v7"
        ));
    }
    for excluded in ["0.52.0", "0.58.0", "0.100.1-preview.1", "0.99.2-preview.13"] {
        assert!(!claim.permits(&version_value(excluded)));
    }
    assert!(matches!(
        claim.assess(&version_value("0.100.1")),
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

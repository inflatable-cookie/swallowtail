use serde_json::Value;
use swallowtail_adapter_claude_agent::{
    CLAUDE_AGENT_ACP_AXIS, CLAUDE_AGENT_ACP_BASELINE_VERSION,
    CLAUDE_AGENT_ACP_LATEST_QUALIFIED_VERSION, claude_agent_acp_binding, claude_agent_acp_claim,
};
use swallowtail_core::{
    InterfaceCompatibilityAssessment, InterfaceSupportStatus, InterfaceVersion,
};

const IDENTITY: &str = include_str!("fixtures/claude-agent-acp-0.76.0/identity.json");
const PROTOCOL: &str = include_str!("fixtures/claude-agent-acp-0.76.0/protocol.json");

#[test]
fn identity_and_claim_qualify_0_76_0_as_compatible_extension() {
    let identity: Value =
        serde_json::from_str(IDENTITY).expect("Claude Agent 0.76.0 identity corpus is valid JSON");
    let protocol: Value =
        serde_json::from_str(PROTOCOL).expect("Claude Agent 0.76.0 protocol corpus is valid JSON");

    assert_eq!(identity["axis"], CLAUDE_AGENT_ACP_AXIS);
    assert_eq!(
        identity["npm_package"],
        "@agentclientprotocol/claude-agent-acp"
    );
    assert_eq!(identity["npm_latest"], true);
    assert_eq!(identity["not_claude_code"], true);
    assert_eq!(identity["official_binary_executed"], false);
    assert_eq!(identity["previous_ceiling"], "0.73.0");
    assert_eq!(identity["host"]["version"], "0.63.0");
    assert_eq!(identity["host"]["matches_frozen_0_70_host_digest"], true);
    assert_eq!(identity["host"]["installed_or_updated"], false);
    assert_eq!(identity["host"]["observed_beyond_version_flag"], false);
    assert_eq!(identity["official"]["version"], "0.76.0");
    assert_eq!(
        identity["official"]["published_at"],
        "2026-09-09T21:16:32.982Z"
    );
    assert_eq!(
        identity["official"]["npm_integrity"],
        "sha512-8Xh1ryud5b1fOGLfF+S7dW2hhIZ/h4WopKtD//JRcQWxu7dutZvzN/MhRSDjqRcxqf3UIZRx1K0OhnlY/hMaOg=="
    );
    assert_eq!(
        identity["official"]["github_commit"],
        "c2e4815029ef3962787ecaefe208b0b6f8b81302"
    );
    assert_eq!(identity["official"]["gitHead_matches_tag"], true);
    assert_eq!(identity["official"]["acp_sdk"], "1.4.0");
    assert_eq!(identity["official"]["agent_sdk"], "0.3.257");
    assert_eq!(identity["official"]["acp_registry_version"], "0.76.0");
    assert_eq!(
        identity["previous_ceiling_0_73_0"]["tarball_sha256"],
        "eb03d0c6c1934726535d5c6b8defd3b37c2c2d77f5d9d037d3a97d624ea733c3"
    );
    assert_eq!(
        identity["previous_ceiling_0_73_0"]["matches_frozen_0_73_0_tarball"],
        true
    );
    assert_eq!(
        identity["published_hop_0_74_0"]["github_commit"],
        "847a3b642929654b5bf32b143b3e1efd5884d331"
    );
    assert_eq!(
        identity["published_hop_0_75_0"]["github_commit"],
        "a0122dae4c6f0d19a6716561683607dac4f50215"
    );
    assert_eq!(
        identity["published_hop_0_75_1"]["github_commit"],
        "3e23c5b960b66a6d2c892e7524c952e731c076a7"
    );
    assert_eq!(
        identity["published_hop_0_74_0"]["gitHead_matches_tag"],
        true
    );
    assert_eq!(
        identity["published_hop_0_75_0"]["gitHead_matches_tag"],
        true
    );
    assert_eq!(
        identity["published_hop_0_75_1"]["gitHead_matches_tag"],
        true
    );
    assert_eq!(
        identity["published_stables_from_previous_ceiling"],
        serde_json::json!(["0.74.0", "0.75.0", "0.75.1", "0.76.0"])
    );
    assert!(is_sha256(
        identity["official"]["tarball_sha256"]
            .as_str()
            .expect("tarball digest is text")
    ));
    assert_eq!(identity["unpublished_0_58_0"], true);
    assert_eq!(identity["unpublished_0_73_1"], true);
    assert_eq!(identity["unpublished_0_74_1"], true);
    assert_eq!(identity["unpublished_0_75_2"], true);
    assert_eq!(identity["unpublished_0_76_1"], true);
    assert_eq!(identity["unpublished_0_77_0"], true);
    assert_eq!(identity["prerelease_only_0_75_2"], true);
    assert_eq!(identity["prerelease_only_0_76_1"], true);

    let observation = &identity["claim_at_observation"];
    assert_eq!(observation["baseline"], "0.53.0");
    assert_eq!(observation["latest_qualified"], "0.73.0");
    assert_eq!(observation["posture"], "allow_unverified");
    assert_eq!(
        observation["excluded"],
        serde_json::json!(["0.52.0", "0.58.0"])
    );
    assert_eq!(observation["classification_of_0_74_0"], "unverified_newer");
    assert_eq!(observation["classification_of_0_75_0"], "unverified_newer");
    assert_eq!(observation["classification_of_0_75_1"], "unverified_newer");
    assert_eq!(observation["classification_of_0_76_0"], "unverified_newer");

    let decision = &identity["identity_decision"];
    assert_eq!(decision["shape"], "compatible-extension");
    assert_eq!(decision["extend_v7"], "0.66.0..=0.76.0");
    assert_eq!(
        decision["v7_behavior"],
        "claude-agent.acp.initialize-meta-extensions-v7"
    );
    assert_eq!(decision["raise_latest_qualified_to"], "0.76.0");
    assert_eq!(
        decision["qualify_published_hops"],
        serde_json::json!(["0.74.0", "0.75.0", "0.75.1", "0.76.0"])
    );
    assert_eq!(decision["keep_baseline"], "0.53.0");
    assert_eq!(decision["keep_exclusion_0_58_0"], true);
    assert_eq!(decision["keep_allow_unverified"], true);
    assert_eq!(decision["map_hide_claude_auth_flag"], false);
    assert_eq!(decision["map_auth_status_extension"], false);
    assert_eq!(decision["map_context_compaction_tool_meta"], false);
    assert_eq!(decision["map_usage_markdown"], false);
    assert_eq!(decision["map_recommended_config_values"], false);
    assert_eq!(decision["map_fork_meta"], false);
    assert_eq!(decision["flatten_to_claude_code"], false);
    assert_eq!(decision["new_public_mapped_operation"], false);
    assert_eq!(decision["later_unverified_after_qualification"], "0.77.0");
    assert_eq!(decision["provider_prompt_sent"], false);
    assert_eq!(decision["live_acp_initialize"], false);
    assert_eq!(decision["host_install_changed"], false);

    assert_eq!(protocol["index_js_identical_0_70_0_through_0_76_0"], true);
    assert_eq!(
        protocol["elicitation_js_identical_0_64_0_through_0_76_0"],
        true
    );
    assert_eq!(protocol["protocol_version"], 1);
    assert_eq!(protocol["wire_protocol_version_unchanged"], true);
    assert_eq!(protocol["acp_sdk_all_hops"], "1.4.0");
    assert_eq!(protocol["agent_sdk_all_hops"], "0.3.257");
    assert_eq!(
        protocol["permissions_tree_byte_identical_0_73_0_through_0_76_0"],
        true
    );
    assert_eq!(
        protocol["session_mode_js_identical_0_73_0_through_0_76_0"],
        true
    );
    assert_eq!(
        protocol["unmapped_0_74_0"]["hide_claude_auth_flag_never_passed_by_swallowtail"],
        true
    );
    assert_eq!(
        protocol["unmapped_0_75_0"]["auth_status_notifications_underscore_prefixed_and_ignored"],
        true
    );
    assert_eq!(
        protocol["unmapped_0_76_0"]["swallowtail_does_not_advertise_recommended_value_capability"],
        true
    );
    assert_eq!(
        protocol["unmapped_0_76_0"]["legacy_config_option_shape_preserved_without_capability"],
        true
    );
    assert_eq!(protocol["provider_prompt_sent"], false);

    assert_eq!(CLAUDE_AGENT_ACP_BASELINE_VERSION, "0.53.0");
    assert_eq!(CLAUDE_AGENT_ACP_LATEST_QUALIFIED_VERSION, "0.76.0");
    assert_eq!(
        identity["claim_at_observation"]["latest_qualified"],
        "0.73.0"
    );
    assert_eq!(
        identity["identity_decision"]["raise_latest_qualified_to"],
        CLAUDE_AGENT_ACP_LATEST_QUALIFIED_VERSION
    );

    let claim = claude_agent_acp_claim();
    assert!(matches!(
        claim.assess(&version_value("0.63.0")),
        InterfaceCompatibilityAssessment::Qualified(matched)
            if matched.support_status() == InterfaceSupportStatus::Deprecated
    ));
    for version in [
        "0.66.0", "0.69.0", "0.70.0", "0.71.0", "0.72.0", "0.73.0", "0.74.0", "0.75.0", "0.75.1",
        "0.76.0",
    ] {
        assert!(matches!(
            claim.assess(&version_value(version)),
            InterfaceCompatibilityAssessment::Qualified(matched)
                if matched.support_status() == InterfaceSupportStatus::Maintained
                    && matched.behavior_revision().as_str()
                        == "claude-agent.acp.initialize-meta-extensions-v7"
        ));
    }
    assert!(!claim.permits(&version_value("0.58.0")));
    assert!(matches!(
        claim.assess(&version_value("0.77.0")),
        InterfaceCompatibilityAssessment::UnverifiedNewer(_)
    ));
    assert_eq!(
        claude_agent_acp_binding("0.76.0")
            .expect("version binds")
            .axis()
            .as_str(),
        CLAUDE_AGENT_ACP_AXIS
    );
}

fn is_sha256(value: &str) -> bool {
    value.len() == 64 && value.bytes().all(|byte| byte.is_ascii_hexdigit())
}

fn version_value(value: &str) -> InterfaceVersion {
    InterfaceVersion::new(value).expect("fixture version is valid")
}

use serde_json::Value;
use swallowtail_adapter_claude_agent::{
    CLAUDE_AGENT_ACP_AXIS, CLAUDE_AGENT_ACP_BASELINE_VERSION, claude_agent_acp_binding,
    claude_agent_acp_claim,
};
use swallowtail_core::{
    InterfaceCompatibilityAssessment, InterfaceSupportStatus, InterfaceVersion,
};

const IDENTITY: &str = include_str!("fixtures/claude-agent-acp-0.79.0/identity.json");
const PROTOCOL: &str = include_str!("fixtures/claude-agent-acp-0.79.0/protocol.json");

#[test]
fn identity_and_claim_qualify_0_79_0_as_compatible_extension() {
    let identity: Value =
        serde_json::from_str(IDENTITY).expect("Claude Agent 0.79.0 identity corpus is valid JSON");
    let protocol: Value =
        serde_json::from_str(PROTOCOL).expect("Claude Agent 0.79.0 protocol corpus is valid JSON");

    assert_eq!(identity["axis"], CLAUDE_AGENT_ACP_AXIS);
    assert_eq!(
        identity["npm_package"],
        "@agentclientprotocol/claude-agent-acp"
    );
    assert_eq!(identity["npm_latest"], true);
    assert_eq!(identity["not_claude_code"], true);
    assert_eq!(identity["official_binary_executed"], false);
    assert_eq!(identity["previous_ceiling"], "0.76.0");
    assert_eq!(identity["host"]["present"], false);
    assert_eq!(identity["host"]["on_path"], false);
    assert_eq!(identity["host"]["installed_or_updated"], false);
    assert_eq!(identity["host"]["observed_beyond_version_flag"], false);
    assert_eq!(identity["official"]["version"], "0.79.0");
    assert_eq!(
        identity["official"]["published_at"],
        "2026-09-17T14:55:56.043Z"
    );
    assert_eq!(
        identity["official"]["npm_integrity"],
        "sha512-/liYDBHElfzgbeijv8EZzvDUUAC8wUi1WSCZ+bw/DIKHQ3t3DLid+LS+6lszuQamHAWijjaZl1i5xZVRTnNPoA=="
    );
    assert_eq!(
        identity["official"]["github_commit"],
        "d421f56a6c43cde16d9a7531d08a750a5ef2f04a"
    );
    assert_eq!(identity["official"]["gitHead_matches_tag"], true);
    assert_eq!(identity["official"]["acp_sdk"], "1.4.0");
    assert_eq!(identity["official"]["agent_sdk"], "0.3.274");
    assert_eq!(identity["official"]["acp_registry_version"], "0.79.0");
    assert_eq!(
        identity["previous_ceiling_0_76_0"]["tarball_sha256"],
        "b836570348c9400062e39f4d5866734b5fe89613d80b5cf1c47c9446c1b4eafd"
    );
    assert_eq!(
        identity["previous_ceiling_0_76_0"]["matches_frozen_0_76_0_tarball"],
        true
    );
    assert_eq!(
        identity["published_hop_0_77_0"]["github_commit"],
        "dfe823b9581979cd22db40272d4469cc7e42b77e"
    );
    assert_eq!(
        identity["published_hop_0_78_0"]["github_commit"],
        "becd854dd813af769475cf66e081d48c3f25f081"
    );
    assert_eq!(
        identity["published_hop_0_77_0"]["gitHead_matches_tag"],
        true
    );
    assert_eq!(
        identity["published_hop_0_78_0"]["gitHead_matches_tag"],
        true
    );
    assert_eq!(
        identity["published_stables_from_previous_ceiling"],
        serde_json::json!(["0.77.0", "0.78.0", "0.79.0"])
    );
    assert!(is_sha256(
        identity["official"]["tarball_sha256"]
            .as_str()
            .expect("tarball digest is text")
    ));
    assert_eq!(identity["unpublished_0_58_0"], true);
    assert_eq!(identity["unpublished_0_76_1"], true);
    assert_eq!(identity["unpublished_0_77_1"], true);
    assert_eq!(identity["unpublished_0_78_1"], true);
    assert_eq!(identity["unpublished_0_80_0"], true);
    assert_eq!(identity["prerelease_only_0_77_1"], true);
    assert_eq!(identity["prerelease_only_0_78_1"], true);

    let observation = &identity["claim_at_observation"];
    assert_eq!(observation["baseline"], "0.53.0");
    assert_eq!(observation["latest_qualified"], "0.76.0");
    assert_eq!(observation["posture"], "allow_unverified");
    assert_eq!(
        observation["excluded"],
        serde_json::json!(["0.52.0", "0.58.0"])
    );
    assert_eq!(observation["classification_of_0_77_0"], "unverified_newer");
    assert_eq!(observation["classification_of_0_78_0"], "unverified_newer");
    assert_eq!(observation["classification_of_0_79_0"], "unverified_newer");

    let decision = &identity["identity_decision"];
    assert_eq!(decision["shape"], "compatible-extension");
    assert_eq!(decision["extend_v7"], "0.66.0..=0.79.0");
    assert_eq!(
        decision["v7_behavior"],
        "claude-agent.acp.initialize-meta-extensions-v7"
    );
    assert_eq!(decision["raise_latest_qualified_to"], "0.79.0");
    assert_eq!(
        decision["qualify_published_hops"],
        serde_json::json!(["0.77.0", "0.78.0", "0.79.0"])
    );
    assert_eq!(decision["keep_baseline"], "0.53.0");
    assert_eq!(decision["keep_exclusion_0_58_0"], true);
    assert_eq!(decision["keep_allow_unverified"], true);
    assert_eq!(decision["widen_elicitation_other_descriptions"], true);
    assert_eq!(decision["map_agent_config_option"], false);
    assert_eq!(decision["map_compaction_update_kinds"], false);
    assert_eq!(decision["flatten_to_claude_code"], false);
    assert_eq!(decision["flatten_to_claude_agent_sdk"], false);
    assert_eq!(decision["new_public_mapped_operation"], false);
    assert_eq!(decision["later_unverified_after_qualification"], "0.80.0");
    assert_eq!(decision["provider_prompt_sent"], false);
    assert_eq!(decision["live_acp_initialize"], false);
    assert_eq!(decision["host_install_changed"], false);

    assert_eq!(protocol["index_js_identical_0_70_0_through_0_79_0"], true);
    assert_eq!(protocol["protocol_version"], 1);
    assert_eq!(protocol["wire_protocol_version_unchanged"], true);
    assert_eq!(protocol["acp_sdk_all_hops"], "1.4.0");
    assert_eq!(protocol["agent_sdk_0_79_0"], "0.3.274");
    assert_eq!(
        protocol["unmapped_0_77_0"]["swallowtail_never_mapped_agent_config_option"],
        true
    );
    assert_eq!(
        protocol["unmapped_0_78_0"]["swallowtail_does_not_advertise_session_compaction"],
        true
    );
    assert_eq!(
        protocol["unmapped_0_79_0"]["do_not_flatten_to_claude_code_or_sdk_family"],
        true
    );
    assert_eq!(
        protocol["selected_compatible_because"]
            ["already_mapped_elicitation_accepts_new_other_descriptions"],
        true
    );
    assert_eq!(protocol["provider_prompt_sent"], false);

    assert_eq!(CLAUDE_AGENT_ACP_BASELINE_VERSION, "0.53.0");
    assert_eq!(
        identity["claim_at_observation"]["latest_qualified"],
        "0.76.0"
    );
    assert_eq!(
        identity["identity_decision"]["raise_latest_qualified_to"],
        "0.79.0"
    );

    let claim = claude_agent_acp_claim();
    assert!(matches!(
        claim.assess(&version_value("0.63.0")),
        InterfaceCompatibilityAssessment::Qualified(matched)
            if matched.support_status() == InterfaceSupportStatus::Deprecated
    ));
    for version in [
        "0.66.0", "0.69.0", "0.70.0", "0.71.0", "0.72.0", "0.73.0", "0.74.0", "0.75.0", "0.75.1",
        "0.76.0", "0.77.0", "0.78.0", "0.79.0",
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
        claim.assess(&version_value("0.80.0")),
        InterfaceCompatibilityAssessment::Qualified(matched)
            if matched.support_status() == InterfaceSupportStatus::Maintained
    ));
    assert_eq!(
        claude_agent_acp_binding("0.79.0")
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

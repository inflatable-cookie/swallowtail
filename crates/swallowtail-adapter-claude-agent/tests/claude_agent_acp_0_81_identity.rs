use serde_json::Value;
use swallowtail_adapter_claude_agent::{
    CLAUDE_AGENT_ACP_AXIS, CLAUDE_AGENT_ACP_BASELINE_VERSION,
    CLAUDE_AGENT_ACP_LATEST_QUALIFIED_VERSION, claude_agent_acp_binding, claude_agent_acp_claim,
};
use swallowtail_core::{
    InterfaceCompatibilityAssessment, InterfaceSupportStatus, InterfaceVersion,
};

const IDENTITY: &str = include_str!("fixtures/claude-agent-acp-0.81.2/identity.json");
const PROTOCOL: &str = include_str!("fixtures/claude-agent-acp-0.81.2/protocol.json");

#[test]
fn identity_and_claim_qualify_0_81_2_as_compatible_extension() {
    let identity: Value =
        serde_json::from_str(IDENTITY).expect("Claude Agent 0.81.2 identity corpus is valid JSON");
    let protocol: Value =
        serde_json::from_str(PROTOCOL).expect("Claude Agent 0.81.2 protocol corpus is valid JSON");

    assert_eq!(identity["axis"], CLAUDE_AGENT_ACP_AXIS);
    assert_eq!(
        identity["npm_package"],
        "@agentclientprotocol/claude-agent-acp"
    );
    assert_eq!(identity["npm_latest"], true);
    assert_eq!(identity["not_claude_code"], true);
    assert_eq!(identity["official_binary_executed"], false);
    assert_eq!(identity["previous_ceiling"], "0.79.0");
    assert_eq!(identity["host"]["present"], false);
    assert_eq!(identity["host"]["on_path"], false);
    assert_eq!(identity["host"]["installed_or_updated"], false);
    assert_eq!(identity["host"]["observed_beyond_version_flag"], false);
    assert_eq!(identity["official"]["version"], "0.81.2");
    assert_eq!(
        identity["official"]["published_at"],
        "2026-09-24T10:18:05.741Z"
    );
    assert_eq!(
        identity["official"]["npm_integrity"],
        "sha512-/yvpesq8e6Jv8kl5PHEhwbKRVvzVudXD+ujC0q6PZFmrZK0+xPtiVQYtZIdlUtdhgNKBgBTsGwZkbb2Iw+rbjA=="
    );
    assert_eq!(
        identity["official"]["github_commit"],
        "5dbb453c63a89746627799b2b06b31ba01a1b674"
    );
    assert_eq!(identity["official"]["gitHead_matches_tag"], true);
    assert_eq!(identity["official"]["acp_sdk"], "1.5.0");
    assert_eq!(identity["official"]["agent_sdk"], "0.3.280");
    assert_eq!(identity["official"]["acp_registry_id"], "claude-acp");
    assert_eq!(identity["official"]["acp_registry_version"], "0.81.2");
    assert_eq!(
        identity["official"]["tarball_sha256"],
        "15368e30e06789df93c057d910247e15a3b38f59bddd039c2e44a0e946341b0c"
    );
    assert_eq!(
        identity["previous_ceiling_0_79_0"]["tarball_sha256"],
        "8c7a692b0266389eb7d81d4cb836c1f21293fb6a0ed11bff2e15db42c36177cb"
    );
    assert_eq!(
        identity["previous_ceiling_0_79_0"]["matches_frozen_0_79_0_tarball"],
        true
    );
    assert_eq!(
        identity["published_hop_0_80_0"]["github_commit"],
        "16c9d1a3af6bd81e5a8b07420b4bb17bfda22ffb"
    );
    assert_eq!(
        identity["published_hop_0_81_0"]["github_commit"],
        "d571358e267ed21ed0ec9ec2622fda8935535d57"
    );
    assert_eq!(
        identity["published_hop_0_81_1"]["github_commit"],
        "b264b52bee80e49f20caf1941f7d7cb89edb80c4"
    );
    assert_eq!(identity["published_hop_0_80_0"]["gitHead_matches_tag"], true);
    assert_eq!(identity["published_hop_0_81_0"]["gitHead_matches_tag"], true);
    assert_eq!(identity["published_hop_0_81_1"]["gitHead_matches_tag"], true);
    assert_eq!(
        identity["published_stables_from_previous_ceiling"],
        serde_json::json!(["0.80.0", "0.81.0", "0.81.1", "0.81.2"])
    );
    assert_eq!(identity["unpublished_0_58_0"], true);
    assert_eq!(identity["unpublished_0_79_1"], true);
    assert_eq!(identity["unpublished_0_80_1"], true);
    assert_eq!(identity["unpublished_0_81_3"], true);
    assert_eq!(identity["prerelease_only_0_79_1"], true);
    assert_eq!(identity["prerelease_only_0_80_1"], true);

    let observation = &identity["claim_at_observation"];
    assert_eq!(observation["baseline"], "0.53.0");
    assert_eq!(observation["latest_qualified"], "0.79.0");
    assert_eq!(observation["posture"], "allow_unverified");
    assert_eq!(
        observation["excluded"],
        serde_json::json!(["0.52.0", "0.58.0"])
    );
    assert_eq!(observation["classification_of_0_80_0"], "unverified_newer");
    assert_eq!(observation["classification_of_0_81_0"], "unverified_newer");
    assert_eq!(observation["classification_of_0_81_1"], "unverified_newer");
    assert_eq!(observation["classification_of_0_81_2"], "unverified_newer");

    let decision = &identity["identity_decision"];
    assert_eq!(decision["shape"], "compatible-extension");
    assert_eq!(decision["extend_v7"], "0.66.0..=0.81.2");
    assert_eq!(
        decision["v7_behavior"],
        "claude-agent.acp.initialize-meta-extensions-v7"
    );
    assert_eq!(decision["raise_latest_qualified_to"], "0.81.2");
    assert_eq!(
        decision["qualify_published_hops"],
        serde_json::json!(["0.80.0", "0.81.0", "0.81.1", "0.81.2"])
    );
    assert_eq!(decision["keep_baseline"], "0.53.0");
    assert_eq!(decision["keep_exclusion_0_58_0"], true);
    assert_eq!(decision["keep_allow_unverified"], true);
    assert_eq!(decision["acp_sdk_runtime_byte_identical"], true);
    assert_eq!(decision["map_notice_session_update"], false);
    assert_eq!(decision["map_usage_model_meta"], false);
    assert_eq!(decision["flatten_to_claude_code"], false);
    assert_eq!(decision["flatten_to_claude_agent_sdk"], false);
    assert_eq!(decision["new_public_mapped_operation"], false);
    assert_eq!(decision["later_unverified_after_qualification"], "0.81.3");
    assert_eq!(decision["provider_prompt_sent"], false);
    assert_eq!(decision["live_acp_initialize"], false);
    assert_eq!(decision["host_install_changed"], false);

    assert_eq!(protocol["protocol_version"], 1);
    assert_eq!(protocol["wire_protocol_version_unchanged"], true);
    assert_eq!(protocol["acp_sdk_0_79_0"], "1.4.0");
    assert_eq!(protocol["acp_sdk_0_80_0_through_0_81_2"], "1.5.0");
    assert_eq!(
        protocol["acp_sdk_runtime_acp_js_byte_identical_1_4_0_and_1_5_0"],
        true
    );
    assert_eq!(
        protocol["acp_sdk_runtime_acp_js_sha256"],
        "cc717d74b018c1fe3e1e53e31ff6355ccf52e729e753666051bda42499be7b9f"
    );
    assert_eq!(protocol["notice_requires_client_session_notices"], true);
    assert_eq!(
        protocol["swallowtail_does_not_advertise_session_notices"],
        true
    );
    assert_eq!(
        protocol["selected_compatible_because"]["elicitation_byte_identical"],
        true
    );
    assert_eq!(protocol["provider_prompt_sent"], false);

    assert_eq!(CLAUDE_AGENT_ACP_BASELINE_VERSION, "0.53.0");
    assert_eq!(CLAUDE_AGENT_ACP_LATEST_QUALIFIED_VERSION, "0.81.2");
    assert_eq!(
        identity["claim_at_observation"]["latest_qualified"],
        "0.79.0"
    );
    assert_eq!(
        identity["identity_decision"]["raise_latest_qualified_to"],
        CLAUDE_AGENT_ACP_LATEST_QUALIFIED_VERSION
    );

    let claim = claude_agent_acp_claim();
    for version in [
        "0.66.0", "0.79.0", "0.80.0", "0.81.0", "0.81.1", "0.81.2",
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
        claim.assess(&version_value("0.81.3")),
        InterfaceCompatibilityAssessment::UnverifiedNewer(_)
    ));
    assert_eq!(
        claude_agent_acp_binding("0.81.2")
            .expect("version binds")
            .axis()
            .as_str(),
        CLAUDE_AGENT_ACP_AXIS
    );
}

fn version_value(value: &str) -> InterfaceVersion {
    InterfaceVersion::new(value).expect("fixture version is non-empty")
}

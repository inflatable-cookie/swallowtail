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
    assert_eq!(identity["host"]["present"], true);
    assert_eq!(identity["host"]["on_path"], true);
    assert_eq!(identity["host"]["version"], "0.63.0");
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
    assert_eq!(identity["official"]["acp_registry_version"], "0.81.2");
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
    assert_eq!(
        identity["published_hop_0_80_0"]["gitHead_matches_tag"],
        true
    );
    assert_eq!(
        identity["published_hop_0_81_0"]["gitHead_matches_tag"],
        true
    );
    assert_eq!(
        identity["published_hop_0_81_1"]["gitHead_matches_tag"],
        true
    );
    assert_eq!(
        identity["published_stables_from_previous_ceiling"],
        serde_json::json!(["0.80.0", "0.81.0", "0.81.1", "0.81.2"])
    );
    assert!(is_sha256(
        identity["official"]["tarball_sha256"]
            .as_str()
            .expect("tarball digest is text")
    ));
    assert_eq!(identity["unpublished_0_58_0"], true);
    assert_eq!(identity["unpublished_0_79_1"], true);
    assert_eq!(identity["unpublished_0_80_1"], true);
    assert_eq!(identity["unpublished_0_81_3"], true);
    assert_eq!(identity["prerelease_only_0_79_1"], true);
    assert_eq!(identity["prerelease_only_0_80_1"], true);
    assert_eq!(identity["prerelease_only_0_81_3"], true);

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
    assert_eq!(decision["http_mcp_honouring_stays_exact_0_79_0"], true);
    assert_eq!(
        decision["close_session_teardown_close_query_stream_identical_through_hops"],
        true
    );
    assert_eq!(
        decision["mcp_servers_http_mapping_identical_through_hops"],
        true
    );
    assert_eq!(
        decision["cancel_adds_optional_compaction_interrupt_noop_on_selected_route"],
        true
    );
    assert_eq!(decision["query_interrupt_await_still_unbounded"], true);
    assert_eq!(decision["map_session_notices"], false);
    assert_eq!(decision["map_managed_policy"], false);
    assert_eq!(decision["flatten_to_claude_code"], false);
    assert_eq!(decision["flatten_to_claude_agent_sdk"], false);
    assert_eq!(decision["new_public_mapped_operation"], false);
    assert_eq!(decision["later_unverified_after_qualification"], "0.82.0");
    assert_eq!(decision["provider_prompt_sent"], false);
    assert_eq!(decision["live_acp_initialize"], false);
    assert_eq!(decision["host_install_changed"], false);

    assert_eq!(protocol["protocol_version"], 1);
    assert_eq!(protocol["wire_protocol_version_unchanged"], true);
    assert_eq!(protocol["acp_sdk_0_80_0_through_0_81_2"], "1.5.0");
    assert_eq!(protocol["acp_sdk_schema_protocol_version_stays_1"], true);
    assert_eq!(protocol["agent_sdk_0_81_0_through_0_81_2"], "0.3.280");
    assert_eq!(
        protocol["close_session_identical_0_79_0_through_0_81_2"],
        true
    );
    assert_eq!(
        protocol["mcp_servers_http_mapping_identical_0_79_0_through_0_81_2"],
        true
    );
    assert_eq!(protocol["http_mcp_honouring_stays_exact_0_79_0"], true);
    assert_eq!(
        protocol["unmapped_0_80_0"]["cancel_optional_compaction_interrupt_noop_without_session_compaction"],
        true
    );
    assert_eq!(
        protocol["unmapped_0_81_0"]["swallowtail_does_not_advertise_session_notices"],
        true
    );
    assert_eq!(
        protocol["unmapped_0_81_1"]["usage_model_rides_in_underscore_meta"],
        true
    );
    assert_eq!(
        protocol["selected_compatible_because"]["close_teardown_and_mcp_http_mapping_unchanged"],
        true
    );
    assert_eq!(protocol["provider_prompt_sent"], false);

    assert_eq!(CLAUDE_AGENT_ACP_BASELINE_VERSION, "0.53.0");
    assert_eq!(CLAUDE_AGENT_ACP_LATEST_QUALIFIED_VERSION, "0.81.2");
    assert_ne!(
        CLAUDE_AGENT_ACP_LATEST_QUALIFIED_VERSION, "0.79.0",
        "raising the qualified ceiling must not reuse the Research 361 honouring pin"
    );
    assert_eq!(
        identity["claim_at_observation"]["latest_qualified"],
        "0.79.0"
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
        "0.76.0", "0.77.0", "0.78.0", "0.79.0", "0.80.0", "0.81.0", "0.81.1", "0.81.2",
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
        claim.assess(&version_value("0.82.0")),
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

fn is_sha256(value: &str) -> bool {
    value.len() == 64 && value.bytes().all(|byte| byte.is_ascii_hexdigit())
}

fn version_value(value: &str) -> InterfaceVersion {
    InterfaceVersion::new(value).expect("fixture version is valid")
}

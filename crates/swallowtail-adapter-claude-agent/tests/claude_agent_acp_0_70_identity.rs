use serde_json::Value;
use swallowtail_adapter_claude_agent::{
    CLAUDE_AGENT_ACP_AXIS, CLAUDE_AGENT_ACP_BASELINE_VERSION,
    CLAUDE_AGENT_ACP_LATEST_QUALIFIED_VERSION, claude_agent_acp_binding, claude_agent_acp_claim,
};
use swallowtail_core::{
    InterfaceCompatibilityAssessment, InterfaceSupportStatus, InterfaceVersion,
};

const IDENTITY: &str = include_str!("fixtures/claude-agent-acp-0.70.0/identity.json");
const PROTOCOL: &str = include_str!("fixtures/claude-agent-acp-0.70.0/protocol.json");

#[test]
fn identity_and_claim_qualify_0_70_0_as_compatible_extension() {
    let identity: Value =
        serde_json::from_str(IDENTITY).expect("Claude Agent 0.70.0 identity corpus is valid JSON");
    let protocol: Value =
        serde_json::from_str(PROTOCOL).expect("Claude Agent 0.70.0 protocol corpus is valid JSON");

    assert_eq!(identity["axis"], CLAUDE_AGENT_ACP_AXIS);
    assert_eq!(
        identity["npm_package"],
        "@agentclientprotocol/claude-agent-acp"
    );
    assert_eq!(identity["npm_latest"], true);
    assert_eq!(identity["not_claude_code"], true);
    assert_eq!(identity["host"]["version"], "0.63.0");
    assert_eq!(identity["official"]["version"], "0.70.0");
    assert_eq!(
        identity["official"]["npm_integrity"],
        "sha512-Psqj6fhV4pQ8IM480zpJ+xGiMMIqNLxlsTj5Mzn+T8KSURCVNJdl0ktcqLMjgHJC/QnOvDdDkFf3xTW9VIV9aQ=="
    );
    assert_eq!(
        identity["official"]["github_commit"],
        "d0aafb1ca26427285ffaeac8d8a4452fff28e9c3"
    );
    assert!(is_sha256(
        identity["host"]["executable_sha256"]
            .as_str()
            .expect("host digest is text")
    ));
    assert_eq!(identity["unpublished_0_69_1"], true);
    assert_eq!(identity["unpublished_0_70_1"], true);
    assert_eq!(identity["unpublished_0_58_0"], true);

    let decision = &identity["identity_decision"];
    assert_eq!(decision["shape"], "compatible-extension");
    assert_eq!(decision["extend_v7"], "0.66.0..=0.70.0");
    assert_eq!(
        decision["v7_behavior"],
        "claude-agent.acp.initialize-meta-extensions-v7"
    );
    assert_eq!(decision["raise_latest_qualified_to"], "0.70.0");
    assert_eq!(decision["keep_baseline"], "0.53.0");
    assert_eq!(decision["keep_exclusion_0_58_0"], true);
    assert_eq!(decision["map_goal_air_file_change"], false);
    assert_eq!(decision["map_providers_api"], false);
    assert_eq!(decision["flatten_to_claude_code"], false);
    assert_eq!(decision["new_public_mapped_operation"], false);
    assert_eq!(decision["later_unverified_after_qualification"], "0.70.1");
    assert_eq!(decision["provider_prompt_sent"], false);
    assert_eq!(decision["live_acp_initialize"], false);
    assert_eq!(decision["host_install_changed"], false);

    assert_eq!(
        protocol["elicitation_js_identical_0_64_0_through_0_70_0"],
        true
    );
    assert_eq!(protocol["tools_js_identical_0_69_0_through_0_70_0"], true);
    assert_eq!(protocol["protocol_version"], 1);
    assert_eq!(protocol["allow_always_still_unmapped"], true);
    assert_eq!(protocol["unmapped_providers_api"]["from"], "0.70.0");
    assert_eq!(protocol["provider_prompt_sent"], false);
    assert_eq!(protocol["live_acp_initialize"], false);

    assert_eq!(CLAUDE_AGENT_ACP_BASELINE_VERSION, "0.53.0");
    assert_eq!(CLAUDE_AGENT_ACP_LATEST_QUALIFIED_VERSION, "0.70.0");
    assert_eq!(
        identity["claim_at_observation"]["latest_qualified"],
        "0.69.0"
    );

    let claim = claude_agent_acp_claim();
    assert!(matches!(
        claim.assess(&version_value("0.63.0")),
        InterfaceCompatibilityAssessment::Qualified(matched)
            if matched.support_status() == InterfaceSupportStatus::Deprecated
    ));
    for version in ["0.66.0", "0.69.0", "0.70.0"] {
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
        claim.assess(&version_value("0.70.1")),
        InterfaceCompatibilityAssessment::UnverifiedNewer(_)
    ));
    assert_eq!(
        claude_agent_acp_binding("0.70.0")
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

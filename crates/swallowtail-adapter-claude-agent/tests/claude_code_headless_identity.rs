use serde_json::Value;
use swallowtail_adapter_claude_agent::{
    CLAUDE_CODE_HEADLESS_AXIS, CLAUDE_CODE_HEADLESS_BASELINE_VERSION,
    CLAUDE_CODE_HEADLESS_LATEST_QUALIFIED_VERSION, claude_code_headless_binding,
    claude_code_headless_claim,
};
use swallowtail_core::{
    InterfaceCompatibilityAssessment, InterfaceSupportStatus, InterfaceVersion,
};

const IDENTITY: &str = include_str!("fixtures/claude-code-2.1.238/identity.json");
const PROTOCOL: &str = include_str!("fixtures/claude-code-2.1.238/protocol.json");

#[test]
fn identity_and_claim_qualify_2_1_238_as_compatible_extension() {
    let identity: Value =
        serde_json::from_str(IDENTITY).expect("Claude Code 2.1.238 identity corpus is valid JSON");
    let protocol: Value =
        serde_json::from_str(PROTOCOL).expect("Claude Code 2.1.238 protocol corpus is valid JSON");

    assert_eq!(identity["axis"], CLAUDE_CODE_HEADLESS_AXIS);
    assert_eq!(identity["version"], "2.1.238");
    assert_eq!(identity["npm_package"], "@anthropic-ai/claude-code");
    assert_eq!(identity["npm_latest"], true);
    assert_eq!(
        identity["npm_integrity"],
        "sha512-8AgGrM8qxsA5B8KU/MvVND/fMUsF3vZQxeYjz+1Z/rGx/ZmNr0iqjfmUVKVASKN7P9OzkAUHoXgKEpyvgRfUkA=="
    );
    assert_eq!(identity["host"]["not_installed"], true);
    assert_eq!(
        identity["published_stables_from_previous_ceiling"],
        serde_json::json!(["2.1.236", "2.1.237", "2.1.238"])
    );
    assert_eq!(identity["unpublished_2_1_239"], true);

    let decision = &identity["identity_decision"];
    assert_eq!(decision["shape"], "compatible-extension");
    assert_eq!(
        decision["reuse_behavior_revision"],
        "claude-code.headless.stream-json.v1"
    );
    assert_eq!(decision["raise_latest_qualified_to"], "2.1.238");
    assert_eq!(decision["keep_baseline"], "2.1.220");
    assert_eq!(decision["qualify_intermediates"], true);
    assert_eq!(decision["new_milestone"], false);
    assert_eq!(decision["mix_response_only_axis"], false);
    assert_eq!(decision["flatten_to_claude_agent_acp"], false);
    assert_eq!(decision["provider_prompt_sent"], false);
    assert_eq!(decision["later_unverified_after_qualification"], "2.1.239");

    let flags = protocol["help_selected_flags_present"]
        .as_array()
        .expect("selected flags are an array");
    for required in [
        "-p",
        "--output-format",
        "--no-session-persistence",
        "--permission-mode",
        "--tools",
        "--mcp-config",
        "--strict-mcp-config",
    ] {
        assert!(
            flags.iter().any(|flag| flag == required),
            "missing selected flag {required}"
        );
    }
    assert_eq!(protocol["selected_permission_mode"], "plan");
    assert_eq!(protocol["include_partial_messages_selected"], false);
    assert_eq!(protocol["decoder_corpus"], "claude-code-2.1.220");
    assert_eq!(protocol["provider_prompt_sent"], false);

    assert_eq!(CLAUDE_CODE_HEADLESS_BASELINE_VERSION, "2.1.220");
    assert_eq!(CLAUDE_CODE_HEADLESS_LATEST_QUALIFIED_VERSION, "2.1.238");
    assert_eq!(
        identity["claim_at_observation"]["headless_latest_qualified"],
        "2.1.235"
    );

    let claim = claude_code_headless_claim();
    assert!(claim.supports(&version("2.1.220")));
    assert!(claim.supports(&version("2.1.221")));
    assert!(claim.supports(&version("2.1.235")));
    assert!(claim.supports(&version("2.1.236")));
    assert!(claim.supports(&version("2.1.237")));
    assert!(matches!(
        claim.assess(&version("2.1.238")),
        InterfaceCompatibilityAssessment::Qualified(matched)
            if matched.support_status() == InterfaceSupportStatus::Maintained
    ));
    assert!(matches!(
        claim.assess(&version("2.1.239")),
        InterfaceCompatibilityAssessment::UnverifiedNewer(_)
    ));
    assert_eq!(
        claude_code_headless_binding("2.1.238")
            .expect("version binds")
            .axis()
            .as_str(),
        CLAUDE_CODE_HEADLESS_AXIS
    );
}

fn version(value: &str) -> InterfaceVersion {
    InterfaceVersion::new(value).expect("fixture version is valid")
}

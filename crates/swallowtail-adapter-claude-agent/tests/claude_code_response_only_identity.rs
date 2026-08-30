use serde_json::Value;
use swallowtail_adapter_claude_agent::{
    CLAUDE_CODE_RESPONSE_ONLY_AXIS, CLAUDE_CODE_RESPONSE_ONLY_BASELINE_VERSION,
    CLAUDE_CODE_RESPONSE_ONLY_DENIED_VERSIONS, CLAUDE_CODE_RESPONSE_ONLY_LATEST_QUALIFIED_VERSION,
    claude_code_response_only_binding, claude_code_response_only_claim,
};
use swallowtail_core::{
    InterfaceCompatibilityAssessment, InterfaceSupportStatus, InterfaceVersion,
};

const PACKAGE: &str = include_str!("fixtures/claude-code-2.1.251/identity.json");
const RESPONSE_ONLY: &str = include_str!("fixtures/claude-code-2.1.251/response-only.json");

#[test]
fn response_only_qualifies_2_1_251_as_compatible_extension() {
    let package: Value =
        serde_json::from_str(PACKAGE).expect("Claude Code 2.1.251 package identity is valid JSON");
    let identity: Value = serde_json::from_str(RESPONSE_ONLY)
        .expect("Claude Code 2.1.251 response-only identity is valid JSON");

    assert_eq!(package["version"], "2.1.251");
    assert_eq!(package["npm_latest"], true);
    assert_eq!(package["host"]["not_installed"], false);
    assert_eq!(identity["axis"], CLAUDE_CODE_RESPONSE_ONLY_AXIS);
    assert_eq!(identity["version"], "2.1.251");
    assert_eq!(identity["provider_prompt_sent"], false);
    assert_eq!(identity["selected_mapped_subset_unchanged"], true);

    let decision = &identity["identity_decision"];
    assert_eq!(decision["shape"], "compatible-extension");
    assert_eq!(
        decision["reuse_behavior_revision"],
        "claude-code.response-only.stream-json.v1"
    );
    assert_eq!(decision["raise_latest_qualified"], true);
    assert_eq!(decision["raise_latest_qualified_to"], "2.1.251");
    assert_eq!(decision["keep_baseline"], "2.1.227");
    assert_eq!(decision["qualify_intermediates"], true);
    assert_eq!(decision["keep_unpublished_2_1_244_incompatible"], true);
    assert_eq!(decision["keep_unpublished_2_1_249_incompatible"], true);
    assert_eq!(decision["deny_2_1_251"], false);
    assert_eq!(decision["mix_headless_axis"], false);
    assert_eq!(decision["flatten_to_claude_agent_acp"], false);
    assert_eq!(decision["later_unverified_after_qualification"], "2.1.252");

    let flags = identity["help_selected_flags_present"]
        .as_array()
        .expect("selected flags are an array");
    for required in [
        "-p",
        "--output-format",
        "--no-session-persistence",
        "--tools",
        "--safe-mode",
        "--disable-slash-commands",
        "--no-chrome",
        "--prompt-suggestions",
        "--mcp-config",
        "--strict-mcp-config",
    ] {
        assert!(
            flags.iter().any(|flag| flag == required),
            "missing selected flag {required}"
        );
    }
    assert_eq!(identity["selected_tools"], "");
    assert_eq!(identity["selected_prompt_suggestions"], "false");

    assert_eq!(CLAUDE_CODE_RESPONSE_ONLY_BASELINE_VERSION, "2.1.227");
    assert_eq!(
        CLAUDE_CODE_RESPONSE_ONLY_LATEST_QUALIFIED_VERSION,
        "2.1.251"
    );
    assert_eq!(
        CLAUDE_CODE_RESPONSE_ONLY_DENIED_VERSIONS,
        &["2.1.244", "2.1.249"]
    );
    assert_eq!(
        identity["claim_at_observation"]["latest_qualified"],
        "2.1.241"
    );

    let claim = claude_code_response_only_claim();
    assert!(claim.supports(&version("2.1.227")));
    assert!(claim.supports(&version("2.1.228")));
    assert!(claim.supports(&version("2.1.229")));
    assert!(claim.supports(&version("2.1.241")));
    assert!(claim.supports(&version("2.1.242")));
    assert!(claim.supports(&version("2.1.250")));
    assert!(!claim.permits(&version("2.1.226")));
    assert!(!claim.permits(&version("2.1.244")));
    assert!(!claim.permits(&version("2.1.249")));
    assert!(matches!(
        claim.assess(&version("2.1.251")),
        InterfaceCompatibilityAssessment::Qualified(matched)
            if matched.support_status() == InterfaceSupportStatus::Maintained
    ));
    assert!(matches!(
        claim.assess(&version("2.1.252")),
        InterfaceCompatibilityAssessment::UnverifiedNewer(_)
    ));
    assert_eq!(
        claude_code_response_only_binding("2.1.251")
            .expect("version binds")
            .axis()
            .as_str(),
        CLAUDE_CODE_RESPONSE_ONLY_AXIS
    );
}

fn version(value: &str) -> InterfaceVersion {
    InterfaceVersion::new(value).expect("fixture version is valid")
}

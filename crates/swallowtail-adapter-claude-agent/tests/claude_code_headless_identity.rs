use serde_json::Value;
use swallowtail_adapter_claude_agent::{
    CLAUDE_CODE_HEADLESS_AXIS, CLAUDE_CODE_HEADLESS_BASELINE_VERSION,
    CLAUDE_CODE_HEADLESS_LATEST_QUALIFIED_VERSION, claude_code_headless_binding,
    claude_code_headless_claim,
};
use swallowtail_core::{
    InterfaceCompatibilityAssessment, InterfaceSupportStatus, InterfaceVersion,
};

const IDENTITY: &str = include_str!("fixtures/claude-code-2.1.234/identity.json");
const PROTOCOL: &str = include_str!("fixtures/claude-code-2.1.234/protocol.json");

#[test]
fn identity_and_claim_qualify_2_1_234_as_compatible_extension() {
    let identity: Value =
        serde_json::from_str(IDENTITY).expect("Claude Code 2.1.234 identity corpus is valid JSON");
    let protocol: Value =
        serde_json::from_str(PROTOCOL).expect("Claude Code 2.1.234 protocol corpus is valid JSON");

    assert_eq!(identity["axis"], CLAUDE_CODE_HEADLESS_AXIS);
    assert_eq!(identity["version"], "2.1.234");
    assert_eq!(identity["npm_package"], "@anthropic-ai/claude-code");
    assert_eq!(identity["npm_latest"], true);
    assert_eq!(
        identity["npm_integrity"],
        "sha512-Q53mRcFLqPAWfkvqn7vOzTtMHprzwKdKGRW4OS/Kgr/Tsa+2pyVwVetLb7DRZxhBkYsYld2l8Eo4SX76YoNOOA=="
    );
    assert_eq!(identity["local_cli"], "2.1.234 (Claude Code)");
    assert!(is_sha256(
        identity["local_executable_sha256"]
            .as_str()
            .expect("executable digest is text")
    ));

    let decision = &identity["identity_decision"];
    assert_eq!(decision["shape"], "compatible-extension");
    assert_eq!(
        decision["reuse_behavior_revision"],
        "claude-code.headless.stream-json.v1"
    );
    assert_eq!(decision["raise_latest_qualified_to"], "2.1.234");
    assert_eq!(decision["keep_baseline"], "2.1.220");
    assert_eq!(decision["qualify_intermediates"], true);
    assert_eq!(decision["new_milestone"], false);
    assert_eq!(decision["mix_response_only_axis"], false);
    assert_eq!(decision["provider_prompt_sent"], false);

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
    assert_eq!(CLAUDE_CODE_HEADLESS_LATEST_QUALIFIED_VERSION, "2.1.234");
    assert_eq!(
        identity["claim_at_observation"]["latest_qualified"],
        "2.1.220"
    );

    let claim = claude_code_headless_claim();
    assert!(claim.supports(&version("2.1.220")));
    assert!(claim.supports(&version("2.1.221")));
    assert!(matches!(
        claim.assess(&version("2.1.234")),
        InterfaceCompatibilityAssessment::Qualified(matched)
            if matched.support_status() == InterfaceSupportStatus::Maintained
    ));
    assert!(matches!(
        claim.assess(&version("2.1.235")),
        InterfaceCompatibilityAssessment::UnverifiedNewer(_)
    ));
    assert_eq!(
        claude_code_headless_binding("2.1.234")
            .expect("version binds")
            .axis()
            .as_str(),
        CLAUDE_CODE_HEADLESS_AXIS
    );
}

fn is_sha256(value: &str) -> bool {
    value.len() == 64 && value.bytes().all(|byte| byte.is_ascii_hexdigit())
}

fn version(value: &str) -> InterfaceVersion {
    InterfaceVersion::new(value).expect("fixture version is valid")
}

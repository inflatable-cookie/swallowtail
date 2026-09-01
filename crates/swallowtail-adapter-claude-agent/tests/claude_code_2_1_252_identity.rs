use serde_json::Value;
use swallowtail_adapter_claude_agent::{
    CLAUDE_CODE_HEADLESS_AXIS, CLAUDE_CODE_HEADLESS_BASELINE_VERSION,
    CLAUDE_CODE_HEADLESS_LATEST_QUALIFIED_VERSION, CLAUDE_CODE_RESPONSE_ONLY_AXIS,
    CLAUDE_CODE_RESPONSE_ONLY_BASELINE_VERSION, CLAUDE_CODE_RESPONSE_ONLY_DENIED_VERSIONS,
    CLAUDE_CODE_RESPONSE_ONLY_LATEST_QUALIFIED_VERSION, claude_code_headless_claim,
    claude_code_response_only_claim,
};
use swallowtail_core::{
    InterfaceCompatibilityAssessment, InterfaceSupportStatus, InterfaceVersion,
};

const IDENTITY: &str = include_str!("fixtures/claude-code-2.1.252/identity.json");
const PROTOCOL: &str = include_str!("fixtures/claude-code-2.1.252/protocol.json");
const RESPONSE_ONLY: &str = include_str!("fixtures/claude-code-2.1.252/response-only.json");

#[test]
fn identity_freezes_official_2_1_252_before_any_claim_edit() {
    let identity: Value =
        serde_json::from_str(IDENTITY).expect("Claude Code 2.1.252 identity corpus is valid JSON");
    let protocol: Value =
        serde_json::from_str(PROTOCOL).expect("Claude Code 2.1.252 protocol corpus is valid JSON");
    let response_only: Value = serde_json::from_str(RESPONSE_ONLY)
        .expect("Claude Code 2.1.252 response-only identity is valid JSON");

    assert_eq!(identity["axis"], CLAUDE_CODE_HEADLESS_AXIS);
    assert_eq!(identity["version"], "2.1.252");
    assert_eq!(identity["npm_package"], "@anthropic-ai/claude-code");
    assert_eq!(identity["npm_latest"], true);
    assert_eq!(
        identity["npm_integrity"],
        "sha512-ftoO0eLOZyEDrA3KDd7QZH5qdvToiTcoip3YdGGx8wzH4R9YUwHO+5VG01JDRn8u7MrRcXkf7FvbMYezEt0VyQ=="
    );
    assert_eq!(identity["github_tag"], "v2.1.252");
    assert_eq!(
        identity["github_tag_commit"],
        "f275fa282e76c5e5456912268f2c367a7f4f4797"
    );
    assert_eq!(identity["host"]["not_installed"], false);
    assert_eq!(identity["host"]["version_output"], "2.1.251 (Claude Code)");
    assert_eq!(identity["host"]["matches_official_darwin_arm64"], false);
    assert_eq!(
        identity["host"]["matches_official_2_1_251_darwin_arm64"],
        true
    );
    assert_eq!(identity["official_help_byte_identical_to_2_1_251"], true);
    assert_eq!(identity["selected_mapped_subset_unchanged"], true);
    assert_eq!(
        identity["published_stables_from_previous_ceiling"],
        serde_json::json!(["2.1.252"])
    );
    assert_eq!(identity["unpublished_2_1_244"], true);
    assert_eq!(identity["unpublished_2_1_249"], true);
    assert_eq!(identity["unpublished_2_1_253"], true);

    let watcher = &identity["watcher_audit"];
    assert_eq!(watcher["help_digest_unchanged"], true);
    assert_eq!(watcher["exact_watcher_version_remains"], "2.1.251");
    assert_eq!(watcher["widen_watcher_help_authorization"], false);
    assert_eq!(watcher["widen_watcher_digest_authorization"], false);
    assert_eq!(watcher["widen_watcher_live_authorization"], false);
    assert_eq!(watcher["copy_watcher_isolation_fixture"], false);
    assert_eq!(watcher["copy_watcher_tool_admission_fixture"], false);

    let decision = &identity["identity_decision"];
    assert_eq!(decision["shape"], "compatible-extension");
    assert_eq!(
        decision["reuse_behavior_revision"],
        "claude-code.headless.stream-json.v1"
    );
    assert_eq!(decision["raise_latest_qualified_to"], "2.1.252");
    assert_eq!(decision["keep_baseline"], "2.1.220");
    assert_eq!(decision["new_milestone"], false);
    assert_eq!(decision["map_watcher_flags"], false);
    assert_eq!(decision["widen_watcher_authorization"], false);
    assert_eq!(decision["widen_maximum_turns"], false);
    assert_eq!(decision["later_unverified_after_qualification"], "2.1.253");
    assert_eq!(decision["later_unverified_published"], false);
    assert_eq!(decision["provider_prompt_sent"], false);
    assert_eq!(decision["host_install_changed"], false);

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
    assert_eq!(protocol["official_help_byte_identical_to_2_1_251"], true);
    assert_eq!(protocol["selected_mapped_subset_unchanged"], true);
    assert_eq!(protocol["decoder_corpus"], "claude-code-2.1.220");
    assert_eq!(
        protocol["mapped_deltas_from_2_1_251"],
        serde_json::json!([])
    );
    assert_eq!(protocol["provider_prompt_sent"], false);

    assert_eq!(response_only["axis"], CLAUDE_CODE_RESPONSE_ONLY_AXIS);
    assert_eq!(response_only["version"], "2.1.252");
    assert_eq!(response_only["selected_mapped_subset_unchanged"], true);
    assert_eq!(
        response_only["identity_decision"]["raise_latest_qualified_to"],
        "2.1.252"
    );
    assert_eq!(
        response_only["identity_decision"]["later_unverified_after_qualification"],
        "2.1.253"
    );

    assert_eq!(
        identity["claim_at_observation"]["headless_latest_qualified"],
        "2.1.251"
    );
    assert_eq!(
        response_only["claim_at_observation"]["latest_qualified"],
        "2.1.251"
    );
    assert_eq!(CLAUDE_CODE_HEADLESS_BASELINE_VERSION, "2.1.220");
    assert_eq!(CLAUDE_CODE_HEADLESS_LATEST_QUALIFIED_VERSION, "2.1.251");
    assert_eq!(CLAUDE_CODE_RESPONSE_ONLY_BASELINE_VERSION, "2.1.227");
    assert_eq!(
        CLAUDE_CODE_RESPONSE_ONLY_LATEST_QUALIFIED_VERSION,
        "2.1.251"
    );
    assert_eq!(
        CLAUDE_CODE_RESPONSE_ONLY_DENIED_VERSIONS,
        &["2.1.244", "2.1.249"]
    );

    let headless = claude_code_headless_claim();
    assert!(matches!(
        headless.assess(&version("2.1.251")),
        InterfaceCompatibilityAssessment::Qualified(matched)
            if matched.support_status() == InterfaceSupportStatus::Maintained
    ));
    assert!(matches!(
        headless.assess(&version("2.1.252")),
        InterfaceCompatibilityAssessment::UnverifiedNewer(_)
    ));
    assert!(!headless.permits(&version("2.1.244")));
    assert!(!headless.permits(&version("2.1.249")));

    let response = claude_code_response_only_claim();
    assert!(matches!(
        response.assess(&version("2.1.251")),
        InterfaceCompatibilityAssessment::Qualified(matched)
            if matched.support_status() == InterfaceSupportStatus::Maintained
    ));
    assert!(matches!(
        response.assess(&version("2.1.252")),
        InterfaceCompatibilityAssessment::UnverifiedNewer(_)
    ));
}

fn version(value: &str) -> InterfaceVersion {
    InterfaceVersion::new(value).expect("fixture version is valid")
}

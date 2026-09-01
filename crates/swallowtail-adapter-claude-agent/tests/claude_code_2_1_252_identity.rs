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
const FROZEN_2_1_251_PROTOCOL: &str = include_str!("fixtures/claude-code-2.1.251/protocol.json");
const FROZEN_HELP_SHA256: &str = "5ff2e7a0bca8535fb9ec097fa0a21e9d6b735ed94104fa0d1f58ac73a841d52d";

#[test]
fn official_package_release_and_artifact_identity_is_exact() {
    let identity = json(IDENTITY);
    assert_eq!(identity["axis"], CLAUDE_CODE_HEADLESS_AXIS);
    assert_eq!(identity["version"], "2.1.252");
    assert_eq!(identity["npm_package"], "@anthropic-ai/claude-code");
    assert_eq!(identity["npm_latest"], true);
    assert_eq!(identity["published_at"], "2026-08-31T17:07:28.168Z");
    assert_eq!(identity["github_tag"], "v2.1.252");
    assert_eq!(identity["github_published_at"], "2026-08-31T19:46:55Z");
    assert_eq!(
        identity["github_tag_commit"],
        "f275fa282e76c5e5456912268f2c367a7f4f4797"
    );
    assert_eq!(identity["github_tag_annotated"], false);
    assert_eq!(
        identity["npm_integrity"],
        "sha512-ftoO0eLOZyEDrA3KDd7QZH5qdvToiTcoip3YdGGx8wzH4R9YUwHO+5VG01JDRn8u7MrRcXkf7FvbMYezEt0VyQ=="
    );
    assert_eq!(
        identity["npm_shasum"],
        "f5396b69ed26971a0e13205ebc760da7d98bf92e"
    );
    assert_sha256(
        &identity["npm_tarball_sha256"],
        "e5e04447d3afdf70f7578f9d22217c530a0ef8c59ae2f78e32d1a6ea2fb3cafa",
    );
    assert_eq!(
        identity["linux_x64_package"],
        "@anthropic-ai/claude-code-linux-x64"
    );
    assert_sha256(
        &identity["linux_x64_tarball_sha256"],
        "ecce38cb26f10215a98608c23ddaf4db6fe07bce651c0367617f8829569824fb",
    );
    assert_sha256(
        &identity["linux_x64_binary_sha256"],
        "a715a45105e593fc9808d035d77781f88480b9897975a9df41837f0c591bd4b3",
    );
    assert_eq!(identity["linux_x64_binary_size"].as_u64(), Some(214371672));
    assert_eq!(
        identity["darwin_arm64_package"],
        "@anthropic-ai/claude-code-darwin-arm64"
    );
    assert_sha256(
        &identity["darwin_arm64_tarball_sha256"],
        "d11551a495051a745ee033160bc379e5a388e3e6d87666e9259da09a7d24049b",
    );
    assert_sha256(
        &identity["darwin_arm64_binary_sha256"],
        "b661c6a094fcc32656bf7c0071c5b45bf900b34d4f0a1ab3d78fd59aeba2c2c7",
    );
    assert_eq!(
        identity["darwin_arm64_binary_size"].as_u64(),
        Some(197220928)
    );
    assert_eq!(identity["official_version_output"], "2.1.252 (Claude Code)");
    assert_ne!(
        identity["host"]["native_sha256"],
        identity["darwin_arm64_binary_sha256"]
    );
    assert_sha256(
        &identity["host"]["native_sha256"],
        "625869b01e0050f260b2980fac248fd9cef9e462612bded4ec9d3d49ff8969a5",
    );
    assert_eq!(identity["host"]["native_size"].as_u64(), Some(197171680));
    assert_eq!(identity["host"]["matches_official_darwin_arm64"], false);
    assert_eq!(
        identity["host"]["matches_official_2_1_251_darwin_arm64"],
        true
    );
}

#[test]
fn help_digest_is_the_frozen_2_1_251_digest() {
    let identity = json(IDENTITY);
    let protocol = json(PROTOCOL);
    let frozen = json(FROZEN_2_1_251_PROTOCOL);
    assert_sha256(&frozen["official_help_sha256"], FROZEN_HELP_SHA256);
    assert_sha256(&identity["official_help_sha256"], FROZEN_HELP_SHA256);
    assert_sha256(&protocol["official_help_sha256"], FROZEN_HELP_SHA256);
    assert_sha256(&protocol["host_help_sha256"], FROZEN_HELP_SHA256);
    assert_sha256(&identity["host"]["help_sha256"], FROZEN_HELP_SHA256);
    assert_eq!(identity["official_help_byte_identical_to_2_1_251"], true);
    assert_eq!(protocol["official_help_byte_identical_to_2_1_251"], true);
    assert_eq!(
        identity["host"]["help_byte_identical_to_official_2_1_252"],
        true
    );
    assert_eq!(
        protocol["host_help_byte_identical_to_official_extracted"],
        true
    );
}

#[test]
fn wrapper_and_sdk_identity_is_exact() {
    let identity = json(IDENTITY);
    assert_eq!(
        strings(&identity["wrapper_files_byte_identical_to_2_1_251"]),
        [
            "cli-wrapper.cjs",
            "install.cjs",
            "bin/claude.exe",
            "LICENSE.md",
            "README.md",
            "sdk-tools.d.ts",
        ]
    );
    assert_sha256(
        &identity["cli_wrapper_sha256"],
        "61ad63033d9c8155d5e60a29f45dc4665afa07631c0b108e62cc83bf45ba490e",
    );
    assert_sha256(
        &identity["install_cjs_sha256"],
        "5cbab1670597f492cd4eeb946f3c344ebcb1fbd43c623ba192c9b33744461b85",
    );
    assert_eq!(
        identity["package_json_delta"],
        "version pin and optionalDependencies platform packages only"
    );
    assert_eq!(
        identity["sdk_tools_delta"],
        "byte-identical to 2.1.251; no new unmapped SDK types"
    );
    assert_eq!(
        identity["agent_sdk_package"],
        "@anthropic-ai/claude-agent-sdk"
    );
    assert_eq!(identity["agent_sdk_latest"], "0.3.252");
    assert_eq!(
        identity["agent_sdk_latest_integrity"],
        "sha512-hCkyZFn3J46aAMNqS6AZbYz91FaLUmX5VvJOzYZqzlVBJN47OxXQugqOzqa6b6GOZRmwiqW2ck8J8TE7bQZswQ=="
    );
    assert_eq!(identity["frozen_corpus_version"], "2.1.220");
    assert_eq!(identity["frozen_agent_sdk"], "0.3.220");
}

#[test]
fn mapped_and_unmapped_ledgers_are_exact() {
    let protocol = json(PROTOCOL);
    assert_eq!(
        protocol["mapped_deltas_from_2_1_251"],
        serde_json::json!([])
    );
    assert_eq!(
        strings(&protocol["unused_help_deltas"]),
        [
            "--bare",
            "--brief",
            "--cloud",
            "--include-hook-events",
            "--include-partial-messages",
            "--forward-subagent-text",
            "--json-schema",
            "--max-budget-usd",
            "--restricted",
            "--all",
        ]
    );
    assert_eq!(
        strings(&protocol["unused_command_deltas"]),
        ["attach", "logs", "stop", "kill", "respawn", "rm"]
    );
    assert_eq!(
        strings(&protocol["unmapped_changelog"]),
        [
            "2.1.252 Bash task-output swap refused on some Macs",
            "2.1.252 always-allow save without existing .claude/settings.local.json",
            "2.1.252 Remote Control stall after tool finish when claude.ai is degraded",
            "2.1.252 large background-task failure output exceeding API request size",
        ]
    );
    let decision = &json(IDENTITY)["identity_decision"];
    assert_eq!(decision["map_restricted"], false);
    assert_eq!(decision["map_watcher_flags"], false);
    assert_eq!(decision["map_background_session_commands"], false);
    assert_eq!(decision["widen_maximum_turns"], false);
}

#[test]
fn selected_headless_flags_and_domains_are_exact() {
    let protocol = json(PROTOCOL);
    assert_eq!(
        strings(&protocol["help_selected_flags_present"]),
        [
            "-p",
            "--print",
            "--input-format",
            "--output-format",
            "--verbose",
            "--no-session-persistence",
            "--model",
            "--effort",
            "--permission-mode",
            "--tools",
            "--setting-sources",
            "--mcp-config",
            "--strict-mcp-config",
        ]
    );
    assert_eq!(
        strings(&protocol["input_format_choices"]),
        ["text", "stream-json"]
    );
    assert_eq!(
        strings(&protocol["output_format_choices"]),
        ["text", "json", "stream-json"]
    );
    assert_eq!(
        strings(&protocol["effort_choices"]),
        ["low", "medium", "high", "xhigh", "max"]
    );
    assert_eq!(
        strings(&protocol["permission_mode_choices"]),
        [
            "acceptEdits",
            "auto",
            "bypassPermissions",
            "manual",
            "dontAsk",
            "plan",
        ]
    );
    assert_eq!(protocol["selected_permission_mode"], "plan");
    assert_eq!(protocol["selected_tools"], "Read,Glob,Grep");
    assert_eq!(protocol["selected_setting_sources"], "user,project,local");
    assert_eq!(protocol["include_partial_messages_selected"], false);
    assert_eq!(protocol["decoder_corpus"], "claude-code-2.1.220");
}

#[test]
fn response_only_selected_flags_are_exact() {
    let response_only = json(RESPONSE_ONLY);
    assert_eq!(response_only["axis"], CLAUDE_CODE_RESPONSE_ONLY_AXIS);
    assert_eq!(response_only["version"], "2.1.252");
    assert_eq!(
        strings(&response_only["help_selected_flags_present"]),
        [
            "-p",
            "--input-format",
            "--output-format",
            "--verbose",
            "--no-session-persistence",
            "--model",
            "--effort",
            "--tools",
            "--safe-mode",
            "--disable-slash-commands",
            "--no-chrome",
            "--prompt-suggestions",
            "--mcp-config",
            "--strict-mcp-config",
        ]
    );
    assert_eq!(response_only["selected_tools"], "");
    assert_eq!(response_only["selected_prompt_suggestions"], "false");
    assert_eq!(response_only["include_partial_messages_selected"], false);
    assert_eq!(
        response_only["qualified_decoder_corpus"],
        serde_json::json!(["claude-code-2.1.227", "claude-code-2.1.228"])
    );
}

#[test]
fn watcher_authorization_stays_on_exact_2_1_251() {
    let identity = json(IDENTITY);
    let watcher = &identity["watcher_audit"];
    assert_sha256(&watcher["frozen_help_sha256"], FROZEN_HELP_SHA256);
    assert_sha256(&watcher["official_2_1_252_help_sha256"], FROZEN_HELP_SHA256);
    assert_eq!(watcher["help_digest_unchanged"], true);
    assert_eq!(watcher["exact_watcher_version_remains"], "2.1.251");
    assert_eq!(watcher["widen_watcher_help_authorization"], false);
    assert_eq!(watcher["widen_watcher_digest_authorization"], false);
    assert_eq!(watcher["widen_watcher_live_authorization"], false);
    assert_eq!(watcher["copy_watcher_isolation_fixture"], false);
    assert_eq!(watcher["copy_watcher_tool_admission_fixture"], false);
    assert_eq!(watcher["mechanism_change_gate_unchanged"], true);
    assert_eq!(
        identity["claim_at_observation"]["watcher_exact_version"],
        "2.1.251"
    );
    assert_eq!(
        identity["identity_decision"]["widen_watcher_authorization"],
        false
    );
    assert_eq!(identity["identity_decision"]["map_watcher_flags"], false);
}

#[test]
fn unpublished_gaps_and_later_2_1_253_stay_classified() {
    let identity = json(IDENTITY);
    let response_only = json(RESPONSE_ONLY);
    assert_eq!(
        strings(&identity["published_stables_from_previous_ceiling"]),
        ["2.1.252"]
    );
    assert_eq!(identity["unpublished_2_1_244"], true);
    assert_eq!(identity["unpublished_2_1_249"], true);
    assert_eq!(identity["unpublished_2_1_253"], true);
    assert_eq!(
        identity["identity_decision"]["keep_unpublished_2_1_244_incompatible"],
        true
    );
    assert_eq!(
        identity["identity_decision"]["keep_unpublished_2_1_249_incompatible"],
        true
    );
    assert_eq!(
        identity["identity_decision"]["later_unverified_after_qualification"],
        "2.1.253"
    );
    assert_eq!(
        identity["identity_decision"]["later_unverified_published"],
        false
    );
    assert_eq!(
        response_only["identity_decision"]["later_unverified_after_qualification"],
        "2.1.253"
    );
    assert_eq!(
        response_only["identity_decision"]["later_unverified_published"],
        false
    );
    assert_eq!(
        CLAUDE_CODE_RESPONSE_ONLY_DENIED_VERSIONS,
        &["2.1.244", "2.1.249"]
    );

    let headless = claude_code_headless_claim();
    assert!(!headless.permits(&version("2.1.244")));
    assert!(!headless.permits(&version("2.1.249")));
    assert!(matches!(
        headless.assess(&version("2.1.253")),
        InterfaceCompatibilityAssessment::UnverifiedNewer(_)
    ));
    let response = claude_code_response_only_claim();
    assert!(!response.permits(&version("2.1.244")));
    assert!(!response.permits(&version("2.1.249")));
    assert!(matches!(
        response.assess(&version("2.1.253")),
        InterfaceCompatibilityAssessment::UnverifiedNewer(_)
    ));
}

#[test]
fn identity_and_claim_qualify_2_1_252_as_compatible_extension() {
    let identity = json(IDENTITY);
    let response_only = json(RESPONSE_ONLY);
    let decision = &identity["identity_decision"];
    assert_eq!(decision["shape"], "compatible-extension");
    assert_eq!(
        decision["reuse_behavior_revision"],
        "claude-code.headless.stream-json.v1"
    );
    assert_eq!(decision["raise_latest_qualified_to"], "2.1.252");
    assert_eq!(decision["keep_baseline"], "2.1.220");
    assert_eq!(decision["new_milestone"], false);
    assert_eq!(
        response_only["identity_decision"]["reuse_behavior_revision"],
        "claude-code.response-only.stream-json.v1"
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
    assert_eq!(CLAUDE_CODE_HEADLESS_LATEST_QUALIFIED_VERSION, "2.1.252");
    assert_eq!(CLAUDE_CODE_RESPONSE_ONLY_BASELINE_VERSION, "2.1.227");
    assert_eq!(
        CLAUDE_CODE_RESPONSE_ONLY_LATEST_QUALIFIED_VERSION,
        "2.1.252"
    );

    let headless = claude_code_headless_claim();
    assert!(matches!(
        headless.assess(&version("2.1.251")),
        InterfaceCompatibilityAssessment::Qualified(matched)
            if matched.support_status() == InterfaceSupportStatus::Maintained
    ));
    assert!(matches!(
        headless.assess(&version("2.1.252")),
        InterfaceCompatibilityAssessment::Qualified(matched)
            if matched.support_status() == InterfaceSupportStatus::Maintained
    ));
    let response = claude_code_response_only_claim();
    assert!(matches!(
        response.assess(&version("2.1.252")),
        InterfaceCompatibilityAssessment::Qualified(matched)
            if matched.support_status() == InterfaceSupportStatus::Maintained
    ));
}

fn json(value: &str) -> Value {
    serde_json::from_str(value).expect("frozen corpus JSON is valid")
}

fn strings(value: &Value) -> Vec<&str> {
    value
        .as_array()
        .expect("value is an array")
        .iter()
        .map(|value| value.as_str().expect("array value is text"))
        .collect()
}

fn assert_sha256(value: &Value, expected: &str) {
    let value = value.as_str().expect("digest is text");
    assert_eq!(value.len(), 64);
    assert!(value.bytes().all(|byte| byte.is_ascii_hexdigit()));
    assert_eq!(value, expected);
}

fn version(value: &str) -> InterfaceVersion {
    InterfaceVersion::new(value).expect("fixture version is valid")
}

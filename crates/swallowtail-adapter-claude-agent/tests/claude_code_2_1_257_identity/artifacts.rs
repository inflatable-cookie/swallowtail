use super::support::{
    FROZEN_2_1_252_HELP_SHA256, FROZEN_2_1_252_PROTOCOL, IDENTITY, OFFICIAL_2_1_257_HELP_SHA256,
    PROTOCOL, assert_sha256, json, strings,
};
use swallowtail_adapter_claude_agent::CLAUDE_CODE_HEADLESS_AXIS;

#[test]
fn official_package_release_and_artifact_identity_is_exact() {
    let identity = json(IDENTITY);
    assert_eq!(identity["axis"], CLAUDE_CODE_HEADLESS_AXIS);
    assert_eq!(identity["version"], "2.1.257");
    assert_eq!(identity["npm_package"], "@anthropic-ai/claude-code");
    assert_eq!(identity["npm_latest"], true);
    assert_eq!(identity["published_at"], "2026-09-01T17:15:33.223Z");
    assert_eq!(identity["github_tag"], "v2.1.257");
    assert_eq!(identity["github_published_at"], "2026-09-01T17:53:52Z");
    assert_eq!(
        identity["github_tag_commit"],
        "a1e64dc407dd57dfb4ea283b0f8049adf3eabee5"
    );
    assert_eq!(identity["github_tag_annotated"], false);
    assert_eq!(
        identity["npm_integrity"],
        "sha512-JzpBQDzbEV+IKV9lIs/SSRIdHGrAmQXhNScoz9PZgdjnatrVnbsBXRDrF26qBBBph38pA/39d+BhDpf+7RwkwA=="
    );
    assert_eq!(
        identity["npm_shasum"],
        "5aa17a093a628f0030c691ed0e11bb50e3228c59"
    );
    assert_sha256(
        &identity["npm_tarball_sha256"],
        "e11188b92a6198945329e4e2657ebff206fbc014b3e5fc95644f76b62300ad5d",
    );
    assert_eq!(
        identity["linux_x64_package"],
        "@anthropic-ai/claude-code-linux-x64"
    );
    assert_sha256(
        &identity["linux_x64_tarball_sha256"],
        "7e53dc103c832c4a34bb3f3a515f8141d9cd4bd19fd2fecd5698030e30a589a2",
    );
    assert_sha256(
        &identity["linux_x64_binary_sha256"],
        "9a64bda9d8722a1fa05bef9a5961d07e0331b99597eda9e2f6a732f3a0ff7f05",
    );
    assert_eq!(identity["linux_x64_binary_size"].as_u64(), Some(215469464));
    assert_eq!(
        identity["darwin_arm64_package"],
        "@anthropic-ai/claude-code-darwin-arm64"
    );
    assert_sha256(
        &identity["darwin_arm64_tarball_sha256"],
        "54c80ce110673637cf932dee41a02f31c95ad1a8bd1455adf480a9a271cdb54a",
    );
    assert_sha256(
        &identity["darwin_arm64_binary_sha256"],
        "64590d7d9d9c189d33fb3dfa58c5408eaf2a10fe556bd84155d95efaab46b60e",
    );
    assert_eq!(
        identity["darwin_arm64_binary_size"].as_u64(),
        Some(199011264)
    );
    assert_eq!(identity["official_version_output"], "2.1.257 (Claude Code)");
    assert_eq!(identity["host"]["version_output"], "2.1.257 (Claude Code)");
    assert_eq!(identity["downloaded_official_binaries_executed"], false);
    assert_eq!(
        identity["host"]["native_sha256"],
        identity["darwin_arm64_binary_sha256"]
    );
    assert_sha256(
        &identity["host"]["native_sha256"],
        "64590d7d9d9c189d33fb3dfa58c5408eaf2a10fe556bd84155d95efaab46b60e",
    );
    assert_eq!(identity["host"]["native_size"].as_u64(), Some(199011264));
    assert_eq!(identity["host"]["matches_official_darwin_arm64"], true);
    assert_eq!(
        identity["host"]["matches_official_2_1_251_darwin_arm64"],
        false
    );
}

#[test]
fn help_digest_moved_off_the_frozen_2_1_252_digest() {
    let identity = json(IDENTITY);
    let protocol = json(PROTOCOL);
    let frozen = json(FROZEN_2_1_252_PROTOCOL);
    assert_sha256(&frozen["official_help_sha256"], FROZEN_2_1_252_HELP_SHA256);
    assert_sha256(
        &protocol["frozen_2_1_252_help_sha256"],
        FROZEN_2_1_252_HELP_SHA256,
    );
    assert_sha256(
        &identity["official_help_sha256"],
        OFFICIAL_2_1_257_HELP_SHA256,
    );
    assert_sha256(
        &protocol["official_help_sha256"],
        OFFICIAL_2_1_257_HELP_SHA256,
    );
    assert_sha256(&protocol["host_help_sha256"], OFFICIAL_2_1_257_HELP_SHA256);
    assert_sha256(
        &identity["host"]["help_sha256"],
        OFFICIAL_2_1_257_HELP_SHA256,
    );
    assert_ne!(OFFICIAL_2_1_257_HELP_SHA256, FROZEN_2_1_252_HELP_SHA256);
    assert_eq!(identity["official_help_byte_identical_to_2_1_252"], false);
    assert_eq!(protocol["official_help_byte_identical_to_2_1_252"], false);
    assert_eq!(
        identity["host"]["help_byte_identical_to_official_2_1_257"],
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
        strings(&identity["wrapper_files_byte_identical_to_2_1_252"]),
        [
            "cli-wrapper.cjs",
            "install.cjs",
            "bin/claude.exe",
            "LICENSE.md",
            "README.md",
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
    assert_sha256(
        &identity["sdk_tools_sha256"],
        "a8bb537bb1624e9e68d5aa7c620260027278a9f83ce81943906a9485b06d7c9d",
    );
    assert_eq!(
        identity["package_json_delta"],
        "version pin and optionalDependencies platform packages only"
    );
    assert_eq!(
        identity["sdk_tools_delta"],
        "SkillCreate comment wording; ArtifactPublish note field removed; REPL result made optional; not selected stream-JSON"
    );
    assert_eq!(
        identity["agent_sdk_package"],
        "@anthropic-ai/claude-agent-sdk"
    );
    assert_eq!(identity["agent_sdk_latest"], "0.3.257");
    assert_eq!(
        identity["agent_sdk_latest_integrity"],
        "sha512-Se55zXv48IYLg/WzoXzpbPLcq86suwDSbRUoNb69l4dkovorqS/47Xuy7MUo/gPNwwcPB4a+aqbXbshU33dcdQ=="
    );
    assert_eq!(identity["frozen_corpus_version"], "2.1.220");
    assert_eq!(identity["frozen_agent_sdk"], "0.3.220");
    assert_eq!(identity["research_266_ceiling"], "2.1.252");
}

use super::support::{
    FROZEN_2_1_251_PROTOCOL, FROZEN_HELP_SHA256, IDENTITY, PROTOCOL, assert_sha256, json, strings,
};
use swallowtail_adapter_claude_agent::CLAUDE_CODE_HEADLESS_AXIS;

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

use super::support::{DIST_INVENTORY, assert_exact_string_set, assert_sha256, json};

#[test]
fn wrapper_and_platform_inventories_are_exact() {
    let inventory = json(DIST_INVENTORY);
    assert_eq!(
        inventory["compared"],
        serde_json::json!(["0.152.0", "0.152.1"])
    );
    assert_eq!(inventory["not_a_complete_semantic_changelog"], true);
    let counts = &inventory["package_file_counts"];
    assert_eq!(counts["wrapper-0.152.0"], 3);
    assert_eq!(counts["wrapper-0.152.1"], 3);
    assert_eq!(counts["darwin-arm64-0.152.0"], 7);
    assert_eq!(counts["darwin-arm64-0.152.1"], 7);
    assert_eq!(counts["linux-x64-0.152.0"], 8);
    assert_eq!(counts["linux-x64-0.152.1"], 8);

    let wrapper = &inventory["from_0_152_0_to_0_152_1_wrapper"];
    assert_exact_string_set(&wrapper["added"], &[]);
    assert_exact_string_set(&wrapper["removed"], &[]);
    assert_exact_string_set(&wrapper["changed"], &["package.json"]);
    assert_exact_string_set(&wrapper["identical"], &["README.md", "bin/codex.js"]);

    let darwin = &inventory["from_0_152_0_to_0_152_1_darwin_arm64"];
    assert_exact_string_set(&darwin["added"], &[]);
    assert_exact_string_set(&darwin["removed"], &[]);
    assert_exact_string_set(
        &darwin["changed"],
        &[
            "package.json",
            "vendor/aarch64-apple-darwin/bin/codex",
            "vendor/aarch64-apple-darwin/bin/codex-code-mode-host",
            "vendor/aarch64-apple-darwin/codex-package.json",
            "vendor/aarch64-apple-darwin/codex-path/rg",
            "vendor/aarch64-apple-darwin/codex-resources/zsh/bin/zsh",
        ],
    );
    assert_exact_string_set(&darwin["identical"], &["README.md"]);

    let linux = &inventory["from_0_152_0_to_0_152_1_linux_x64"];
    assert_exact_string_set(&linux["added"], &[]);
    assert_exact_string_set(&linux["removed"], &[]);
    assert_exact_string_set(
        &linux["changed"],
        &[
            "package.json",
            "vendor/x86_64-unknown-linux-musl/bin/codex",
            "vendor/x86_64-unknown-linux-musl/bin/codex-code-mode-host",
            "vendor/x86_64-unknown-linux-musl/codex-package.json",
        ],
    );
    assert_exact_string_set(
        &linux["identical"],
        &[
            "README.md",
            "vendor/x86_64-unknown-linux-musl/codex-path/rg",
            "vendor/x86_64-unknown-linux-musl/codex-resources/bwrap",
            "vendor/x86_64-unknown-linux-musl/codex-resources/zsh/bin/zsh",
        ],
    );
}

#[test]
fn mapped_feeding_files_stay_byte_identical_except_version_bumps() {
    let hashes = &json(DIST_INVENTORY)["hashes"];
    assert_sha256(
        &hashes["wrapper.bin-codex.js"]["0.152.1"],
        "134063e133f0b4244fa3b251acf973d4fe4b4aeeacbdc135211bf480f59f1477",
    );
    assert_eq!(
        hashes["wrapper.bin-codex.js"]["0.152.0"],
        hashes["wrapper.bin-codex.js"]["0.152.1"]
    );
    assert_ne!(
        hashes["wrapper.package.json"]["0.152.0"],
        hashes["wrapper.package.json"]["0.152.1"]
    );
    assert_ne!(
        hashes["darwin-arm64.bin-codex"]["0.152.0"],
        hashes["darwin-arm64.bin-codex"]["0.152.1"]
    );
    assert_sha256(
        &hashes["darwin-arm64.bin-codex"]["0.152.1"],
        "8194ea3181f330e63023b234b0b231855e5874e0331c5ef7cbc490591497a7bf",
    );
    assert_sha256(
        &hashes["linux-x64.bin-codex"]["0.152.1"],
        "b82018241214a4a7c6b97b198585192d1dbc3aab1ddcdc640f04d8dee8c606f9",
    );
    assert_ne!(
        hashes["darwin-arm64.codex-path-rg"]["0.152.0"],
        hashes["darwin-arm64.codex-path-rg"]["0.152.1"]
    );
    assert_eq!(
        hashes["linux-x64.codex-path-rg"]["0.152.0"],
        hashes["linux-x64.codex-path-rg"]["0.152.1"]
    );
}

#[test]
fn github_source_tree_delta_is_guardian_and_metadata_bounded() {
    let source = &json(DIST_INVENTORY)["github_source_tree"];
    assert_eq!(
        source["base_commit"],
        "316795b3cf2a45e90d121d9f46499d4658b2645c"
    );
    assert_eq!(
        source["head_commit"],
        "5adb68a49933ae446bf11935662c83dba55a0804"
    );
    assert_exact_string_set(&source["added"], &[]);
    assert_exact_string_set(&source["removed"], &[]);
    assert_exact_string_set(
        &source["changed"],
        &[
            "codex-rs/Cargo.toml",
            "codex-rs/core/src/context/guardian_node_repl_policy.rs",
            "codex-rs/core/src/guardian/review.rs",
            "codex-rs/core/src/guardian/review_session.rs",
            "codex-rs/core/src/guardian/review_session_tests.rs",
            "codex-rs/core/src/session/step_activation.rs",
            "codex-rs/core/src/session/step_activation_tests.rs",
            "codex-rs/core/src/session/step_settings_tests.rs",
            "codex-rs/core/tests/suite/guardian_mcp_elicitation.rs",
            "codex-rs/core/tests/suite/guardian_review.rs",
            "codex-rs/models-manager/src/model_info_tests.rs",
            "codex-rs/protocol/src/openai_models.rs",
        ],
    );
    assert_exact_string_set(&source["mapped_feeding_changed_files"], &[]);
}

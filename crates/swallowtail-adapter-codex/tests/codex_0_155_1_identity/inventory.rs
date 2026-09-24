use super::support::{DIST_INVENTORY, assert_exact_string_set, json};

const DARWIN_CHANGED_FIRST_HOP: &[&str] = &[
    "package.json",
    "vendor/aarch64-apple-darwin/bin/codex",
    "vendor/aarch64-apple-darwin/bin/codex-code-mode-host",
    "vendor/aarch64-apple-darwin/codex-package.json",
    "vendor/aarch64-apple-darwin/codex-path/rg",
    "vendor/aarch64-apple-darwin/codex-resources/zsh/bin/zsh",
];

const LINUX_CHANGED_FIRST_HOP: &[&str] = &[
    "package.json",
    "vendor/x86_64-unknown-linux-musl/bin/codex",
    "vendor/x86_64-unknown-linux-musl/bin/codex-code-mode-host",
    "vendor/x86_64-unknown-linux-musl/codex-package.json",
    "vendor/x86_64-unknown-linux-musl/codex-resources/bwrap",
];

#[test]
fn wrapper_and_platform_inventories_are_exact() {
    let inventory = json(DIST_INVENTORY);
    assert_eq!(
        inventory["compared"],
        serde_json::json!(["0.154.0", "0.155.0", "0.155.1"])
    );
    assert_eq!(inventory["not_a_complete_semantic_changelog"], true);
    let counts = &inventory["package_file_counts"];
    assert_eq!(counts["wrapper-0.154.0"], 3);
    assert_eq!(counts["darwin-arm64-0.154.0"], 7);
    assert_eq!(counts["linux-x64-0.154.0"], 8);
    for version in ["0.155.0", "0.155.1"] {
        assert_eq!(counts[format!("wrapper-{version}").as_str()], 3);
        assert_eq!(counts[format!("darwin-arm64-{version}").as_str()], 44);
        assert_eq!(counts[format!("linux-x64-{version}").as_str()], 46);
    }

    for key in [
        "from_0_154_0_to_0_155_0_wrapper",
        "from_0_155_0_to_0_155_1_wrapper",
    ] {
        let hop = &inventory[key];
        assert_exact_string_set(&hop["added"], &[]);
        assert_exact_string_set(&hop["removed"], &[]);
        assert_exact_string_set(&hop["changed"], &["package.json"]);
        assert_exact_string_set(&hop["identical"], &["README.md", "bin/codex.js"]);
    }

    let first_darwin = &inventory["from_0_154_0_to_0_155_0_darwin_arm64"];
    assert_eq!(first_darwin["added"].as_array().expect("added").len(), 37);
    assert_exact_string_set(&first_darwin["removed"], &[]);
    assert_exact_string_set(&first_darwin["changed"], DARWIN_CHANGED_FIRST_HOP);
    assert_exact_string_set(&first_darwin["identical"], &["README.md"]);
    assert!(
        inventory["from_0_154_0_to_0_155_0_darwin_arm64"]["added"]
            .as_array()
            .expect("added")
            .iter()
            .any(|path| path.as_str()
                == Some("vendor/aarch64-apple-darwin/codex-resources/voice/bin/codex-voice-host"))
    );

    let first_linux = &inventory["from_0_154_0_to_0_155_0_linux_x64"];
    assert_eq!(first_linux["added"].as_array().expect("added").len(), 38);
    assert_exact_string_set(&first_linux["removed"], &[]);
    assert_exact_string_set(&first_linux["changed"], LINUX_CHANGED_FIRST_HOP);

    let second_darwin = &inventory["from_0_155_0_to_0_155_1_darwin_arm64"];
    assert_exact_string_set(&second_darwin["added"], &[]);
    assert_exact_string_set(&second_darwin["removed"], &[]);
    assert_eq!(
        second_darwin["changed"].as_array().expect("changed").len(),
        34
    );
    let second_linux = &inventory["from_0_155_0_to_0_155_1_linux_x64"];
    assert_exact_string_set(&second_linux["added"], &[]);
    assert_exact_string_set(&second_linux["removed"], &[]);
    assert_eq!(
        second_linux["changed"].as_array().expect("changed").len(),
        30
    );

    let hashes = &inventory["hashes"];
    assert_eq!(
        hashes["darwin-arm64-cli-0.155.1"],
        serde_json::json!("8eaf1ad12fe6bf89b1710330f58900014322c7c5af677e43be116d8ac5fc0a9e"),
        "official ceiling binary digest"
    );
    assert_eq!(
        hashes["linux-x64-cli-0.155.1"],
        serde_json::json!("0753dfe1d8b87a52436deb13eb1c549661ef4c84fee2c5aa688385eebeccb761"),
        "official ceiling binary digest"
    );
    assert_eq!(
        hashes["darwin-arm64-cli-0.154.0"],
        serde_json::json!("4f85982624b3898c8991cb80c0981b2aa71070e3537046c9a95950318a95afcc"),
        "prior ceiling digest reproduces Research 311"
    );
    assert_eq!(
        hashes["wrapper-codex-js-0.154.0"], hashes["wrapper-codex-js-0.155.1"],
        "wrapper launcher is byte-identical across the window"
    );
}

#[test]
fn github_source_tree_hop_counts_and_tags_are_exact() {
    let inventory = json(DIST_INVENTORY);
    for (key, added, removed, changed) in [
        ("from_0_154_0_to_0_155_0_github_source_tree", 399, 49, 890),
        ("from_0_155_0_to_0_155_1_github_source_tree", 0, 0, 3),
    ] {
        let hop = &inventory[key];
        assert_eq!(hop["added"].as_array().expect("added").len(), added);
        assert_eq!(hop["removed"].as_array().expect("removed").len(), removed);
        assert_eq!(hop["changed"].as_array().expect("changed").len(), changed);
    }
    let second = &inventory["from_0_155_0_to_0_155_1_github_source_tree"];
    assert_exact_string_set(
        &second["changed"],
        &[
            "codex-rs/Cargo.toml",
            "codex-rs/tui/src/app_server_session.rs",
            "codex-rs/tui/src/app_server_session/reasoning_defaults_tests.rs",
        ],
    );
    let tags = &inventory["github_tags"];
    assert_eq!(
        tags["rust-v0.154.0"]["commit"],
        "6b9826e3aa83b1a5947db50f4332cb9c65f1b340"
    );
    assert_eq!(
        tags["rust-v0.155.0"]["commit"],
        "f0a1b8f0849d90960bc406b848f32e5a129b0457"
    );
    assert_eq!(
        tags["rust-v0.155.1"]["commit"],
        "be2951ea34f0d295ed0becf97079f92fa5f6950e"
    );
    assert_eq!(
        tags["rust-v0.155.1"]["tag_object"],
        "4e21628f9ec9ee656650cd2b62ef92225725b5ac"
    );
    let classifications = inventory["changed_file_classifications"]
        .as_array()
        .expect("classifications");
    assert_eq!(classifications.len(), 9);
}

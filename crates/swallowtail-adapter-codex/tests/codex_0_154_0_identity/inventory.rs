use super::support::{DIST_INVENTORY, assert_exact_string_set, json};

const DARWIN_CHANGED_EVERY_HOP: &[&str] = &[
    "package.json",
    "vendor/aarch64-apple-darwin/bin/codex",
    "vendor/aarch64-apple-darwin/bin/codex-code-mode-host",
    "vendor/aarch64-apple-darwin/codex-package.json",
    "vendor/aarch64-apple-darwin/codex-path/rg",
    "vendor/aarch64-apple-darwin/codex-resources/zsh/bin/zsh",
];

const LINUX_CHANGED_BASE: &[&str] = &[
    "package.json",
    "vendor/x86_64-unknown-linux-musl/bin/codex",
    "vendor/x86_64-unknown-linux-musl/bin/codex-code-mode-host",
    "vendor/x86_64-unknown-linux-musl/codex-package.json",
];

#[test]
fn wrapper_and_platform_inventories_are_exact() {
    let inventory = json(DIST_INVENTORY);
    assert_eq!(
        inventory["compared"],
        serde_json::json!([
            "0.152.1", "0.153.0", "0.153.1", "0.153.2", "0.153.3", "0.153.4", "0.154.0"
        ])
    );
    assert_eq!(inventory["not_a_complete_semantic_changelog"], true);
    let counts = &inventory["package_file_counts"];
    for version in [
        "0.152.1", "0.153.0", "0.153.1", "0.153.2", "0.153.3", "0.153.4", "0.154.0",
    ] {
        assert_eq!(counts[format!("wrapper-{version}").as_str()], 3);
        assert_eq!(counts[format!("darwin-arm64-{version}").as_str()], 7);
        assert_eq!(counts[format!("linux-x64-{version}").as_str()], 8);
    }

    let wrapper_first = &inventory["from_0_152_1_to_0_153_0_wrapper"];
    assert_exact_string_set(&wrapper_first["added"], &[]);
    assert_exact_string_set(&wrapper_first["removed"], &[]);
    assert_exact_string_set(&wrapper_first["changed"], &["bin/codex.js", "package.json"]);
    assert_exact_string_set(&wrapper_first["identical"], &["README.md"]);
    for key in [
        "from_0_153_0_to_0_153_1_wrapper",
        "from_0_153_1_to_0_153_2_wrapper",
        "from_0_153_2_to_0_153_3_wrapper",
        "from_0_153_3_to_0_153_4_wrapper",
        "from_0_153_4_to_0_154_0_wrapper",
    ] {
        let hop = &inventory[key];
        assert_exact_string_set(&hop["added"], &[]);
        assert_exact_string_set(&hop["removed"], &[]);
        assert_exact_string_set(&hop["changed"], &["package.json"]);
        assert_exact_string_set(&hop["identical"], &["README.md", "bin/codex.js"]);
    }

    for key in [
        "from_0_152_1_to_0_153_0_darwin_arm64",
        "from_0_153_0_to_0_153_1_darwin_arm64",
        "from_0_153_1_to_0_153_2_darwin_arm64",
        "from_0_153_2_to_0_153_3_darwin_arm64",
        "from_0_153_3_to_0_153_4_darwin_arm64",
        "from_0_153_4_to_0_154_0_darwin_arm64",
    ] {
        let hop = &inventory[key];
        assert_exact_string_set(&hop["added"], &[]);
        assert_exact_string_set(&hop["removed"], &[]);
        assert_exact_string_set(&hop["changed"], DARWIN_CHANGED_EVERY_HOP);
        assert_exact_string_set(&hop["identical"], &["README.md"]);
    }

    for key in [
        "from_0_152_1_to_0_153_0_linux_x64",
        "from_0_153_0_to_0_153_1_linux_x64",
        "from_0_153_3_to_0_153_4_linux_x64",
    ] {
        let hop = &inventory[key];
        assert_exact_string_set(&hop["added"], &[]);
        assert_exact_string_set(&hop["removed"], &[]);
        assert_exact_string_set(&hop["changed"], LINUX_CHANGED_BASE);
    }
    for key in [
        "from_0_153_1_to_0_153_2_linux_x64",
        "from_0_153_2_to_0_153_3_linux_x64",
        "from_0_153_4_to_0_154_0_linux_x64",
    ] {
        let hop = &inventory[key];
        assert_exact_string_set(&hop["added"], &[]);
        assert_exact_string_set(&hop["removed"], &[]);
        assert_exact_string_set(
            &hop["changed"],
            &[
                "package.json",
                "vendor/x86_64-unknown-linux-musl/bin/codex",
                "vendor/x86_64-unknown-linux-musl/bin/codex-code-mode-host",
                "vendor/x86_64-unknown-linux-musl/codex-package.json",
                "vendor/x86_64-unknown-linux-musl/codex-resources/bwrap",
            ],
        );
    }

    let hashes = &inventory["hashes"];
    assert_eq!(
        hashes["darwin-arm64-cli-0.154.0"],
        serde_json::json!(
            "4f85982624b3898c8991cb80c0981b2aa71070e3537046c9a95950318a95afcc"
        ),
        "official ceiling binary digest"
    );
    assert_eq!(
        hashes["linux-x64-cli-0.154.0"],
        serde_json::json!(
            "3188814c35471432d4123203e0eb38e5bddc60226e3d7ddf0e59e649ea140022"
        ),
        "official ceiling binary digest"
    );
    assert_eq!(
        hashes["darwin-arm64-cli-0.153.3"],
        serde_json::json!(
            "0e1f892695844ad0798dab8895955846450a9e7663476ebf24615814dd377216"
        ),
        "host binary digest equals the frozen host record"
    );
    assert_eq!(
        hashes["darwin-arm64-cli-0.152.1"],
        serde_json::json!(
            "8194ea3181f330e63023b234b0b231855e5874e0331c5ef7cbc490591497a7bf"
        ),
        "prior ceiling digest reproduces Research 275"
    );
}

#[test]
fn github_source_tree_hop_counts_and_tags_are_exact() {
    let inventory = json(DIST_INVENTORY);
    for (key, added, removed, changed) in [
        ("from_0_152_1_to_0_153_0_github_source_tree", 151, 12, 576),
        ("from_0_153_0_to_0_153_1_github_source_tree", 1, 0, 10),
        ("from_0_153_1_to_0_153_2_github_source_tree", 0, 0, 2),
        ("from_0_153_2_to_0_153_3_github_source_tree", 5, 0, 8),
        ("from_0_153_3_to_0_153_4_github_source_tree", 0, 0, 3),
        ("from_0_153_4_to_0_154_0_github_source_tree", 430, 37, 955),
    ] {
        let hop = &inventory[key];
        assert_eq!(hop["added"].as_array().expect("added").len(), added);
        assert_eq!(hop["removed"].as_array().expect("removed").len(), removed);
        assert_eq!(hop["changed"].as_array().expect("changed").len(), changed);
    }
    let tags = &inventory["github_tags"];
    assert_eq!(
        tags["rust-v0.152.1"]["commit"],
        "5adb68a49933ae446bf11935662c83dba55a0804"
    );
    assert_eq!(
        tags["rust-v0.154.0"]["commit"],
        "6b9826e3aa83b1a5947db50f4332cb9c65f1b340"
    );
    assert_eq!(
        tags["rust-v0.154.0"]["tag_object"],
        "36eab01061df3cde5f95ec20a526777b430091ba"
    );
    let classifications = inventory["changed_file_classifications"]
        .as_array()
        .expect("classifications");
    assert_eq!(classifications.len(), 7);
}

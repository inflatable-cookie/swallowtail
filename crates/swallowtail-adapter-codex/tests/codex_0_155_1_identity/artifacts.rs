use super::support::{IDENTITY, PROTOCOL, assert_sha256, json};

#[test]
fn official_npm_and_github_identity_reconcile() {
    let identity = json(IDENTITY);
    let official = &identity["official"];
    assert_eq!(identity["axis"], "codex.cli");
    assert_eq!(identity["npm_package"], "@openai/codex");
    assert_eq!(identity["npm_latest"], true);
    assert_eq!(official["version"], "0.155.1");
    assert_eq!(official["cli"], "codex-cli 0.155.1");
    assert_eq!(official["published_at"], "2026-09-18T20:09:23.628Z");
    assert_eq!(official["github_published_at"], "2026-09-18T20:03:04Z");
    let integrity = official["npm_integrity"].as_str().expect("integrity");
    assert!(integrity.starts_with("sha512-"));
    assert_eq!(
        official["npm_shasum"],
        serde_json::json!("7190987abfcabc8fcba7774518539d694a490fc8")
    );
    assert_sha256(
        &official["tarball_sha256"],
        "fded5b71797aaaf9b1c3229c0e2747b53b39887ef25f36ec7196f6d511db1a66",
    );
}

#[test]
fn official_platform_and_binary_digests_are_exact() {
    let official = &json(IDENTITY)["official"];
    assert_sha256(
        &official["darwin_arm64_tarball_sha256"],
        "93cc218b25b71c8da3edb50a013fbd22acf8f39058fb64083fefff638a084976",
    );
    assert_eq!(
        official["extracted_cli_sha256"],
        "8eaf1ad12fe6bf89b1710330f58900014322c7c5af677e43be116d8ac5fc0a9e"
    );
    assert_eq!(official["extracted_cli_size"], 228803200);
    assert_sha256(
        &official["linux_x64_tarball_sha256"],
        "f110cccdd50b0be8130b84f45b3144ea775c233f1c8bd8226da6ee719d63d206",
    );
    assert_eq!(
        official["linux_x64_cli_sha256"],
        "0753dfe1d8b87a52436deb13eb1c549661ef4c84fee2c5aa688385eebeccb761"
    );
    assert_eq!(official["linux_x64_cli_size"], 269273536);
    assert_eq!(official["git_tag"], "rust-v0.155.1");
    assert_eq!(
        official["tag_object"],
        "4e21628f9ec9ee656650cd2b62ef92225725b5ac"
    );
    assert_eq!(
        official["tag_commit"],
        "be2951ea34f0d295ed0becf97079f92fa5f6950e"
    );
}

#[test]
fn downloaded_binaries_were_hashed_and_never_executed() {
    let identity = json(IDENTITY);
    assert_eq!(
        identity["identity_decision"]["downloaded_binaries_executed"],
        false
    );
    assert_eq!(json(PROTOCOL)["downloaded_binaries_executed"], false);
    let note = identity["official"]["version_literal_note"]
        .as_str()
        .expect("version literal note");
    assert!(note.contains("never executed"));
    assert!(note.contains("0.155.1"));
}

#[test]
fn host_is_absent_and_was_not_installed() {
    let host = &json(IDENTITY)["host"];
    assert_eq!(host["installed"], false);
    let note = host["note"].as_str().expect("host note");
    assert!(note.contains("not on PATH"));
    let decision = &json(IDENTITY)["identity_decision"];
    assert_eq!(decision["host_install_changed"], false);
    assert_eq!(decision["provider_prompt_sent"], false);
    assert_eq!(decision["live_session"], false);
}

#[test]
fn published_and_unpublished_boundaries_stay_exact() {
    let identity = json(IDENTITY);
    assert_eq!(
        identity["published_stables_from_previous_ceiling"],
        serde_json::json!(["0.155.0", "0.155.1"])
    );
    assert_eq!(identity["unpublished_0_149_2"], true);
    assert_eq!(identity["unpublished_0_150_2"], true);
    assert_eq!(identity["unpublished_0_151_1"], true);
    assert_eq!(identity["unpublished_0_152_2"], true);
    assert_eq!(identity["unpublished_0_154_1"], true);
    assert_eq!(identity["alpha_ignored"], "0.156.0-alpha.14");
    assert_eq!(
        identity["claim_at_observation"]["latest_qualified"],
        "0.154.0"
    );
    assert_eq!(
        identity["claim_at_observation"]["classification_of_0_154_0"],
        "qualified_maintained"
    );
    assert_eq!(
        identity["claim_at_observation"]["classification_of_0_155_0"],
        "unverified_newer"
    );
    assert_eq!(
        identity["claim_at_observation"]["classification_of_0_155_1"],
        "unverified_newer"
    );
}

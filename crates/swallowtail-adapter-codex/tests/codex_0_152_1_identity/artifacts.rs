use super::support::{IDENTITY, PROTOCOL, assert_sha256, json};

#[test]
fn official_npm_and_github_identity_reconcile() {
    let identity = json(IDENTITY);
    let official = &identity["official"];
    assert_eq!(identity["axis"], "codex.cli");
    assert_eq!(identity["npm_package"], "@openai/codex");
    assert_eq!(identity["npm_latest"], true);
    assert_eq!(official["version"], "0.152.1");
    assert_eq!(official["cli"], "codex-cli 0.152.1");
    assert_eq!(official["published_at"], "2026-09-01T22:36:50.784Z");
    assert_eq!(official["github_published_at"], "2026-09-01T22:33:02Z");
    let integrity = official["npm_integrity"].as_str().expect("integrity");
    assert!(integrity.starts_with("sha512-"));
    assert_eq!(
        official["npm_shasum"],
        serde_json::json!("9e51ebd177c5523b299636a2d5f788922fe6eb03")
    );
    assert_sha256(
        &official["tarball_sha256"],
        "3db7aab0e08454c908a874c561f75a93d3b304f2da21957272cd7b73ff45195b",
    );
}

#[test]
fn official_platform_and_binary_digests_are_exact() {
    let official = &json(IDENTITY)["official"];
    assert_sha256(
        &official["darwin_arm64_tarball_sha256"],
        "a780ff1a424724778f85c1ccb4de3b908ad1804ef09260cd8140a2ceb7e2ab12",
    );
    assert_eq!(
        official["extracted_cli_sha256"],
        "8194ea3181f330e63023b234b0b231855e5874e0331c5ef7cbc490591497a7bf"
    );
    assert_eq!(official["extracted_cli_size"], 217778592);
    assert_sha256(
        &official["linux_x64_tarball_sha256"],
        "0ed4978e80117a5e203a436026c37276029a3642d633b6916ab45143d10565cd",
    );
    assert_eq!(
        official["linux_x64_cli_sha256"],
        "b82018241214a4a7c6b97b198585192d1dbc3aab1ddcdc640f04d8dee8c606f9"
    );
    assert_eq!(official["linux_x64_cli_size"], 255505120);
    assert_eq!(official["git_tag"], "rust-v0.152.1");
    assert_eq!(
        official["tag_object"],
        "3c6cfbab81e44218c729dc8c6b304cb760d1b8a1"
    );
    assert_eq!(
        official["tag_commit"],
        "5adb68a49933ae446bf11935662c83dba55a0804"
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
    assert!(note.contains("0.152.1"));
}

#[test]
fn host_keeps_its_recorded_0_150_1_identity_without_change() {
    let host = &json(IDENTITY)["host"];
    assert_eq!(host["installed"], true);
    assert_eq!(host["version"], "0.150.1");
    assert_eq!(host["cli"], "codex-cli 0.150.1");
    assert_eq!(host["target"], "aarch64-apple-darwin");
    assert_eq!(
        host["binary_sha256"],
        "a14f9a907c12c8812878b70e6b7d65f81c39ed795513e46a55817d7428c0ca6b"
    );
    assert_eq!(host["binary_size"], 228986048);
    assert_eq!(host["codesign_team_identifier"], "2DC432GLL2");
    assert_eq!(host["matches_prior_host_identity_record"], true);
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
        serde_json::json!(["0.152.1"])
    );
    assert_eq!(identity["unpublished_0_149_2"], true);
    assert_eq!(identity["unpublished_0_150_2"], true);
    assert_eq!(identity["unpublished_0_151_1"], true);
    assert_eq!(identity["unpublished_0_152_2"], true);
    assert_eq!(identity["alpha_ignored"], "0.153.0-alpha.4");
    assert_eq!(
        identity["claim_at_observation"]["latest_qualified"],
        "0.152.0"
    );
    assert_eq!(
        identity["claim_at_observation"]["classification_of_0_152_0"],
        "qualified_maintained"
    );
    assert_eq!(
        identity["claim_at_observation"]["classification_of_0_152_1"],
        "unverified_newer"
    );
}

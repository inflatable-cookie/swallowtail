use super::support::{IDENTITY, PROTOCOL, assert_sha256, json};

#[test]
fn official_npm_and_github_identity_reconcile() {
    let identity = json(IDENTITY);
    let official = &identity["official"];
    assert_eq!(identity["axis"], "codex.cli");
    assert_eq!(identity["npm_package"], "@openai/codex");
    assert_eq!(identity["npm_latest"], true);
    assert_eq!(official["version"], "0.154.0");
    assert_eq!(official["cli"], "codex-cli 0.154.0");
    assert_eq!(official["published_at"], "2026-09-09T22:40:10.746Z");
    assert_eq!(official["github_published_at"], "2026-09-09T22:35:38Z");
    let integrity = official["npm_integrity"].as_str().expect("integrity");
    assert!(integrity.starts_with("sha512-"));
    assert_eq!(
        official["npm_shasum"],
        serde_json::json!("eb1ec011cca427c606ef6cd4814014e59a1c3d70")
    );
    assert_sha256(
        &official["tarball_sha256"],
        "870663d4e65042dd358305e96a22af58708a28317cd4e74a85aa867c69f5859b",
    );
}

#[test]
fn official_platform_and_binary_digests_are_exact() {
    let official = &json(IDENTITY)["official"];
    assert_sha256(
        &official["darwin_arm64_tarball_sha256"],
        "2a98662d79316a59993c7233e3e25a1aa1d42da4b45904585d33a5a7da1cade1",
    );
    assert_eq!(
        official["extracted_cli_sha256"],
        "4f85982624b3898c8991cb80c0981b2aa71070e3537046c9a95950318a95afcc"
    );
    assert_eq!(official["extracted_cli_size"], 222655232);
    assert_sha256(
        &official["linux_x64_tarball_sha256"],
        "e27c83a49e6031685ee7f956c12aad5f16484d3a80181dd3fea930fb96b3832b",
    );
    assert_eq!(
        official["linux_x64_cli_sha256"],
        "3188814c35471432d4123203e0eb38e5bddc60226e3d7ddf0e59e649ea140022"
    );
    assert_eq!(official["linux_x64_cli_size"], 262858016);
    assert_eq!(official["git_tag"], "rust-v0.154.0");
    assert_eq!(
        official["tag_object"],
        "36eab01061df3cde5f95ec20a526777b430091ba"
    );
    assert_eq!(
        official["tag_commit"],
        "6b9826e3aa83b1a5947db50f4332cb9c65f1b340"
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
    assert!(note.contains("0.154.0"));
}

#[test]
fn host_matches_official_0_153_3_without_change() {
    let host = &json(IDENTITY)["host"];
    assert_eq!(host["installed"], true);
    assert_eq!(host["version"], "0.153.3");
    assert_eq!(host["cli"], "codex-cli 0.153.3");
    assert_eq!(host["target"], "aarch64-apple-darwin");
    assert_eq!(
        host["binary_sha256"],
        "0e1f892695844ad0798dab8895955846450a9e7663476ebf24615814dd377216"
    );
    assert_eq!(host["binary_size"], 220584480);
    assert_eq!(host["codesign_team_identifier"], "2DC432GLL2");
    assert_eq!(host["matches_official_0_153_3_darwin_arm64_digest"], true);
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
        serde_json::json!([
            "0.153.0", "0.153.1", "0.153.2", "0.153.3", "0.153.4", "0.154.0"
        ])
    );
    assert_eq!(identity["unpublished_0_149_2"], true);
    assert_eq!(identity["unpublished_0_150_2"], true);
    assert_eq!(identity["unpublished_0_151_1"], true);
    assert_eq!(identity["unpublished_0_152_2"], true);
    assert_eq!(identity["alpha_ignored"], "0.155.0-alpha.3.10");
    assert_eq!(
        identity["claim_at_observation"]["latest_qualified"],
        "0.152.1"
    );
    assert_eq!(
        identity["claim_at_observation"]["classification_of_0_152_1"],
        "qualified_maintained"
    );
    assert_eq!(
        identity["claim_at_observation"]["classification_of_0_153_3"],
        "unverified_newer"
    );
    assert_eq!(
        identity["claim_at_observation"]["classification_of_0_154_0"],
        "unverified_newer"
    );
}

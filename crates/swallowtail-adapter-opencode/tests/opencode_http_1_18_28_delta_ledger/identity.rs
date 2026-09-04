use super::{IDENTITY, assert_exact_strings, assert_nonempty_string, json};

#[test]
fn official_hop_identity_is_exact_and_identity_first() {
    let identity = json(IDENTITY);
    assert_eq!(identity["axis"], "opencode.server");
    assert_eq!(identity["version"], "1.18.28");
    assert_eq!(identity["npm_package"], "opencode-ai");
    assert_eq!(identity["npm_latest"], true);
    assert_eq!(identity["github_latest_tag"], "v1.18.28");
    assert_eq!(identity["official_channels_agree"], true);
    assert_eq!(identity["host"]["version"], "1.18.18");
    assert_eq!(identity["host"]["size"], 143_182_562);
    assert_eq!(identity["unpublished_next"], "1.18.29");
    assert_exact_strings(
        &identity["published_stables_from_previous_ceiling"],
        &[
            "1.18.21", "1.18.22", "1.18.23", "1.18.24", "1.18.25", "1.18.26", "1.18.27", "1.18.28",
        ],
    );

    let hops = identity["official_hops"].as_array().expect("hop array");
    assert_eq!(hops.len(), 9);
    assert_eq!(
        hops.iter()
            .map(|hop| hop["version"].as_str().unwrap())
            .collect::<Vec<_>>(),
        [
            "1.18.20", "1.18.21", "1.18.22", "1.18.23", "1.18.24", "1.18.25", "1.18.26", "1.18.27",
            "1.18.28"
        ]
    );
    for hop in hops {
        for key in [
            "npm_published_at",
            "npm_integrity",
            "npm_shasum",
            "npm_tarball_sha256",
            "github_tag_commit",
            "github_release_published_at",
            "source_archive_sha256",
        ] {
            assert_nonempty_string(hop, key);
        }
    }

    let decision = &identity["identity_decision"];
    assert_eq!(decision["shape"], "compatible-extension");
    assert_eq!(decision["raise_latest_qualified_to"], "1.18.28");
    assert_eq!(decision["claim_changed_in_identity_card"], false);
    assert_eq!(decision["downloaded_artifact_executed"], false);
    assert_eq!(decision["host_install_changed"], false);
    assert_eq!(decision["claim_card"], "g05 batch card 078");
}

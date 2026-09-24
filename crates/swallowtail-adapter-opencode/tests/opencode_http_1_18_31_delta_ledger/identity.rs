use super::{IDENTITY, assert_exact_strings, json};

const EXPECTED_HOPS: [[&str; 9]; 2] = [
    [
        "1.18.30",
        "2026-09-09T03:33:55.588Z",
        "sha512-oLcOLQE4XzDKy6T5L5d1RdVJvXHXwVlD4hRF5V317JbUQorrl2EyDdGZk5kbgv675J9FXp8usg92MZbEWhh6gQ==",
        "7917bb2ccf6a3f63b76da353798c39ac699bfca7",
        "3a97a99230d07fcbe6fd1ea2a001556297225d57f55d7880d3c9e6e29960ae90",
        "3104c1428ec91f809e5ab86631300de41eb6952e",
        "5cd8e68fdd72b27818d26d168b9c7a06b359567e",
        "2026-09-09T03:34:27Z",
        "d54574de6a2b02d58fe4d403035103a08bdca0f4eafac63d3681cda774e85cd9",
    ],
    [
        "1.18.31",
        "2026-09-14T17:47:43.078Z",
        "sha512-J95feefeWwtIaw3irx76WjzWcgQXxmuHmDVphvs5ep9X30fBJ6T6bFhw50i9Kx50MG/xPn5w2pafXIfNtdry9w==",
        "af4a04d634555c02e4f410937c232c3c087c8a10",
        "b6baa53003cd2e096981474ba9461a33aba2e6948b2b4b08c4aa1a88e6dc1b59",
        "014614d35b397775e5d397a490fc72368c894ec2",
        "a97622c801f4ca571530ddc51076af659a9c32cd",
        "2026-09-14T17:47:30Z",
        "76f69fe27ec2b44e23fa1749029e7c012eb7e975a0f0c7819e9458198dfd3896",
    ],
];

#[test]
fn official_hop_identity_is_exact_and_identity_first() {
    let identity = json(IDENTITY);
    assert_eq!(identity["axis"], "opencode.server");
    assert_eq!(identity["version"], "1.18.31");
    assert_eq!(identity["npm_package"], "opencode-ai");
    assert_eq!(identity["npm_latest"], true);
    assert_eq!(identity["github_latest_tag"], "v1.18.31");
    assert_eq!(identity["official_channels_agree"], true);
    assert_eq!(identity["github_tag_comparison_status"], "diverged");
    assert_eq!(
        identity["github_tag_comparison"]["linear_descendant"],
        false
    );
    assert_eq!(identity["github_tag_comparison"]["ahead_by"], 25);
    assert_eq!(identity["github_tag_comparison"]["behind_by"], 1);
    assert_eq!(
        identity["github_tag_comparison"]["merge_base_commit"],
        "5cd8e68fdd72b27818d26d168b9c7a06b359567e"
    );
    assert_eq!(identity["host"]["present"], false);
    assert_eq!(identity["host"]["on_path"], false);
    assert_eq!(identity["host"]["observation_only"], true);
    assert_eq!(identity["host"]["installed_or_updated"], false);
    assert_eq!(identity["unpublished_next"], "1.18.32");
    assert_exact_strings(
        &identity["published_stables_from_previous_ceiling"],
        &["1.18.31"],
    );
    assert_eq!(
        identity["unpublished_gap_in_1_18_30_through_1_18_31"],
        false
    );

    let hops = identity["official_hops"].as_array().expect("hop array");
    assert_eq!(hops.len(), EXPECTED_HOPS.len());
    for (hop, expected) in hops.iter().zip(EXPECTED_HOPS.iter()) {
        for (key, value) in [
            ("version", expected[0]),
            ("npm_published_at", expected[1]),
            ("npm_integrity", expected[2]),
            ("npm_shasum", expected[3]),
            ("npm_tarball_sha256", expected[4]),
            ("github_tag_commit", expected[5]),
            ("github_release_target_commitish", expected[6]),
            ("github_release_published_at", expected[7]),
            ("source_archive_sha256", expected[8]),
        ] {
            assert_eq!(hop[key], value, "{key} drifted for {}", expected[0]);
        }
    }

    let observation = &identity["claim_at_observation"];
    assert_eq!(observation["latest_qualified"], "1.18.30");
    assert_eq!(observation["classification_of_1_18_31"], "unverified_newer");

    let decision = &identity["identity_decision"];
    assert_eq!(decision["shape"], "compatible-extension");
    assert_eq!(decision["keep_surface_19"], true);
    assert_eq!(decision["raise_latest_qualified_to"], "1.18.31");
    assert_eq!(
        decision["selected_route_and_handler_files_byte_identical"],
        true
    );
    assert_eq!(decision["openapi_byte_identical"], true);
    assert_eq!(decision["claim_changed_in_identity_card"], false);
    assert_eq!(decision["downloaded_artifact_executed"], false);
    assert_eq!(decision["host_install_changed"], false);
    assert_eq!(decision["new_public_operation"], false);
    assert_eq!(decision["provider_prompt_sent"], false);
    assert_eq!(decision["live_server_started"], false);
}

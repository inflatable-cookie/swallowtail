use super::{IDENTITY, assert_exact_strings, json};

const EXPECTED_HOPS: [[&str; 9]; 2] = [
    [
        "1.18.29",
        "2026-09-04T23:46:25.653Z",
        "sha512-syIDVwlrYTgTOXzZe9SkInJWethbq6l3SNC762UeXyO0a9V0wGfd+U4yACvppwNBnhIsl0j2QPYYCyLpNaSomg==",
        "e7d5d249b55f6e138f195e5b81f67cb58efd5af5",
        "bec74d1c33582ac16489e524d947f4f1f02ee0eadc6053577d66b32f7034a5be",
        "16747470f976aca3d362ad730bcd3fe82ecc2c9a",
        "02a167e048d3bd7299225068d79e4fce5c830d67",
        "2026-09-04T23:47:16Z",
        "8fd2a4e179a6a001e68f1f0986e6687be524a9a28239b8997c6b7e9a72033231",
    ],
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
];

#[test]
fn official_hop_identity_is_exact_and_identity_first() {
    let identity = json(IDENTITY);
    assert_eq!(identity["axis"], "opencode.server");
    assert_eq!(identity["version"], "1.18.30");
    assert_eq!(identity["npm_package"], "opencode-ai");
    assert_eq!(identity["npm_latest"], true);
    assert_eq!(identity["github_latest_tag"], "v1.18.30");
    assert_eq!(identity["official_channels_agree"], true);
    assert_eq!(identity["github_tag_comparison_status"], "diverged");
    assert_eq!(
        identity["github_tag_comparison"]["linear_descendant"],
        false
    );
    assert_eq!(identity["github_tag_comparison"]["ahead_by"], 30);
    assert_eq!(identity["github_tag_comparison"]["behind_by"], 1);
    assert_eq!(
        identity["github_tag_comparison"]["merge_base_commit"],
        "02a167e048d3bd7299225068d79e4fce5c830d67"
    );
    assert_eq!(identity["host"]["present"], true);
    assert_eq!(identity["host"]["on_path"], true);
    assert_eq!(identity["host"]["version"], "1.18.18");
    assert_eq!(
        identity["host"]["sha256"],
        "4f5979c2dadb06fbff1335335afaaea274e58f92e79aa43cf2ed98618d555422"
    );
    assert_eq!(identity["host"]["size_bytes"], 143182562);
    assert_eq!(identity["host"]["observation_only"], true);
    assert_eq!(identity["host"]["installed_or_updated"], false);
    assert_eq!(identity["unpublished_next"], "1.18.31");
    assert_exact_strings(
        &identity["published_stables_from_previous_ceiling"],
        &["1.18.30"],
    );
    assert_eq!(
        identity["unpublished_gap_in_1_18_29_through_1_18_30"],
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
    assert_eq!(observation["latest_qualified"], "1.18.29");
    assert_eq!(observation["classification_of_1_18_30"], "unverified_newer");

    let decision = &identity["identity_decision"];
    assert_eq!(decision["shape"], "compatible-extension");
    assert_eq!(decision["keep_surface_19"], true);
    assert_eq!(decision["raise_latest_qualified_to"], "1.18.30");
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
    assert_eq!(decision["identity_card"], "g05.051");
    assert_eq!(decision["claim_card"], "g05.051");
}

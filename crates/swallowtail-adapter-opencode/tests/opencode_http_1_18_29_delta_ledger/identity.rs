use super::{IDENTITY, assert_exact_strings, json};

const EXPECTED_HOPS: [[&str; 8]; 2] = [
    [
        "1.18.28",
        "2026-09-04T15:40:40.661Z",
        "sha512-T7FvoXv0gT0fKuEdiomQuke2KbIFb8B8xj9L/4ZFnMIt70GnU1uUj0y/OkhDZ2dCgterQLdRXrdhrLLZ4lyv6Q==",
        "32b51b0e9e5054d2ea62b7c9983904234d3dcf5e",
        "ae46e3653cb85edb4eab36127f289ba71833d70c0efb56992f99eca2940117c4",
        "22006d97652839999596a34a48ff6be7dbb40c6e",
        "2026-09-04T15:38:23Z",
        "8eea501a6a00cbebe524af7c3248c0bfc56290f444671903e32aa6b799ee6616",
    ],
    [
        "1.18.29",
        "2026-09-04T23:46:25.653Z",
        "sha512-syIDVwlrYTgTOXzZe9SkInJWethbq6l3SNC762UeXyO0a9V0wGfd+U4yACvppwNBnhIsl0j2QPYYCyLpNaSomg==",
        "e7d5d249b55f6e138f195e5b81f67cb58efd5af5",
        "bec74d1c33582ac16489e524d947f4f1f02ee0eadc6053577d66b32f7034a5be",
        "16747470f976aca3d362ad730bcd3fe82ecc2c9a",
        "2026-09-04T23:47:16Z",
        "8fd2a4e179a6a001e68f1f0986e6687be524a9a28239b8997c6b7e9a72033231",
    ],
];

#[test]
fn official_hop_identity_is_exact_and_identity_first() {
    let identity = json(IDENTITY);
    assert_eq!(identity["axis"], "opencode.server");
    assert_eq!(identity["version"], "1.18.29");
    assert_eq!(identity["npm_package"], "opencode-ai");
    assert_eq!(identity["npm_latest"], true);
    assert_eq!(identity["github_latest_tag"], "v1.18.29");
    assert_eq!(identity["official_channels_agree"], true);
    assert_eq!(identity["host"]["present"], false);
    assert_eq!(identity["host"]["on_path"], false);
    assert_eq!(identity["unpublished_next"], "1.18.30");
    assert_exact_strings(
        &identity["published_stables_from_previous_ceiling"],
        &["1.18.29"],
    );
    assert_eq!(
        identity["unpublished_gap_in_1_18_28_through_1_18_29"],
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
            ("github_release_published_at", expected[6]),
            ("source_archive_sha256", expected[7]),
        ] {
            assert_eq!(hop[key], value, "{key} drifted for {}", expected[0]);
        }
    }

    let observation = &identity["claim_at_observation"];
    assert_eq!(observation["latest_qualified"], "1.18.28");
    assert_eq!(observation["classification_of_1_18_29"], "unverified_newer");

    let decision = &identity["identity_decision"];
    assert_eq!(decision["shape"], "compatible-extension");
    assert_eq!(decision["keep_surface_19"], true);
    assert_eq!(decision["raise_latest_qualified_to"], "1.18.29");
    assert_eq!(decision["claim_changed_in_identity_card"], false);
    assert_eq!(decision["downloaded_artifact_executed"], false);
    assert_eq!(decision["host_install_changed"], false);
    assert_eq!(decision["new_public_operation"], false);
    assert_eq!(decision["claim_card"], "g05 batch card 135");
}

//! Research 326: Kimi Code `0.38.0..=0.43.0` local-server containment.
//!
//! The corpus in `tests/fixtures/kimi-local-server-0.43.0/` was frozen before
//! any claim moved. These tests tie it to the live claim so a fixture edit, a
//! silently dropped hop, a re-admitted uncontained point, or a posture drift
//! back to `AllowUnverified` cannot pass.

use super::support::{
    FROZEN_0_41_0_IDENTITY, IDENTITY, assert_sha1, assert_sha256, json, strings, version,
};
use swallowtail_adapter_kimi::KIMI_CODE_AXIS;

/// Every published stable in the window with its exact frozen digests:
/// `(version, npm tarball SHA-256, GitHub commit, GitHub tree)`.
const POINTS: [[&str; 4]; 8] = [
    [
        "0.38.0",
        "d5c047dbfbbdfddf8d20030327e723ea9121af66260983a8556124580d64b549",
        "0999454bdcb5ddd98f39bffee434dcf0a810f394",
        "b0f988c19b396db2d7127d1f6482743f8c8d4a26",
    ],
    [
        "0.39.0",
        "b42ab69386d260c40f1397a6b319d05331554711815934054af815f04ca7ff48",
        "52e8d19dbd17efebc2e73f8e1a879bef7f23c2b1",
        "5baec16caa6dce8263ab011132990a4e18298ebe",
    ],
    [
        "0.39.1",
        "22594a76d0aec0cdabd41050fdd354381c106c48a2f8f5edf98394b4b5e987f7",
        "5efca0c3116743855c28426000073bfe34a4862f",
        "884913004aeb243aa4f366b9f482961fddf1fc8d",
    ],
    [
        "0.40.0",
        "e947fa378eb3f36b306ab1ebc39f74af5aadacba49e8968a5b98ec47f0cc2e83",
        "e27ee60894d714e5844db75da69f29120a2bce43",
        "4a9246c06dbad6443f0719d3a22f00d067b83a20",
    ],
    [
        "0.40.1",
        "dd6dd058384a500a08bc9d3982a8e04eb248c69403869dd16bd20353ef75e5c3",
        "0d45dddc57510e6b1306dd12c0b0703c37b8c63a",
        "c00bba788bcd263ca2833e0e981be7069b7f25fd",
    ],
    [
        "0.41.0",
        "4421e1277bbfa5e46a8e1a863fd9ba4d1a3db8dd890d928f571171ac62a80c1e",
        "95478e8c7ba248fd2470d5bb151555ec7fedd19d",
        "8d692694e0957d5c73e13fa1e8a8c174ab223f20",
    ],
    [
        "0.42.0",
        "686f888cfe7ef888159ffeb760bcdce9f4816e8b7fe9962529e59251b0ac3372",
        "6954d2c8bf94a5c7fc29cc6ae35b15d042cc4dcb",
        "c4f8f55387398906e44e6e566a181f2d7eabea40",
    ],
    [
        "0.43.0",
        "225bc17f06243edf6bcf0fc82bbe8838cab1cd426eb8e467e9ebab93d53ace90",
        "ffa94fae854dedf594919acbea280d98cbe8e14e",
        "cfad0ffd418c396bae69a96322050738aa49dcda",
    ],
];

#[test]
fn official_package_release_and_artifact_identity_is_exact() {
    let identity = json(IDENTITY);
    assert_eq!(identity["axis"], KIMI_CODE_AXIS);
    assert_eq!(identity["route"], "kimi-code.local-server");
    assert_eq!(identity["npm_package"], "@moonshot-ai/kimi-code");
    assert_eq!(identity["npm_latest"], "0.43.0");
    assert_eq!(
        identity["npm_latest_published_at"],
        "2026-09-14T12:10:41.073Z"
    );
    assert_eq!(
        identity["github_latest_published_at"],
        "2026-09-14T12:03:30Z"
    );

    for [point, tarball, commit, tree] in POINTS {
        // Each point binds on the version axis.
        version(point);
        let entry = &identity["points"][point];
        assert_eq!(
            entry["github_tag_name"],
            format!("@moonshot-ai/kimi-code@{point}")
        );
        assert_sha256(&entry["npm_tarball_sha256"], tarball);
        assert_sha1(&entry["github_commit"], commit);
        assert_sha1(&entry["github_tree"], tree);
    }

    assert_sha256(
        &identity["points"]["0.42.0"]["npm_dist_main_mjs_sha256"],
        "3f632148344f68c15633215244e1ca8c106116051cd0e744968906773230930a",
    );
    assert_sha256(
        &identity["points"]["0.43.0"]["npm_dist_main_mjs_sha256"],
        "5b300a573d306f879361eb7a2baec3a19448ea94eadca28001408b19c42eeb69",
    );
    assert_eq!(
        identity["points"]["0.42.0"]["npm_file_count"].as_u64(),
        Some(547)
    );
    assert_eq!(
        identity["points"]["0.43.0"]["npm_file_count"].as_u64(),
        Some(543)
    );
}

#[test]
fn publication_adjacency_names_every_stable_after_the_previous_ceiling() {
    let adjacency = &json(IDENTITY)["publication_adjacency"];
    assert_eq!(adjacency["previous_ceiling"], "0.38.0");
    assert_eq!(
        strings(&adjacency["published_stables_after_previous_ceiling"]),
        [
            "0.39.0", "0.39.1", "0.40.0", "0.40.1", "0.41.0", "0.42.0", "0.43.0"
        ]
    );
    assert_eq!(adjacency["major_line_reset"], false);
    for unpublished in [
        "unpublished_0_38_1",
        "unpublished_0_39_2",
        "unpublished_0_40_2",
        "unpublished_0_41_1",
        "unpublished_0_42_1",
        "unpublished_0_43_1",
    ] {
        assert_eq!(adjacency[unpublished], true, "{unpublished} stays absent");
    }
}

#[test]
fn revalidated_0_38_0_and_0_41_0_values_match_the_frozen_corpora() {
    let identity = json(IDENTITY);
    let frozen_0_41_0 = json(FROZEN_0_41_0_IDENTITY);
    assert_eq!(
        identity["points"]["0.38.0"]["npm_tarball_sha256"],
        frozen_0_41_0["publication_adjacency"]["revalidated_0_38_0"]["npm_tarball_sha256"]
    );
    assert_eq!(
        identity["points"]["0.38.0"]["github_commit"],
        frozen_0_41_0["publication_adjacency"]["revalidated_0_38_0"]["github_commit"]
    );
    assert_eq!(
        identity["points"]["0.41.0"]["npm_tarball_sha256"],
        frozen_0_41_0["official"]["npm_tarball_sha256"]
    );
    assert_eq!(
        identity["points"]["0.41.0"]["github_commit"],
        frozen_0_41_0["official"]["github_commit"]
    );
    assert_eq!(
        identity["points"]["0.39.1"]["github_commit"],
        frozen_0_41_0["publication_adjacency"]["0.39.1"]["github_commit"]
    );
    assert_eq!(
        identity["points"]["0.40.0"]["github_commit"],
        frozen_0_41_0["publication_adjacency"]["0.40.0"]["github_commit"]
    );
}

#[test]
fn host_observation_is_official_untouched() {
    let host = &json(IDENTITY)["host"];
    assert_eq!(host["version"], "0.34.0");
    assert_sha256(
        &host["sha256"],
        "9f4337e10da47843f6b550474012a53ba8b30dd665f83b176a5cd479c5f7e859",
    );
    assert_eq!(host["size"].as_u64(), Some(176_894_272));
    for guard in [
        "not_installed_by_this_run",
        "not_updated_by_this_run",
        "not_executed_by_this_run",
    ] {
        assert_eq!(host[guard], true, "{guard} holds");
    }
}

#[test]
fn no_provider_authentication_session_or_binary_execution_was_required() {
    let decision = &json(IDENTITY)["identity_decision"];
    for guard in [
        "provider_prompt_sent",
        "authentication_performed",
        "live_probe_run",
        "local_server_started",
        "host_install_changed",
        "downloaded_binaries_executed",
    ] {
        assert_eq!(decision[guard], false, "{guard} stays false");
    }
}

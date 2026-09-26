//! Official identity for Research 354. Fails if a hop is dropped, a digest
//! drifts, or the 0.43.0 revalidation no longer matches Research 326.

use super::support::{
    FROZEN_0_43_0_IDENTITY, IDENTITY, assert_sha1, assert_sha256, json, strings, version,
};
use swallowtail_adapter_kimi::KIMI_CODE_AXIS;

const POINTS: [[&str; 4]; 7] = [
    [
        "0.43.0",
        "225bc17f06243edf6bcf0fc82bbe8838cab1cd426eb8e467e9ebab93d53ace90",
        "ffa94fae854dedf594919acbea280d98cbe8e14e",
        "cfad0ffd418c396bae69a96322050738aa49dcda",
    ],
    [
        "0.43.1",
        "2ac671a704bc4f4d6f0cd1ffcec76ab10f185c376aa7edd6b0450f210563603c",
        "75ac010bcb2050338444455de8328492d152c919",
        "b102aab935f2cc837ab78b0aae37d4d02aca6e1b",
    ],
    [
        "2.0.0",
        "d1450598a1844d9bc204f07ae9fd3bb371e7df1ec841cf7c9e86bde7c09d3cd1",
        "1b89e4b039f052d10f258464413b2047acca12ba",
        "e0455fd76cf5dc3c0291661105672ab1876c90ab",
    ],
    [
        "2.0.1",
        "5b0dfb03a3e5f79b0030888c6b69679459afe8f1dc1e4175d921fe5701dd3b3e",
        "caf7d4e2fef06967280b325da06e44a4b0516eba",
        "3773a94fcd3a1890ffa9c48ccc31297625d85c71",
    ],
    [
        "2.0.2",
        "432cd0b0ed4184d01c29c5ef21a88303b81d3539f3a771ec64af6cbfa4aa8a77",
        "9d07f634be94ebeb1deba2f55d247807cf729315",
        "6ccfdc69ac48d00137a2e860b232fd25c9e0f74b",
    ],
    [
        "2.1.0",
        "25a081b7783806226434aab6c40462192d3f9e362391cba7b068af02d8b5fd71",
        "52437299ff78de3d0aff7f38f054e5eb20c512e5",
        "f75b5366d0a9b130ff9ff593ab202b990a4ea65e",
    ],
    [
        "2.1.1",
        "6690a29d7b5e14812754dd100136b7f4ee2577add8895f00fe27025b4412049f",
        "f67e6398fb3210ad8ace970e2dfd5bcc984ed61f",
        "3269cb11ea507083ee03b975d4e3edd795a12d9f",
    ],
];

#[test]
fn official_package_release_and_artifact_identity_is_exact() {
    let identity = json(IDENTITY);
    assert_eq!(identity["axis"], KIMI_CODE_AXIS);
    assert_eq!(identity["route"], "kimi-code.local-server");
    assert_eq!(identity["research"], 354);
    assert_eq!(identity["npm_package"], "@moonshot-ai/kimi-code");
    assert_eq!(identity["npm_latest"], "2.1.1");
    assert_eq!(
        identity["npm_latest_published_at"],
        "2026-09-24T07:27:15.480Z"
    );
    assert_eq!(
        identity["github_latest_published_at"],
        "2026-09-24T07:24:08Z"
    );

    for [point, tarball, commit, tree] in POINTS {
        version(point);
        let entry = &identity["points"][point];
        assert_eq!(
            entry["github_tag_name"],
            format!("@moonshot-ai/kimi-code@{point}")
        );
        assert_sha256(&entry["npm_tarball_sha256"], tarball);
        assert_sha1(&entry["github_commit"], commit);
        assert_sha1(&entry["github_tree"], tree);
        assert_eq!(entry["npm_bin_kimi"], "dist/main.mjs");
    }

    assert_sha256(
        &identity["points"]["2.1.1"]["npm_dist_main_mjs_sha256"],
        "2563a338c94868a34e1bfd2e5579067d5a472acf4cb0bf350a3bfe1092bbe59d",
    );
    assert_eq!(
        identity["points"]["2.1.1"]["npm_file_count"].as_u64(),
        Some(541)
    );
}

#[test]
fn publication_adjacency_names_the_major_line_and_every_new_stable() {
    let adjacency = &json(IDENTITY)["publication_adjacency"];
    assert_eq!(adjacency["previous_observation_ceiling"], "0.43.0");
    assert_eq!(adjacency["qualified_ceiling_at_observation"], "0.39.1");
    assert_eq!(
        strings(&adjacency["published_stables_after_previous_observation"]),
        ["0.43.1", "2.0.0", "2.0.1", "2.0.2", "2.1.0", "2.1.1"]
    );
    assert_eq!(adjacency["major_line_reset"], true);
    assert_eq!(adjacency["major_line_reset_at"], "2.0.0");
    assert_eq!(adjacency["first_unpublished_later_stable"], "2.1.2");
    for unpublished in [
        "unpublished_0_43_2",
        "unpublished_2_0_3",
        "unpublished_2_1_2",
    ] {
        assert_eq!(adjacency[unpublished], true, "{unpublished} stays absent");
    }
}

#[test]
fn revalidated_0_43_0_matches_research_326() {
    let identity = json(IDENTITY);
    let frozen = json(FROZEN_0_43_0_IDENTITY);
    let live = &identity["points"]["0.43.0"];
    let prior = &frozen["points"]["0.43.0"];
    assert_eq!(live["npm_tarball_sha256"], prior["npm_tarball_sha256"]);
    assert_eq!(
        live["npm_dist_main_mjs_sha256"],
        prior["npm_dist_main_mjs_sha256"]
    );
    assert_eq!(live["github_commit"], prior["github_commit"]);
    assert_eq!(live["github_tree"], prior["github_tree"]);
    assert_eq!(live["github_annotated_tag"], prior["github_annotated_tag"]);
    assert_eq!(live["reproduces_research_326"], true);
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
        "not_replaced_by_this_run",
        "observed_only_with_version_and_digest",
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
    assert_eq!(decision["live_turn_not_taken"], true);
}

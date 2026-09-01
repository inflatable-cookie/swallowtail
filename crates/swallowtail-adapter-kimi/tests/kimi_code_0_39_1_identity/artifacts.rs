use super::support::{FROZEN_0_38_0_IDENTITY, IDENTITY, assert_sha1, assert_sha256, json, strings};
use swallowtail_adapter_kimi::KIMI_CODE_AXIS;

#[test]
fn official_package_release_and_artifact_identity_is_exact() {
    let identity = json(IDENTITY);
    assert_eq!(identity["axis"], KIMI_CODE_AXIS);
    assert_eq!(identity["npm_package"], "@moonshot-ai/kimi-code");
    assert_eq!(identity["npm_latest"], "0.39.1");

    let official = &identity["official"];
    assert_eq!(official["version"], "0.39.1");
    assert_eq!(official["published_at"], "2026-08-28T10:01:03.520Z");
    assert_eq!(official["github_published_at"], "2026-08-28T10:01:05Z");
    assert_eq!(
        official["npm_integrity"],
        "sha512-prxUZEhr4hFTnPtm3JLEpE3+1jFH9HvCpkAGFy4WErFHX0Ax7+7KdEgplOC+lm7IGv1cZLU6HdqT6con1IFbdQ=="
    );
    assert_sha1(
        &official["npm_shasum"],
        "6ee2d2ce457b8fd4bdd110a87f013268979dce49",
    );
    assert_sha256(
        &official["npm_tarball_sha256"],
        "22594a76d0aec0cdabd41050fdd354381c106c48a2f8f5edf98394b4b5e987f7",
    );
    assert_eq!(official["npm_file_count"].as_u64(), Some(545));
    assert_eq!(official["npm_unpacked_size"].as_u64(), Some(58_051_777));
    assert_sha1(
        &official["github_annotated_tag"],
        "1c142e2b20378bfdc92629abfcc68499946bf96f",
    );
    assert_eq!(official["github_tag_name"], "@moonshot-ai/kimi-code@0.39.1");
    assert_sha1(
        &official["github_commit"],
        "5efca0c3116743855c28426000073bfe34a4862f",
    );
    assert_sha256(
        &official["darwin_arm64_zip_sha256"],
        "d3a9cc0272caa68e89e747e68e1730ab86b29cdeee8d05a976f207d19020449a",
    );
    assert_sha256(
        &official["extracted_darwin_arm64_sha256"],
        "762ee3be8b67796657409b8d5074ab0beed6f42162035bd4a274055ef0c44cdd",
    );
    assert_eq!(
        official["extracted_darwin_arm64_size"].as_u64(),
        Some(180_163_648)
    );
    assert_sha256(
        &official["linux_x64_zip_sha256"],
        "9c301ac70fa5d1f7c73a3138bae1b5664ccc05159b10c93e5eb87b3beea04c21",
    );
    assert_sha256(
        &official["extracted_linux_x64_sha256"],
        "585547e082f2f3a32dd80825626a1c8dd4e82f55b4d6a8aa14e6397c00758eca",
    );
    assert_eq!(official["sidecar_sha256_matches_manifest"], true);
    assert_eq!(official["downloaded_binaries_not_executed"], true);
}

#[test]
fn publication_adjacency_names_both_stables_and_keeps_unpublished_points_absent() {
    let adjacency = &json(IDENTITY)["publication_adjacency"];
    assert_eq!(adjacency["previous_ceiling"], "0.38.0");
    assert_eq!(
        strings(&adjacency["published_stables_after_previous_ceiling"]),
        ["0.39.0", "0.39.1"]
    );
    assert_eq!(adjacency["major_line_reset"], false);
    for unpublished in [
        "unpublished_0_38_1",
        "unpublished_0_39_2",
        "unpublished_0_40_0",
    ] {
        assert_eq!(adjacency[unpublished], true, "{unpublished} stays absent");
    }

    let intermediate = &adjacency["0.39.0"];
    assert_eq!(intermediate["published_at"], "2026-08-27T11:36:25.525Z");
    assert_sha1(
        &intermediate["npm_shasum"],
        "5b610a0b029742b5e152d3a8d3a40509603661f7",
    );
    assert_sha256(
        &intermediate["npm_tarball_sha256"],
        "b42ab69386d260c40f1397a6b319d05331554711815934054af815f04ca7ff48",
    );
    assert_sha1(
        &intermediate["github_commit"],
        "52e8d19dbd17efebc2e73f8e1a879bef7f23c2b1",
    );
}

#[test]
fn revalidated_0_38_0_digests_match_the_frozen_corpus() {
    let revalidated = &json(IDENTITY)["publication_adjacency"]["revalidated_0_38_0"];
    let frozen = json(FROZEN_0_38_0_IDENTITY);
    let frozen_official = &frozen["official"];

    assert_eq!(revalidated["matches_frozen_kimi_code_0_38_0_corpus"], true);
    assert_eq!(
        revalidated["npm_tarball_sha256"],
        frozen_official["npm_tarball_sha256"]
    );
    assert_eq!(
        revalidated["github_annotated_tag"],
        frozen_official["github_annotated_tag"]
    );
    assert_eq!(
        revalidated["github_commit"],
        frozen_official["github_commit"]
    );
    assert_eq!(
        revalidated["darwin_arm64_zip_sha256"],
        frozen_official["darwin_arm64_zip_sha256"]
    );
    assert_eq!(
        revalidated["extracted_darwin_arm64_sha256"],
        frozen_official["extracted_darwin_arm64_sha256"]
    );
    assert_eq!(
        revalidated["linux_x64_zip_sha256"],
        frozen_official["linux_x64_zip_sha256"]
    );
    assert_eq!(
        revalidated["extracted_linux_x64_sha256"],
        frozen_official["extracted_linux_x64_sha256"]
    );
}

#[test]
fn host_observation_is_official_untouched_and_below_the_new_ceiling() {
    let host = &json(IDENTITY)["host"];
    assert_eq!(host["version"], "0.34.0");
    assert_sha256(
        &host["sha256"],
        "9f4337e10da47843f6b550474012a53ba8b30dd665f83b176a5cd479c5f7e859",
    );
    assert_eq!(host["size"].as_u64(), Some(176_894_272));
    assert_eq!(host["matches_official_0_34_0_darwin_arm64_extracted"], true);
    assert_eq!(host["official_0_34_0_manifest_checksum_matches"], true);
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
    let identity = json(IDENTITY);
    let decision = &identity["identity_decision"];
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

use super::support::{FROZEN_0_38_0_IDENTITY, IDENTITY, assert_sha1, assert_sha256, json, strings};
use swallowtail_adapter_kimi::KIMI_CODE_AXIS;

#[test]
fn official_package_release_and_artifact_identity_is_exact() {
    let identity = json(IDENTITY);
    assert_eq!(identity["axis"], KIMI_CODE_AXIS);
    assert_eq!(identity["route"], "kimi-code.local-server");
    assert_eq!(identity["npm_package"], "@moonshot-ai/kimi-code");
    assert_eq!(identity["npm_latest"], "0.41.0");
    assert_eq!(identity["retargeted_from"], "0.40.1");

    let official = &identity["official"];
    assert_eq!(official["version"], "0.41.0");
    assert_eq!(official["published_at"], "2026-09-04T11:01:04.740Z");
    assert_eq!(official["github_published_at"], "2026-09-04T11:01:07Z");
    assert_eq!(
        official["npm_integrity"],
        "sha512-9F89UvhJpUVnxZm1Jjj9b+Tnb8+5Wr0BpzTE1IGedy8KXZQDZ2GErjqy5fxEfdyfHRXDOjRM6xI4N/kPfDyMAA=="
    );
    assert_sha1(
        &official["npm_shasum"],
        "b0190679d15448b31450f9485cfd074e8deb8edf",
    );
    assert_sha256(
        &official["npm_tarball_sha256"],
        "4421e1277bbfa5e46a8e1a863fd9ba4d1a3db8dd890d928f571171ac62a80c1e",
    );
    assert_eq!(official["npm_file_count"].as_u64(), Some(547));
    assert_eq!(official["npm_unpacked_size"].as_u64(), Some(58_167_778));
    assert_sha1(
        &official["github_annotated_tag"],
        "d723a3937d4b92325a71e7c9272a056cf8997baa",
    );
    assert_eq!(official["github_tag_name"], "@moonshot-ai/kimi-code@0.41.0");
    assert_sha1(
        &official["github_commit"],
        "95478e8c7ba248fd2470d5bb151555ec7fedd19d",
    );
    assert_sha256(
        &official["darwin_arm64_zip_sha256"],
        "e7d32a5e261f40e3034c34026116f458e486d8f13d7d72ca6edcf29290c51d1a",
    );
    assert_sha256(
        &official["extracted_darwin_arm64_sha256"],
        "72b3cda45275ff66a8017149806c844ddc9eee724f62e0c079d319e33691ac66",
    );
    assert_eq!(
        official["extracted_darwin_arm64_size"].as_u64(),
        Some(180_279_232)
    );
    assert_sha256(
        &official["linux_x64_zip_sha256"],
        "a51fbf04dfd39554d9b090653a6f3fbaf955c25ff93d876d9b82b8e9510cac89",
    );
    assert_sha256(
        &official["extracted_linux_x64_sha256"],
        "5031a83b21fd3792cb43ea3f4a7eb590d6f10db6043b16c814bb15bccfb89147",
    );
    assert_eq!(official["sidecar_sha256_matches_zip"], true);
    assert_eq!(official["downloaded_binaries_not_executed"], true);
}

#[test]
fn publication_adjacency_names_every_stable_after_the_ceiling() {
    let adjacency = &json(IDENTITY)["publication_adjacency"];
    assert_eq!(adjacency["previous_ceiling"], "0.38.0");
    assert_eq!(
        strings(&adjacency["published_stables_after_previous_ceiling"]),
        ["0.39.0", "0.39.1", "0.40.0", "0.40.1", "0.41.0"]
    );
    assert_eq!(adjacency["major_line_reset"], false);
    for unpublished in [
        "unpublished_0_38_1",
        "unpublished_0_39_2",
        "unpublished_0_40_2",
        "unpublished_0_41_1",
    ] {
        assert_eq!(adjacency[unpublished], true, "{unpublished} stays absent");
    }

    let collected = &adjacency["0.40.1"];
    assert_eq!(
        collected["collected_as_assigned_target_then_retained_as_adjacency"],
        true
    );
    assert_sha256(
        &collected["npm_tarball_sha256"],
        "dd6dd058384a500a08bc9d3982a8e04eb248c69403869dd16bd20353ef75e5c3",
    );
    assert_sha1(
        &collected["github_commit"],
        "0d45dddc57510e6b1306dd12c0b0703c37b8c63a",
    );
}

#[test]
fn revalidated_0_38_0_digests_match_the_frozen_corpus() {
    let revalidated = &json(IDENTITY)["publication_adjacency"]["revalidated_0_38_0"];
    let frozen_official = &json(FROZEN_0_38_0_IDENTITY)["official"];

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

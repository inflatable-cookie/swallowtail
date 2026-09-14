use super::support::{
    COMPARED_VERSIONS, DIST_INVENTORY, FROZEN_2_1_257_HELP_SHA256, FROZEN_2_1_257_IDENTITY,
    FROZEN_2_1_257_PROTOCOL, IDENTITY, INIT_RECORD_SHAPE_SHA256, OFFICIAL_2_1_258_HELP_SHA256,
    PROTOCOL, PUBLISHED_HOPS, assert_sha256, json, strings,
};
use swallowtail_adapter_claude_agent::CLAUDE_CODE_HEADLESS_AXIS;

struct HopIdentity {
    version: &'static str,
    npm_published_at: &'static str,
    github_tag_commit: &'static str,
    darwin_arm64_binary: &'static str,
    linux_x64_binary: &'static str,
    wrapper_files_changed: &'static [&'static str],
}

/// Per-hop npm/GitHub identity and the wrapper files changed from the
/// previous hop.
const HOP_IDENTITY: &[HopIdentity] = &[
    HopIdentity {
        version: "2.1.258",
        npm_published_at: "2026-09-01T22:25:07.449Z",
        github_tag_commit: "aef74afe01f65b602258d6102b0da9730ac6f0aa",
        darwin_arm64_binary: "b63136194160791c27cfa7b0403060d85eb0752991625fde8c09f9acacb17c78",
        linux_x64_binary: "704f1334ac65d3e89e1c6c1d7663293ad786a6166afdb71b5075337df630f976",
        wrapper_files_changed: &["package.json"],
    },
    HopIdentity {
        version: "2.1.259",
        npm_published_at: "2026-09-02T21:21:42.115Z",
        github_tag_commit: "f173a697aa6486945f1b9c4aa9ce5383d2c87db6",
        darwin_arm64_binary: "884baa38fe1a624be25c4a91568bf5a08b5cf4e7d7acf29b7760e3525d964898",
        linux_x64_binary: "f7dd62ae415378018cd21dd950eb3bac174ab085830304d3b8b098146bfd47b6",
        wrapper_files_changed: &["package.json", "sdk-tools.d.ts"],
    },
    HopIdentity {
        version: "2.1.260",
        npm_published_at: "2026-09-03T22:32:02.087Z",
        github_tag_commit: "b3f0e501b79fe5cfc8c10d18cf3b0b6715c5c2fb",
        darwin_arm64_binary: "3c269f66801028823e24a63ced9fdd3988cb86cf85fccd9f03f87e463b9d3e3c",
        linux_x64_binary: "7a2fdc74b6836ea3d183f665b869f0ee3baebc9713cbebffe5838da4ea7bd82e",
        wrapper_files_changed: &["package.json", "sdk-tools.d.ts"],
    },
    HopIdentity {
        version: "2.1.261",
        npm_published_at: "2026-09-04T17:49:34.927Z",
        github_tag_commit: "d7dbd9a09f59775726ed14bbea8fc9dfdff62f7b",
        darwin_arm64_binary: "5efecaff231b798be3c66def9be54183623b328b80eaef17f93c43987024e82a",
        linux_x64_binary: "4ae40dd1784e85753e742e09f267d29ecbb82890361ad3817d27560866d364a6",
        wrapper_files_changed: &["package.json"],
    },
    HopIdentity {
        version: "2.1.263",
        npm_published_at: "2026-09-06T02:07:58.391Z",
        github_tag_commit: "ab9b2cf7bb9e4f98ff264c07a22e46d83c29c558",
        darwin_arm64_binary: "ef5d2909c8af49f31ab6d5487e90316777bc2fac170adfe8160716caa8aaf4f9",
        linux_x64_binary: "26d020351e8112f4006790f3cfce43b4c9df0c1bb1d0e542364d64151b81d5ba",
        wrapper_files_changed: &["package.json"],
    },
    HopIdentity {
        version: "2.1.265",
        npm_published_at: "2026-09-08T19:05:16.492Z",
        github_tag_commit: "8e02f6ddce21f4c2585d2be501651c1d6b16246a",
        darwin_arm64_binary: "164b09eb800dedb9bb06304129fbf07743ab9db972ae07333ef4f18cde0cb8d5",
        linux_x64_binary: "e14738e3a58d1fc6ccc23b9c919451b4846bc27074a3fb48db976a7d595bdeeb",
        wrapper_files_changed: &["package.json", "sdk-tools.d.ts"],
    },
    HopIdentity {
        version: "2.1.266",
        npm_published_at: "2026-09-08T23:32:32.880Z",
        github_tag_commit: "347b38e4a733d95b2f00690a4ca58ac1544f8a1c",
        darwin_arm64_binary: "553d1b9e9e7068b275c0a783c7e139ff6503096f286e674c8c919379fb0eca62",
        linux_x64_binary: "19842705e989393fce936804df6d2ab034860e24b8f8880357981d87ffd83fac",
        wrapper_files_changed: &["package.json"],
    },
    HopIdentity {
        version: "2.1.267",
        npm_published_at: "2026-09-09T18:25:42.820Z",
        github_tag_commit: "9cdc2a4d946c586a8472e504fb20b3e79106518c",
        darwin_arm64_binary: "a681f3008f0050029aeebcab3af51bb6a55ddeb625a3af3141a4416d43cd2558",
        linux_x64_binary: "0399c793ff571d5946ef923d80b4f330d05ac4b6842a6b0775468f5d389403c0",
        wrapper_files_changed: &["package.json", "sdk-tools.d.ts"],
    },
    HopIdentity {
        version: "2.1.268",
        npm_published_at: "2026-09-10T18:41:11.770Z",
        github_tag_commit: "536a2e23d9e28586f81f17b3535281b5f2995a70",
        darwin_arm64_binary: "06a96d5423f83770f120859f1c58e60d7252cc4c122aa13043b7e7cd716bc76a",
        linux_x64_binary: "9691a2b7bd796712ca8cffb8e32e54ff7fc45b662540233171a16a94a0425653",
        wrapper_files_changed: &["package.json", "sdk-tools.d.ts"],
    },
    HopIdentity {
        version: "2.1.269",
        npm_published_at: "2026-09-11T18:12:49.253Z",
        github_tag_commit: "df52d04a4e65195c1621fe6222e0564bcccb1804",
        darwin_arm64_binary: "c942e1228b93cb4d52183b3dfbc77f28264f35aa947acd9c0853d029164cf450",
        linux_x64_binary: "25e44883f54419569a3d739f38cbbdaebe83b09895da0f343e1b003710a4775b",
        wrapper_files_changed: &["package.json", "sdk-tools.d.ts"],
    },
    HopIdentity {
        version: "2.1.270",
        npm_published_at: "2026-09-12T18:52:44.937Z",
        github_tag_commit: "2b40e76d3f03b9070e2431e0bd05b4f3ace77982",
        darwin_arm64_binary: "a506b6d970a4cf44f6abdb53a81ddcd5d3b0ce042a95c502fe9d1f946bdb8807",
        linux_x64_binary: "3a624a5a7cd79bbad4d32bd7db36f1197ecf458bc5bf1e2aed81834a01ad3ef0",
        wrapper_files_changed: &["package.json"],
    },
];

#[test]
fn official_package_release_and_artifact_identity_is_exact() {
    let identity = json(IDENTITY);
    assert_eq!(identity["axis"], CLAUDE_CODE_HEADLESS_AXIS);
    assert_eq!(identity["version"], "2.1.270");
    assert_eq!(identity["npm_package"], "@anthropic-ai/claude-code");
    assert_eq!(identity["npm_latest"], true);
    assert_eq!(identity["published_at"], "2026-09-12T18:52:44.937Z");
    assert_eq!(identity["github_tag"], "v2.1.270");
    assert_eq!(identity["github_published_at"], "2026-09-12T19:45:44Z");
    assert_eq!(
        identity["github_tag_commit"],
        "2b40e76d3f03b9070e2431e0bd05b4f3ace77982"
    );
    assert_eq!(identity["github_tag_annotated"], false);
    assert_eq!(
        identity["npm_integrity"],
        "sha512-0zMkfIWQu7/SG56VP8r780HZWvrNShzK28AbAnhKRK0ns+ToGXPT0W8UqyZmZCUKAkJDd5//TrwSOhk1+hysiw=="
    );
    assert_eq!(
        identity["npm_shasum"],
        "6cba495abac4528f76fddf32acd46b016a0d48a6"
    );
    assert_eq!(
        identity["darwin_arm64_package"],
        "@anthropic-ai/claude-code-darwin-arm64"
    );
    assert_sha256(
        &identity["darwin_arm64_binary_sha256"],
        "a506b6d970a4cf44f6abdb53a81ddcd5d3b0ce042a95c502fe9d1f946bdb8807",
    );
    assert_eq!(
        identity["linux_x64_package"],
        "@anthropic-ai/claude-code-linux-x64"
    );
    assert_sha256(
        &identity["linux_x64_binary_sha256"],
        "3a624a5a7cd79bbad4d32bd7db36f1197ecf458bc5bf1e2aed81834a01ad3ef0",
    );
    assert_eq!(identity["official_version_output"], "2.1.270 (Claude Code)");
    assert_eq!(identity["downloaded_official_binaries_executed"], false);
    assert_eq!(identity["previous_ceiling"], "2.1.257");
    assert_eq!(identity["frozen_corpus_version"], "2.1.220");
    assert_eq!(identity["frozen_agent_sdk"], "0.3.220");
    assert_eq!(
        strings(&identity["published_stables_from_previous_ceiling"]),
        PUBLISHED_HOPS
    );
    assert_eq!(identity["published_hop_count"].as_u64(), Some(11));
    assert_eq!(identity["first_unpublished_after_official"], "2.1.271");
}

#[test]
fn every_published_hop_is_frozen_with_exact_identity() {
    let identity = json(IDENTITY);
    assert_eq!(
        strings(&identity["published_stables_from_previous_ceiling"]),
        PUBLISHED_HOPS
    );
    assert_eq!(HOP_IDENTITY.len(), PUBLISHED_HOPS.len());
    for (index, hop_identity) in HOP_IDENTITY.iter().enumerate() {
        let version = hop_identity.version;
        assert_eq!(version, PUBLISHED_HOPS[index]);
        let hop = &identity["hop_ledger"][version];
        assert_eq!(
            hop["npm_published_at"], hop_identity.npm_published_at,
            "{version}"
        );
        assert_eq!(
            hop["github_tag_commit"], hop_identity.github_tag_commit,
            "{version}"
        );
        assert_sha256(
            &hop["darwin_arm64_binary_sha256"],
            hop_identity.darwin_arm64_binary,
        );
        assert_sha256(
            &hop["linux_x64_binary_sha256"],
            hop_identity.linux_x64_binary,
        );
        assert_eq!(
            strings(&hop["wrapper_files_changed_from_previous"]),
            hop_identity.wrapper_files_changed,
            "{version}"
        );
        assert_eq!(
            strings(&hop["platform_files_changed_from_previous"]),
            ["claude", "package.json"],
            "{version}"
        );
        assert_eq!(hop["mapped_source_delta"], "none", "{version}");
        assert_eq!(hop["npm_wrapper_file_count"].as_u64(), Some(7), "{version}");
        assert_eq!(
            hop["npm_platform_file_count"]["darwin-arm64"].as_u64(),
            Some(4),
            "{version}"
        );
        assert_eq!(
            hop["npm_platform_file_count"]["linux-x64"].as_u64(),
            Some(4),
            "{version}"
        );
        assert!(
            hop["npm_wrapper_integrity"]
                .as_str()
                .expect("integrity is text")
                .starts_with("sha512-")
        );
        assert_eq!(
            hop["github_release_published_at"]
                .as_str()
                .expect("release time is text")
                .len(),
            20,
            "{version}"
        );
    }
}

#[test]
fn host_observes_a_qualified_neighbour_with_unchanged_help() {
    let identity = json(IDENTITY);
    let protocol = json(PROTOCOL);
    let frozen_identity = json(FROZEN_2_1_257_IDENTITY);
    let frozen_protocol = json(FROZEN_2_1_257_PROTOCOL);
    assert_sha256(
        &frozen_identity["official_help_sha256"],
        FROZEN_2_1_257_HELP_SHA256,
    );
    assert_sha256(
        &frozen_protocol["official_help_sha256"],
        FROZEN_2_1_257_HELP_SHA256,
    );
    assert_eq!(identity["host"]["version_output"], "2.1.258 (Claude Code)");
    assert_eq!(identity["host"]["not_installed"], false);
    assert_sha256(
        &identity["host"]["native_sha256"],
        "b63136194160791c27cfa7b0403060d85eb0752991625fde8c09f9acacb17c78",
    );
    assert_eq!(
        identity["host"]["matches_official_darwin_arm64_2_1_258"],
        true
    );
    assert_eq!(identity["host"]["native_size"].as_u64(), Some(199027600));
    assert_sha256(
        &identity["host"]["help_sha256"],
        OFFICIAL_2_1_258_HELP_SHA256,
    );
    assert_sha256(
        &identity["official_2_1_258_help_sha256"],
        OFFICIAL_2_1_258_HELP_SHA256,
    );
    assert_eq!(
        identity["official_2_1_258_help_byte_identical_to_frozen_2_1_257"],
        true
    );
    assert_eq!(
        identity["host"]["help_byte_identical_to_frozen_2_1_257"],
        true
    );
    assert_eq!(
        protocol["official_2_1_258_help_byte_identical_to_frozen_2_1_257"],
        true
    );
    assert_eq!(
        identity["host"]["codesign_identifier"],
        "com.anthropic.claude-code"
    );
    assert_eq!(identity["host"]["codesign_team"], "Q6L2SF6YDW");
}

#[test]
fn embedded_init_record_shape_is_shared_across_every_hop() {
    let protocol = json(PROTOCOL);
    assert_sha256(
        &protocol["stream_json_stream_anchors"]["init_record_shape_sha256"],
        INIT_RECORD_SHAPE_SHA256,
    );
    assert_eq!(strings(&protocol["compared_versions"]), COMPARED_VERSIONS);
    let inventory = json(DIST_INVENTORY);
    assert_sha256(
        &inventory["published_hop_tarball_sha256"]["wrapper"]["2.1.270"],
        "ee37736066e3349c977db70bdfaf4f3cf7c398a06e2a051fe8e00728e1f6e932",
    );
    assert_sha256(
        &inventory["published_hop_tarball_sha256"]["wrapper"]["2.1.257"],
        "e11188b92a6198945329e4e2657ebff206fbc014b3e5fc95644f76b62300ad5d",
    );
}

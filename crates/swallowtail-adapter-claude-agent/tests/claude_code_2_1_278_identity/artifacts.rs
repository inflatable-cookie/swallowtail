use super::support::{
    COMPARED_VERSIONS, DIST_INVENTORY, FROZEN_2_1_270_IDENTITY, IDENTITY, PROTOCOL, PUBLISHED_HOPS,
    assert_sha256, json, strings,
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
        version: "2.1.271",
        npm_published_at: "2026-09-14T19:45:19.456Z",
        github_tag_commit: "f2ccbe279409fc6357c2fb1bfd7e195e0e1bcfca",
        darwin_arm64_binary: "87d119eb46782a1369d79d6e8cc00557f1b51bc9db7a51bd01fa2f909d8fe3dc",
        linux_x64_binary: "5e7b6fc24d0e124f68e99a0641c47b662945c6d197d768d7ad7a72cbf1897f58",
        wrapper_files_changed: &["package.json", "sdk-tools.d.ts"],
    },
    HopIdentity {
        version: "2.1.272",
        npm_published_at: "2026-09-14T23:34:13.565Z",
        github_tag_commit: "f96c3b49c4c8721685206aaab23609b2d399df4e",
        darwin_arm64_binary: "195e24e8e1f9bf46f1eaee72d434a33e18f9f5796f29a6348a00d16c5f8aee75",
        linux_x64_binary: "d81396a668eb76fbddb49a2a5841f1b5d7af96b4c1f6500ced92f2c988f5bcd4",
        wrapper_files_changed: &["package.json"],
    },
    HopIdentity {
        version: "2.1.273",
        npm_published_at: "2026-09-15T18:06:34.098Z",
        github_tag_commit: "aad35ba32864f3f7cdaf1a08bb2e400c6e93358b",
        darwin_arm64_binary: "953e9880dbcb0b70f31c1f508de6a3fd389753d131688557fd992da9184693fb",
        linux_x64_binary: "6c752e2cc7c110c9df15f26d8d134d438c5ae95dbd610efc1a308bf7f9c5f6c1",
        wrapper_files_changed: &["package.json", "sdk-tools.d.ts"],
    },
    HopIdentity {
        version: "2.1.274",
        npm_published_at: "2026-09-16T22:36:09.883Z",
        github_tag_commit: "68ac8bbf0245b615b41517bf8f2b2f35af1ae31d",
        darwin_arm64_binary: "3509913f9d1576316c8845b88837f8fd3bbbcf26625833ac82cfb6b8985da94a",
        linux_x64_binary: "15e2d05148f801b5774032faad87e624ecd172e9903288bda448b892eb58fa07",
        wrapper_files_changed: &["package.json", "sdk-tools.d.ts"],
    },
    HopIdentity {
        version: "2.1.275",
        npm_published_at: "2026-09-17T20:20:31.805Z",
        github_tag_commit: "38035964a79edd8995a556e2027f891a826e533c",
        darwin_arm64_binary: "1b8177fe49f2be5bacc75e89b5f88fa7454791283113ace16a453fe9171d179b",
        linux_x64_binary: "13586f3150a7ca1655f36e1dba759fb404e0f7cf7021d4a3dcd5e6f604e56156",
        wrapper_files_changed: &["package.json", "sdk-tools.d.ts"],
    },
    HopIdentity {
        version: "2.1.276",
        npm_published_at: "2026-09-18T01:39:31.986Z",
        github_tag_commit: "31a3b00bef145a0393d9dbf840a98674fec07712",
        darwin_arm64_binary: "9de364db11a410d53cbbb0f6b1f18c66c90053efc9a63370072856d10db66329",
        linux_x64_binary: "8a56c8a14bd3cb246e2bdb7e60aefe0f609bff78c8bbcc5ea6b1817c111c6145",
        wrapper_files_changed: &["package.json"],
    },
    HopIdentity {
        version: "2.1.277",
        npm_published_at: "2026-09-18T16:22:26.548Z",
        github_tag_commit: "ca02e7deeb0707f558b0afd7e9e5d67a382e12b3",
        darwin_arm64_binary: "73d6a2a55c46907e49bd8bb7608e134333bd71173351ee16ddce7d7db9914b9c",
        linux_x64_binary: "722210f05ba494d8f6df69423c4d4f2960900f7a007d0532851c7a36e375cab7",
        wrapper_files_changed: &["package.json", "sdk-tools.d.ts"],
    },
    HopIdentity {
        version: "2.1.278",
        npm_published_at: "2026-09-19T01:48:59.758Z",
        github_tag_commit: "bf7d404e26a5fb6167d21b46c93a2bf6c22ab274",
        darwin_arm64_binary: "bd245662fb8a0e321b3bf133e930371d6563c387527885f30b2613aef3ba14d6",
        linux_x64_binary: "5c4735937844e84f8a93306e841a5b0e12252909b07870f789b190468da147ab",
        wrapper_files_changed: &["package.json"],
    },
];

#[test]
fn official_package_release_and_artifact_identity_is_exact() {
    let identity = json(IDENTITY);
    assert_eq!(identity["axis"], CLAUDE_CODE_HEADLESS_AXIS);
    assert_eq!(identity["version"], "2.1.278");
    assert_eq!(identity["npm_package"], "@anthropic-ai/claude-code");
    assert_eq!(identity["npm_latest"], true);
    assert_eq!(identity["published_at"], "2026-09-19T01:48:59.758Z");
    assert_eq!(identity["github_tag"], "v2.1.278");
    assert_eq!(identity["github_published_at"], "2026-09-19T03:10:40Z");
    assert_eq!(
        identity["github_tag_commit"],
        "bf7d404e26a5fb6167d21b46c93a2bf6c22ab274"
    );
    assert_eq!(identity["github_tag_annotated"], false);
    assert_eq!(
        identity["npm_integrity"],
        "sha512-mfNRqC0GaEXqmP97NiwJBeYBmRuqe2VzgLUreUUaEhyJxJWx2Z6ClW1tBOncGNNXdhj6EY4LPvUIWP+oq311CA=="
    );
    assert_eq!(
        identity["npm_shasum"],
        "f4067f95f4925cd3c2ba8dc55f4376e0185c2fc7"
    );
    assert_eq!(
        identity["darwin_arm64_package"],
        "@anthropic-ai/claude-code-darwin-arm64"
    );
    assert_sha256(
        &identity["darwin_arm64_binary_sha256"],
        "bd245662fb8a0e321b3bf133e930371d6563c387527885f30b2613aef3ba14d6",
    );
    assert_eq!(
        identity["linux_x64_package"],
        "@anthropic-ai/claude-code-linux-x64"
    );
    assert_sha256(
        &identity["linux_x64_binary_sha256"],
        "5c4735937844e84f8a93306e841a5b0e12252909b07870f789b190468da147ab",
    );
    assert_eq!(identity["official_version_output"], "2.1.278 (Claude Code)");
    assert_eq!(identity["downloaded_official_binaries_executed"], false);
    assert_eq!(identity["previous_ceiling"], "2.1.270");
    assert_eq!(identity["frozen_corpus_version"], "2.1.220");
    assert_eq!(identity["frozen_agent_sdk"], "0.3.220");
    assert_eq!(
        strings(&identity["published_stables_from_previous_ceiling"]),
        PUBLISHED_HOPS
    );
    assert_eq!(identity["published_hop_count"].as_u64(), Some(8));
    assert_eq!(identity["first_unpublished_after_official"], "2.1.279");
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
fn host_is_absent_and_prior_ceiling_reproduces() {
    let identity = json(IDENTITY);
    let protocol = json(PROTOCOL);
    let frozen = json(FROZEN_2_1_270_IDENTITY);
    assert_eq!(identity["host"]["not_installed"], true);
    assert_eq!(identity["host"]["version_output"], serde_json::Value::Null);
    assert_eq!(protocol["host_not_installed"], true);
    assert_eq!(protocol["host_help_reprobed"], false);
    assert_eq!(frozen["version"], "2.1.270");
    assert_sha256(
        &frozen["darwin_arm64_binary_sha256"],
        "a506b6d970a4cf44f6abdb53a81ddcd5d3b0ce042a95c502fe9d1f946bdb8807",
    );
    assert_sha256(
        &frozen["linux_x64_binary_sha256"],
        "3a624a5a7cd79bbad4d32bd7db36f1197ecf458bc5bf1e2aed81834a01ad3ef0",
    );
}

#[test]
fn prior_ceiling_wrapper_tarball_reproduces_research_307() {
    let inventory = json(DIST_INVENTORY);
    assert_eq!(
        strings(&json(PROTOCOL)["compared_versions"]),
        COMPARED_VERSIONS
    );
    assert_sha256(
        &inventory["published_hop_tarball_sha256"]["wrapper"]["2.1.270"],
        "ee37736066e3349c977db70bdfaf4f3cf7c398a06e2a051fe8e00728e1f6e932",
    );
    assert_sha256(
        &inventory["published_hop_tarball_sha256"]["wrapper"]["2.1.278"],
        "08c6dfcf3dafcfd30e09b2926c596e274f0fa20844a5801ada7f1c8e6227157e",
    );
}

use serde_json::Value;
use std::collections::BTreeSet;
use swallowtail_adapter_antigravity::{
    ANTIGRAVITY_BASELINE_VERSION, ANTIGRAVITY_CATALOGUE_LATEST_QUALIFIED_VERSION,
    ANTIGRAVITY_HEADLESS_LATEST_QUALIFIED_VERSION, ANTIGRAVITY_RELEASE_AXIS,
    antigravity_catalogue_claim, antigravity_headless_claim,
};
use swallowtail_core::{
    InterfaceCompatibilityAssessment, InterfaceNewerVersionPosture, InterfaceSupportStatus,
    InterfaceVersion,
};

const IDENTITY: &str = include_str!("fixtures/antigravity-cli-1.3.1/identity.json");
const DIST: &str = include_str!("fixtures/antigravity-cli-1.3.1/dist-inventory.json");
const PROTOCOL: &str = include_str!("fixtures/antigravity-cli-1.3.1/protocol.json");

const HOPS: &[&str] = &[
    "1.2.12", "1.2.13", "1.2.14", "1.2.15", "1.2.16", "1.2.17", "1.3.0", "1.3.1",
];
const CATALOGUE_BEHAVIOR: &str = "antigravity.catalogue.cli-1.1.8-artifact-1.1.9-v1";
const RELEASE_ASSET_NAMES: &[&str] = &[
    "agy_cli_linux_arm64.tar.gz",
    "agy_cli_linux_arm64_musl.tar.gz",
    "agy_cli_linux_x64.tar.gz",
    "agy_cli_linux_x64_musl.tar.gz",
    "agy_cli_mac_arm64.tar.gz",
    "agy_cli_mac_x64.tar.gz",
    "agy_cli_windows_arm64.zip",
    "agy_cli_windows_x64.zip",
];

struct ExpectedArtifact {
    version: &'static str,
    published_at: &'static str,
    commit: &'static str,
    linux_archive: &'static str,
    linux_binary: &'static str,
    mac_archive: &'static str,
    mac_binary: &'static str,
}

const ARTIFACTS: &[ExpectedArtifact] = &[
    ExpectedArtifact {
        version: "1.2.12",
        published_at: "2026-09-27T05:06:09Z",
        commit: "8cb7cd1bbab008cada5e8b27b56ed29107f45de0",
        linux_archive: "26c7c4c661d6c9beda734fcf305031056a6ea46e697c4533e8151179724e2950",
        linux_binary: "ce6fdd9e7621ee9ac6eedaa337731ca1f235e412ff57cf9eabcd2aa23b3576ca",
        mac_archive: "076a1f0a1874a2843862af9d0eeae751775a84e736e35a84de0dd268069c28cb",
        mac_binary: "6f4785c17acc4b8b539e055e926ddc65cc83279581d24357a3b6e051a2337ae3",
    },
    ExpectedArtifact {
        version: "1.2.13",
        published_at: "2026-09-29T03:58:00Z",
        commit: "77b1aad0cb850c184a987661b149985de41a3568",
        linux_archive: "b0f195d37973be7b08c3b705d7fbbcd948dacb4a3c2176ee52ca29f7159bdc21",
        linux_binary: "574b0234656c44564f3ff3b013225c86f14ae79b67d0f3b45f6aaf1b1625440e",
        mac_archive: "092513fcc213cf5034680146a8bad24c4064ecec723a630f42ee7d1046eacc98",
        mac_binary: "ef3728208834483754b06fd99963b4b6320740bf237212768dfed5256ae24ddb",
    },
    ExpectedArtifact {
        version: "1.2.14",
        published_at: "2026-09-30T04:03:27Z",
        commit: "eaf9e06660d2ca8f20f1474ed03f10e3dbfd35e5",
        linux_archive: "68cf4d221cb62e0289245439d3d37f599bdc8e0c4e1e3dae03f326463a0c26dc",
        linux_binary: "0d0d3eba22daf29504dd290151c7ed9a4d33b0c6aa0acfc5da27bc3b01d2f029",
        mac_archive: "468edcc454b6bb1c321d8d42591a16ace4d1a1d628a4f1ce95ad236c9ee4cc19",
        mac_binary: "a33fdf084ecd199df00694f35a243200a3efacb1f4f3adf04ca19d76f7f714c4",
    },
    ExpectedArtifact {
        version: "1.2.15",
        published_at: "2026-10-02T17:00:58Z",
        commit: "ae27bff644d78ed0c759b4d8a6b79506f1bed96d",
        linux_archive: "bbd4a4b29f0e9fe1fc2e1345b5d44fa08540e43014da46bc2c4bf70cf05745d8",
        linux_binary: "5f9c16b286895f8f7fdecd423883ca256a85077b8acf9a6bc1111761d34df164",
        mac_archive: "66f7e9e8750a506e8a2caaedaadf479f023820f712015c9c55cfb91a2891521b",
        mac_binary: "d15693410c904242c1c3423a579f60a81e018444bb51fbccc6505f988b62a91d",
    },
    ExpectedArtifact {
        version: "1.2.16",
        published_at: "2026-10-03T03:56:08Z",
        commit: "65a3c69e388148c9327f307efe82ddb1c0c8d7d4",
        linux_archive: "d4247430e04cebdbe1ca93d9ccb483cd2f3daeb4cdb0ace5a71cd130e0bdab84",
        linux_binary: "a759ce7c7a235d9b6c281a25ead97cbbf2e92314a3ffd224e2f9144f3fae7a86",
        mac_archive: "97b03ea3e90916e0c8a49edea615406f8ec69047dde17228a478c1854445d32a",
        mac_binary: "7dca095cfc1df2c057a385ed88a76c7ba98dc103258a80be87a8f42e484cb3aa",
    },
    ExpectedArtifact {
        version: "1.2.17",
        published_at: "2026-10-05T06:20:11Z",
        commit: "274d81b9929aaa2b91a7266106d0d0b7f19adf52",
        linux_archive: "b0ed8a7c375b5af3af973f08a601e41aebb38bac7e80b922ab54d973a4275493",
        linux_binary: "c54ef90651a8646ae67334d39212c81f5946feec373ad6aa335f9ef401662bc5",
        mac_archive: "700b4c1f3544d547784baa0e4c727019ca7f34944d5269cd636e897a4d6320c6",
        mac_binary: "132ef8e1c0cba05e9a8259c4ee10ce30375ab93656bf71fa9ec255c7ba292611",
    },
    ExpectedArtifact {
        version: "1.3.0",
        published_at: "2026-10-06T06:05:28Z",
        commit: "5e9c9c6c3fb1aea16dbc77918d687a842620cd1f",
        linux_archive: "54731cc8ed8fe4a1840c38626b7e3251c6dc4484387f98005dba682507f495be",
        linux_binary: "19be6af38f7beeaa0db415df9297e314ab3d33fdd6f853434d49f88819bc68e4",
        mac_archive: "7fca9f07c3fd4b7ffcf8c61903ad6ea1550d0a35790b809ab078ee6b4a9581bf",
        mac_binary: "0e895226cb31f3ca07c780fb1eb13356f74b2470d721d266363467289bb462fb",
    },
    ExpectedArtifact {
        version: "1.3.1",
        published_at: "2026-10-07T03:22:02Z",
        commit: "968f1170bd0e002e9d0914730975bc8a2cc65861",
        linux_archive: "0e313b309ea58c71431ce86bb820936a3700e17e44eabce7bf2264617dc822db",
        linux_binary: "ce1bdaed3201bb84f35d69d2773caec4f18af52af00e8c25f6cace07e4359615",
        mac_archive: "ef5e385b32afda4cf1612368bb4bf155d3f8f4c55d51488649f508baefe77c86",
        mac_binary: "88db8b4d21ece4999fa58e0b54cea77154e47b319ee178d086c446262317f3fa",
    },
];

#[test]
fn identity_freezes_each_stable_hop_and_complete_published_asset_manifest() {
    let identity = json(IDENTITY);
    assert_eq!(identity["axis"], ANTIGRAVITY_RELEASE_AXIS);
    assert_eq!(identity["observed_at"], "2026-10-08");
    assert_eq!(
        identity["official_channel"]["repository"],
        "google-antigravity/antigravity-cli"
    );
    assert_eq!(identity["official_channel"]["latest_tag"], "1.3.1");
    assert_eq!(
        identity["official_channel"]["latest_tag_commit"],
        ARTIFACTS[7].commit
    );
    assert_eq!(
        identity["official_channel"]["latest_published_at"],
        ARTIFACTS[7].published_at
    );
    assert_eq!(
        identity["official_channel"]["latest_release_url"],
        "https://github.com/google-antigravity/antigravity-cli/releases/tag/1.3.1"
    );
    assert_eq!(identity["official_channel"]["version_command_run"], false);
    assert_exact_string_array(
        &identity["official_channel"]["stable_hops_after_current_ceiling"],
        HOPS,
    );
    assert_exact_keys(&identity["artifacts"], HOPS);

    for expected in ARTIFACTS {
        let version = expected.version;
        let artifact = &identity["artifacts"][version];
        assert_eq!(artifact["published_at"], expected.published_at);
        assert_eq!(artifact["tag_commit"], expected.commit);
        let assets = artifact["complete_published_asset_manifest"]
            .as_array()
            .expect("complete published asset manifest");
        assert_eq!(
            assets.len(),
            8,
            "{version}: all release assets are inventoried"
        );
        let names: BTreeSet<&str> = assets
            .iter()
            .map(|asset| asset["name"].as_str().expect("asset name is text"))
            .collect();
        let expected_names: BTreeSet<&str> = RELEASE_ASSET_NAMES.iter().copied().collect();
        assert_eq!(names, expected_names, "{version}: exact asset name set");
        for asset in assets {
            assert_exact_keys(asset, &["name", "sha256", "size"]);
            assert_sha256(&asset["sha256"]);
            assert!(
                asset["size"].as_u64().is_some(),
                "{version}: asset size is exact"
            );
        }

        let downloaded = &artifact["downloaded_and_verified_platforms"];
        assert_exact_keys(downloaded, &["linux_x64", "mac_arm64"]);
        for (platform, archive_digest, member_digest) in [
            ("linux_x64", expected.linux_archive, expected.linux_binary),
            ("mac_arm64", expected.mac_archive, expected.mac_binary),
        ] {
            let selected = &downloaded[platform];
            assert_exact_keys(
                selected,
                &[
                    "name",
                    "published_digest",
                    "download_sha256",
                    "size",
                    "published_digest_matched",
                    "archive_file_count",
                    "archive_members",
                ],
            );
            assert_eq!(
                selected["published_digest_matched"], true,
                "{version}/{platform}"
            );
            assert_eq!(selected["archive_file_count"], 1, "{version}/{platform}");
            assert_sha256(&selected["published_digest"]);
            assert_eq!(
                selected["published_digest"], archive_digest,
                "{version}/{platform}"
            );
            assert_eq!(selected["download_sha256"], selected["published_digest"]);
            let manifest_asset = assets
                .iter()
                .find(|asset| asset["name"] == selected["name"])
                .expect("downloaded artifact belongs to published manifest");
            assert_eq!(
                manifest_asset["sha256"], archive_digest,
                "{version}/{platform}"
            );
            assert_eq!(
                manifest_asset["size"], selected["size"],
                "{version}/{platform}"
            );
            let member = selected["archive_members"]
                .as_array()
                .expect("member inventory");
            assert_eq!(member.len(), 1, "{version}/{platform}");
            assert_exact_keys(&member[0], &["path", "kind", "size", "sha256"]);
            assert_eq!(member[0]["path"], "antigravity");
            assert_eq!(member[0]["kind"], "regular_file");
            assert_sha256(&member[0]["sha256"]);
            assert_eq!(member[0]["sha256"], member_digest, "{version}/{platform}");
        }
    }
    assert_eq!(
        identity["bounded_binary_observations"]["exhaustive_binary_forensics"],
        false
    );
    assert_eq!(
        identity["bounded_binary_observations"]["downloaded_artifacts_executed"],
        false
    );
    assert_eq!(
        identity["bounded_binary_observations"]["host_agy_version_probe_run"],
        false
    );
    assert_eq!(identity["qualification"]["provider_prompt_sent"], false);
    assert_eq!(identity["qualification"]["live_catalogue"], false);
    assert_eq!(identity["qualification"]["host_install_changed"], false);

    let hops = identity["public_git_hops"]
        .as_array()
        .expect("public Git hops");
    assert_eq!(hops.len(), HOPS.len());
    for (index, hop) in hops.iter().enumerate() {
        assert_eq!(
            hop["from"],
            if index == 0 {
                "1.2.11"
            } else {
                HOPS[index - 1]
            }
        );
        assert_eq!(hop["to"], HOPS[index]);
        assert_eq!(hop["ahead_by"], 1);
        assert_eq!(hop["behind_by"], 0);
        assert_eq!(hop["total_commits"], 1);
        assert_exact_string_array(&hop["changed_source_paths"], &["CHANGELOG.md"]);
    }

    let gaps = identity["unpublished_points"]
        .as_array()
        .expect("unpublished points");
    assert_eq!(gaps.len(), 2);
    assert_eq!(gaps[0]["version"], "1.2.18");
    assert_eq!(gaps[0]["release_present"], false);
    assert_eq!(gaps[0]["tag_present"], false);
    assert_eq!(gaps[1]["version"], "1.3.2");
    assert_eq!(gaps[1]["release_present"], false);
    assert_eq!(gaps[1]["tag_present"], false);
}

#[test]
fn every_published_hop_classifies_the_exact_changed_file_set_per_route() {
    let identity = json(IDENTITY);
    let protocol = json(PROTOCOL);
    assert_eq!(protocol["scope"], "antigravity.catalogue only");
    assert_eq!(protocol["result"]["classification"], "compatible-extension");
    assert_eq!(
        protocol["result"]["catalogue_changed_hops"],
        serde_json::json!([])
    );
    assert_eq!(protocol["result"]["headless_changed"], false);
    let dist = json(DIST);
    let hops = protocol["hops"].as_array().expect("protocol hops");
    assert_eq!(hops.len(), HOPS.len());
    let mut previous = "1.2.11";
    for (index, hop) in hops.iter().enumerate() {
        let version = HOPS[index];
        assert_exact_keys(
            hop,
            &[
                "published_at",
                "tag_commit",
                "source_compare",
                "changed_source_files",
                "release_note_change_classes",
                "catalogue_classification",
                "catalogue_classification_reason",
                "runtime_tree_inventory",
                "headless_claim_touched",
                "changed_runtime_files",
            ],
        );
        assert_exact_keys(
            &hop["source_compare"],
            &[
                "base_tag",
                "head_tag",
                "ahead_by",
                "behind_by",
                "total_commits",
                "changed_paths",
            ],
        );
        assert_eq!(hop["source_compare"]["base_tag"], previous);
        assert_eq!(hop["source_compare"]["head_tag"], version);
        assert_eq!(hop["source_compare"]["ahead_by"], 1);
        assert_eq!(hop["source_compare"]["behind_by"], 0);
        assert_eq!(hop["source_compare"]["total_commits"], 1);
        assert_exact_string_array(&hop["source_compare"]["changed_paths"], &["CHANGELOG.md"]);
        let files = hop["changed_source_files"]
            .as_array()
            .expect("changed files");
        assert_eq!(
            files.len(),
            1,
            "{version}: every changed source file classified"
        );
        assert_exact_keys(&files[0], &["path", "classification", "catalogue_mapping"]);
        assert_eq!(files[0]["path"], "CHANGELOG.md");
        assert_eq!(
            files[0]["classification"],
            "discovery and official release-note behavior record"
        );
        assert_eq!(hop["catalogue_classification"], "unchanged");
        assert_eq!(hop["headless_claim_touched"], false);
        assert!(
            hop["release_note_change_classes"]
                .as_array()
                .is_some_and(|classes| !classes.is_empty()),
            "{version}: release notes are bounded per hop"
        );
        assert_eq!(
            hop["tag_commit"], identity["artifacts"][version]["tag_commit"],
            "{version}: tag commit identity is shared"
        );
        let runtime_files = hop["changed_runtime_files"]
            .as_array()
            .expect("per-hop changed runtime files");
        assert_eq!(
            runtime_files.len(),
            2,
            "{version}: selected runtime file set"
        );
        for (runtime, platform) in runtime_files.iter().zip(["linux_x64", "mac_arm64"]) {
            assert_exact_keys(
                runtime,
                &[
                    "platform",
                    "path",
                    "from_sha256",
                    "to_sha256",
                    "classification",
                ],
            );
            assert_eq!(runtime["platform"], platform);
            assert_eq!(runtime["path"], "antigravity");
            let from = if previous == "1.2.11" {
                &dist["baseline_1_2_11"][platform]["archive_members"][0]["sha256"]
            } else {
                &dist["versions"][previous][platform]["archive_members"][0]["sha256"]
            };
            let to = &dist["versions"][version][platform]["archive_members"][0]["sha256"];
            assert_eq!(&runtime["from_sha256"], from);
            assert_eq!(&runtime["to_sha256"], to);
            assert_eq!(
                runtime["classification"],
                "release-note-bounded executable change; no selected catalogue behavior change named"
            );
        }
        previous = version;
    }
    assert!(
        protocol["bounded_changes"]["model_api_retry_note"]
            .as_str()
            .is_some_and(|note| note.contains("inference"))
    );
    assert!(protocol["bounded_changes"]["deferred"].is_string());

    assert!(dist["method"].is_string());
    assert_exact_keys(&dist["versions"], HOPS);
    for (index, version) in HOPS.iter().enumerate() {
        assert_eq!(
            protocol["hops"][index]["runtime_tree_inventory"],
            format!("dist-inventory.json#{version}")
        );
        for platform in ["linux_x64", "mac_arm64"] {
            let members = dist["versions"][*version][platform]["archive_members"]
                .as_array()
                .expect("complete extracted tree");
            assert_eq!(members.len(), 1);
            assert_eq!(members[0]["path"], "antigravity");
        }
    }
}

#[test]
fn catalogue_claim_extends_while_preserving_baseline_behavior_and_holes() {
    assert_eq!(ANTIGRAVITY_BASELINE_VERSION, "1.1.9");
    assert_eq!(ANTIGRAVITY_CATALOGUE_LATEST_QUALIFIED_VERSION, "1.3.1");
    let identity = json(IDENTITY);
    let claim = antigravity_catalogue_claim();
    assert_eq!(
        claim.id().as_str(),
        "antigravity.catalogue.release-window-1"
    );
    assert_eq!(claim.baseline().as_str(), "1.1.9");
    assert_eq!(claim.latest_qualified().as_str(), "1.3.1");
    assert_eq!(
        claim.newer_version_posture(),
        InterfaceNewerVersionPosture::AllowUnverified
    );
    assert_eq!(claim.milestones().len(), 1);
    let segment = claim.milestones().next().expect("maintained segment");
    assert_eq!(segment.behavior_revision().as_str(), CATALOGUE_BEHAVIOR);
    assert_eq!(segment.support_status(), InterfaceSupportStatus::Maintained);
    let exclusions: Vec<&str> = claim.exclusions().map(InterfaceVersion::as_str).collect();
    assert_eq!(exclusions, ["1.2.18"]);
    assert_eq!(
        identity["qualification"]["exclude_exact"],
        serde_json::json!(["1.2.18"])
    );

    for version in [
        "1.1.9", "1.1.27", "1.2.11", "1.2.12", "1.2.17", "1.3.0", "1.3.1",
    ] {
        assert!(
            matches!(
                claim.assess(&version_value(version)),
                InterfaceCompatibilityAssessment::Qualified(matched)
                    if matched.behavior_revision().as_str() == CATALOGUE_BEHAVIOR
                        && matched.support_status() == InterfaceSupportStatus::Maintained
            ),
            "{version} keeps the catalogue behavior"
        );
    }
    for excluded in ["1.1.8", "1.2.18"] {
        assert!(
            matches!(
                claim.assess(&version_value(excluded)),
                InterfaceCompatibilityAssessment::Incompatible
            ),
            "{excluded} remains excluded"
        );
    }
    assert!(matches!(
        claim.assess(&version_value("1.3.2")),
        InterfaceCompatibilityAssessment::UnverifiedNewer(_)
    ));
}

#[test]
fn headless_claim_and_its_interior_hole_are_unchanged() {
    assert_eq!(ANTIGRAVITY_HEADLESS_LATEST_QUALIFIED_VERSION, "1.2.11");
    let headless = antigravity_headless_claim();
    assert_eq!(headless.latest_qualified().as_str(), "1.2.11");
    assert!(matches!(
        headless.assess(&version_value("1.2.11")),
        InterfaceCompatibilityAssessment::Qualified(_)
    ));
    for gap in ["1.1.18", "1.2.10"] {
        assert!(matches!(
            headless.assess(&version_value(gap)),
            InterfaceCompatibilityAssessment::Incompatible
        ));
    }
    for newer in ["1.2.12", "1.3.1"] {
        assert!(matches!(
            headless.assess(&version_value(newer)),
            InterfaceCompatibilityAssessment::UnverifiedNewer(_)
        ));
    }
}

fn json(value: &str) -> Value {
    serde_json::from_str(value).expect("frozen Antigravity catalogue JSON is valid")
}

fn version_value(value: &str) -> InterfaceVersion {
    InterfaceVersion::new(value).expect("stable Antigravity version")
}

fn assert_sha256(value: &Value) {
    let digest = value.as_str().expect("SHA-256 is text");
    assert_eq!(digest.len(), 64);
    assert!(digest.bytes().all(|byte| byte.is_ascii_hexdigit()));
}

fn assert_exact_keys(value: &Value, expected: &[&str]) {
    let actual: BTreeSet<&str> = value
        .as_object()
        .expect("object with exact keys")
        .keys()
        .map(String::as_str)
        .collect();
    let expected: BTreeSet<&str> = expected.iter().copied().collect();
    assert_eq!(actual, expected);
}

fn assert_exact_string_array(value: &Value, expected: &[&str]) {
    let actual = value
        .as_array()
        .expect("string array")
        .iter()
        .map(|entry| entry.as_str().expect("array entry is text"))
        .collect::<Vec<_>>();
    assert_eq!(actual, expected);
}

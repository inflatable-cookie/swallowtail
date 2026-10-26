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

const IDENTITY: &str = include_str!("fixtures/antigravity-cli-1.4.7/identity.json");
const DIST: &str = include_str!("fixtures/antigravity-cli-1.4.7/dist-inventory.json");
const PROTOCOL: &str = include_str!("fixtures/antigravity-cli-1.4.7/protocol.json");

const HOPS: &[&str] = &[
    "1.3.2", "1.3.3", "1.3.4", "1.3.5", "1.3.6", "1.4.0", "1.4.1", "1.4.2", "1.4.3", "1.4.4",
    "1.4.5", "1.4.6", "1.4.7",
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
        version: "1.3.1",
        published_at: "2026-10-07T03:22:02Z",
        commit: "968f1170bd0e002e9d0914730975bc8a2cc65861",
        linux_archive: "0e313b309ea58c71431ce86bb820936a3700e17e44eabce7bf2264617dc822db",
        linux_binary: "ce1bdaed3201bb84f35d69d2773caec4f18af52af00e8c25f6cace07e4359615",
        mac_archive: "ef5e385b32afda4cf1612368bb4bf155d3f8f4c55d51488649f508baefe77c86",
        mac_binary: "88db8b4d21ece4999fa58e0b54cea77154e47b319ee178d086c446262317f3fa",
    },
    ExpectedArtifact {
        version: "1.3.2",
        published_at: "2026-10-08T22:39:48Z",
        commit: "8c1310b2179d1d4d952c54bd0e4424bd53e904f6",
        linux_archive: "bf8504c72097c97de77d271b160bdb10b956031cf79cf95d96cb948df161c25f",
        linux_binary: "32f9478e47d1d456e090f7ed0a297fcde2fa45ec10839904f03ea1ba00645e49",
        mac_archive: "133d497faa7ddd3aa7754d25fa1ba17afa9175eee6c2baa9e3a92ca9b345a50f",
        mac_binary: "07ca8348fa9724e988c7236f2dbecf6b089684a6f18be5f7dd61f97e8fd4561a",
    },
    ExpectedArtifact {
        version: "1.3.3",
        published_at: "2026-10-10T03:06:26Z",
        commit: "800852ea61796e9fe53077a2f945e74daacb4be6",
        linux_archive: "8ee3ca32574c431285efbf4d4732f7f8d53e959370bc4a61646e4fa496f7f2b6",
        linux_binary: "d97edcf90eda1799cd6cedc92616ef402f23c9752a723e8b8ae3e8f062ce458b",
        mac_archive: "c39926f3312e87eaa56d750658cf442669d0a716f49319d06369be7fa80520eb",
        mac_binary: "96dd362d5583c60485f5277fa64e43dd64c3ff02a2bf109fd683b99777da370d",
    },
    ExpectedArtifact {
        version: "1.3.4",
        published_at: "2026-10-13T05:09:33Z",
        commit: "9eebbedf051ffd75adb15e468f48b7b75be4b15b",
        linux_archive: "db5e81e8dee72d282cb1fb5fcd98dc5a43680730d71e3d637a67c874ec077ad3",
        linux_binary: "5b93565c5f04ea4f7b79d49d5c3237449bf6ce47b6779e8cf46c1fae97957c98",
        mac_archive: "ede28149c568c69f549b796a6142ba592b44294111acf8b79cdbffae74e72147",
        mac_binary: "9da414c0624c27aa2969a86030a070b5341c0657700512eba70b21725ce253f5",
    },
    ExpectedArtifact {
        version: "1.3.5",
        published_at: "2026-10-14T03:16:53Z",
        commit: "89081f72e10a98c3689f34b1af7261f43136ab5d",
        linux_archive: "007a5a44dbaf1caea464c3d9848c63b2f69945b01a8bf8852c56c477a64374cf",
        linux_binary: "b5385638174a5e720e904f3a2678752534fb8414d10c2c302fa16daf56cbcfc9",
        mac_archive: "60b46713d0e7217b796560ffc1c3c569f4e0b3357076ee6b60d464832d6d4105",
        mac_binary: "ebd71c4db14f79fca3fd0160332a38d9037146737422dcfc1f1d21ba01a13fff",
    },
    ExpectedArtifact {
        version: "1.3.6",
        published_at: "2026-10-15T17:51:16Z",
        commit: "680e58585fa49c266ab38bd986816f40455895cf",
        linux_archive: "e88ed835cbc4af504f64dfe20175f5572dc2a425dbfa2fcf55c0358bb444d80c",
        linux_binary: "e14253aa79d4e910cbf8bcb40b68e80f9e85476725e130ed085122c98face92b",
        mac_archive: "44e2c5a5982f67994599c533625d81b7ed56f568452ec132388bd30c42ed7465",
        mac_binary: "827ba22b4aa33557614a57aac2cc13f386e17db6b802febd3932226906ac05ab",
    },
    ExpectedArtifact {
        version: "1.4.0",
        published_at: "2026-10-16T05:24:39Z",
        commit: "e8a400403c82b7eb27a333784dc3c37985aa622e",
        linux_archive: "fd96967583c1eed94733c039638fa59b71ddbd823c75ebc8dd0e677463565aad",
        linux_binary: "60e72242f07c131f75afcd9a9e425d6ee097a1a1b3190ea98ae78fa4180bbfa8",
        mac_archive: "92b4bfb5b51e7b44ff3dcdb1a3179cb2c31b798a79185e19df3a51ae9ff121c7",
        mac_binary: "a4502aaf310b2fd6eb657fcfec2fcc31a690fbbd0c2d14c8eefbb556409024ce",
    },
    ExpectedArtifact {
        version: "1.4.1",
        published_at: "2026-10-17T20:43:34Z",
        commit: "4433f3a6dabf04ce0a5eef5b08d8e3e4067b2d8f",
        linux_archive: "e165aa7bccbd1c43e3db4a89f499b6bcf32598ce59b1de6184188cdec4418dab",
        linux_binary: "16e228cb243fa098eeb27b3b2aca963caec84e4a8ac49e3821b00c56f99f7508",
        mac_archive: "603c83664ba6c92e3f53b634d522341c415c9dd6a6ddeb5cfe0546f0c4353900",
        mac_binary: "da7e0766b076e411343542db4c09380bf4656df24da47168c0fa0f7986813e11",
    },
    ExpectedArtifact {
        version: "1.4.2",
        published_at: "2026-10-18T00:36:59Z",
        commit: "86a5bdda81a063d707ac4ab87fb72f7dfe30ab67",
        linux_archive: "6395837ac0e193dd3fe50f1228e1c4aea345ca99636d54404ab7e74397d4091c",
        linux_binary: "085f43e0d0628a8a8dabe1e547067818fa2c0adc86cfac98a3c4492918f455ef",
        mac_archive: "4fed2474da314f109cf505f1325eb84307afcd586242e6b691f074fd1a73e6aa",
        mac_binary: "74935642426b5bb1fcf07a5555f847c478f4517dd002c72b9c98159cb773b932",
    },
    ExpectedArtifact {
        version: "1.4.3",
        published_at: "2026-10-20T04:28:40Z",
        commit: "fdfd4becaee2439086d71e34ee06d326e06f7346",
        linux_archive: "319b2cb3240d244930bd077033b466941fd7ba6f0881d783eee801dbd2fb9371",
        linux_binary: "d003c9d47e12f689d3448cad0f614289ccde5fb4c58bf8b4b543a9b514c097bc",
        mac_archive: "be7a40acd87ff43e453ae216e57843919c77dc7c0278ba2d97e21591fc3b27cb",
        mac_binary: "836c446f6118140e86679ec6966a785eeedbf56b3711a8eb1f41947483e9802a",
    },
    ExpectedArtifact {
        version: "1.4.4",
        published_at: "2026-10-21T06:38:01Z",
        commit: "99b8981632ddf117c4eab7b1f82b615d4f683f01",
        linux_archive: "9ae4f7f0667a2868e15bb9ad93819ff7527c7187f12d12e3b5191137c726287b",
        linux_binary: "337dcde668441a2828968644d7982829c2d2a5e0c4144f346e1f3fc6abb3cdc7",
        mac_archive: "85482aa8969300183f71c9653d9f581248fba7f506342a8eb087f3d2503203bd",
        mac_binary: "9abb478686dc08d48699ba1604c14d15819ef6a2542d5d32173acbd5f02d712d",
    },
    ExpectedArtifact {
        version: "1.4.5",
        published_at: "2026-10-22T05:01:38Z",
        commit: "902d212319e365583596a014c152745cdf36c49a",
        linux_archive: "90cc1894b6862510869ad4cf1a6f4c043b7ae7c095bad8d5c355788a0ca935c7",
        linux_binary: "8af8e7a2a68f72fa7164e00bb59b8902c018fa43e528fc401ad7db8c2972583c",
        mac_archive: "6d628d7d25dfa188ef676564e39714667c4899b52bea6b5639d6591bc22b8696",
        mac_binary: "b89677b741d8d9d38cf0a6911ab468fac629158194039904cbfd645fdb897554",
    },
    ExpectedArtifact {
        version: "1.4.6",
        published_at: "2026-10-23T04:18:36Z",
        commit: "fd851f1da40e4347f1d05d211dcb45f94aede4c4",
        linux_archive: "513c74ad3e70323839a65d8dccc6f9977b0903e5facd914cbb06be283c2bb640",
        linux_binary: "128780f5776959ba6a991bde1f0a8b699eee37a59ed30deb9926147fedfa56c4",
        mac_archive: "8854514fc3f577c8b4271e735a7605562ecd353aae270c323c3ee6876e892296",
        mac_binary: "cab004f3c5c6cb7dc8b8712d3bc30431bd4480b17ff38a07eb1865abeffd2f27",
    },
    ExpectedArtifact {
        version: "1.4.7",
        published_at: "2026-10-24T02:47:49Z",
        commit: "bf1e6e6d4a05803d21dbe9af5ffba04d8b4fc0ff",
        linux_archive: "67fd4e70be739b1f8b18f90b511da8cd2a7aef24e888e5837402e025bf7fdbfc",
        linux_binary: "40349135371a8ed1f1af1e9f8b741de99174ae9fce6380b96c2d1f543480af7a",
        mac_archive: "9fed7bfe90cb158ac00b5b2da64dd54af732f7c062259e68ebd060b827fd0d88",
        mac_binary: "b7a13e53c9f49921692676ef8edc2e60ceb67064c00bc732d9a364ec3ad75b32",
    },
];

#[test]
fn identity_freezes_each_stable_hop_and_complete_published_asset_manifest() {
    let identity = json(IDENTITY);
    assert_eq!(identity["axis"], ANTIGRAVITY_RELEASE_AXIS);
    assert_eq!(identity["observed_at"], "2026-10-26");
    assert_eq!(
        identity["official_channel"]["repository"],
        "google-antigravity/antigravity-cli"
    );
    assert_eq!(identity["official_channel"]["latest_tag"], "1.4.7");
    assert_eq!(
        identity["official_channel"]["latest_tag_commit"],
        ARTIFACTS[13].commit
    );
    assert_eq!(
        identity["official_channel"]["latest_published_at"],
        ARTIFACTS[13].published_at
    );
    assert_eq!(identity["official_channel"]["version_command_run"], false);
    assert_exact_string_array(
        &identity["official_channel"]["stable_hops_after_current_ceiling"],
        HOPS,
    );
    assert_exact_keys(&identity["artifacts"], HOPS);

    let baseline = &identity["baseline_artifact_1_3_1"];
    assert_eq!(baseline["tag_commit"], ARTIFACTS[0].commit);
    assert_eq!(
        baseline["linux_x64"]["archive_sha256"],
        ARTIFACTS[0].linux_archive
    );
    assert_eq!(
        baseline["linux_x64"]["archive_members"][0]["sha256"],
        ARTIFACTS[0].linux_binary
    );
    assert_eq!(
        baseline["mac_arm64"]["archive_sha256"],
        ARTIFACTS[0].mac_archive
    );
    assert_eq!(
        baseline["mac_arm64"]["archive_members"][0]["sha256"],
        ARTIFACTS[0].mac_binary
    );

    for expected in &ARTIFACTS[1..] {
        let artifact = &identity["artifacts"][expected.version];
        assert_eq!(artifact["published_at"], expected.published_at);
        assert_eq!(artifact["tag_commit"], expected.commit);
        let assets = artifact["complete_published_asset_manifest"]
            .as_array()
            .expect("complete published asset manifest");
        assert_eq!(
            assets.len(),
            8,
            "{}: all release assets are inventoried",
            expected.version
        );
        let names: BTreeSet<&str> = assets
            .iter()
            .map(|asset| asset["name"].as_str().expect("asset name is text"))
            .collect();
        let expected_names: BTreeSet<&str> = RELEASE_ASSET_NAMES.iter().copied().collect();
        assert_eq!(names, expected_names);
        for asset in assets {
            assert_exact_keys(asset, &["name", "sha256", "size"]);
            assert_sha256(&asset["sha256"]);
        }
        let downloaded = &artifact["downloaded_and_verified_platforms"];
        assert_exact_keys(downloaded, &["linux_x64", "mac_arm64"]);
        for (platform, archive_digest, member_digest) in [
            ("linux_x64", expected.linux_archive, expected.linux_binary),
            ("mac_arm64", expected.mac_archive, expected.mac_binary),
        ] {
            let selected = &downloaded[platform];
            assert_eq!(selected["published_digest_matched"], true);
            assert_eq!(selected["archive_file_count"], 1);
            assert_eq!(selected["published_digest"], archive_digest);
            assert_eq!(selected["download_sha256"], selected["published_digest"]);
            let manifest_asset = assets
                .iter()
                .find(|asset| asset["name"] == selected["name"])
                .expect("downloaded artifact belongs to published manifest");
            assert_eq!(manifest_asset["sha256"], archive_digest);
            assert_eq!(manifest_asset["size"], selected["size"]);
            let member = selected["archive_members"]
                .as_array()
                .expect("member inventory");
            assert_eq!(member.len(), 1);
            assert_eq!(member[0]["path"], "antigravity");
            assert_eq!(member[0]["kind"], "regular_file");
            assert_eq!(member[0]["sha256"], member_digest);
        }
    }

    let writer = &identity["bounded_binary_observations"];
    assert_eq!(writer["exhaustive_binary_forensics"], false);
    assert_eq!(writer["downloaded_artifacts_executed"], false);
    assert_eq!(writer["host_agy_version_probe_run"], false);
    assert_eq!(
        writer["host_executable_static_observation"]["present"],
        false
    );
    assert_eq!(
        writer["selected_writer"]["func2_normalized_identical_from_1_3_1_through_1_4_7"],
        true
    );
    assert_eq!(
        writer["selected_writer"]["models_report_element_stride_bytes"],
        32
    );
    assert_eq!(writer["selected_writer"]["models_report_map_key"], "models");
    assert_eq!(
        writer["selected_writer"]["source_model_copy_grew_at"],
        "1.3.4"
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
            if index == 0 { "1.3.1" } else { HOPS[index - 1] }
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
    assert_eq!(gaps.len(), 6);
    assert_eq!(gaps[0]["version"], "1.2.18");
    assert_eq!(gaps[1]["version"], "1.3.7");
    assert_eq!(gaps[1]["release_present"], false);
    assert_eq!(gaps[1]["tag_present"], false);
    assert_eq!(gaps[2]["version"], "1.3.8");
    assert_eq!(gaps[3]["version"], "1.3.9");
    assert_eq!(gaps[4]["version"], "1.4.8");
    assert_eq!(gaps[4]["release_present"], false);
    assert_eq!(gaps[5]["version"], "1.4.9");
}

#[test]
fn every_published_hop_classifies_the_exact_changed_file_set_per_route() {
    let identity = json(IDENTITY);
    let protocol = json(PROTOCOL);
    let dist = json(DIST);
    assert_eq!(protocol["scope"], "antigravity.catalogue only");
    assert_eq!(protocol["result"]["classification"], "compatible-extension");
    assert_eq!(
        protocol["result"]["catalogue_changed_hops"],
        serde_json::json!([])
    );
    assert_eq!(protocol["result"]["headless_changed"], false);
    assert_eq!(protocol["selected_mapping"]["new_ceiling"], "1.4.7");
    assert_eq!(
        protocol["selected_mapping"]["exact_unpublished_exclusions"],
        serde_json::json!(["1.2.18", "1.3.7"])
    );
    assert!(
        protocol["bounded_changes"]["selected_writer"]
            .as_str()
            .is_some_and(|note| note.contains("32-byte") && note.contains("1.4.7"))
    );
    assert!(
        protocol["bounded_changes"]["model_picker_and_retry_notes"]
            .as_str()
            .is_some_and(|note| note.contains("inference"))
    );
    assert!(
        protocol["bounded_changes"]["minor_line"]
            .as_str()
            .is_some_and(|note| note.contains("1.4.0"))
    );

    let hops = protocol["hops"].as_array().expect("protocol hops");
    assert_eq!(hops.len(), HOPS.len());
    let mut previous = "1.3.1";
    for (index, hop) in hops.iter().enumerate() {
        let version = HOPS[index];
        assert_eq!(hop["source_compare"]["base_tag"], previous);
        assert_eq!(hop["source_compare"]["head_tag"], version);
        assert_eq!(hop["source_compare"]["ahead_by"], 1);
        assert_eq!(hop["source_compare"]["total_commits"], 1);
        assert_exact_string_array(&hop["source_compare"]["changed_paths"], &["CHANGELOG.md"]);
        assert_eq!(hop["changed_source_files"][0]["path"], "CHANGELOG.md");
        assert_eq!(hop["catalogue_classification"], "unchanged");
        assert_eq!(hop["headless_claim_touched"], false);
        assert!(
            hop["release_note_change_classes"]
                .as_array()
                .is_some_and(|classes| !classes.is_empty())
        );
        assert_eq!(
            hop["tag_commit"],
            identity["artifacts"][version]["tag_commit"]
        );
        let runtime_files = hop["changed_runtime_files"]
            .as_array()
            .expect("per-hop changed runtime files");
        assert_eq!(runtime_files.len(), 2);
        for (runtime, platform) in runtime_files.iter().zip(["linux_x64", "mac_arm64"]) {
            assert_eq!(runtime["platform"], platform);
            assert_eq!(runtime["path"], "antigravity");
            let from = if previous == "1.3.1" {
                &dist["baseline_1_3_1"][platform]["archive_members"][0]["sha256"]
            } else {
                &dist["versions"][previous][platform]["archive_members"][0]["sha256"]
            };
            let to = &dist["versions"][version][platform]["archive_members"][0]["sha256"];
            assert_eq!(&runtime["from_sha256"], from);
            assert_eq!(&runtime["to_sha256"], to);
            assert_ne!(from, to);
        }
        previous = version;
    }
}

#[test]
fn catalogue_claim_extends_through_1_4_7_and_keeps_headless_exact() {
    assert_eq!(ANTIGRAVITY_BASELINE_VERSION, "1.1.9");
    assert_eq!(ANTIGRAVITY_CATALOGUE_LATEST_QUALIFIED_VERSION, "1.4.7");
    assert_eq!(ANTIGRAVITY_HEADLESS_LATEST_QUALIFIED_VERSION, "1.2.11");
    let claim = antigravity_catalogue_claim();
    assert_eq!(
        claim.id().as_str(),
        "antigravity.catalogue.release-window-1"
    );
    assert_eq!(claim.baseline().as_str(), "1.1.9");
    assert_eq!(claim.latest_qualified().as_str(), "1.4.7");
    assert_eq!(
        claim.newer_version_posture(),
        InterfaceNewerVersionPosture::AllowUnverified
    );
    let segment = claim.milestones().next().expect("maintained segment");
    assert_eq!(segment.behavior_revision().as_str(), CATALOGUE_BEHAVIOR);
    assert_eq!(segment.support_status(), InterfaceSupportStatus::Maintained);
    let exclusions: Vec<&str> = claim.exclusions().map(InterfaceVersion::as_str).collect();
    assert_eq!(exclusions, ["1.2.18", "1.3.7"]);

    for version in [
        "1.1.9", "1.2.11", "1.3.1", "1.3.2", "1.3.6", "1.4.0", "1.4.7",
    ] {
        assert!(
            matches!(
                claim.assess(&version_value(version)),
                InterfaceCompatibilityAssessment::Qualified(matched)
                    if matched.behavior_revision().as_str() == CATALOGUE_BEHAVIOR
            ),
            "{version} keeps the catalogue behavior"
        );
    }
    for excluded in ["1.1.8", "1.2.18", "1.3.7"] {
        assert!(
            matches!(
                claim.assess(&version_value(excluded)),
                InterfaceCompatibilityAssessment::Incompatible
            ),
            "{excluded} remains excluded"
        );
    }
    assert!(matches!(
        claim.assess(&version_value("1.4.8")),
        InterfaceCompatibilityAssessment::UnverifiedNewer(_)
    ));
    assert!(matches!(
        claim.assess(&version_value("1.3.8")),
        InterfaceCompatibilityAssessment::Qualified(_)
    ));

    let headless = antigravity_headless_claim();
    assert_eq!(headless.latest_qualified().as_str(), "1.2.11");
    assert!(matches!(
        headless.assess(&version_value("1.2.11")),
        InterfaceCompatibilityAssessment::Qualified(_)
    ));
    for newer in ["1.3.1", "1.4.7"] {
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

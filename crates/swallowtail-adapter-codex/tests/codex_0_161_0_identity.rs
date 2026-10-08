use serde_json::Value;
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::PathBuf;
use swallowtail_adapter_codex::codex_exec_claim;
use swallowtail_core::{
    InterfaceCompatibilityAssessment, InterfaceNewerVersionPosture, InterfaceSupportStatus,
    InterfaceVersion, InterfaceVersionScheme,
};

const IDENTITY: &str =
    include_str!("../../../docs/research/370-codex-exec-currentness/identity.json");
const ARTIFACTS: &str =
    include_str!("../../../docs/research/370-codex-exec-currentness/artifacts.tsv");
const FILE_INVENTORY: &str =
    include_str!("../../../docs/research/370-codex-exec-currentness/artifact-file-inventory.tsv");
const SOURCE_TAGS: &str =
    include_str!("../../../docs/research/370-codex-exec-currentness/source-tags.tsv");
const SOURCE_HOPS: &str =
    include_str!("../../../docs/research/370-codex-exec-currentness/source-name-status.tsv");
const SELECTED_SOURCE_MAP: &str =
    include_str!("../../../docs/research/370-codex-exec-currentness/selected-source-map.tsv");

const STABLE_POINTS: &[&str] = &[
    "0.155.1", "0.156.0", "0.156.1", "0.157.0", "0.157.1", "0.158.0", "0.159.0", "0.159.1",
    "0.159.2", "0.159.3", "0.160.0", "0.160.1", "0.161.0",
];

fn json(value: &str) -> Value {
    serde_json::from_str(value).expect("frozen research JSON is valid")
}

fn table_rows(value: &str) -> impl Iterator<Item = Vec<&str>> {
    value
        .lines()
        .skip(1)
        .map(|line| line.split('\t').collect::<Vec<_>>())
}

fn version(value: &str) -> InterfaceVersion {
    InterfaceVersion::new(value).expect("fixture version is valid")
}

fn evidence_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../docs/research/370-codex-exec-currentness")
}

#[test]
fn npm_github_and_source_tags_freeze_every_stable_hop_to_current() {
    let identity = json(IDENTITY);
    assert_eq!(identity["axis"], "codex.cli");
    assert_eq!(identity["package"], "@openai/codex");
    assert_eq!(identity["qualifiedBaseline"], "0.155.1");
    assert_eq!(identity["officialChannels"]["npmLatest"], "0.161.0");
    assert_eq!(
        identity["officialChannels"]["githubLatestStableTag"],
        "rust-v0.161.0"
    );
    assert_eq!(
        identity["officialChannels"]["githubLatestStablePublishedAt"],
        "2026-10-07T15:58:45Z"
    );
    assert_eq!(
        identity["officialChannels"]["prereleaseIgnored"],
        "0.162.0-alpha.18"
    );

    let points = identity["stablePoints"]
        .as_array()
        .expect("stable points are an array")
        .iter()
        .map(|item| item.as_str().expect("stable point is text"))
        .collect::<Vec<_>>();
    assert_eq!(points, STABLE_POINTS);
    let hops = identity["sourceTags"]
        .as_array()
        .expect("source tag hops are an array");
    assert_eq!(hops.len(), STABLE_POINTS.len());
    assert_eq!(
        hops.iter()
            .map(|hop| hop["version"].as_str().expect("version is text"))
            .collect::<Vec<_>>(),
        STABLE_POINTS
    );

    let source_rows = table_rows(SOURCE_TAGS).collect::<Vec<_>>();
    assert_eq!(source_rows.len(), STABLE_POINTS.len());
    assert!(source_rows.iter().all(|row| row.len() == 4));
    assert_eq!(
        source_rows.iter().map(|row| row[0]).collect::<Vec<_>>(),
        STABLE_POINTS
    );
    assert_eq!(
        source_rows.last().expect("current source tag exists")[2..],
        [
            "7e21416b38834816c224ea0dfd135c3de94b2f15",
            "979011409de0a60b52f179721948e65531d26144"
        ]
    );
}

#[test]
fn every_npm_archive_has_exact_identity_and_complete_file_hash_inventory() {
    let artifacts = table_rows(ARTIFACTS).collect::<Vec<_>>();
    assert_eq!(artifacts.len(), STABLE_POINTS.len() * 3);
    let expected_kinds = ["wrapper", "darwin-arm64", "linux-x64"];
    let mut counts = BTreeMap::<(&str, &str), usize>::new();
    let mut inventory = BTreeMap::<(&str, &str), BTreeMap<&str, &str>>::new();

    for row in table_rows(FILE_INVENTORY) {
        assert_eq!(row.len(), 4);
        let (version, kind, sha, path) = (row[0], row[1], row[2], row[3]);
        assert_eq!(sha.len(), 64);
        assert!(sha.bytes().all(|byte| byte.is_ascii_hexdigit()));
        assert!(path.starts_with("./package/"));
        let files = inventory.entry((version, kind)).or_default();
        assert!(
            files.insert(path, sha).is_none(),
            "duplicate {version} {kind} {path}"
        );
        *counts.entry((version, kind)).or_default() += 1;
    }

    assert_eq!(inventory.len(), STABLE_POINTS.len() * expected_kinds.len());
    for row in &artifacts {
        assert_eq!(row.len(), 9);
        let (version, kind, package, package_version, tarball, integrity, archive_sha) =
            (row[0], row[1], row[2], row[3], row[4], row[5], row[6]);
        assert!(STABLE_POINTS.contains(&version));
        assert!(expected_kinds.contains(&kind));
        assert_eq!(package, "@openai/codex");
        assert!(package_version.starts_with(version));
        assert!(tarball.starts_with("https://registry.npmjs.org/@openai/codex/-/codex-"));
        assert!(integrity.starts_with("sha512-"));
        assert_eq!(archive_sha.len(), 64);
        assert!(archive_sha.bytes().all(|byte| byte.is_ascii_hexdigit()));
        assert_eq!(counts[&(version, kind)], row[7].parse::<usize>().unwrap());
    }

    for version in STABLE_POINTS {
        for kind in expected_kinds {
            let path = evidence_dir()
                .join("artifact-inventory")
                .join(format!("{version}-{kind}.sha256"));
            let manifest = fs::read_to_string(path).expect("complete per-file inventory exists");
            let rows = manifest
                .lines()
                .map(|line| {
                    let (sha, path) = line.split_once("  ").expect("sha256 manifest row");
                    (path, sha)
                })
                .collect::<BTreeMap<_, _>>();
            assert_eq!(rows, inventory[&(*version, kind)]);
            let expected_count = match kind {
                "wrapper" => 3,
                "darwin-arm64" => 44,
                "linux-x64" => 46,
                _ => unreachable!(),
            };
            assert_eq!(rows.len(), expected_count, "{version} {kind}");
        }
    }

    let current = artifacts
        .iter()
        .filter(|row| row[0] == "0.161.0")
        .map(|row| (row[1], row[6]))
        .collect::<BTreeMap<_, _>>();
    assert_eq!(
        current,
        BTreeMap::from([
            (
                "wrapper",
                "4e472c1098f1162e1b46d4935e5f2264f7076182949b95bb47d839b35108c5f0"
            ),
            (
                "darwin-arm64",
                "21c99b848b25ed91b92afadabe3751cc33878d2684f3e2d01cc1782e6d0abc7a"
            ),
            (
                "linux-x64",
                "d50e02c2755be551b576527b2b13c6b6a4af3340149afdf792d5491e5d899978"
            ),
        ])
    );

    let host = &json(IDENTITY)["hostObservation"];
    assert_eq!(host["binary"], "codex-cli 0.159.0");
    assert_eq!(
        host["sha256"],
        "e89718aa1969bfc4a471277bdc4679a3a3529293de0a309909822dfd67ddb77a"
    );
    assert_eq!(host["sizeBytes"], 240166592);
    assert_eq!(host["localPathRecorded"], false);
    assert_eq!(json(IDENTITY)["downloadedArtifactsExecuted"], false);
}

fn assert_sha256(value: &str, expected: &str) {
    let digest = Sha256::digest(value.as_bytes());
    let rendered = digest
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect::<String>();
    assert_eq!(rendered, expected);
}

#[test]
fn reused_research_370_identity_and_inventories_are_digest_pinned() {
    assert_sha256(
        IDENTITY,
        "cde427fdd726db08ad20b980b5dc4a78aa4003ccfe17a9472efad06603da34ea",
    );
    assert_sha256(
        ARTIFACTS,
        "f21cd03b8853694769f463569aaeebca36a4ef462259cac25e1279ecacbe4e24",
    );
    assert_sha256(
        FILE_INVENTORY,
        "7cfb5077df20a1f0c92ad992e4bff3fd969fb683a6d99c4773b8e469aaafb805",
    );
    assert_sha256(
        SOURCE_TAGS,
        "f57d6c08ce524044c45b16ec568b089fb7c03b6d4229e35f7f4ff7dd894df3d3",
    );
    assert_sha256(
        SOURCE_HOPS,
        "2098fa5860a092883bfc0b667071d547dd97acb4046c29ca76243e646d762741",
    );
    assert_sha256(
        SELECTED_SOURCE_MAP,
        "230e68f76c5682ed68468cdfa1109771f8bea95fd92119e31350a42165925ebb",
    );
    assert_sha256(
        include_str!("../../../docs/research/370-codex-exec-currentness-stop.md"),
        "4ed374209f0d071b8c451188c148e0d257ad8ff1f7f994fb598490fcce07a569",
    );
}

#[test]
fn exact_exec_claim_extends_as_a_compatible_behavior_preserving_extension() {
    let claim = codex_exec_claim();
    assert_eq!(claim.id().as_str(), "codex.exec.cli-window-2");
    assert_eq!(claim.axis().as_str(), "codex.cli");
    assert_eq!(claim.scheme(), InterfaceVersionScheme::Semantic);
    assert_eq!(
        claim.newer_version_posture(),
        InterfaceNewerVersionPosture::AllowUnverified
    );
    let segments = claim
        .milestones()
        .map(|segment| {
            (
                segment.minimum().as_str(),
                segment.maximum().as_str(),
                segment.behavior_revision().as_str(),
                segment.support_status(),
            )
        })
        .collect::<Vec<_>>();
    assert_eq!(
        segments,
        [
            (
                "0.80.0",
                "0.81.0",
                "codex.exec.jsonl-v1.retained-boolean-search",
                InterfaceSupportStatus::Deprecated
            ),
            (
                "0.84.0",
                "0.98.0",
                "codex.exec.jsonl-v1.retained-search-mode",
                InterfaceSupportStatus::Deprecated
            ),
            (
                "0.99.0",
                "0.121.0",
                "codex.exec.jsonl-v1.ephemeral-ambient",
                InterfaceSupportStatus::Deprecated
            ),
            (
                "0.122.0",
                "0.155.1",
                "codex.exec.jsonl-v1",
                InterfaceSupportStatus::Maintained
            ),
            (
                "0.156.0",
                "0.161.0",
                "codex.exec.jsonl-v1",
                InterfaceSupportStatus::Maintained
            ),
        ]
    );
    assert_eq!(
        claim
            .exclusions()
            .map(InterfaceVersion::as_str)
            .collect::<Vec<_>>(),
        [
            "0.108.0", "0.109.0", "0.149.2", "0.150.2", "0.151.1", "0.152.2", "0.154.1",
        ]
    );
    let decision = &json(IDENTITY)["claimDecision"];
    assert_eq!(decision["latestQualifiedRemains"], "0.155.1");
    assert_eq!(decision["firstStopHop"]["from"], "0.155.1");
    assert_eq!(decision["firstStopHop"]["to"], "0.156.0");
    let newer = decision["unverifiedStablePoints"]
        .as_array()
        .expect("unverified points are an array");
    assert_eq!(newer.len(), STABLE_POINTS.len() - 1);
    for point in newer {
        let point = point.as_str().expect("unverified point is text");
        assert!(STABLE_POINTS.contains(&point));
    }
    for gap in [
        "0.82.0", "0.83.0", "0.108.0", "0.109.0", "0.149.2", "0.150.2", "0.151.1", "0.152.2",
        "0.154.1", "0.155.2",
    ] {
        assert_eq!(
            claim.assess(&version(gap)),
            InterfaceCompatibilityAssessment::Incompatible,
            "{gap} remains a gap"
        );
    }

    for point in STABLE_POINTS {
        assert!(matches!(
            claim.assess(&version(point)),
            InterfaceCompatibilityAssessment::Qualified(_)
        ));
    }
    for gap in [
        "0.149.2", "0.150.2", "0.151.1", "0.152.2", "0.154.1", "0.155.2",
    ] {
        assert_eq!(
            claim.assess(&version(gap)),
            InterfaceCompatibilityAssessment::Incompatible
        );
    }
    let InterfaceCompatibilityAssessment::UnverifiedNewer(newer) =
        claim.assess(&version("0.161.1"))
    else {
        panic!("newer exec releases remain permitted as unverified");
    };
    assert_eq!(newer.latest_qualified().as_str(), "0.161.0");
}

#[test]
fn exact_source_hops_pin_both_authority_stops_and_keep_exec_only_scope() {
    let identity = json(IDENTITY);
    let stops = identity["claimDecision"]["stops"]
        .as_array()
        .expect("stops are listed");
    assert_eq!(stops.len(), 2);
    assert_eq!(stops[0]["from"], "0.155.1");
    assert_eq!(stops[0]["to"], "0.156.0");
    assert_eq!(stops[0]["kind"], "projectless-auto-trust");
    assert_eq!(stops[1]["from"], "0.156.1");
    assert_eq!(stops[1]["to"], "0.157.0");
    assert_eq!(stops[1]["kind"], "host-managed-network-policy");
    assert_eq!(identity["claimDecision"]["appServerClaimChanged"], false);
    assert_eq!(
        identity["claimDecision"]["claimIds"],
        serde_json::json!(["codex.exec.cli-window-2"])
    );

    let source_hops = table_rows(SOURCE_HOPS).collect::<Vec<_>>();
    let hop_keys = source_hops
        .iter()
        .map(|row| row[0])
        .collect::<BTreeSet<_>>();
    assert_eq!(hop_keys.len(), STABLE_POINTS.len() - 1);
    for pair in STABLE_POINTS.windows(2) {
        assert!(hop_keys.contains(format!("{}-{}", pair[0], pair[1]).as_str()));
    }

    let selected = table_rows(SELECTED_SOURCE_MAP).collect::<Vec<_>>();
    let projectless_files = selected
        .iter()
        .filter(|row| row[0] == "0.155.1-0.156.0" && row[3].starts_with("projectless-"))
        .map(|row| row[2])
        .collect::<BTreeSet<_>>();
    assert_eq!(
        projectless_files,
        BTreeSet::from([
            "codex-rs/app-server/src/request_processors/thread_processor.rs",
            "codex-rs/app-server/src/request_processors/thread_processor_tests.rs",
            "codex-rs/app-server/tests/suite/v2/thread_start.rs",
            "codex-rs/config/src/loader/mod.rs",
            "codex-rs/config/src/loader/projectless_directory_tests.rs",
            "codex-rs/config/src/state.rs",
        ])
    );
    let network_files = selected
        .iter()
        .filter(|row| row[0] == "0.156.1-0.157.0" && row[3] == "host-managed-network-policy-stop")
        .map(|row| row[2])
        .collect::<BTreeSet<_>>();
    assert_eq!(
        network_files,
        BTreeSet::from([
            "codex-rs/app-server-client/src/lib.rs",
            "codex-rs/app-server/src/application_network.rs",
            "codex-rs/app-server/src/in_process.rs",
            "codex-rs/app-server/src/in_process_bootstrap.rs",
            "codex-rs/app-server/src/lib.rs",
            "codex-rs/config/src/loader/application.rs",
            "codex-rs/config/src/loader/managed_requirements.rs",
            "codex-rs/config/src/loader/mod.rs",
            "codex-rs/exec/src/lib.rs",
        ])
    );
    let network_regression_files = selected
        .iter()
        .filter(|row| row[0] == "0.156.1-0.157.0" && row[3] == "stop-regression-evidence")
        .map(|row| row[2])
        .collect::<BTreeSet<_>>();
    assert_eq!(
        network_regression_files,
        BTreeSet::from([
            "codex-rs/app-server/src/application_network_tests.rs",
            "codex-rs/app-server/src/in_process_bootstrap_tests.rs",
            "codex-rs/app-server/tests/suite/v2/application_network.rs",
        ])
    );
    let refresh_files = selected
        .iter()
        .filter(|row| row[0] == "0.160.1-0.161.0" && row[3] == "network-policy-refresh")
        .map(|row| row[2])
        .collect::<BTreeSet<_>>();
    assert_eq!(
        refresh_files,
        BTreeSet::from([
            "codex-rs/app-server/src/application_network.rs",
            "codex-rs/app-server/src/in_process_bootstrap.rs",
            "codex-rs/app-server/src/in_process_bootstrap_tests.rs",
        ])
    );
}

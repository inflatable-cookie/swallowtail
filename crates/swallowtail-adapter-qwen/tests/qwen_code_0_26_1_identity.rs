use serde_json::Value;
use sha2::{Digest, Sha256};
use std::path::{Component, Path};
use swallowtail_adapter_qwen::{QWEN_CODE_LATEST_QUALIFIED_VERSION, qwen_headless_claim};
use swallowtail_core::{InterfaceCompatibilityAssessment, InterfaceVersion};

const ROOT: &str = "tests/fixtures/qwen-code-0.26.1";
const IDENTITY: &str = include_str!("fixtures/qwen-code-0.26.1/identity.json");
const SELECTED_SOURCES: &str =
    include_str!("fixtures/qwen-code-0.26.1/selected-source-identities.json");
const REVIEWED_SOURCES: &str =
    include_str!("fixtures/qwen-code-0.26.1/reviewed-source-identities.json");
const IDENTITY_SHA256: &str = "6d78677f0afd81a936cfce8b32f1c8cc3b783816e49bf038dceb7d269d167c9f";
const SELECTED_SOURCES_SHA256: &str =
    "ccfffe2ade51be51099871e306bde25dff3e657593e11bb5a5760551035d18c5";
const REVIEWED_SOURCES_SHA256: &str =
    "b38e96679844fd7e8ba3672196bcf2e7349ab4595a13945e7e1905de223150d7";

const VERSIONS: [&str; 6] = ["0.25.0", "0.25.1", "0.25.2", "0.25.3", "0.26.0", "0.26.1"];
const MAPPED_SOURCE_KEYS: [&str; 11] = [
    "approval_mode",
    "config",
    "dashscope",
    "exit_plan_mode",
    "permission_flow",
    "plan_mode_shell_policy",
    "reasoning_effort",
    "session",
    "stream_types",
    "system_controller",
    "top_level_options",
];
const SELECTED_SOURCE_HOPS: [&[&str]; 5] = [
    &["config", "permission_flow", "session"],
    &["config", "dashscope", "session", "top_level_options"],
    &["config", "dashscope", "session"],
    &["config", "dashscope", "session", "stream_types"],
    &["config", "dashscope"],
];

fn json(source: &str) -> Value {
    serde_json::from_str(source).expect("Qwen identity evidence is valid JSON")
}

fn fixture(relative: &str) -> String {
    format!("{ROOT}/{relative}")
}

fn file(relative: &str) -> &'static str {
    match relative {
        "dist-inventory-0.25.0.json" => {
            include_str!("fixtures/qwen-code-0.26.1/dist-inventory-0.25.0.json")
        }
        "dist-inventory-0.25.1.json" => {
            include_str!("fixtures/qwen-code-0.26.1/dist-inventory-0.25.1.json")
        }
        "dist-inventory-0.25.2.json" => {
            include_str!("fixtures/qwen-code-0.26.1/dist-inventory-0.25.2.json")
        }
        "dist-inventory-0.25.3.json" => {
            include_str!("fixtures/qwen-code-0.26.1/dist-inventory-0.25.3.json")
        }
        "dist-inventory-0.26.0.json" => {
            include_str!("fixtures/qwen-code-0.26.1/dist-inventory-0.26.0.json")
        }
        "dist-inventory-0.26.1.json" => {
            include_str!("fixtures/qwen-code-0.26.1/dist-inventory-0.26.1.json")
        }
        "dist-diff-0.25.0-to-0.25.1.json" => {
            include_str!("fixtures/qwen-code-0.26.1/dist-diff-0.25.0-to-0.25.1.json")
        }
        "dist-diff-0.25.1-to-0.25.2.json" => {
            include_str!("fixtures/qwen-code-0.26.1/dist-diff-0.25.1-to-0.25.2.json")
        }
        "dist-diff-0.25.2-to-0.25.3.json" => {
            include_str!("fixtures/qwen-code-0.26.1/dist-diff-0.25.2-to-0.25.3.json")
        }
        "dist-diff-0.25.3-to-0.26.0.json" => {
            include_str!("fixtures/qwen-code-0.26.1/dist-diff-0.25.3-to-0.26.0.json")
        }
        "dist-diff-0.26.0-to-0.26.1.json" => {
            include_str!("fixtures/qwen-code-0.26.1/dist-diff-0.26.0-to-0.26.1.json")
        }
        other => panic!("unexpected Qwen evidence file {other}"),
    }
}

fn sha256(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

fn inventory(version: &str) -> Value {
    let name = format!("dist-inventory-{version}.json");
    json(file(&name))
}

fn assert_path_set(value: &Value, key: &str, mut expected: Vec<String>) {
    let mut actual = value[key]
        .as_array()
        .expect("inventory delta is an array")
        .iter()
        .map(|path| path.as_str().expect("inventory path is text").to_owned())
        .collect::<Vec<_>>();
    actual.sort();
    expected.sort();
    assert_eq!(actual, expected, "complete path set for {key}");
}

#[test]
fn official_identity_freezes_npm_latest_and_preserves_the_claim_shape() {
    let identity = json(IDENTITY);
    assert_eq!(sha256(IDENTITY.as_bytes()), IDENTITY_SHA256);
    assert_eq!(identity["axis"], "qwen-code.package");
    assert_eq!(identity["observed_at"], "2026-10-26");
    assert_eq!(identity["npm_package"], "@qwen-code/qwen-code");
    assert_eq!(identity["npm_latest"], "0.26.1");
    assert_eq!(identity["previous_ceiling"], "0.25.0");
    assert_eq!(identity["unpublished_interior_stable"], "0.25.4");
    assert_eq!(identity["first_unpublished_later_stable"], "0.26.2");
    assert_eq!(
        identity["npm_other_dist_tags"]["preview"],
        "0.25.4-preview.1"
    );
    assert_eq!(
        identity["npm_other_dist_tags"]["nightly"],
        "0.26.1-nightly.20261025.8e28d5380c"
    );
    assert_eq!(
        identity["published_stable_hops_after_previous_ceiling"],
        serde_json::json!(["0.25.1", "0.25.2", "0.25.3", "0.26.0", "0.26.1"])
    );
    assert_eq!(
        identity["identity_decision"]["shape"],
        "compatible-extension"
    );
    assert_eq!(
        identity["identity_decision"]["raise_latest_qualified_to"],
        "0.26.1"
    );
    assert_eq!(
        identity["identity_decision"]["exclude_unpublished_0_25_4"],
        true
    );

    let versions = identity["versions"]
        .as_array()
        .expect("published artifact identities are an array");
    assert_eq!(versions.len(), VERSIONS.len());
    for (record, version) in versions.iter().zip(VERSIONS) {
        assert_eq!(record["npm_version"], version);
        assert_eq!(record["npm_dist_file_count"], record["inventory_entries"]);
        assert_eq!(
            record["mapped_source_keys"],
            serde_json::json!(MAPPED_SOURCE_KEYS)
        );
        assert_eq!(record["source_commit"].as_str().unwrap().len(), 40);
        assert_eq!(record["tarball_sha256"].as_str().unwrap().len(), 64);
    }

    assert_eq!(QWEN_CODE_LATEST_QUALIFIED_VERSION, "0.26.1");
    let claim = qwen_headless_claim();
    for stable in ["0.25.0", "0.25.1", "0.25.2", "0.25.3", "0.26.0", "0.26.1"] {
        assert!(
            claim.supports(&version(stable)),
            "published stable {stable} is qualified"
        );
    }
    for excluded in ["0.22.4", "0.23.5", "0.25.4"] {
        assert_eq!(
            claim.assess(&version(excluded)),
            InterfaceCompatibilityAssessment::Incompatible,
            "excluded unpublished point {excluded} remains excluded"
        );
    }
    assert_eq!(
        claim.assess(&version("0.25.4-preview.1")),
        InterfaceCompatibilityAssessment::Incompatible
    );
    assert_eq!(
        claim.assess(&version("0.26.1-nightly.20261025.8e28d5380c")),
        InterfaceCompatibilityAssessment::Incompatible
    );
    assert!(matches!(
        claim.assess(&version("0.26.2")),
        InterfaceCompatibilityAssessment::UnverifiedNewer(_)
    ));
}

#[test]
fn complete_package_inventories_and_each_published_hop_are_mutation_checked() {
    let identity = json(IDENTITY);
    let versions = identity["versions"].as_array().unwrap();
    for (record, version) in versions.iter().zip(VERSIONS) {
        let name = format!("dist-inventory-{version}.json");
        let source = file(&name);
        assert_eq!(sha256(source.as_bytes()), record["inventory_sha256"]);
        let inventory = json(source);
        assert_eq!(
            inventory.as_object().unwrap().len(),
            record["npm_dist_file_count"].as_u64().unwrap() as usize
        );
        for (path, digest) in inventory.as_object().unwrap() {
            assert!(!path.is_empty());
            assert_eq!(digest.as_str().unwrap().len(), 64, "sha256 for {path}");
        }
        for required in ["LICENSE", "README.md", "package.json", "cli.js"] {
            assert!(
                inventory.get(required).is_some(),
                "{version} includes {required}"
            );
        }
    }

    let hops = identity["hops"]
        .as_array()
        .expect("published hop ledger is an array");
    assert_eq!(hops.len(), SELECTED_SOURCE_HOPS.len());
    let selected_sources = json(SELECTED_SOURCES);
    for (index, hop) in hops.iter().enumerate() {
        let from = hop["from"].as_str().unwrap();
        let to = hop["to"].as_str().unwrap();
        let name = format!("dist-diff-{from}-to-{to}.json");
        let source = file(&name);
        assert_eq!(sha256(source.as_bytes()), hop["dist_diff_sha256"]);
        let delta = json(source);
        for key in ["added", "removed", "changed", "identical"] {
            assert_eq!(
                delta[key].as_array().unwrap().len(),
                hop[key].as_u64().unwrap() as usize
            );
        }

        let before = inventory(from);
        let after = inventory(to);
        let before = before.as_object().unwrap();
        let after = after.as_object().unwrap();
        let added = after
            .keys()
            .filter(|path| !before.contains_key(*path))
            .cloned()
            .collect();
        let removed = before
            .keys()
            .filter(|path| !after.contains_key(*path))
            .cloned()
            .collect();
        let mut changed = Vec::new();
        let mut identical = Vec::new();
        for (path, before_digest) in before {
            match after.get(path) {
                Some(after_digest) if after_digest != before_digest => changed.push(path.clone()),
                Some(_) => identical.push(path.clone()),
                None => {}
            }
        }
        assert_path_set(&delta, "added", added);
        assert_path_set(&delta, "removed", removed);
        assert_path_set(&delta, "changed", changed);
        assert_path_set(&delta, "identical", identical);

        let changed_keys = hop["selected_source_changes"]
            .as_array()
            .unwrap()
            .iter()
            .map(|source| source["key"].as_str().unwrap())
            .collect::<Vec<_>>();
        assert_eq!(changed_keys, SELECTED_SOURCE_HOPS[index]);
        for change in hop["selected_source_changes"].as_array().unwrap() {
            let key = change["key"].as_str().unwrap();
            assert_eq!(change["path"], selected_sources[to][key]["path"]);
            assert_eq!(
                change["before_sha256"],
                selected_sources[from][key]["sha256"]
            );
            assert_eq!(change["after_sha256"], selected_sources[to][key]["sha256"]);
        }
    }
}

#[test]
fn mapped_source_snapshots_have_exact_keys_and_match_their_frozen_digests() {
    let identity = json(IDENTITY);
    let selected = json(SELECTED_SOURCES);
    assert_eq!(sha256(SELECTED_SOURCES.as_bytes()), SELECTED_SOURCES_SHA256);
    let actual_versions = selected
        .as_object()
        .unwrap()
        .keys()
        .map(String::as_str)
        .collect::<Vec<_>>();
    assert_eq!(actual_versions.as_slice(), VERSIONS);

    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    for version in VERSIONS {
        let sources = selected[version].as_object().unwrap();
        let keys = sources.keys().map(String::as_str).collect::<Vec<_>>();
        assert_eq!(
            keys.as_slice(),
            MAPPED_SOURCE_KEYS,
            "exact selected source keys for {version}"
        );
        for (key, source) in sources {
            let relative = Path::new(source["path"].as_str().unwrap());
            assert!(
                relative
                    .components()
                    .all(|component| matches!(component, Component::Normal(_)))
            );
            let path = root.join(fixture(&format!("source/{version}/{}", relative.display())));
            let bytes = std::fs::read(&path)
                .unwrap_or_else(|error| panic!("read {key} at {}: {error}", path.display()));
            assert_eq!(bytes.len(), source["bytes"].as_u64().unwrap() as usize);
            assert_eq!(
                sha256(&bytes),
                source["sha256"],
                "frozen source {key} at {version}"
            );
        }
    }

    let selected_digest = sha256(SELECTED_SOURCES.as_bytes());
    assert_eq!(
        identity["selected_source_identities_sha256"], selected_digest,
        "the exact source identity ledger is frozen"
    );
}

#[test]
fn ssh_selection_and_approval_helper_sources_are_frozen_with_exact_coverage() {
    let identity = json(IDENTITY);
    let reviewed = json(REVIEWED_SOURCES);
    assert_eq!(sha256(REVIEWED_SOURCES.as_bytes()), REVIEWED_SOURCES_SHA256);
    assert_eq!(
        identity["reviewed_source_identities_sha256"],
        sha256(REVIEWED_SOURCES.as_bytes())
    );
    let mut keys = reviewed
        .as_object()
        .unwrap()
        .keys()
        .map(String::as_str)
        .collect::<Vec<_>>();
    keys.sort_unstable();
    assert_eq!(
        keys.as_slice(),
        [
            "approval_mode_value",
            "ssh_workspace_store",
            "ssh_workspace_store_test"
        ]
    );

    let versions = ["0.25.0", "0.25.1", "0.25.2", "0.25.3", "0.26.0", "0.26.1"];
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    for key in [
        "approval_mode_value",
        "ssh_workspace_store",
        "ssh_workspace_store_test",
    ] {
        let records = reviewed[key].as_object().unwrap();
        let mut actual_versions = records.keys().map(String::as_str).collect::<Vec<_>>();
        actual_versions.sort_unstable();
        assert_eq!(
            actual_versions.as_slice(),
            versions,
            "exact source coverage for {key}"
        );
        for version in versions {
            let record = &records[version];
            let relative = Path::new(record["path"].as_str().unwrap());
            assert!(
                relative
                    .components()
                    .all(|component| matches!(component, Component::Normal(_)))
            );
            let path = root.join(fixture(&format!("source/{version}/{}", relative.display())));
            let bytes = std::fs::read(&path)
                .unwrap_or_else(|error| panic!("read {}: {error}", path.display()));
            assert_eq!(bytes.len(), record["bytes"].as_u64().unwrap() as usize);
            assert_eq!(
                sha256(&bytes),
                record["sha256"],
                "reviewed source {key} at {version}"
            );
        }
    }
}

fn version(value: &str) -> InterfaceVersion {
    InterfaceVersion::new(value).expect("valid Qwen Code package version")
}

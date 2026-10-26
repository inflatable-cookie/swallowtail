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
const SOURCE_TO_SHIPPED: &str = include_str!("fixtures/qwen-code-0.26.1/source-to-shipped.json");
const IDENTITY_SHA256: &str = "b7f5c72bdf37bfdac5cffd00a5563436fa5b14e90d490a05376e8ab39eeb6adf";
const SELECTED_SOURCES_SHA256: &str =
    "ccfffe2ade51be51099871e306bde25dff3e657593e11bb5a5760551035d18c5";
const REVIEWED_SOURCES_SHA256: &str =
    "f92718e090d69d89bd792c8042b4ff23c143c671ce90e8abc8b159758d3f038e";
const SOURCE_TO_SHIPPED_SHA256: &str =
    "22d49c0703d6f6b54a2b83c867ed6c817a88ebd63410b7db8bad00c73504d9c1";

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
        "source-to-shipped.json" => {
            include_str!("fixtures/qwen-code-0.26.1/source-to-shipped.json")
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
    assert_eq!(identity["source_to_shipped_file"], "source-to-shipped.json");
    assert_eq!(
        identity["source_to_shipped_sha256"],
        SOURCE_TO_SHIPPED_SHA256
    );
    assert_eq!(
        identity["evidence_limits"]["source_maps_in_npm_package"],
        false
    );
    assert_eq!(
        identity["evidence_limits"]["slsa_provenance_binds_tarball_sha512_to_git_commit"],
        true
    );
    assert_eq!(
        identity["evidence_limits"]["selected_sources_byte_identical_at_slsa_commit_and_tag"],
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
            "interactive_mode",
            "loop_detection_service",
            "loop_protection",
            "sdk_human_input",
            "ssh_workspace_store",
            "ssh_workspace_store_test"
        ]
    );

    let expected_versions = [
        (
            "approval_mode_value",
            vec!["0.25.0", "0.25.1", "0.25.2", "0.25.3", "0.26.0", "0.26.1"],
        ),
        ("interactive_mode", vec!["0.26.1"]),
        ("loop_detection_service", vec!["0.25.3", "0.26.0", "0.26.1"]),
        ("loop_protection", vec!["0.26.0", "0.26.1"]),
        ("sdk_human_input", vec!["0.26.0", "0.26.1"]),
        (
            "ssh_workspace_store",
            vec!["0.25.0", "0.25.1", "0.25.2", "0.25.3", "0.26.0", "0.26.1"],
        ),
        (
            "ssh_workspace_store_test",
            vec!["0.25.0", "0.25.1", "0.25.2", "0.25.3", "0.26.0", "0.26.1"],
        ),
    ];
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    for (key, versions) in expected_versions {
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

#[test]
fn slsa_provenance_binds_each_githead_null_hop_tarball_to_selected_tag_sources() {
    let identity = json(IDENTITY);
    let selected = json(SELECTED_SOURCES);
    let reviewed = json(REVIEWED_SOURCES);
    let shipped = json(SOURCE_TO_SHIPPED);
    assert_eq!(
        sha256(SOURCE_TO_SHIPPED.as_bytes()),
        SOURCE_TO_SHIPPED_SHA256
    );
    assert_eq!(
        sha256(SOURCE_TO_SHIPPED.as_bytes()),
        identity["source_to_shipped_sha256"]
    );
    assert_eq!(
        shipped["method"],
        "npm-slsa-provenance-v1-tarball-subject-plus-selected-source-identity"
    );
    assert_eq!(
        shipped["historical_0_25_0"],
        "Research 375 remains the 0.25.0 identity record and is not rewritten"
    );

    let versions = shipped["versions"]
        .as_array()
        .expect("source-to-shipped versions");
    assert_eq!(
        versions
            .iter()
            .map(|record| record["npm_version"].as_str().unwrap())
            .collect::<Vec<_>>(),
        ["0.25.1", "0.25.2", "0.25.3", "0.26.0", "0.26.1"]
    );

    let identity_versions = identity["versions"].as_array().unwrap();
    for record in versions {
        let version = record["npm_version"].as_str().unwrap();
        let identity_record = identity_versions
            .iter()
            .find(|item| item["npm_version"] == version)
            .unwrap();
        assert_eq!(record["artifact_sha256"], identity_record["tarball_sha256"]);
        assert_eq!(record["artifact_sha512"], identity_record["tarball_sha512"]);
        assert_eq!(record["tag_commit"], identity_record["source_commit"]);
        assert_eq!(
            record["slsa_git_commit"],
            identity_record["slsa_git_commit"]
        );
        assert_eq!(record["npm_git_head"], identity_record["npm_git_head"]);
        assert_eq!(record["tag_commit_equals_slsa_git_commit"], false);
        assert_eq!(record["source_maps_in_package"], 0);
        assert_eq!(record["source_map_paths"], serde_json::json!([]));
        assert_eq!(record["slsa"]["subject_sha512"], record["artifact_sha512"]);
        assert_eq!(record["slsa"]["subject_sha512_equals_tarball_sha512"], true);
        assert_eq!(
            record["slsa"]["predicate_type"],
            "https://slsa.dev/provenance/v1"
        );
        assert_eq!(
            record["slsa"]["repository"],
            "https://github.com/QwenLM/qwen-code"
        );
        assert_eq!(record["slsa"]["workflow"], ".github/workflows/release.yml");
        assert_eq!(record["slsa"]["workflow_ref"], "refs/heads/main");
        assert_eq!(record["slsa"]["event_name"], "workflow_dispatch");
        assert_eq!(record["slsa"]["payload_sha256"].as_str().unwrap().len(), 64);
        assert_eq!(
            record["slsa"]["subject_sha512"].as_str().unwrap().len(),
            128
        );
        assert_eq!(record["slsa_git_commit"].as_str().unwrap().len(), 40);
        assert_ne!(record["slsa_git_commit"], record["tag_commit"]);

        if matches!(version, "0.25.1" | "0.25.2" | "0.25.3") {
            assert!(record["npm_git_head"].is_null());
        } else {
            assert_eq!(record["npm_git_head"], record["tag_commit"]);
        }

        let inventory = inventory(version);
        for path in inventory.as_object().unwrap().keys() {
            assert!(
                !path.ends_with(".map"),
                "{version} shipped tree includes source map {path}"
            );
        }

        let changed = record["tag_to_slsa_changed_paths"]
            .as_array()
            .unwrap()
            .iter()
            .map(|path| path.as_str().unwrap())
            .collect::<Vec<_>>();
        assert!(!changed.is_empty());
        for path in &changed {
            assert!(
                path.ends_with("package.json") || path.ends_with("qwen-extension.json"),
                "tag-to-slsa delta stays workspace version metadata: {path}"
            );
            assert!(
                !path.contains("/src/"),
                "tag-to-slsa delta must not touch selected source {path}"
            );
        }
        assert_eq!(
            record["tag_to_slsa_selected_source_changes"],
            serde_json::json!([])
        );

        let selected_files = record["selected_files"].as_array().unwrap();
        assert_eq!(selected_files.len(), MAPPED_SOURCE_KEYS.len());
        for source in selected_files {
            let key = source["key"].as_str().unwrap();
            assert_eq!(source["source_path"], selected[version][key]["path"]);
            assert_eq!(source["source_sha256"], selected[version][key]["sha256"]);
            assert_eq!(source["source_bytes"], selected[version][key]["bytes"]);
            assert_eq!(source["tag_commit"], record["tag_commit"]);
            assert_eq!(source["slsa_commit"], record["slsa_git_commit"]);
            assert_eq!(source["tag_and_slsa_byte_identical"], true);
        }

        for reviewed_file in record["reviewed_files"].as_array().unwrap() {
            let key = reviewed_file["key"].as_str().unwrap();
            assert_eq!(
                reviewed_file["source_sha256"],
                reviewed[key][version]["sha256"]
            );
            assert_eq!(reviewed_file["tag_and_slsa_byte_identical"], true);
        }
    }

    let hops = shipped["hops"].as_array().unwrap();
    assert_eq!(hops.len(), SELECTED_SOURCE_HOPS.len());
    for (index, hop) in hops.iter().enumerate() {
        let to = hop["to"].as_str().unwrap();
        let expected_keys = SELECTED_SOURCE_HOPS[index];
        let boundaries = hop["affected_selected_boundaries"].as_array().unwrap();
        let actual_keys = boundaries
            .iter()
            .map(|boundary| boundary["key"].as_str().unwrap())
            .collect::<Vec<_>>();
        assert_eq!(actual_keys, expected_keys);
        let version = versions
            .iter()
            .find(|record| record["npm_version"] == to)
            .unwrap();
        for boundary in boundaries {
            let key = boundary["key"].as_str().unwrap();
            assert_eq!(boundary["artifact_sha256"], version["artifact_sha256"]);
            assert_eq!(boundary["artifact_sha512"], version["artifact_sha512"]);
            assert_eq!(boundary["source_commit"], version["slsa_git_commit"]);
            assert_eq!(boundary["source_path"], selected[to][key]["path"]);
            assert_eq!(boundary["source_sha256"], selected[to][key]["sha256"]);
            assert_eq!(
                boundary["method"],
                "npm-slsa-provenance-v1-tarball-subject-plus-selected-source-identity"
            );
        }
    }
}

fn version(value: &str) -> InterfaceVersion {
    InterfaceVersion::new(value).expect("valid Qwen Code package version")
}

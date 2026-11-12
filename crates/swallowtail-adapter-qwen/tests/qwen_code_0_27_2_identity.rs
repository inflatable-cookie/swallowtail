use serde_json::Value;
use sha2::{Digest, Sha256};
use std::path::{Component, Path};
use swallowtail_adapter_qwen::{QWEN_CODE_LATEST_QUALIFIED_VERSION, qwen_headless_claim};
use swallowtail_core::{InterfaceCompatibilityAssessment, InterfaceVersion};

const ROOT: &str = "tests/fixtures/qwen-code-0.27.2";
const IDENTITY: &str = include_str!("fixtures/qwen-code-0.27.2/identity.json");
const SELECTED_SOURCES: &str =
    include_str!("fixtures/qwen-code-0.27.2/selected-source-identities.json");
const REVIEWED_SOURCES: &str =
    include_str!("fixtures/qwen-code-0.27.2/reviewed-source-identities.json");
const SOURCE_TO_SHIPPED: &str = include_str!("fixtures/qwen-code-0.27.2/source-to-shipped.json");
const FROZEN_0_26_6_IDENTITY: &str = include_str!("fixtures/qwen-code-0.26.6/identity.json");
const IDENTITY_SHA256: &str = "6eea24e80940d32442c9018e5d3a8e205fef457addc1504f7697159e2af52dc4";
const SELECTED_SOURCES_SHA256: &str =
    "d3ec452b63d832d77050281c561bb663cd597dcb610a2c535f78cae5f8f7e348";
const REVIEWED_SOURCES_SHA256: &str =
    "3a100eb96decedb3bb50088c0859f167e9a2df14d742d5e6b65f42fa92bb642e";
const SOURCE_TO_SHIPPED_SHA256: &str =
    "9e48ccfccf5d1e90eade3ff275fcd0e526ba98a619d637fb6d3e93df251dd52d";

const VERSIONS: [&str; 7] = [
    "0.26.6", "0.26.7", "0.26.8", "0.26.9", "0.27.0", "0.27.1", "0.27.2",
];
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
const SELECTED_SOURCE_HOPS: [&[&str]; 6] = [
    &["config"],
    &["config", "top_level_options"],
    &[],
    &["config"],
    &["config", "session"],
    &["dashscope"],
];
const REVIEWED_KEYS: [&str; 13] = [
    "approval_mode_value",
    "autofix_pr_tool",
    "dashscope_source_config",
    "interactive_mode",
    "loop_detection_service",
    "loop_protection",
    "mod_session",
    "sdk_human_input",
    "ssh_workspace_store",
    "ssh_workspace_store_test",
    "unified_loop_core_config",
    "unified_loop_non_interactive_cli",
    "unified_loop_settings_schema",
];

fn json(source: &str) -> Value {
    serde_json::from_str(source).expect("Qwen identity evidence is valid JSON")
}

fn fixture(relative: &str) -> String {
    format!("{ROOT}/{relative}")
}

fn file(relative: &str) -> &'static str {
    match relative {
        "dist-inventory-0.26.6.json" => {
            include_str!("fixtures/qwen-code-0.27.2/dist-inventory-0.26.6.json")
        }
        "dist-inventory-0.26.7.json" => {
            include_str!("fixtures/qwen-code-0.27.2/dist-inventory-0.26.7.json")
        }
        "dist-inventory-0.26.8.json" => {
            include_str!("fixtures/qwen-code-0.27.2/dist-inventory-0.26.8.json")
        }
        "dist-inventory-0.26.9.json" => {
            include_str!("fixtures/qwen-code-0.27.2/dist-inventory-0.26.9.json")
        }
        "dist-inventory-0.27.0.json" => {
            include_str!("fixtures/qwen-code-0.27.2/dist-inventory-0.27.0.json")
        }
        "dist-inventory-0.27.1.json" => {
            include_str!("fixtures/qwen-code-0.27.2/dist-inventory-0.27.1.json")
        }
        "dist-inventory-0.27.2.json" => {
            include_str!("fixtures/qwen-code-0.27.2/dist-inventory-0.27.2.json")
        }
        "dist-diff-0.26.6-to-0.26.7.json" => {
            include_str!("fixtures/qwen-code-0.27.2/dist-diff-0.26.6-to-0.26.7.json")
        }
        "dist-diff-0.26.7-to-0.26.8.json" => {
            include_str!("fixtures/qwen-code-0.27.2/dist-diff-0.26.7-to-0.26.8.json")
        }
        "dist-diff-0.26.8-to-0.26.9.json" => {
            include_str!("fixtures/qwen-code-0.27.2/dist-diff-0.26.8-to-0.26.9.json")
        }
        "dist-diff-0.26.9-to-0.27.0.json" => {
            include_str!("fixtures/qwen-code-0.27.2/dist-diff-0.26.9-to-0.27.0.json")
        }
        "dist-diff-0.27.0-to-0.27.1.json" => {
            include_str!("fixtures/qwen-code-0.27.2/dist-diff-0.27.0-to-0.27.1.json")
        }
        "dist-diff-0.27.1-to-0.27.2.json" => {
            include_str!("fixtures/qwen-code-0.27.2/dist-diff-0.27.1-to-0.27.2.json")
        }
        "source-to-shipped.json" => {
            include_str!("fixtures/qwen-code-0.27.2/source-to-shipped.json")
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

fn snapshot(version: &str, relative: &str) -> String {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = root.join(fixture(&format!("source/{version}/{relative}")));
    std::fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("read {}: {error}", path.display()))
}

#[test]
fn official_identity_freezes_npm_latest_and_preserves_the_claim_shape() {
    let identity = json(IDENTITY);
    assert_eq!(sha256(IDENTITY.as_bytes()), IDENTITY_SHA256);
    assert_eq!(identity["axis"], "qwen-code.package");
    assert_eq!(identity["observed_at"], "2026-11-12");
    assert_eq!(identity["npm_package"], "@qwen-code/qwen-code");
    assert_eq!(identity["npm_latest"], "0.27.2");
    assert_eq!(identity["previous_ceiling"], "0.26.6");
    assert_eq!(identity["first_unpublished_later_stable"], Value::Null);
    assert_eq!(
        identity["npm_other_dist_tags"]["preview"],
        "0.27.2-preview.0"
    );
    assert_eq!(
        identity["npm_other_dist_tags"]["nightly"],
        "0.26.9-nightly.20261107.65e1f1aeaa"
    );
    assert_eq!(
        identity["published_stable_hops_after_previous_ceiling"],
        serde_json::json!(["0.26.7", "0.26.8", "0.26.9", "0.27.0", "0.27.1", "0.27.2"])
    );
    assert_eq!(identity["acp_registry_discovery"]["version"], "0.27.2");
    assert_eq!(
        identity["identity_decision"]["shape"],
        "compatible-extension"
    );
    assert_eq!(
        identity["identity_decision"]["raise_latest_qualified_to"],
        "0.27.2"
    );
    assert_eq!(
        identity["identity_decision"]["extend_same_revision_segment"],
        "0.22.0..=0.27.2"
    );
    assert_eq!(
        identity["identity_decision"]["reuse_behavior_revision"],
        "qwen-code.headless.v0.21.15-reasoning-control"
    );
    assert_eq!(
        identity["identity_decision"]["keep_0_22_4_incompatible"],
        true
    );
    assert_eq!(
        identity["identity_decision"]["keep_0_23_5_incompatible"],
        true
    );
    assert_eq!(
        identity["identity_decision"]["keep_0_25_4_incompatible"],
        true
    );
    assert_eq!(identity["identity_decision"]["keep_exact_0_21_15"], true);
    assert_eq!(
        identity["identity_decision"]["extend_plan_exact_list"],
        false
    );
    assert_eq!(
        identity["identity_decision"]["extend_reasoning_beyond_0_21_15"],
        false
    );
    assert_eq!(
        identity["identity_decision"]["extend_budgets_beyond_0_21_15"],
        false
    );
    assert_eq!(
        identity["identity_decision"]["new_public_mapped_operation"],
        false
    );
    assert_eq!(
        identity["identity_decision"]["unified_agent_loop_default"],
        false
    );
    assert_eq!(
        identity["identity_decision"]["unified_agent_loop_opt_in_bounds_proven"],
        true
    );
    assert_eq!(
        identity["identity_decision"]["dashscope_source_config_identity_frozen"],
        true
    );
    assert_eq!(
        identity["identity_decision"]["autofix_pr_exclusion_set_kept"],
        true
    );
    assert_eq!(
        identity["identity_decision"]["autofix_pr_in_place_always_attended"],
        true
    );
    assert_eq!(
        identity["identity_decision"]["safe_mode_bounds_hooks_extensions_mods_and_lsp"],
        true
    );
    assert_eq!(
        identity["identity_decision"]["mods_opt_in_and_safe_mode_gated"],
        true
    );
    assert_eq!(
        identity["identity_decision"]["project_hooks_approval_gate_inert_under_safe_mode"],
        true
    );
    assert_eq!(
        identity["identity_decision"]["file_changed_hooks_inert_under_safe_mode"],
        true
    );
    assert_eq!(
        identity["identity_decision"]["post_result_memory_drain_bounded_by_adapter_deadline"],
        true
    );
    assert_eq!(
        identity["identity_decision"]["node_engine_range_unchanged"],
        true
    );
    assert_eq!(
        identity["identity_decision"]["tag_to_build_checkout_divergence_classified"],
        true
    );
    assert_eq!(
        identity["claim_at_observation"]["latest_qualified"],
        "0.26.6"
    );
    assert_eq!(
        identity["claim_at_observation"]["maintained_segment"],
        "0.22.0..=0.26.6"
    );
    assert_eq!(identity["channel_reprobe"]["stable_current"], true);
    assert_eq!(
        identity["runtime_requirement"]["node_engine_range"],
        ">=22.0.0"
    );
    assert_eq!(
        identity["runtime_requirement"]["unchanged_across_hops"],
        true
    );
    assert_eq!(
        identity["runtime_context"]["host_installed_qwen_executed"],
        false
    );
    assert_eq!(
        identity["evidence_limits"]["selected_sources_frozen_from_attested_build_checkout"],
        true
    );
    assert_eq!(
        identity["evidence_limits"]["tag_and_build_checkout_selected_sources_compared"],
        true
    );
    assert_eq!(identity["evidence_limits"]["tarballs_executed"], false);
    assert_eq!(identity["evidence_limits"]["provider_turns"], false);
    assert_eq!(identity["evidence_limits"]["vendor_execution"], false);

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
        assert_eq!(record["tarball_sha512"].as_str().unwrap().len(), 128);
        assert_eq!(record["node_engine_range"], ">=22.0.0");
    }

    // The 0.26.6 artifact identity continues Research 475's frozen record
    // without rewriting it.
    let frozen = json(FROZEN_0_26_6_IDENTITY);
    let frozen_record = frozen["versions"]
        .as_array()
        .unwrap()
        .iter()
        .find(|record| record["npm_version"] == "0.26.6")
        .unwrap();
    let record = versions
        .iter()
        .find(|record| record["npm_version"] == "0.26.6")
        .unwrap();
    assert_eq!(record["tarball_sha256"], frozen_record["tarball_sha256"]);
    assert_eq!(record["tarball_sha512"], frozen_record["tarball_sha512"]);
    assert_eq!(record["source_commit"], frozen_record["source_commit"]);
    assert_eq!(record["slsa_git_commit"], frozen_record["slsa_git_commit"]);
    assert_eq!(record["shasum"], frozen_record["shasum"]);

    assert_eq!(QWEN_CODE_LATEST_QUALIFIED_VERSION, "0.27.2");
    let claim = qwen_headless_claim();
    for stable in [
        "0.26.0", "0.26.1", "0.26.2", "0.26.3", "0.26.4", "0.26.5", "0.26.6", "0.26.7", "0.26.8",
        "0.26.9", "0.27.0", "0.27.1", "0.27.2",
    ] {
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
    for prerelease in [
        "0.26.7-nightly.20261104.b9e20830f7",
        "0.26.8-nightly.20261105.7bd78683e6",
        "0.26.9-nightly.20261107.65e1f1aeaa",
        "0.27.2-preview.0",
    ] {
        assert_eq!(
            claim.assess(&version(prerelease)),
            InterfaceCompatibilityAssessment::Incompatible,
            "prerelease {prerelease} is not a stable point"
        );
    }
    assert!(matches!(
        claim.assess(&version("0.27.3")),
        InterfaceCompatibilityAssessment::UnverifiedNewer(_)
    ));
}

#[test]
fn complete_package_inventories_and_the_published_hops_are_mutation_checked() {
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
            assert!(
                !path.ends_with(".map"),
                "{version} shipped tree includes source map {path}"
            );
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

    assert_eq!(
        identity["selected_source_identities_sha256"],
        sha256(SELECTED_SOURCES.as_bytes()),
        "the exact source identity ledger is frozen"
    );
}

#[test]
fn reviewed_boundary_sources_are_frozen_with_exact_coverage() {
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
    assert_eq!(keys.as_slice(), REVIEWED_KEYS);

    // Files whose stable identity bounds the read-only, approval, and
    // SSH-workspace surfaces stay byte-identical across the whole window.
    for key in ["ssh_workspace_store", "ssh_workspace_store_test"] {
        let mut hashes = reviewed[key]
            .as_object()
            .unwrap()
            .values()
            .map(|record| record["sha256"].as_str().unwrap())
            .collect::<Vec<_>>();
        hashes.sort_unstable();
        hashes.dedup();
        assert_eq!(
            hashes.len(),
            1,
            "{key} is byte-identical at 0.26.6 through 0.27.2"
        );
    }
    let mut feeding_hashes = reviewed["dashscope_source_config"]
        .as_object()
        .unwrap()
        .values()
        .map(|record| record["sha256"].as_str().unwrap())
        .collect::<Vec<_>>();
    feeding_hashes.sort_unstable();
    feeding_hashes.dedup();
    assert_eq!(
        feeding_hashes.len(),
        1,
        "dashscope-source-config.ts is byte-identical at 0.26.6 through 0.27.2"
    );

    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    for key in REVIEWED_KEYS {
        let records = reviewed[key].as_object().unwrap();
        let mut actual_versions = records.keys().map(String::as_str).collect::<Vec<_>>();
        actual_versions.sort_unstable();
        assert_eq!(
            actual_versions.as_slice(),
            VERSIONS,
            "exact source coverage for {key}"
        );
        for version in VERSIONS {
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
fn slsa_provenance_binds_every_tarball_to_the_attested_build_checkout() {
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
    assert!(
        shipped["build_checkout_authority"]
            .as_str()
            .unwrap()
            .contains("0.26.9")
    );

    let versions = shipped["versions"]
        .as_array()
        .expect("source-to-shipped versions");
    assert_eq!(
        versions
            .iter()
            .map(|record| record["npm_version"].as_str().unwrap())
            .collect::<Vec<_>>()
            .as_slice(),
        VERSIONS
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
        assert_eq!(record["npm_git_head_equals_tag_commit"], true);
        assert_eq!(record["tag_commit_equals_slsa_git_commit"], false);
        assert_ne!(record["slsa_git_commit"], record["tag_commit"]);
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
        assert_eq!(
            record["slsa"]["builder_id"],
            "https://github.com/actions/runner/github-hosted"
        );
        assert_eq!(record["slsa"]["payload_sha256"].as_str().unwrap().len(), 64);
        assert_eq!(record["slsa"]["git_commit"], record["slsa_git_commit"]);

        // Tag-to-build-checkout parity holds everywhere except the 0.26.9
        // build, where the release workflow checked out a later `main`.
        let changed = record["tag_to_slsa_changed_paths"]
            .as_array()
            .unwrap()
            .iter()
            .map(|path| path.as_str().unwrap())
            .collect::<Vec<_>>();
        assert!(!changed.is_empty());
        if version == "0.26.9" {
            assert_eq!(changed.len(), 111);
            assert_eq!(
                record["tag_to_slsa_build_only_paths"]
                    .as_array()
                    .unwrap()
                    .len(),
                11
            );
            assert_eq!(
                record["tag_to_slsa_reviewed_source_changes"],
                serde_json::json!(["unified_loop_core_config", "unified_loop_settings_schema"])
            );
            assert_eq!(
                record["tag_to_slsa_selected_source_changes"],
                serde_json::json!([])
            );
        } else {
            assert_eq!(changed.len(), 26);
            assert_eq!(
                record["tag_to_slsa_build_only_paths"],
                serde_json::json!([])
            );
            assert_eq!(
                record["tag_to_slsa_selected_source_changes"],
                serde_json::json!([])
            );
            assert_eq!(
                record["tag_to_slsa_reviewed_source_changes"],
                serde_json::json!([])
            );
        }
        for path in &changed {
            if version == "0.26.9" {
                continue;
            }
            assert!(
                path.ends_with("package.json") || path.ends_with("qwen-extension.json"),
                "tag-to-build delta stays workspace version metadata: {path}"
            );
            assert!(
                !path.contains("/src/"),
                "tag-to-build delta must not touch selected source {path}"
            );
        }

        let selected_files = record["selected_files"].as_array().unwrap();
        assert_eq!(selected_files.len(), MAPPED_SOURCE_KEYS.len());
        for source in selected_files {
            let key = source["key"].as_str().unwrap();
            assert_eq!(source["source_path"], selected[version][key]["path"]);
            assert_eq!(source["source_sha256"], selected[version][key]["sha256"]);
            assert_eq!(source["source_bytes"], selected[version][key]["bytes"]);
            assert_eq!(source["slsa_commit"], record["slsa_git_commit"]);
            assert_eq!(source["tag_and_slsa_byte_identical"], true);
        }

        for reviewed_file in record["reviewed_files"].as_array().unwrap() {
            let key = reviewed_file["key"].as_str().unwrap();
            assert_eq!(
                reviewed_file["source_sha256"],
                reviewed[key][version]["sha256"]
            );
            let divergent = version == "0.26.9"
                && (key == "unified_loop_core_config" || key == "unified_loop_settings_schema");
            assert_eq!(
                reviewed_file["tag_and_slsa_byte_identical"], !divergent,
                "{key} tag/build parity at {version}"
            );
        }
    }

    // The frozen tag copy of the two divergent 0.26.9 files proves the
    // recorded divergence instead of asserting it.
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let record = versions
        .iter()
        .find(|record| record["npm_version"] == "0.26.9")
        .unwrap();
    let mut divergent_files = record["reviewed_files"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|file| {
            file["key"] == "unified_loop_core_config"
                || file["key"] == "unified_loop_settings_schema"
        })
        .collect::<Vec<_>>();
    divergent_files.sort_by_key(|file| file["key"].as_str().unwrap().to_owned());
    assert_eq!(divergent_files.len(), 2);
    for divergent in divergent_files {
        let relative = divergent["source_path"].as_str().unwrap();
        let path = root.join(fixture(&format!("source/0.26.9-tag/{relative}")));
        let bytes =
            std::fs::read(&path).unwrap_or_else(|error| panic!("read {}: {error}", path.display()));
        assert_eq!(sha256(&bytes), divergent["tag_sha256"]);
        assert_eq!(divergent["slsa_sha256"], divergent["source_sha256"]);
        assert_ne!(divergent["tag_sha256"], divergent["slsa_sha256"]);
    }

    let hops = shipped["hops"].as_array().unwrap();
    assert_eq!(hops.len(), SELECTED_SOURCE_HOPS.len());
    for (index, hop) in hops.iter().enumerate() {
        let to = hop["to"].as_str().unwrap();
        let boundaries = hop["affected_selected_boundaries"].as_array().unwrap();
        let actual_keys = boundaries
            .iter()
            .map(|boundary| boundary["key"].as_str().unwrap())
            .collect::<Vec<_>>();
        assert_eq!(actual_keys, SELECTED_SOURCE_HOPS[index]);
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

#[test]
fn selected_loop_and_safety_bounds_are_frozen_and_mutation_checked() {
    // Research 475's selected-loop and autofix bounds must survive every new
    // hop; the hook, mod, and post-result-memory changes are bounded by the
    // safe-mode and opt-in gates the tests below mutation-check.
    let reviewed = json(REVIEWED_SOURCES);
    let core_config_snippets = [
        "QWEN_CODE_ENABLE_UNIFIED_AGENT_LOOP'] === '1'",
        "isUnifiedAgentLoopEnabled(): boolean {",
        "private readonly unifiedAgentLoopEnabled: boolean = false;",
        // All hook scopes stay off under the selected `--safe-mode`.
        "getUserHooks(): { [K in HookEventName]?: HookDefinition[] } | undefined {",
        "getSystemHooks(): { [K in HookEventName]?: HookDefinition[] } | undefined {",
        "if (this.getBareMode() || this.isSafeMode()) {",
    ];
    let non_interactive_cli_snippets = [
        "const sharedAgentLoop = config.isUnifiedAgentLoopEnabled?.() === true;",
        "scope.onAbort?.();",
        "beforeToolLaunch: (request, settled) => {",
        "--max-tool-calls=N stops at",
        "case 'max_session_turns':",
        "case 'cancelled':",
        "options.controlService.permission.getToolCallUpdateCallback()",
        "InputFormat.STREAM_JSON && options.controlService",
        "adapter.processEvent(event);",
    ];
    let settings_schema_snippets = [
        "unifiedAgentLoop: {",
        "QWEN_CODE_ENABLE_UNIFIED_AGENT_LOOP=1 environment variable.",
    ];
    let autofix_command_snippets = ["  'status',", "  'attended',", "  'unattended',"];
    let autofix_in_place_snippets = ["watch.inPlace === true", "always attended"];
    let mod_session_snippets = [
        "settings.merged.experimental?.mods",
        "!this.config.isSafeMode()",
    ];
    for version in VERSIONS {
        let core_config = snapshot(
            version,
            reviewed["unified_loop_core_config"][version]["path"]
                .as_str()
                .unwrap(),
        );
        for snippet in core_config_snippets {
            assert!(
                core_config.contains(snippet),
                "core config.ts at {version} lost bound snippet {snippet}"
            );
        }
        // Every hook accessor keeps the safe-mode gate.
        assert!(
            core_config
                .matches("if (this.getBareMode() || this.isSafeMode()) {")
                .count()
                >= 3,
            "core config.ts at {version} lost a hook safe-mode gate"
        );
        let non_interactive_cli = snapshot(
            version,
            reviewed["unified_loop_non_interactive_cli"][version]["path"]
                .as_str()
                .unwrap(),
        );
        for snippet in non_interactive_cli_snippets {
            assert!(
                non_interactive_cli.contains(snippet),
                "nonInteractiveCli.ts at {version} lost bound snippet {snippet}"
            );
        }
        let settings_schema = snapshot(
            version,
            reviewed["unified_loop_settings_schema"][version]["path"]
                .as_str()
                .unwrap(),
        );
        for snippet in settings_schema_snippets {
            assert!(
                settings_schema.contains(snippet),
                "settingsSchema.ts at {version} lost settings opt-in snippet {snippet}"
            );
        }
        let autofix = snapshot(
            version,
            reviewed["autofix_pr_tool"][version]["path"]
                .as_str()
                .unwrap(),
        );
        for snippet in autofix_command_snippets {
            assert!(
                autofix.contains(snippet),
                "autofix-pr.ts at {version} lost command-bound snippet {snippet}"
            );
        }
        let mod_session = snapshot(
            version,
            reviewed["mod_session"][version]["path"].as_str().unwrap(),
        );
        for snippet in mod_session_snippets {
            assert!(
                mod_session.contains(snippet),
                "mod-session.ts at {version} lost gate snippet {snippet}"
            );
        }
        // The in_place prepare variant is always attended; it arrived with
        // `0.26.5`, so every version in this window carries the refusal.
        for snippet in autofix_in_place_snippets {
            assert!(
                autofix.contains(snippet),
                "autofix-pr.ts at {version} lost in-place attended snippet {snippet}"
            );
        }

        // Project-hooks approval gating and the FileChanged hook event only
        // exist from 0.26.8 / 0.26.9; both stay behind the same safe-mode
        // accessor gate frozen above. Mods queued prompts and their
        // `mod_prompt` system message also arrive at 0.26.8, but the Mods
        // subsystem they belong to requires the `experimental.mods` setting
        // and refuses safe mode in `mod-session.ts`, so the path is
        // unreachable for the selected route.
        let mods_prompt_path = non_interactive_cli.contains("const pullModPrompts = () => {")
            && non_interactive_cli.contains("adapter.emitSystemMessage('mod_prompt'");
        assert_eq!(
            mods_prompt_path,
            !matches!(version, "0.26.6" | "0.26.7"),
            "Mods queued-prompt path presence at {version}"
        );
        let project_hooks_approved = core_config.contains("private readProjectHooks():");
        assert_eq!(
            project_hooks_approved,
            !matches!(version, "0.26.6" | "0.26.7"),
            "hook approval gate presence at {version}"
        );
        let file_changed = core_config.contains("case 'FileChanged': {");
        assert_eq!(
            file_changed,
            !matches!(version, "0.26.6" | "0.26.7" | "0.26.8"),
            "FileChanged hook event presence at {version}"
        );

        let session = snapshot(version, "packages/cli/src/nonInteractive/session.ts");
        let memory_drain = session.contains("EOF_MEMORY_WAIT_TIMEOUT_MS = 5_000");
        assert_eq!(
            memory_drain,
            matches!(version, "0.27.1" | "0.27.2"),
            "post-result memory drain presence at {version}"
        );
    }
}

fn version(value: &str) -> InterfaceVersion {
    InterfaceVersion::new(value).expect("valid Qwen Code package version")
}

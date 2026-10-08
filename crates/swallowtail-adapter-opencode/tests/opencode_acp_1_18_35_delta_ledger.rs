use std::collections::{BTreeMap, BTreeSet};

use serde_json::{Map, Value, json};
use sha2::{Digest, Sha256};
use swallowtail_adapter_opencode::{
    OPENCODE_ACP_LATEST_QUALIFIED_VERSION, opencode_acp_binding, opencode_acp_claim,
};
use swallowtail_core::InterfaceCompatibilityAssessment;

const IDENTITY: &str = include_str!("fixtures/opencode-acp-1.18.35/identity.json");
const INVENTORY: &str = include_str!("fixtures/opencode-acp-1.18.35/dist-inventory.json");
const PROTOCOL: &str = include_str!("fixtures/opencode-acp-1.18.35/protocol.json");
const SOURCE_TREE: &str = include_str!("fixtures/opencode-acp-1.18.35/source-tree.tsv");

const SELECTED_SOURCE_FILES: &[&str] = &[
    "packages/core/src/config/mcp.ts",
    "packages/core/src/v1/config/mcp.ts",
    "packages/opencode/package.json",
    "packages/opencode/src/acp/agent.ts",
    "packages/opencode/src/acp/config-option.ts",
    "packages/opencode/src/acp/content.ts",
    "packages/opencode/src/acp/directory.ts",
    "packages/opencode/src/acp/error.ts",
    "packages/opencode/src/acp/event.ts",
    "packages/opencode/src/acp/permission.ts",
    "packages/opencode/src/acp/profile.ts",
    "packages/opencode/src/acp/service.ts",
    "packages/opencode/src/acp/session.ts",
    "packages/opencode/src/acp/tool.ts",
    "packages/opencode/src/acp/usage.ts",
    "packages/opencode/src/cli/cmd/acp.ts",
    "packages/opencode/src/mcp/index.ts",
    "packages/opencode/src/server/routes/instance/httpapi/groups/mcp.ts",
    "packages/opencode/src/server/routes/instance/httpapi/handlers/mcp.ts",
];

const ROUTE_BEHAVIOR_FILES: &[&str] = &[
    "packages/core/src/config/mcp.ts",
    "packages/core/src/v1/config/mcp.ts",
    "packages/opencode/src/acp/agent.ts",
    "packages/opencode/src/acp/config-option.ts",
    "packages/opencode/src/acp/content.ts",
    "packages/opencode/src/acp/directory.ts",
    "packages/opencode/src/acp/error.ts",
    "packages/opencode/src/acp/event.ts",
    "packages/opencode/src/acp/permission.ts",
    "packages/opencode/src/acp/profile.ts",
    "packages/opencode/src/acp/service.ts",
    "packages/opencode/src/acp/session.ts",
    "packages/opencode/src/acp/tool.ts",
    "packages/opencode/src/acp/usage.ts",
    "packages/opencode/src/cli/cmd/acp.ts",
    "packages/opencode/src/mcp/index.ts",
    "packages/opencode/src/server/routes/instance/httpapi/groups/mcp.ts",
    "packages/opencode/src/server/routes/instance/httpapi/handlers/mcp.ts",
];

#[test]
fn exact_artifact_and_semantic_fixtures_remain_frozen() {
    assert_eq!(
        sha256(IDENTITY),
        "12f364b66b6577adccc79180e289a439cd9c3c99b80ce67242c0b8e220745f55"
    );
    assert_eq!(
        sha256(INVENTORY),
        "d960a3ca890f21335ee9d66eef6d21c8a171235b2b9b471d8ae53cd721f05357"
    );
    assert_eq!(
        sha256(PROTOCOL),
        "ff0e289a5c8a88647fa526e9829b87fe4e65d6f25d911caec33c745cb9f3e75a"
    );
    assert_eq!(
        sha256(SOURCE_TREE),
        "6ccadbecb9d91183cb79e236b046cf89ffe031602e96a1451c3c88e64102faff"
    );
}

#[test]
fn current_point_extends_only_the_existing_acp_v2_segment() {
    let identity = json(IDENTITY);
    let window = &identity["qualification_window"];
    let claim_fixture = &identity["claim"];
    let claim = opencode_acp_claim();

    assert_eq!(
        OPENCODE_ACP_LATEST_QUALIFIED_VERSION,
        window["new_ceiling"].as_str().unwrap()
    );
    assert_eq!(window["previous_ceiling"], "1.18.32");
    assert_eq!(window["baseline"], "1.18.18");
    assert_eq!(window["v1_segment"], "1.18.18..=1.18.30");
    assert_eq!(window["v2_segment"], "1.18.31..=1.18.35");
    assert_eq!(claim.id().as_str(), claim_fixture["id"]);
    assert_eq!(claim.axis().as_str(), claim_fixture["axis"]);
    assert_eq!(claim.baseline().as_str(), "1.18.18");
    assert_eq!(
        claim.latest_qualified().as_str(),
        OPENCODE_ACP_LATEST_QUALIFIED_VERSION
    );
    assert_exact_strings(&claim_fixture["excluded_versions"], &[]);
    assert_eq!(claim_fixture["newer_version_posture"], "allow_unverified");
    assert_eq!(
        claim_fixture["http_mcp_live_honouring_exact_version"],
        "1.18.18"
    );

    assert_exact_strings(
        &window["published_stable_hops"],
        &["1.18.33", "1.18.34", "1.18.35"],
    );
    for version in ["1.18.32", "1.18.33", "1.18.34", "1.18.35"] {
        let binding = opencode_acp_binding(version).expect("published exact stable version");
        assert!(matches!(
            claim.assess(binding.version()),
            InterfaceCompatibilityAssessment::Qualified(_)
        ));
    }
    let next = opencode_acp_binding("1.18.36").expect("next synthetic stable version");
    assert!(matches!(
        claim.assess(next.version()),
        InterfaceCompatibilityAssessment::UnverifiedNewer(_)
    ));
}

#[test]
fn npm_package_trees_and_hop_file_sets_are_exact() {
    let identity = json(IDENTITY);
    let inventory = json(INVENTORY);
    let versions = ["1.18.32", "1.18.33", "1.18.34", "1.18.35"];
    let package_names = ["opencode-ai", "opencode-darwin-arm64"];
    let hops = [
        "1.18.32_to_1.18.33",
        "1.18.33_to_1.18.34",
        "1.18.34_to_1.18.35",
    ];
    let npm = inventory["npm_packages"]
        .as_object()
        .expect("npm package map");
    assert_exact_object_keys(npm, &package_names);

    for package in package_names {
        let records = &npm[package];
        let mut expected_keys = versions.to_vec();
        expected_keys.push("hops");
        assert_exact_object_keys(records.as_object().unwrap(), &expected_keys);
        for version in versions {
            let record = &records[version];
            assert_eq!(record["integrity_matches_downloaded_tarball"], true);
            assert_eq!(record["npm_gitHead_present"], false);
            let files = record["files"].as_array().expect("complete package tree");
            let expected_paths = if package == "opencode-ai" {
                &[
                    "LICENSE",
                    "bin/opencode.exe",
                    "package.json",
                    "postinstall.mjs",
                ][..]
            } else {
                &["bin/opencode", "package.json"][..]
            };
            assert_eq!(files.len(), expected_paths.len());
            assert_exact_strings(
                &Value::Array(files.iter().map(|entry| entry["path"].clone()).collect()),
                expected_paths,
            );
            assert_eq!(
                record["tarball_sha256"],
                identity["npm_artifacts"][package][version]["tarball_sha256"]
            );
        }
        assert_exact_object_keys(records["hops"].as_object().unwrap(), &hops);
        for hop in hops {
            let delta = &records["hops"][hop];
            assert_exact_object_keys(
                delta.as_object().unwrap(),
                &["added", "removed", "changed", "identical"],
            );
            assert_exact_strings(&delta["added"], &[]);
            assert_exact_strings(&delta["removed"], &[]);
            if package == "opencode-ai" {
                assert_exact_strings(&delta["changed"], &["package.json"]);
                assert_exact_strings(
                    &delta["identical"],
                    &["LICENSE", "bin/opencode.exe", "postinstall.mjs"],
                );
            } else {
                assert_exact_strings(&delta["changed"], &["bin/opencode", "package.json"]);
                assert_exact_strings(&delta["identical"], &[]);
            }
        }
    }
}

#[test]
fn selected_surface_hashes_and_full_source_hop_ledger_match() {
    let identity = json(IDENTITY);
    let inventory = json(INVENTORY);
    let protocol = json(PROTOCOL);
    let versions = ["1.18.32", "1.18.33", "1.18.34", "1.18.35"];
    let hop_names = [
        "1.18.32_to_1.18.33",
        "1.18.33_to_1.18.34",
        "1.18.34_to_1.18.35",
    ];
    assert_exact_object_keys(
        inventory["source_hop_complete_tree_deltas"]
            .as_object()
            .unwrap(),
        &hop_names,
    );
    assert_exact_strings(
        &protocol["source_slice"]["selected_behavior_files"],
        SELECTED_SOURCE_FILES,
    );
    assert_exact_strings(
        &protocol["source_slice"]["route_behavior_files"],
        ROUTE_BEHAVIOR_FILES,
    );
    assert_eq!(
        protocol["source_slice"]["package_manifest_file"],
        "packages/opencode/package.json"
    );
    assert_exact_object_keys(
        protocol["semantic_behavior_files_by_axis"]
            .as_object()
            .unwrap(),
        &[
            "wire",
            "lifecycle",
            "failure",
            "permission",
            "usage",
            "configuration",
            "tools",
        ],
    );
    assert_eq!(
        protocol["http_mcp_live_evidence"]["qualified_exact_version"],
        "1.18.18"
    );
    assert_eq!(
        protocol["http_mcp_live_evidence"]["transfer_to_1_18_33_through_1_18_35"],
        false
    );

    let expected_counts = [
        ("1.18.32", 6572, 60, 772),
        ("1.18.33", 6578, 60, 773),
        ("1.18.34", 6581, 60, 773),
        ("1.18.35", 6568, 60, 768),
    ];
    let mut trees = BTreeMap::<String, BTreeMap<String, Value>>::new();
    let mut counts = BTreeMap::<String, (usize, usize, usize)>::new();
    let mut lines = SOURCE_TREE.lines();
    assert_eq!(
        lines.next(),
        Some("version\tkind\tpath\tsize\tsha256_or_target")
    );
    for line in lines {
        let fields = line.splitn(5, '\t').collect::<Vec<_>>();
        assert_eq!(fields.len(), 5);
        let (version, kind, path, size, value) =
            (fields[0], fields[1], fields[2], fields[3], fields[4]);
        let entry = match kind {
            "dir" => {
                let count = counts.entry(version.to_owned()).or_default();
                count.2 += 1;
                json!({"kind": kind})
            }
            "file" => {
                let count = counts.entry(version.to_owned()).or_default();
                count.0 += 1;
                json!({"kind": kind, "size": size.parse::<u64>().unwrap(), "sha256": value})
            }
            "symlink" => {
                let count = counts.entry(version.to_owned()).or_default();
                count.1 += 1;
                json!({"kind": kind, "target": value})
            }
            other => panic!("unknown source tree entry kind: {other}"),
        };
        let previous = trees
            .entry(version.to_owned())
            .or_default()
            .insert(path.to_owned(), entry);
        assert!(previous.is_none(), "duplicate path {path} in {version}");
    }

    for (version, files, symlinks, directories) in expected_counts {
        let actual = counts.get(version).expect("source archive counts");
        assert_eq!(*actual, (files, symlinks, directories));
        assert_eq!(
            inventory["source_versions"][version]["full_repository_regular_file_count"],
            files
        );
        let selected_hashes = inventory["source_versions"][version]["selected_file_sha256"]
            .as_object()
            .expect("selected source hash map");
        assert_exact_object_keys(selected_hashes, SELECTED_SOURCE_FILES);
        let boundary_hashes = inventory["source_versions"][version]["boundary_file_sha256"]
            .as_object()
            .expect("provider boundary hash map");
        assert_exact_object_keys(
            boundary_hashes,
            &[
                "packages/core/src/session/runner/llm.ts",
                "packages/core/test/session-runner.test.ts",
                "packages/opencode/src/session/llm/request.ts",
                "packages/opencode/test/session/llm.test.ts",
            ],
        );
        for (path, digest) in selected_hashes {
            assert_eq!(trees[version][path]["sha256"], *digest);
        }
        for (path, digest) in boundary_hashes {
            assert_eq!(trees[version][path]["sha256"], *digest);
        }
    }

    for (previous, current) in versions.iter().zip(versions.iter().skip(1)) {
        let old = &trees[*previous];
        let new = &trees[*current];
        let all_paths = old
            .keys()
            .chain(new.keys())
            .map(String::as_str)
            .collect::<BTreeSet<_>>();
        let mut added = Vec::new();
        let mut removed = Vec::new();
        let mut changed = Vec::new();
        for path in all_paths {
            match (old.get(path), new.get(path)) {
                (None, Some(after)) => added.push(json!({"path": path, "kind": after["kind"], "size": after.get("size"), "sha256": after.get("sha256"), "target": after.get("target")})),
                (Some(before), None) => removed.push(json!({"path": path, "kind": before["kind"], "size": before.get("size"), "sha256": before.get("sha256"), "target": before.get("target")})),
                (Some(before), Some(after)) if before != after => changed.push(json!({"path": path, "before": before, "after": after})),
                _ => {}
            }
        }
        let expected =
            &inventory["source_hop_complete_tree_deltas"][format!("{previous}_to_{current}")];
        let normalize = |entries: Vec<Value>| {
            entries
                .into_iter()
                .map(|mut entry| {
                    if let Some(object) = entry.as_object_mut() {
                        object.retain(|_, value| !value.is_null());
                    }
                    entry
                })
                .collect::<Vec<_>>()
        };
        assert_eq!(
            json!({"added":normalize(added),"removed":normalize(removed),"changed":changed}),
            *expected,
            "complete source delta {previous} to {current}"
        );
    }

    let selected_hops = inventory["source_hop_selected_classification"]
        .as_object()
        .expect("selected source hop ledger");
    assert_exact_object_keys(selected_hops, &hop_names);
    for hop in hop_names {
        assert_exact_strings(
            &protocol["source_slice"]["changed_by_hop"][hop],
            &["packages/opencode/package.json"],
        );
        assert_exact_strings(
            &protocol["source_slice"]["invariant_selected_behavior_files_by_hop"][hop],
            ROUTE_BEHAVIOR_FILES,
        );
        assert_exact_strings(
            &selected_hops[hop]["changed_selected_files"],
            &["packages/opencode/package.json"],
        );
        assert_exact_strings(
            &selected_hops[hop]["identical_selected_files"],
            ROUTE_BEHAVIOR_FILES,
        );
    }
    assert_exact_strings(
        &selected_hops["1.18.32_to_1.18.33"]["changed_boundary_files"],
        &[],
    );
    assert_exact_strings(
        &selected_hops["1.18.33_to_1.18.34"]["changed_boundary_files"],
        &[
            "packages/core/src/session/runner/llm.ts",
            "packages/core/test/session-runner.test.ts",
            "packages/opencode/src/session/llm/request.ts",
            "packages/opencode/test/session/llm.test.ts",
        ],
    );
    assert_exact_strings(
        &selected_hops["1.18.34_to_1.18.35"]["changed_boundary_files"],
        &[],
    );

    let implementation_deltas = [
        (
            "1.18.32_to_1.18.33",
            &[
                "packages/core/src/open.ts",
                "packages/opencode/src/cli/cmd/account.ts",
                "packages/opencode/src/cli/cmd/debug/config.ts",
                "packages/opencode/src/cli/cmd/debug/redact.ts",
                "packages/opencode/src/cli/cmd/web.ts",
                "packages/opencode/src/mcp/browser.ts",
                "packages/opencode/src/mcp/oauth-provider.ts",
                "packages/opencode/src/plugin/digitalocean.ts",
                "packages/opencode/src/plugin/openai/codex.ts",
                "packages/opencode/src/plugin/snowflake-cortex.ts",
                "packages/opencode/src/provider/provider.ts",
                "packages/opencode/src/provider/transform.ts",
            ][..],
        ),
        (
            "1.18.33_to_1.18.34",
            &[
                "packages/core/src/session/runner/llm.ts",
                "packages/opencode/src/session/llm/request.ts",
            ][..],
        ),
        (
            "1.18.34_to_1.18.35",
            &["packages/opencode/src/session/message-v2.ts"][..],
        ),
    ];
    for (hop, expected) in implementation_deltas {
        let delta = &inventory["source_hop_complete_tree_deltas"][hop];
        let paths = delta["added"]
            .as_array()
            .unwrap()
            .iter()
            .chain(delta["removed"].as_array().unwrap())
            .chain(delta["changed"].as_array().unwrap())
            .map(|entry| entry["path"].as_str().unwrap())
            .filter(|path| {
                path.starts_with("packages/opencode/src/") || path.starts_with("packages/core/src/")
            })
            .collect::<Vec<_>>();
        assert_exact_strings(&json!(paths), expected);
    }

    assert_eq!(
        identity["complete_inventory"]["source_canonical_tsv_sha256"],
        "6ccadbecb9d91183cb79e236b046cf89ffe031602e96a1451c3c88e64102faff"
    );
    assert_eq!(
        sha256(SOURCE_TREE),
        "6ccadbecb9d91183cb79e236b046cf89ffe031602e96a1451c3c88e64102faff"
    );
}

fn json(input: &str) -> Value {
    serde_json::from_str(input).expect("fixture is valid JSON")
}

fn sha256(input: &str) -> String {
    format!("{:x}", Sha256::digest(input.as_bytes()))
}

fn assert_exact_strings(actual: &Value, expected: &[&str]) {
    let actual = actual.as_array().expect("string array");
    assert_eq!(actual.len(), expected.len());
    let actual = actual
        .iter()
        .map(|value| value.as_str().expect("string value"))
        .collect::<BTreeSet<_>>();
    let expected = expected.iter().copied().collect::<BTreeSet<_>>();
    assert_eq!(actual, expected);
}

fn assert_exact_object_keys(actual: &Map<String, Value>, expected: &[&str]) {
    let actual = actual.keys().map(String::as_str).collect::<BTreeSet<_>>();
    let expected = expected.iter().copied().collect::<BTreeSet<_>>();
    assert_eq!(actual, expected);
}

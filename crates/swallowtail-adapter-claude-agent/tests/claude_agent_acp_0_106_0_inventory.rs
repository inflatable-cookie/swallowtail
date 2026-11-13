use serde_json::Value;
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;

const INVENTORY: &str = include_str!("fixtures/claude-agent-acp-0.106.0/dist-inventory.json");
const REGISTRY: &str = include_str!("fixtures/claude-agent-acp-0.106.0/registry.json");
const PERMISSION: &str = include_str!("fixtures/claude-agent-acp-0.106.0/permission-boundary.json");
const DEPENDENCIES: &str = include_str!("fixtures/claude-agent-acp-0.106.0/dependencies.json");
const CLOSURE: &str = include_str!("fixtures/claude-agent-acp-0.106.0/static-closure.json");
const IDLE: &str = include_str!("fixtures/claude-agent-acp-0.106.0/idle-compaction-boundary.json");
const IDLE_TRACE: &str = include_str!("fixtures/claude-agent-acp-0.106.0/idle-control-trace.json");
const GUARD_MANIFEST: &str = include_str!("../sidecar/claude-agent-acp-guard-v1-inventory.json");
const GUARD_EDGES: &str = include_str!("fixtures/claude-agent-acp-0.106.0/guard-import-edges.json");
const PERMISSION_CONTROL: &str =
    include_str!("fixtures/claude-agent-acp-0.106.0/native-permission-control.json");

#[test]
fn current_acp_inventory_reconstructs_all_hops_without_qualifying_them() {
    for (text, expected) in [
        (
            INVENTORY,
            "7c8c799593c9cba984c975d645b894efdc4d324b49e2094d1a358d2951172f00",
        ),
        (
            REGISTRY,
            "f95b1f9aa27ed0f892e983d423b11b6aab8dd393e649d0e2becbd1cb4cec9e45",
        ),
        (
            PERMISSION,
            "12406aa68ae19e9cce2664ebebf5ebe4300ce806f5b27973b744c06f75cbebe2",
        ),
    ] {
        assert_eq!(format!("{:x}", Sha256::digest(text.as_bytes())), expected);
    }
    let inventory: Value = serde_json::from_str(INVENTORY).expect("inventory JSON");
    let versions = inventory["versions"].as_array().expect("versions");
    assert_eq!(
        versions
            .iter()
            .map(|v| v.as_str().expect("version"))
            .collect::<Vec<_>>(),
        [
            "0.100.0", "0.101.0", "0.101.1", "0.102.0", "0.103.0", "0.104.0", "0.104.1", "0.105.0",
            "0.106.0"
        ]
    );
    for pair in versions.windows(2) {
        let old = pair[0].as_str().expect("old version");
        let new = pair[1].as_str().expect("new version");
        let before = inventory["sha256_by_path"][old]
            .as_object()
            .expect("old tree");
        let after = inventory["sha256_by_path"][new]
            .as_object()
            .expect("new tree");
        let old_paths = before.keys().collect::<BTreeSet<_>>();
        let new_paths = after.keys().collect::<BTreeSet<_>>();
        let shared = old_paths
            .intersection(&new_paths)
            .copied()
            .collect::<BTreeSet<_>>();
        let expected = [
            (
                "added",
                new_paths
                    .difference(&old_paths)
                    .copied()
                    .collect::<BTreeSet<_>>(),
            ),
            (
                "removed",
                old_paths
                    .difference(&new_paths)
                    .copied()
                    .collect::<BTreeSet<_>>(),
            ),
            (
                "changed",
                shared
                    .iter()
                    .copied()
                    .filter(|key| before[*key] != after[*key])
                    .collect::<BTreeSet<_>>(),
            ),
            (
                "identical",
                shared
                    .iter()
                    .copied()
                    .filter(|key| before[*key] == after[*key])
                    .collect::<BTreeSet<_>>(),
            ),
        ];
        let delta = &inventory["deltas"][format!("{old}..{new}")];
        for (kind, paths) in expected {
            let recorded = delta[kind]
                .as_array()
                .expect("delta array")
                .iter()
                .map(|v| v.as_str().expect("path"))
                .collect::<Vec<_>>();
            assert_eq!(recorded.len(), paths.len(), "{old}..{new} {kind}");
            assert_eq!(
                recorded.into_iter().collect::<BTreeSet<_>>(),
                paths.into_iter().map(String::as_str).collect()
            );
        }
    }
}

#[test]
fn current_acp_dependency_and_closure_evidence_is_complete_and_pinned() {
    assert_eq!(
        format!(
            "{:x}",
            Sha256::digest(
                swallowtail_adapter_claude_agent::CLAUDE_AGENT_ACP_GUARD_LOADER_SOURCE.as_bytes()
            )
        ),
        "5694068ca15f0a42c6e21310906b5dc330b9d04be8a3c3783571dc11df4a2475"
    );
    for (text, expected) in [
        (
            DEPENDENCIES,
            "e681777dced05f1ea6ac8d2a206b3b6bc31a9ebd9680cae8a7309dc52fe36ac3",
        ),
        (
            CLOSURE,
            "af885bf7ea393691d79ab67cb12c82d897e3478e8c39cdea7efca082536265b0",
        ),
        (
            IDLE,
            "fe0f77219190d123fdcf55efcdd46aac2767eb96d17bcf758bdd2882eb611129",
        ),
        (
            IDLE_TRACE,
            "e6e04b042909d0e321e21a9965b501647075aa1943e0f6e6a10d466fc48efcb7",
        ),
        (
            GUARD_MANIFEST,
            "d057fcb97d4a7a514b6b14dcb4548d61bbda47b1067b31da90bbd4be4f620141",
        ),
        (
            GUARD_EDGES,
            "0c1eb292ba566fc267714dcd6b2b352bdee2d4978c60b75de6b4bbddaa886a04",
        ),
        (
            PERMISSION_CONTROL,
            "e8125782d88cd32160141e3fec2e67e870e98a7311b440b5753bf03bac1c8b38",
        ),
    ] {
        assert_eq!(format!("{:x}", Sha256::digest(text.as_bytes())), expected);
    }
    let inventory: Value = serde_json::from_str(INVENTORY).expect("inventory JSON");
    let closure: Value = serde_json::from_str(CLOSURE).expect("closure JSON");
    let manifest: Value = serde_json::from_str(GUARD_MANIFEST).expect("guard manifest JSON");
    let versions = [
        "0.101.0", "0.101.1", "0.102.0", "0.103.0", "0.104.0", "0.104.1", "0.105.0", "0.106.0",
    ];
    assert_eq!(
        manifest.as_object().expect("manifest versions").len(),
        versions.len()
    );
    for version in versions {
        let expected_paths = closure["reachable_js"][version]
            .as_array()
            .expect("reachable paths");
        let rows = manifest[version].as_object().expect("manifest rows");
        assert_eq!(rows.len(), expected_paths.len());
        for path in expected_paths {
            let path = path.as_str().expect("path");
            assert_eq!(rows[path], inventory["sha256_by_path"][version][path]);
        }
        assert_eq!(
            swallowtail_adapter_claude_agent::claude_agent_acp_guard_v1_source_manifest(version)
                .expect("known hop")
                .len(),
            rows.len()
        );
    }
    for version in ["0.100.0", "0.106.1", "latest"] {
        assert!(
            swallowtail_adapter_claude_agent::claude_agent_acp_guard_v1_source_manifest(version)
                .is_none()
        );
    }
    let edges: Value = serde_json::from_str(GUARD_EDGES).expect("guard edges JSON");
    let registry: Value = serde_json::from_str(REGISTRY).expect("registry JSON");
    let hops = edges["hops"].as_object().expect("hop edges");
    assert_eq!(hops.len(), 8);
    for (index, version) in [
        "0.101.0", "0.101.1", "0.102.0", "0.103.0", "0.104.0", "0.104.1", "0.105.0", "0.106.0",
    ]
    .into_iter()
    .enumerate()
    {
        let row = &hops[version];
        let caller = if index < 3 {
            "dist/acp-agent.js"
        } else {
            "dist/v1/acp-agent.js"
        };
        let import = if index < 3 {
            "./idle-compaction.js"
        } else {
            "../idle-compaction.js"
        };
        assert_eq!(row["caller"], caller);
        assert_eq!(row["import"], import);
        assert_eq!(row["idle_module"], "dist/idle-compaction.js");
        assert_eq!(
            row["caller_sha256"],
            inventory["sha256_by_path"][version][caller]
        );
        assert_eq!(
            row["idle_sha256"],
            inventory["sha256_by_path"][version]["dist/idle-compaction.js"]
        );
        assert_eq!(
            row["tarball_sha256"],
            registry["versions"][version]["tarball_sha256"]
        );
        assert_eq!(row["construction_count"], 1);
        assert_eq!(row["literal_dot_method_assignment_count"], 0);
    }
    for (hop, delta) in inventory["deltas"].as_object().expect("deltas") {
        let rows = closure["changes"][hop]
            .as_object()
            .expect("classified changes");
        let changes = ["added", "removed", "changed"]
            .into_iter()
            .flat_map(|kind| {
                delta[kind]
                    .as_array()
                    .expect("delta")
                    .iter()
                    .map(move |path| (path.as_str().expect("path"), kind))
            })
            .collect::<Vec<_>>();
        assert_eq!(rows.len(), changes.len(), "{hop}");
        for (path, kind) in changes {
            assert_eq!(rows[path]["delta"], kind);
            assert!(rows[path]["class"].as_str().is_some());
            if rows[path]["class"] == "v1-reachable" {
                assert!(rows[path]["role"].as_str().is_some());
            }
        }
    }
}

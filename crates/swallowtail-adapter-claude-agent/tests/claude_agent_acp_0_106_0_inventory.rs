use serde_json::Value;
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;

const INVENTORY: &str = include_str!("fixtures/claude-agent-acp-0.106.0/dist-inventory.json");
const REGISTRY: &str = include_str!("fixtures/claude-agent-acp-0.106.0/registry.json");
const PERMISSION: &str = include_str!("fixtures/claude-agent-acp-0.106.0/permission-boundary.json");
const DEPENDENCIES: &str = include_str!("fixtures/claude-agent-acp-0.106.0/dependencies.json");
const IDLE: &str = include_str!("fixtures/claude-agent-acp-0.106.0/idle-compaction-boundary.json");
const IDLE_TRACE: &str = include_str!("fixtures/claude-agent-acp-0.106.0/idle-control-trace.json");
const GUARD_MANIFEST: &str = include_str!("../sidecar/claude-agent-acp-guard-v1-inventory.json");
const GUARD_EDGES: &str = include_str!("fixtures/claude-agent-acp-0.106.0/guard-import-edges.json");
const PERMISSION_CONTROL: &str =
    include_str!("fixtures/claude-agent-acp-0.106.0/native-permission-control.json");
const IMPORT_ARTIFACTS: &str =
    include_str!("fixtures/claude-agent-acp-0.106.0/import-edge-artifacts.json");

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
fn frozen_acp_dependencies_and_idle_evidence_stay_pinned() {
    for (source, expected) in [
        (
            DEPENDENCIES,
            "e681777dced05f1ea6ac8d2a206b3b6bc31a9ebd9680cae8a7309dc52fe36ac3",
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
        (
            IMPORT_ARTIFACTS,
            "674bfe4f5d73b62ac8c705107eb5cac3508d532bc332d80121fd68bd2bf8d690",
        ),
    ] {
        assert_eq!(format!("{:x}", Sha256::digest(source.as_bytes())), expected);
    }
    assert_eq!(
        format!(
            "{:x}",
            Sha256::digest(
                swallowtail_adapter_claude_agent::CLAUDE_AGENT_ACP_GUARD_LOADER_SOURCE.as_bytes()
            )
        ),
        "34cf60596e2c202949d7acf86f8878559fbbf22e4905e24d60248c0d10121a94"
    );
}

#[test]
fn guarded_production_graphs_pin_every_source_format_and_edge() {
    assert_eq!(
        swallowtail_adapter_claude_agent::CLAUDE_AGENT_ACP_GUARDED_PERMISSION_BEHAVIOR,
        "claude-agent.acp.guarded-permission-inheritance-v8"
    );
    for (source, expected) in [
        (
            include_str!("../sidecar/claude-agent-acp-guard-graphs/0.101.0.json"),
            "8ef8bf48279fa6ed2cbe12c8b77220f47d238df83e00962fcb612a98d8622be1",
        ),
        (
            include_str!("../sidecar/claude-agent-acp-guard-graphs/0.101.1.json"),
            "9a2b5c118e097e9c3de153dbded1b0a83a99acaa7072e9c5e6d362293ee1a4b5",
        ),
        (
            include_str!("../sidecar/claude-agent-acp-guard-graphs/0.102.0.json"),
            "b7c5d491fb7d71dea46ff4cd03849489bd7af4b13d6ab250601655c0e185d811",
        ),
        (
            include_str!("../sidecar/claude-agent-acp-guard-graphs/0.103.0.json"),
            "2551b9679140c0c252e16a6c8176fe31ac3a17703e51ce8839be40d57e578b28",
        ),
        (
            include_str!("../sidecar/claude-agent-acp-guard-graphs/0.104.0.json"),
            "b13669b6d243392a9a6a835b93721117d79de5cf4b41354cdaa9189f4d2f1a8a",
        ),
        (
            include_str!("../sidecar/claude-agent-acp-guard-graphs/0.104.1.json"),
            "a25da1c72f5dc94dfb9d539c2612aa93edd25d8dbf88f38bfe88cd5d9bfa304d",
        ),
        (
            include_str!("../sidecar/claude-agent-acp-guard-graphs/0.105.0.json"),
            "c01615f0ea9a0ecec00ebb3860079f20f655e5a8e84183bd2e45e5e4acbed4cc",
        ),
        (
            include_str!("../sidecar/claude-agent-acp-guard-graphs/0.106.0.json"),
            "1ce9e874d840aad7c28621abf1a850df53481d01fdda26eb930e60a9bee68331",
        ),
        (
            include_str!("fixtures/claude-agent-acp-0.106.0/guard-package-metadata.json"),
            "ec892ea5aa52fe6deb77d912d4c5747cfec41dcaff4fc966e40e1e20bd822080",
        ),
    ] {
        assert_eq!(format!("{:x}", Sha256::digest(source.as_bytes())), expected);
    }
}

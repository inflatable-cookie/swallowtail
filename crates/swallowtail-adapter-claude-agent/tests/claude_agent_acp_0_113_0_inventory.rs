use serde_json::Value;
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;

const INVENTORY: &str = include_str!("fixtures/claude-agent-acp-0.113.0/dist-inventory.json");
const REGISTRY: &str = include_str!("fixtures/claude-agent-acp-0.113.0/registry.json");
const DEPENDENCIES: &str = include_str!("fixtures/claude-agent-acp-0.113.0/dependencies.json");

#[test]
fn frozen_0_113_0_inventory_reconstructs_all_thirteen_hops() {
    for (text, expected) in [
        (
            INVENTORY,
            "48d54c68fa8b216659d172e4f0d498a70c9bc8fb2041acc8372d4ce05a3d9780",
        ),
        (
            REGISTRY,
            "beb2e0febbeafd2bbc39099e0c94183353fe25318bb0b2ca4aff0d02079e9478",
        ),
        (
            DEPENDENCIES,
            "343c097ffd33dd73dece8b48933b8dd7084c90d3fced37dc27996a996b5b6564",
        ),
    ] {
        assert_eq!(hex::encode(Sha256::digest(text.as_bytes())), expected);
    }

    let inventory: Value = serde_json::from_str(INVENTORY).expect("inventory JSON");
    let versions = inventory["versions"].as_array().expect("versions");
    assert_eq!(
        versions
            .iter()
            .map(|v| v.as_str().expect("version"))
            .collect::<Vec<_>>(),
        [
            "0.106.0", "0.107.0", "0.107.1", "0.107.2", "0.107.3", "0.108.0", "0.108.1", "0.109.0",
            "0.109.1", "0.110.0", "0.110.1", "0.111.0", "0.112.0", "0.113.0",
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
fn guarded_graphs_0_107_0_through_0_113_0_pin_ruling_targets_and_digests() {
    let hops = [
        (
            "0.107.0",
            include_str!("../sidecar/claude-agent-acp-guard-graphs/0.107.0.json"),
            "639a31c887701c81e48adb49fe664d200c953a90d77a8259caa63b38a06a857e",
            false,
            false,
            false,
        ),
        (
            "0.107.1",
            include_str!("../sidecar/claude-agent-acp-guard-graphs/0.107.1.json"),
            "5fe046079e7f1819ef380f9b10130f33581935d7d87633052b1ca78f4c5014d5",
            false,
            false,
            false,
        ),
        (
            "0.107.2",
            include_str!("../sidecar/claude-agent-acp-guard-graphs/0.107.2.json"),
            "04490b8efd032812d36ecef6bca3bdc0b570f789b58879e31e934dffa2e1d285",
            false,
            false,
            false,
        ),
        (
            "0.107.3",
            include_str!("../sidecar/claude-agent-acp-guard-graphs/0.107.3.json"),
            "fda559e308640e570d5eaa420edd52cd1df7b47c994a7479ea6473bf064ad04a",
            false,
            false,
            false,
        ),
        (
            "0.108.0",
            include_str!("../sidecar/claude-agent-acp-guard-graphs/0.108.0.json"),
            "70fb96274454111c184a441be74e3abc31ca4b3934674100400c223ca562c4d9",
            false,
            false,
            false,
        ),
        (
            "0.108.1",
            include_str!("../sidecar/claude-agent-acp-guard-graphs/0.108.1.json"),
            "c89cbc60de916298387423d401c3588b54a811f530b766b653aa6faffdde97fd",
            false,
            false,
            false,
        ),
        (
            "0.109.0",
            include_str!("../sidecar/claude-agent-acp-guard-graphs/0.109.0.json"),
            "8b9092f2bc664da0f806e5303e8cc30899b57f04a271941949cacd5241aa85e5",
            false,
            false,
            false,
        ),
        (
            "0.109.1",
            include_str!("../sidecar/claude-agent-acp-guard-graphs/0.109.1.json"),
            "20fcb52e60e5ed0173904521391bb7f838f9d36988e5c46406141f6443e08ba7",
            false,
            false,
            false,
        ),
        (
            "0.110.0",
            include_str!("../sidecar/claude-agent-acp-guard-graphs/0.110.0.json"),
            "b3f898a172cc44f16a5ccc0a6ae11ad2e5b4b99d78d96d89fe86b7bec10df938",
            false,
            false,
            false,
        ),
        (
            "0.110.1",
            include_str!("../sidecar/claude-agent-acp-guard-graphs/0.110.1.json"),
            "7f34381f3fe80cb90001fd25d8ad68c237bb7c6d2b96cc84f6acd91d0c94b0e0",
            false,
            false,
            false,
        ),
        (
            "0.111.0",
            include_str!("../sidecar/claude-agent-acp-guard-graphs/0.111.0.json"),
            "6431c31db2262b31ac82a253df215ec1845453d3f6fec1afb252b58313ebeb49",
            true,
            false,
            false,
        ),
        (
            "0.112.0",
            include_str!("../sidecar/claude-agent-acp-guard-graphs/0.112.0.json"),
            "c029a9c2baaa3989167e0b2a165a98ff7f6481ce48f1e570eb633c2366485295",
            true,
            false,
            false,
        ),
        (
            "0.113.0",
            include_str!("../sidecar/claude-agent-acp-guard-graphs/0.113.0.json"),
            "d88911cbc0c8b06e764b6c135341c2a5d0a62c6921285cb79c7329b555111e9b",
            true,
            true,
            true,
        ),
    ];

    for (version, source, expected, has_mcp_veto, has_rc_veto, has_plugin_deny) in hops {
        assert_eq!(
            hex::encode(Sha256::digest(source.as_bytes())),
            expected,
            "{version}"
        );
        let graph: Value = serde_json::from_str(source).expect("frozen graph");
        assert_eq!(
            graph["files"]["acp/dist/idle-compaction.js"]["sha256"],
            "3128c2566a2f8c3b69be35e63780f8c87faf7f3184cd723d7a095a44bde7f28d",
            "{version} idle compaction must remain identical"
        );
        assert!(
            graph["denied_dynamic"]
                .as_array()
                .expect("denied_dynamic")
                .is_empty(),
            "{version}"
        );
        let programs = graph["platform_programs"]
            .as_array()
            .expect("platform_programs");
        assert_eq!(programs.len(), 1, "{version}");
        assert_eq!(
            programs[0]["parent"],
            "acp/dist/session-index/live-registry.js"
        );
        assert_eq!(programs[0]["name"], "ps");
        assert_eq!(programs[0]["arguments"], "live-process-starts-v1");

        if has_mcp_veto {
            assert_eq!(
                graph["mcp_refusal_fallback"], "acp/dist/mcp-servers.js",
                "{version}"
            );
        } else {
            assert!(graph.get("mcp_refusal_fallback").is_none(), "{version}");
        }
        if has_rc_veto {
            assert_eq!(
                graph["remote_control"], "acp/dist/remote-control.js",
                "{version}"
            );
        } else {
            assert!(graph.get("remote_control").is_none(), "{version}");
        }
        if has_plugin_deny {
            assert_eq!(
                graph["denied_child_process_parents"],
                serde_json::json!(["acp/dist/plugin-command.js"]),
                "{version}"
            );
        } else {
            assert!(
                graph.get("denied_child_process_parents").is_none(),
                "{version}"
            );
        }

        // Guard manifest public helpers must expose every frozen hop on both platforms.
        assert!(
            swallowtail_adapter_claude_agent::claude_agent_acp_guard_v1_source_manifest(version)
                .is_some(),
            "{version}"
        );
        for platform in ["darwin-arm64", "linux-x64"] {
            assert!(
                swallowtail_adapter_claude_agent::claude_agent_acp_guarded_v1_frozen(
                    version, platform
                )
                .is_some(),
                "{version}/{platform}"
            );
        }
    }
}

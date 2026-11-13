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
const IMPORT_INVENTORY: &str =
    include_str!("fixtures/claude-agent-acp-0.106.0/import-edge-inventory.json");
const IMPORT_ARTIFACTS: &str =
    include_str!("fixtures/claude-agent-acp-0.106.0/import-edge-artifacts.json");
const SDK_ROOT_EXPORTS: &str =
    include_str!("fixtures/claude-agent-acp-0.106.0/sdk-root-exports.json");

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
        "b89661de39326987f606faa61bf1a54b600d7d1fc3bffb3b62caafa226aae7fa"
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
        (
            IMPORT_INVENTORY,
            "cb2c20e7ab60a174f5ffb34368647acfeb5a6f4a78f3b0dd477e1bd1fd3c8cfb",
        ),
        (
            IMPORT_ARTIFACTS,
            "674bfe4f5d73b62ac8c705107eb5cac3508d532bc332d80121fd68bd2bf8d690",
        ),
        (
            SDK_ROOT_EXPORTS,
            "b06ea5d8ae3b50a8c9fc17f8821a8ae6d587c46388a78c002e26898ec1595a17",
        ),
    ] {
        assert_eq!(format!("{:x}", Sha256::digest(text.as_bytes())), expected);
    }
    let inventory: Value = serde_json::from_str(INVENTORY).expect("inventory JSON");
    let imports: Value = serde_json::from_str(IMPORT_INVENTORY).expect("import inventory JSON");
    let dependencies: Value = serde_json::from_str(DEPENDENCIES).expect("dependency JSON");
    let packages = imports["packages"].as_object().expect("import packages");
    assert_eq!(packages.len(), 17);
    for (name, files) in packages {
        let expected = if name.starts_with("0.10") {
            &inventory["sha256_by_path"][name]
        } else if let Some(version) = name.strip_prefix("_agentclientprotocol_sdk_") {
            &dependencies["trees"]["acpsdk"][version]
        } else if let Some(version) = name.strip_prefix("_anthropic-ai_claude-agent-sdk_") {
            &dependencies["trees"]["agentsdk"][version]
        } else {
            assert!(name == "diff_9.0.0" || name == "zod_4.6.7");
            continue;
        };
        let tree = expected.as_object().expect("frozen package tree");
        let js_paths = tree
            .keys()
            .filter(|name| {
                name.ends_with(".js") || name.ends_with(".mjs") || name.ends_with(".cjs")
            })
            .collect::<BTreeSet<_>>();
        let rows = files.as_object().expect("package imports");
        assert_eq!(rows.keys().collect::<BTreeSet<_>>(), js_paths, "{name}");
        for (path, row) in rows {
            assert_eq!(row["sha256"], tree[path], "{name}/{path}");
        }
    }
    for version in [
        "0.101.0", "0.101.1", "0.102.0", "0.103.0", "0.104.0", "0.104.1", "0.105.0", "0.106.0",
    ] {
        let cli = &imports["packages"][version]["dist/claude-cli.js"];
        let edges = cli["edges"].as_array().expect("CLI edges");
        assert!(
            edges
                .iter()
                .any(|edge| edge["kind"] == "import-meta-resolve"
                    && edge["specifier"] == "@anthropic-ai/claude-agent-sdk")
        );
        assert!(
            edges
                .iter()
                .any(|edge| edge["kind"] == "dynamic" && edge["specifier"] == "node:module")
        );
        assert!(
            cli["cautions"]
                .as_array()
                .expect("CLI cautions")
                .iter()
                .any(|item| item["kind"] == "identifier:createRequire")
        );
    }
    let closure: Value = serde_json::from_str(CLOSURE).expect("closure JSON");
    let registry_for_roots: Value = serde_json::from_str(REGISTRY).expect("registry JSON");
    let sdk_roots: Value = serde_json::from_str(SDK_ROOT_EXPORTS).expect("SDK exports JSON");
    let manifest: Value = serde_json::from_str(GUARD_MANIFEST).expect("guard manifest JSON");
    let versions = [
        "0.101.0", "0.101.1", "0.102.0", "0.103.0", "0.104.0", "0.104.1", "0.105.0", "0.106.0",
    ];
    assert_eq!(
        manifest.as_object().expect("manifest versions").len(),
        versions.len()
    );
    for (index, version) in versions.into_iter().enumerate() {
        let expected_paths = closure["reachable_js"][version]
            .as_array()
            .expect("reachable paths");
        let caller = if index < 3 {
            "dist/acp-agent.js"
        } else {
            "dist/v1/acp-agent.js"
        };
        assert!(expected_paths.iter().any(|path| path == caller));
        assert!(
            expected_paths
                .iter()
                .any(|path| path == "dist/claude-cli.js")
        );
        assert!(
            imports["packages"][version][caller]["edges"]
                .as_array()
                .expect("caller imports")
                .iter()
                .any(|edge| edge["kind"] == "static"
                    && edge["specifier"] == "@anthropic-ai/claude-agent-sdk")
        );
        let sdk_version = registry_for_roots["versions"][version]["dependencies"]["@anthropic-ai/claude-agent-sdk"].as_str().expect("SDK version");
        let root = &sdk_roots[sdk_version];
        assert_eq!(root["root_export"]["default"], "./sdk.mjs");
        assert_eq!(
            root["package_json_sha256"],
            dependencies["trees"]["agentsdk"][sdk_version]["package.json"]
        );
        assert_eq!(
            root["archive_sha256"],
            dependencies["artifacts"][format!("@anthropic-ai/claude-agent-sdk@{sdk_version}")]["tarball_sha256"]
        );
        let sdk = &imports["packages"][format!("_anthropic-ai_claude-agent-sdk_{sdk_version}")]["sdk.mjs"];
        assert!(
            sdk["cautions"]
                .as_array()
                .expect("SDK cautions")
                .iter()
                .any(|site| site["kind"] == "call:createRequire-alias")
        );
        assert!(
            sdk["cautions"]
                .as_array()
                .expect("SDK cautions")
                .iter()
                .any(|site| site["kind"] == "identifier:Function")
        );
        assert!(
            sdk["edges"]
                .as_array()
                .expect("SDK imports")
                .iter()
                .any(|edge| edge["specifier"] == "child_process"
                    || edge["specifier"] == "node:child_process")
        );
        let mut builtins = BTreeSet::new();
        for path in expected_paths {
            let path = path.as_str().expect("reachable path");
            for edge in imports["packages"][version][path]["edges"]
                .as_array()
                .expect("wrapper edges")
            {
                let specifier = edge["specifier"].as_str().expect("specifier");
                if specifier.starts_with("node:") {
                    builtins.insert(specifier);
                }
            }
        }
        let mut expected_builtins = BTreeSet::from([
            "node:child_process",
            "node:crypto",
            "node:fs",
            "node:fs/promises",
            "node:module",
            "node:os",
            "node:path",
            "node:readline",
            "node:stream/web",
            "node:util",
        ]);
        if index < 3 {
            expected_builtins.insert("node:string_decoder");
        }
        assert_eq!(
            builtins, expected_builtins,
            "{version} conservative wrapper builtins"
        );
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

#[test]
fn guarded_probe_proposal_keeps_exact_tuples_and_unproved_gates_separate() {
    const PROPOSAL: &str =
        include_str!("fixtures/claude-agent-acp-0.106.0/guarded-probe-proposal.json");
    assert_eq!(
        format!("{:x}", Sha256::digest(PROPOSAL.as_bytes())),
        "552a24d1f3d4d0ccce811d5366bc2d080ee21fff4bb530fcfdc5b422dab14821"
    );
    let plan: Value = serde_json::from_str(PROPOSAL).expect("proposal JSON");
    let registry: Value = serde_json::from_str(REGISTRY).expect("registry JSON");
    let dependencies: Value = serde_json::from_str(DEPENDENCIES).expect("dependency JSON");
    for (name, source) in [
        (
            "crates/swallowtail-adapter-claude-agent/examples/guarded_acp_component_probe.rs",
            include_bytes!("../examples/guarded_acp_component_probe.rs").as_slice(),
        ),
        (
            "crates/swallowtail-adapter-claude-agent/sidecar/claude-agent-acp-guard-loader.mjs",
            include_bytes!("../sidecar/claude-agent-acp-guard-loader.mjs").as_slice(),
        ),
        (
            "crates/swallowtail-adapter-claude-agent/tests/guard_component_probe/entry.mjs",
            include_bytes!("guard_component_probe/entry.mjs").as_slice(),
        ),
        (
            "crates/swallowtail-adapter-claude-agent/tests/guard_component_probe/idle.mjs",
            include_bytes!("guard_component_probe/idle.mjs").as_slice(),
        ),
    ] {
        assert_eq!(
            plan["component_stage"]["source_sha256"][name],
            format!("{:x}", Sha256::digest(source)),
            "proposal source drift: {name}"
        );
    }
    assert!(plan["original_runner"].is_null());
    assert_eq!(plan["qualification_ceiling"], "0.100.0");
    assert_eq!(plan["node"]["version"], "22.23.2");
    assert_eq!(plan["tuples"].as_array().expect("tuples").len(), 16);
    let mut actual = BTreeSet::new();
    for tuple in plan["tuples"].as_array().expect("tuples") {
        let hop = tuple["acp"].as_str().expect("ACP version");
        let platform = tuple["platform"].as_str().expect("platform");
        assert!(actual.insert((hop, platform)), "duplicate proposal tuple");
        let sdk = registry["versions"][hop]["dependencies"]["@anthropic-ai/claude-agent-sdk"]
            .as_str()
            .expect("SDK version");
        assert_eq!(tuple["agent_sdk"], sdk);
        assert_eq!(
            tuple["native_version"],
            format!("2.1.{}", sdk.split('.').next_back().expect("patch"))
        );
        assert_eq!(
            tuple["acp_tarball_sha256"],
            registry["versions"][hop]["tarball_sha256"]
        );
        assert_eq!(
            tuple["agent_sdk_tarball_sha256"],
            dependencies["artifacts"][format!("@anthropic-ai/claude-agent-sdk@{sdk}")]["tarball_sha256"]
        );
        assert_eq!(
            tuple["native_sha256"],
            dependencies["native_manifests"][sdk]["platforms"][platform]["checksum"]
        );
        assert_eq!(
            tuple["native_bytes"],
            dependencies["native_manifests"][sdk]["platforms"][platform]["size"]
        );
        assert_eq!(
            tuple["node_sha256"],
            plan["node"]["platforms"][platform]["node_sha256"]
        );
    }
    let expected: BTreeSet<_> = [
        "0.101.0", "0.101.1", "0.102.0", "0.103.0", "0.104.0", "0.104.1", "0.105.0", "0.106.0",
    ]
    .into_iter()
    .flat_map(|hop| ["darwin-arm64", "linux-x64"].map(|platform| (hop, platform)))
    .collect();
    assert_eq!(actual, expected);
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

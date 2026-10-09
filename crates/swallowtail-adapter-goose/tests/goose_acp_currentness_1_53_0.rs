use serde_json::Value;
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use swallowtail_adapter_goose::{GOOSE_RELEASE_VERSION, goose_acp_claim, goose_release_binding};
use swallowtail_core::{
    InterfaceCompatibilityAssessment, InterfaceNewerVersionPosture, InterfaceSupportStatus,
    InterfaceVersion,
};

const IDENTITY: &str = include_str!("fixtures/goose-acp-1.53.0/identity.json");
const INVENTORY: &str = include_str!("fixtures/goose-acp-1.53.0/dist-inventory.json");
const PROTOCOL: &str = include_str!("fixtures/goose-acp-1.53.0/protocol.json");
const MANIFEST_1_50_1: &str = include_str!("fixtures/goose-acp-1.53.0/manifest-1.50.1.tsv");
const MANIFEST_1_51_0: &str = include_str!("fixtures/goose-acp-1.53.0/manifest-1.51.0.tsv");
const MANIFEST_1_52_0: &str = include_str!("fixtures/goose-acp-1.53.0/manifest-1.52.0.tsv");
const MANIFEST_1_53_0: &str = include_str!("fixtures/goose-acp-1.53.0/manifest-1.53.0.tsv");

const POINTS: [(&str, &str); 4] = [
    ("1.50.1", MANIFEST_1_50_1),
    ("1.51.0", MANIFEST_1_51_0),
    ("1.52.0", MANIFEST_1_52_0),
    ("1.53.0", MANIFEST_1_53_0),
];

const EXPECTED_SELECTED_SOURCE_PATHS: &[&str] = &[
    "Cargo.lock",
    "Cargo.toml",
    "crates/goose-cli/src/cli.rs",
    "crates/goose/acp-meta.json",
    "crates/goose/acp-schema.json",
    "crates/goose-provider-types/src/conversation/message.rs",
    "crates/goose-provider-types/src/errors.rs",
    "crates/goose-provider-types/src/goose_mode.rs",
    "crates/goose/src/acp/common.rs",
    "crates/goose/src/acp/mod.rs",
    "crates/goose/src/acp/provider.rs",
    "crates/goose/src/acp/response_builder.rs",
    "crates/goose/src/acp/server.rs",
    "crates/goose/src/acp/server/config.rs",
    "crates/goose/src/acp/server/custom_dispatch.rs",
    "crates/goose/src/acp/server/dispatch.rs",
    "crates/goose/src/acp/server/extensions.rs",
    "crates/goose/src/acp/server/fork_session.rs",
    "crates/goose/src/acp/server/load_session.rs",
    "crates/goose/src/acp/server/manage_sessions.rs",
    "crates/goose/src/acp/server/new_session.rs",
    "crates/goose/src/acp/server/prompts.rs",
    "crates/goose/src/acp/server/providers.rs",
    "crates/goose/src/acp/server/schedule.rs",
    "crates/goose/src/acp/server/tools.rs",
    "crates/goose/src/acp/server_factory.rs",
    "crates/goose/src/acp/transport/auth.rs",
    "crates/goose/src/acp/transport/mod.rs",
    "crates/goose/src/agents/agent.rs",
    "crates/goose/src/agents/extension_manager.rs",
    "crates/goose/src/agents/extension_manager/mod.rs",
    "crates/goose/src/agents/extension_manager/stdio.rs",
    "crates/goose/src/agents/extension_manager/streamable_http.rs",
    "crates/goose/src/agents/platform_extensions/mod.rs",
    "crates/goose/src/agents/state_machine/effects.rs",
    "crates/goose/src/agents/state_machine/mod.rs",
    "crates/goose/src/agents/state_machine/ops_tool_approval.rs",
    "crates/goose/src/agents/state_machine/ops_toolcalling.rs",
    "crates/goose/src/agents/state_machine/session.rs",
    "crates/goose/src/agents/state_machine/tool_confirmation.rs",
    "crates/goose/src/agents/state_machine/usage.rs",
    "crates/goose/src/agents/tool_execution.rs",
    "crates/goose/src/bin/goose-acp.rs",
    "crates/goose/src/config/base.rs",
    "crates/goose/src/config/declarative_providers.rs",
    "crates/goose/src/config/extensions.rs",
    "crates/goose/src/config/permission.rs",
    "crates/goose/src/execution/active_run.rs",
    "crates/goose/src/permission/permission_inspector.rs",
    "crates/goose/src/session/session_manager.rs",
    "crates/goose/src/session_context.rs",
];

#[derive(Clone, Debug, Eq, PartialEq)]
struct Entry {
    mode: String,
    blob: String,
}

type Manifest = BTreeMap<String, Entry>;

fn parse_manifest(body: &str, version: &str) -> Manifest {
    let mut manifest = Manifest::new();
    let mut original_paths = Vec::new();
    for line in body.lines() {
        let fields: Vec<_> = line.split('\t').collect();
        assert_eq!(
            fields.len(),
            3,
            "{version}: malformed manifest row {line:?}"
        );
        original_paths.push(fields[0]);
        assert!(
            manifest
                .insert(
                    fields[0].to_owned(),
                    Entry {
                        mode: fields[1].to_owned(),
                        blob: fields[2].to_owned(),
                    },
                )
                .is_none(),
            "{version}: duplicate path {}",
            fields[0]
        );
    }
    let sorted: Vec<_> = manifest.keys().map(String::as_str).collect();
    assert_eq!(
        original_paths, sorted,
        "{version}: manifest paths must be sorted"
    );
    manifest
}

fn manifest_digest(body: &str) -> String {
    let mut canonical = Vec::new();
    for line in body.lines() {
        let fields: Vec<_> = line.split('\t').collect();
        assert_eq!(fields.len(), 3);
        for (index, field) in fields.iter().enumerate() {
            if index > 0 {
                canonical.push(0);
            }
            canonical.extend_from_slice(field.as_bytes());
        }
        canonical.push(b'\n');
    }
    format!("{:x}", Sha256::digest(canonical))
}

fn strings(value: &Value) -> BTreeSet<String> {
    let rows = value.as_array().expect("string array");
    let values: Vec<_> = rows
        .iter()
        .map(|row| row.as_str().expect("string row").to_owned())
        .collect();
    let set: BTreeSet<_> = values.iter().cloned().collect();
    assert_eq!(
        values.len(),
        set.len(),
        "path arrays must not contain duplicates"
    );
    assert_eq!(
        values,
        set.iter().cloned().collect::<Vec<_>>(),
        "path arrays must be sorted"
    );
    set
}

fn object_keys(value: &Value) -> BTreeSet<String> {
    value.as_object().expect("object").keys().cloned().collect()
}

#[test]
fn production_claim_qualifies_every_published_point_and_keeps_the_holes() {
    let claim = goose_acp_claim();
    assert_eq!(GOOSE_RELEASE_VERSION, "1.53.0");
    assert_eq!(claim.id().as_str(), "goose.acp.release-window-1");
    assert_eq!(claim.axis().as_str(), "goose.release");
    assert_eq!(claim.baseline().as_str(), "1.50.1");
    assert_eq!(claim.latest_qualified().as_str(), "1.53.0");
    assert_eq!(claim.milestones().len(), 4);
    assert_eq!(
        claim.newer_version_posture(),
        InterfaceNewerVersionPosture::AllowUnverified
    );

    for (segment, expected) in claim
        .milestones()
        .zip(["1.50.1", "1.51.0", "1.52.0", "1.53.0"])
    {
        assert_eq!(segment.minimum().as_str(), expected);
        assert_eq!(segment.maximum().as_str(), expected);
        assert_eq!(
            segment.behavior_revision().as_str(),
            "goose.acp.stdio-v2.auth-required"
        );
        assert_eq!(segment.support_status(), InterfaceSupportStatus::Maintained);
        assert!(
            claim
                .assess(&InterfaceVersion::new(expected).expect("version"))
                .is_permitted()
        );
    }

    for gap in ["1.50.2", "1.51.1", "1.52.1"] {
        assert!(matches!(
            claim.assess(&InterfaceVersion::new(gap).expect("version")),
            InterfaceCompatibilityAssessment::Incompatible
        ));
    }
    assert!(matches!(
        claim.assess(&InterfaceVersion::new("1.53.1").expect("version")),
        InterfaceCompatibilityAssessment::UnverifiedNewer(_)
    ));

    for version in ["1.50.1", "1.51.0", "1.52.0", "1.53.0", "1.53.1"] {
        assert!(goose_release_binding(version).is_some(), "{version}");
    }
    for malformed in [
        "",
        "v1.53.0",
        "1.53",
        "1.53.0-beta",
        "1.53.0+build",
        " 1.53.0",
    ] {
        assert!(goose_release_binding(malformed).is_none(), "{malformed:?}");
    }
}

#[test]
fn exact_release_identities_and_complete_tree_diffs_are_frozen() {
    let identity: Value = serde_json::from_str(IDENTITY).expect("identity fixture");
    let inventory: Value = serde_json::from_str(INVENTORY).expect("tree inventory");
    let protocol: Value = serde_json::from_str(PROTOCOL).expect("protocol ledger");
    let versions = ["1.50.1", "1.51.0", "1.52.0", "1.53.0"];
    assert_eq!(
        identity["unpublished_interstitial_points"],
        serde_json::json!(["1.50.2", "1.51.1", "1.52.1"])
    );
    assert_eq!(identity["withdrawn_stable_points"], serde_json::json!([]));
    assert_eq!(
        identity["independently_unqualified_published_points"],
        serde_json::json!([])
    );
    assert_eq!(identity["explicit_exclusions"], serde_json::json!([]));
    let identities = identity["points"].as_object().expect("point identities");
    assert_eq!(
        identities.keys().map(String::as_str).collect::<Vec<_>>(),
        versions
    );
    assert_eq!(inventory["compared"], serde_json::json!(versions));
    assert_eq!(
        protocol["selected_source_paths"],
        serde_json::json!(EXPECTED_SELECTED_SOURCE_PATHS)
    );

    let manifests: BTreeMap<_, _> = POINTS
        .iter()
        .map(|(version, body)| (*version, parse_manifest(body, version)))
        .collect();
    for (version, body) in POINTS {
        let point = &identity["points"][version];
        assert_eq!(manifest_digest(body), point["source_manifest_sha256"]);
        assert_eq!(manifest_digest(body), inventory["manifest_sha256"][version]);
        assert_eq!(
            manifests[version].len(),
            point["source_file_count"].as_u64().unwrap() as usize
        );
        assert_eq!(
            manifests[version].len(),
            inventory["package_file_counts"][version].as_u64().unwrap() as usize
        );
        assert_eq!(point["source_commit"], inventory["source_commits"][version]);
        assert_eq!(point["root_tree"], inventory["root_tree_ids"][version]);
    }

    let expected_hops = [
        ("1.50.1_to_1.51.0", "1.50.1", "1.51.0"),
        ("1.51.0_to_1.52.0", "1.51.0", "1.52.0"),
        ("1.52.0_to_1.53.0", "1.52.0", "1.53.0"),
    ];
    assert_eq!(
        object_keys(&inventory["hops"]),
        expected_hops
            .iter()
            .map(|(hop, _, _)| (*hop).to_owned())
            .collect()
    );
    assert_eq!(
        object_keys(&protocol["hop_classifications"]),
        expected_hops
            .iter()
            .map(|(hop, _, _)| (*hop).to_owned())
            .collect()
    );

    let selected: BTreeSet<_> = EXPECTED_SELECTED_SOURCE_PATHS
        .iter()
        .map(|path| (*path).to_owned())
        .collect();
    for (hop, from, to) in expected_hops {
        let before = &manifests[from];
        let after = &manifests[to];
        let paths: BTreeSet<_> = before.keys().chain(after.keys()).cloned().collect();
        let mut added = BTreeSet::new();
        let mut removed = BTreeSet::new();
        let mut changed = BTreeSet::new();
        let mut identical = BTreeSet::new();
        for path in paths {
            match (before.get(&path), after.get(&path)) {
                (None, Some(_)) => {
                    added.insert(path);
                }
                (Some(_), None) => {
                    removed.insert(path);
                }
                (Some(before), Some(after)) if before == after => {
                    identical.insert(path);
                }
                (Some(_), Some(_)) => {
                    changed.insert(path);
                }
                (None, None) => unreachable!(),
            }
        }
        let recorded = &inventory["hops"][hop];
        assert_eq!(strings(&recorded["added"]), added, "{hop} added paths");
        assert_eq!(
            strings(&recorded["removed"]),
            removed,
            "{hop} removed paths"
        );
        assert_eq!(
            strings(&recorded["changed"]),
            changed,
            "{hop} changed paths"
        );
        assert_eq!(
            strings(&recorded["identical"]),
            identical,
            "{hop} identical paths"
        );

        let deltas = added
            .union(&removed)
            .chain(changed.iter())
            .cloned()
            .collect::<BTreeSet<_>>();
        let selected_deltas: BTreeSet<_> = deltas.intersection(&selected).cloned().collect();
        let annotations = protocol["hop_classifications"][hop]["selected_changes"]
            .as_object()
            .expect("selected path classifications");
        assert_eq!(
            annotations.keys().cloned().collect::<BTreeSet<_>>(),
            selected_deltas,
            "{hop} every changed selected source path needs one classification"
        );
        for (path, annotation) in annotations {
            assert_eq!(
                annotation["inventory_change"],
                recorded_change(path, &added, &removed, &changed)
            );
            assert!(
                !annotation["classification"]
                    .as_str()
                    .unwrap_or_default()
                    .is_empty()
            );
            assert!(
                !annotation["evidence"]
                    .as_str()
                    .unwrap_or_default()
                    .is_empty()
            );
        }

        let residual = &protocol["hop_classifications"][hop]["residual_changes"];
        let mut expected_residual: BTreeMap<&str, BTreeSet<String>> = [
            "provider_internal",
            "adjacent_acp_api",
            "agent_session_internal",
            "ui_docs_tests_workflows",
            "other_unselected_upstream",
        ]
        .into_iter()
        .map(|category| (category, BTreeSet::new()))
        .collect();
        for path in deltas.difference(&selected_deltas) {
            expected_residual
                .get_mut(residual_category(path))
                .expect("known residual category")
                .insert(path.clone());
        }
        assert_eq!(
            object_keys(residual),
            expected_residual
                .keys()
                .map(|category| (*category).to_owned())
                .collect(),
            "{hop} residual category key set"
        );
        let mut residual_paths = BTreeSet::new();
        for (class, paths) in residual.as_object().expect("residual categories") {
            assert!(
                !paths.as_array().expect("residual path array").is_empty(),
                "empty {class}"
            );
            for path in strings(paths) {
                assert!(
                    residual_paths.insert(path.clone()),
                    "{hop}: {path} classified twice"
                );
                assert!(
                    deltas.contains(&path),
                    "{hop}: residual path {path} is not a tree delta"
                );
                assert!(
                    !selected.contains(&path),
                    "{hop}: selected path {path} is residual"
                );
            }
            assert_eq!(
                strings(paths),
                expected_residual[class.as_str()],
                "{hop}: {class} exact residual path set"
            );
        }
        assert_eq!(
            residual_paths,
            deltas.difference(&selected_deltas).cloned().collect()
        );
    }
}

fn residual_category(path: &str) -> &'static str {
    if path.starts_with("crates/goose-providers/")
        || path.starts_with("crates/goose-provider-types/")
        || path.starts_with("crates/goose/src/providers/")
    {
        "provider_internal"
    } else if path.starts_with("crates/goose/src/acp/")
        || path.starts_with("crates/goose-acp-")
        || path.starts_with("ui/goose-acp")
        || matches!(
            path,
            "crates/goose/acp-meta.json" | "crates/goose/acp-schema.json"
        )
    {
        "adjacent_acp_api"
    } else if path.starts_with("crates/goose/src/agents/")
        || path.starts_with("crates/goose/src/config/")
        || path.starts_with("crates/goose/src/permission/")
        || path.starts_with("crates/goose/src/session/")
        || path.starts_with("crates/goose/src/execution/")
    {
        "agent_session_internal"
    } else if path.starts_with("ui/")
        || path.starts_with("documentation/")
        || path.starts_with("docs/")
        || path.starts_with(".github/")
        || path.starts_with("scripts/")
        || path.starts_with("crates/goose/tests/")
    {
        "ui_docs_tests_workflows"
    } else {
        "other_unselected_upstream"
    }
}

fn recorded_change(
    path: &str,
    added: &BTreeSet<String>,
    removed: &BTreeSet<String>,
    changed: &BTreeSet<String>,
) -> &'static str {
    if added.contains(path) {
        "added"
    } else if removed.contains(path) {
        "removed"
    } else if changed.contains(path) {
        "changed"
    } else {
        panic!("selected source {path} has no tree delta")
    }
}

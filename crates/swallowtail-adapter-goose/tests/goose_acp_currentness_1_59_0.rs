use serde_json::Value;
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use swallowtail_adapter_goose::{GOOSE_RELEASE_VERSION, goose_acp_claim};
use swallowtail_core::{InterfaceCompatibilityAssessment, InterfaceVersion};

const IDENTITY: &str = include_str!("fixtures/goose-acp-1.59.0/identity.json");
const INVENTORY: &str = include_str!("fixtures/goose-acp-1.59.0/dist-inventory.json");
const PROTOCOL: &str = include_str!("fixtures/goose-acp-1.59.0/protocol.json");
const PRIOR_PROTOCOL: &str = include_str!("fixtures/goose-acp-1.57.0/protocol.json");
const PRIOR_INVENTORY: &str = include_str!("fixtures/goose-acp-1.57.0/dist-inventory.json");
const PRIOR_MANIFEST_1_57_0: &str = include_str!("fixtures/goose-acp-1.57.0/manifest-1.57.0.tsv");
const MANIFEST_1_57_0: &str = include_str!("fixtures/goose-acp-1.59.0/manifest-1.57.0.tsv");
const MANIFEST_1_58_0: &str = include_str!("fixtures/goose-acp-1.59.0/manifest-1.58.0.tsv");
const MANIFEST_1_58_1: &str = include_str!("fixtures/goose-acp-1.59.0/manifest-1.58.1.tsv");
const MANIFEST_1_59_0: &str = include_str!("fixtures/goose-acp-1.59.0/manifest-1.59.0.tsv");

const NEW_SELECTED_SOURCE_PATHS: &[&str] = &[
    "crates/goose-agent/src/inference.rs",
    "crates/goose-agent/src/lib.rs",
    "crates/goose-agent/src/machine.rs",
    "crates/goose-agent/src/operation.rs",
    "crates/goose-agent/src/post_tool_call_hook.rs",
    "crates/goose-agent/src/pre_tool_call_hook.rs",
    "crates/goose-agent/src/stop_hook.rs",
    "crates/goose-agent/src/tool.rs",
    "crates/goose-agent/src/tool_approval.rs",
    "crates/goose-agent/src/tool_call_state.rs",
    "crates/goose-agent/src/unhandled_tool_calls.rs",
    "crates/goose/src/agents/active_run.rs",
    "crates/goose/src/agents/services.rs",
    "crates/goose/src/agents/session_runtime.rs",
    "crates/goose/src/agents/state_machine/proactive_compaction.rs",
    "crates/goose/src/agents/state_machine/tool_approval.rs",
];

const VERSIONS: [(&str, &str); 4] = [
    ("1.57.0", MANIFEST_1_57_0),
    ("1.58.0", MANIFEST_1_58_0),
    ("1.58.1", MANIFEST_1_58_1),
    ("1.59.0", MANIFEST_1_59_0),
];
const ALL_VERSIONS: [&str; 8] = [
    "1.53.0", "1.54.0", "1.55.0", "1.56.0", "1.57.0", "1.58.0", "1.58.1", "1.59.0",
];

const EXPECTED_RELEASE_IDENTITIES: [(&str, &str, &str, &str, u64, &str); 3] = [
    (
        "1.58.0",
        "7c039ac9f90f5d61cf2953d32baf26e7ad1219ae",
        "11348b302b875a15b4a08762c85b1945e87806b6",
        "755e4e7f683e1307c6205bd4a2f2e9642a0f2938c7b9ee31466034a2fee02136",
        50_979_507,
        "38c39a131ed8bf333d9a461fb5d96b6affeb47ad07119693c857733c6d78b119",
    ),
    (
        "1.58.1",
        "c97af00bca96442611bb95155c56d6109afa42d6",
        "82a0d5d84a79f1fc1ec7e2b17b2ef31bd05657e9",
        "00b794c79e8af93aa422408e634bb49b0d7b15be2d637c832d5e65873a723734",
        51_003_360,
        "92a0482e51c0a804b187d78cd2fc2987496fbd05d50aca30f51a80de4a258c90",
    ),
    (
        "1.59.0",
        "5d03ef6ed07ba59826328f1aff18d9a754ff8ad2",
        "fa04fd73e23ba89f43d0b7244d421bdf46a0cadd",
        "78a2db7f7b1223df3d549984d3d8fc7716e9daeded186669869596e8c8f53d81",
        51_105_916,
        "0e7d8be54aa523689ff91e764f9ee4ee5fb0a67395d9289241b9982c50987263",
    ),
];

const EXPECTED_SELECTED_CLASSIFICATION_SHA256: [(&str, &str); 3] = [
    (
        "1.57.0_to_1.58.0",
        "1ad29e6abad73ebd30cd8fe1e9af624e4e827442fd1480c6ef60f61e1f192564",
    ),
    (
        "1.58.0_to_1.58.1",
        "f229328aec2b0dcf49af3fc9621b60e92ca9fa74989f440607e551e4b1e18ad1",
    ),
    (
        "1.58.1_to_1.59.0",
        "448795c4a9b6132502d6570f4422cdf5d67f27d8e96137b8ba5829350e5e72d6",
    ),
];

type Manifest = BTreeMap<String, String>;

fn strings(value: &Value) -> BTreeSet<String> {
    let values: Vec<_> = value
        .as_array()
        .expect("string array")
        .iter()
        .map(|row| row.as_str().expect("string row").to_owned())
        .collect();
    let set: BTreeSet<_> = values.iter().cloned().collect();
    assert_eq!(values.len(), set.len(), "path arrays have no duplicates");
    assert_eq!(values, set.iter().cloned().collect::<Vec<_>>());
    set
}

fn string_set(value: &Value) -> BTreeSet<String> {
    let rows = value.as_array().expect("string array");
    let set: BTreeSet<_> = rows
        .iter()
        .map(|row| row.as_str().expect("string row").to_owned())
        .collect();
    assert_eq!(rows.len(), set.len(), "path arrays have no duplicates");
    set
}

fn object_keys(value: &Value) -> BTreeSet<String> {
    value.as_object().expect("object").keys().cloned().collect()
}

fn parse_manifest(body: &str, version: &str) -> Manifest {
    let mut manifest = Manifest::new();
    let mut paths = Vec::new();
    for line in body.lines() {
        let fields: Vec<_> = line.split('\t').collect();
        assert_eq!(fields.len(), 3, "{version}: malformed row {line:?}");
        paths.push(fields[0]);
        assert!(
            manifest
                .insert(
                    fields[0].to_owned(),
                    format!("{}\t{}", fields[1], fields[2])
                )
                .is_none()
        );
    }
    assert_eq!(
        paths,
        manifest.keys().map(String::as_str).collect::<Vec<_>>()
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

#[test]
fn official_hops_and_complete_source_deltas_are_frozen() {
    let identity: Value = serde_json::from_str(IDENTITY).expect("identity");
    let inventory: Value = serde_json::from_str(INVENTORY).expect("inventory");
    let protocol: Value = serde_json::from_str(PROTOCOL).expect("protocol");
    let prior_protocol: Value = serde_json::from_str(PRIOR_PROTOCOL).expect("prior protocol");
    let prior_inventory: Value = serde_json::from_str(PRIOR_INVENTORY).expect("prior inventory");
    assert_eq!(identity["official_latest"], "1.59.0");
    assert_eq!(identity["previous_qualified_ceiling"], "1.57.0");
    assert_eq!(identity["compared"], serde_json::json!(ALL_VERSIONS));
    assert_eq!(
        identity["published_stable_hops"],
        serde_json::json!(["1.58.0", "1.58.1", "1.59.0"])
    );
    assert_eq!(
        identity["unpublished_interstitial_points"],
        serde_json::json!(["1.57.1", "1.58.2"])
    );
    assert_eq!(identity["withdrawn_stable_points"], serde_json::json!([]));
    assert_eq!(
        identity["independently_unqualified_published_points"],
        serde_json::json!([])
    );
    assert_eq!(identity["explicit_exclusions"], serde_json::json!([]));
    assert_eq!(identity["decision"]["shape"], "compatible-extension");
    assert_eq!(identity["decision"]["baseline_remains"], "1.50.1");
    assert_eq!(
        identity["decision"]["qualified_published_points"],
        serde_json::json!([
            "1.50.1", "1.51.0", "1.52.0", "1.53.0", "1.54.0", "1.55.0", "1.56.0", "1.57.0",
            "1.58.0", "1.58.1", "1.59.0"
        ])
    );
    assert_eq!(
        identity["decision"]["first_later_stable_is_unverified_newer"],
        "1.59.1"
    );
    assert_eq!(
        identity["later_stable"]["synthetic_unverified_newer_fixture"],
        "1.59.1"
    );
    assert_eq!(inventory["compared"], serde_json::json!(ALL_VERSIONS));
    assert_eq!(
        identity["official_repository"]["canonical_name"],
        "aaif-goose/goose"
    );
    assert_eq!(
        identity["official_repository"]["github_repository_id"],
        846_698_999
    );
    assert_eq!(identity["official_repository"]["redirect_verified"], true);
    assert_eq!(
        MANIFEST_1_57_0, PRIOR_MANIFEST_1_57_0,
        "accepted 1.57.0 manifest is reused byte-for-byte"
    );

    let mut selected: BTreeSet<String> = string_set(&prior_protocol["selected_source_paths"]);
    selected.extend(
        NEW_SELECTED_SOURCE_PATHS
            .iter()
            .map(|path| (*path).to_owned()),
    );
    let selected_rows = strings(&protocol["selected_source_paths"]);
    assert_eq!(selected_rows, selected, "selected source closure is exact");
    assert_eq!(
        selected_rows.len(),
        protocol["selected_source_paths"].as_array().unwrap().len()
    );

    let manifests: BTreeMap<_, _> = VERSIONS
        .iter()
        .map(|(v, body)| (*v, parse_manifest(body, v)))
        .collect();
    for (version, body) in VERSIONS {
        assert_eq!(
            manifest_digest(body),
            identity["points"][version]["source_manifest_sha256"]
        );
        assert_eq!(manifest_digest(body), inventory["manifest_sha256"][version]);
        assert_eq!(
            manifests[version].len(),
            identity["points"][version]["source_file_count"]
                .as_u64()
                .unwrap() as usize
        );
        assert_eq!(
            manifests[version].len(),
            inventory["package_file_counts"][version].as_u64().unwrap() as usize
        );
        assert_eq!(
            identity["points"][version]["source_commit"],
            inventory["source_commits"][version]
        );
        assert_eq!(
            identity["points"][version]["root_tree"],
            inventory["root_tree_ids"][version]
        );
    }
    for (version, commit, tree, manifest, asset_bytes, asset_sha) in EXPECTED_RELEASE_IDENTITIES {
        let point = &identity["points"][version];
        assert_eq!(
            point["source_commit"], commit,
            "{version} source tag commit"
        );
        assert_eq!(point["tag_ref_commit"], commit, "{version} stable tag ref");
        assert_eq!(point["root_tree"], tree, "{version} source root tree");
        assert_eq!(
            point["source_manifest_sha256"], manifest,
            "{version} full source inventory"
        );
        assert_eq!(
            point["darwin_aarch64_asset_bytes"], asset_bytes,
            "{version} published asset size"
        );
        assert_eq!(
            point["darwin_aarch64_asset_sha256"], asset_sha,
            "{version} published asset digest"
        );
        assert_eq!(point["stable_tag"], format!("v{version}"));
    }

    let expected_hops = [
        ("1.57.0_to_1.58.0", "1.57.0", "1.58.0"),
        ("1.58.0_to_1.58.1", "1.58.0", "1.58.1"),
        ("1.58.1_to_1.59.0", "1.58.1", "1.59.0"),
    ];
    let historical_hops = [
        "1.53.0_to_1.54.0",
        "1.54.0_to_1.55.0",
        "1.55.0_to_1.56.0",
        "1.56.0_to_1.57.0",
    ];
    for hop in historical_hops {
        assert_eq!(
            protocol["hop_classifications"][hop], prior_protocol["hop_classifications"][hop],
            "accepted {hop} classifications stay unchanged"
        );
        assert_eq!(
            inventory["hops"][hop], prior_inventory["hops"][hop],
            "accepted {hop} inventory stays unchanged"
        );
    }
    let expected_keys: BTreeSet<_> = expected_hops
        .iter()
        .map(|(key, _, _)| (*key).to_owned())
        .chain(historical_hops.iter().map(|hop| (*hop).to_owned()))
        .collect();
    assert_eq!(object_keys(&inventory["hops"]), expected_keys);
    assert_eq!(object_keys(&protocol["hop_classifications"]), expected_keys);

    for (hop, from, to) in expected_hops {
        let before = &manifests[from];
        let after = &manifests[to];
        let all_paths: BTreeSet<_> = before.keys().chain(after.keys()).cloned().collect();
        let mut added = BTreeSet::new();
        let mut removed = BTreeSet::new();
        let mut changed = BTreeSet::new();
        let mut identical = BTreeSet::new();
        for path in all_paths {
            match (before.get(&path), after.get(&path)) {
                (None, Some(_)) => {
                    added.insert(path);
                }
                (Some(_), None) => {
                    removed.insert(path);
                }
                (Some(a), Some(b)) if a == b => {
                    identical.insert(path);
                }
                (Some(_), Some(_)) => {
                    changed.insert(path);
                }
                (None, None) => unreachable!(),
            }
        }
        let recorded = &inventory["hops"][hop];
        assert_eq!(strings(&recorded["added"]), added, "{hop} added");
        assert_eq!(strings(&recorded["removed"]), removed, "{hop} removed");
        assert_eq!(strings(&recorded["changed"]), changed, "{hop} changed");
        assert_eq!(
            strings(&recorded["identical"]),
            identical,
            "{hop} identical"
        );

        let deltas: BTreeSet<_> = added.union(&removed).chain(&changed).cloned().collect();
        let selected_deltas: BTreeSet<_> = deltas.intersection(&selected).cloned().collect();
        let hop_record = &protocol["hop_classifications"][hop];
        let annotations = hop_record["selected_changes"]
            .as_object()
            .expect("selected classifications");
        assert_eq!(
            annotations.keys().cloned().collect::<BTreeSet<_>>(),
            selected_deltas,
            "{hop} selected paths classified exactly once"
        );
        for (path, annotation) in annotations {
            let change = if added.contains(path) {
                "added"
            } else if removed.contains(path) {
                "removed"
            } else {
                "changed"
            };
            assert_eq!(annotation["inventory_change"], change, "{hop} {path}");
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
        let class_json =
            serde_json::to_vec(&hop_record["selected_changes"]).expect("classification JSON");
        let class_sha = format!("{:x}", Sha256::digest(class_json));
        let expected_sha = EXPECTED_SELECTED_CLASSIFICATION_SHA256
            .iter()
            .find(|(key, _)| *key == hop)
            .unwrap()
            .1;
        assert_eq!(
            class_sha, expected_sha,
            "{hop} changed classifications or evidence"
        );
        let mut expected_residual: BTreeMap<&str, BTreeSet<String>> = [
            "provider_internal",
            "adjacent_acp_api",
            "agent_session_internal",
            "ui_docs_tests_workflows",
            "other_unselected_upstream",
        ]
        .into_iter()
        .map(|key| (key, BTreeSet::new()))
        .collect();
        for path in deltas.difference(&selected_deltas) {
            expected_residual
                .get_mut(residual_category(path))
                .unwrap()
                .insert(path.clone());
        }
        let residual = hop_record["residual_changes"]
            .as_object()
            .expect("residual categories");
        assert_eq!(
            residual.keys().cloned().collect::<BTreeSet<_>>(),
            expected_residual
                .iter()
                .filter(|(_, paths)| !paths.is_empty())
                .map(|(key, _)| (*key).to_owned())
                .collect()
        );
        for (category, paths) in residual {
            assert_eq!(
                strings(paths),
                expected_residual[category.as_str()],
                "{hop} residual {category}"
            );
        }
    }

    let extension = &protocol["compatible_extension"];
    assert_eq!(extension["shape"], "compatible-extension");
    assert!(
        extension["interrupted_tool_call_recovery"]
            .as_str()
            .unwrap()
            .contains("deduplicated")
    );
    assert!(
        extension["permission_reply"]
            .as_str()
            .unwrap()
            .contains("active advertised one-shot replies")
    );
    assert_eq!(
        extension["claim"],
        "Keep baseline, claim id, behavior revision, posture, gaps, and prior points; qualify 1.58.0, 1.58.1, and 1.59.0."
    );
}

#[test]
fn current_stable_is_qualified_and_only_a_later_stable_is_unverified() {
    let claim = goose_acp_claim();
    assert_eq!(GOOSE_RELEASE_VERSION, "1.59.0");
    assert_eq!(claim.latest_qualified().as_str(), "1.59.0");
    for version in ["1.58.0", "1.58.1", "1.59.0"] {
        assert!(
            matches!(
                claim.assess(&InterfaceVersion::new(version).unwrap()),
                InterfaceCompatibilityAssessment::Qualified(_)
            ),
            "{version}"
        );
    }
    for version in ["1.57.1", "1.58.2"] {
        assert!(
            matches!(
                claim.assess(&InterfaceVersion::new(version).unwrap()),
                InterfaceCompatibilityAssessment::Incompatible
            ),
            "{version}"
        );
    }
    assert!(matches!(
        claim.assess(&InterfaceVersion::new("1.59.1").unwrap()),
        InterfaceCompatibilityAssessment::UnverifiedNewer(_)
    ));
}

use std::collections::BTreeSet;

use serde_json::Value;
use swallowtail_adapter_command_code::COMMAND_CODE_RELEASE_VERSION;
use swallowtail_core::{InterfaceCompatibilityAssessment, InterfaceVersion};

const IDENTITY: &str = include_str!("fixtures/command-code-1.79.1/identity.json");
const ARTIFACT: &str = include_str!("fixtures/command-code-1.79.1/artifact.json");
const INVENTORY: &str = include_str!("fixtures/command-code-1.79.1/dist-inventory.json");
const PROTOCOL: &str = include_str!("fixtures/command-code-1.79.1/protocol.json");
const MODEL_FIXTURE: &str = include_str!("fixtures/command-code-1.79.1/plan-model-selection.json");

const RELEASES: &[&str] = &[
    "1.65.0", "1.65.1", "1.65.2", "1.65.3", "1.65.4", "1.65.5", "1.66.0", "1.67.0", "1.68.0",
    "1.69.0", "1.70.0", "1.71.0", "1.72.0", "1.72.1", "1.72.2", "1.72.3", "1.72.4", "1.73.0",
    "1.73.1", "1.73.2", "1.73.3", "1.73.4", "1.74.0", "1.74.1", "1.74.2", "1.74.3", "1.75.0",
    "1.75.1", "1.76.0", "1.77.0", "1.78.0", "1.79.0", "1.79.1",
];

const UNPUBLISHED_PATCHES: &[&str] = &[
    "1.65.6", "1.66.1", "1.67.1", "1.68.1", "1.69.1", "1.70.1", "1.71.1", "1.72.5", "1.73.5",
    "1.74.4", "1.75.2", "1.76.1", "1.77.1", "1.78.1",
];

#[test]
fn official_identity_and_claim_cover_exact_published_points() {
    let identity: Value = serde_json::from_str(IDENTITY).expect("identity fixture");
    let artifact: Value = serde_json::from_str(ARTIFACT).expect("artifact fixture");
    let claim = swallowtail_adapter_command_code::command_code_headless_claim();

    assert_eq!(COMMAND_CODE_RELEASE_VERSION, "1.79.1");
    assert_eq!(identity["research_id"], 402);
    assert_eq!(identity["official_channel"], "npm dist-tags.latest");
    assert_eq!(identity["stable_chain"], serde_json::json!(RELEASES));
    assert_eq!(identity["published_stable_count"], RELEASES.len());
    assert_eq!(identity["first_unpublished_later_stable"], "1.79.2");
    assert_eq!(
        identity["first_unpublished_later_stable_absent_at_observation"],
        true
    );
    assert_eq!(artifact["package"], "command-code");
    assert_eq!(artifact["version"], "1.79.1");
    assert_eq!(artifact["tarball"], identity["official"]["tarball"]);
    assert_eq!(artifact["sha1"], identity["official"]["tarball_sha1"]);
    assert_eq!(artifact["sha256"], identity["official"]["tarball_sha256"]);
    assert_eq!(artifact["integrity"], identity["official"]["integrity"]);
    assert_eq!(artifact["file_count"], 72);
    assert_eq!(artifact["unpacked_size"], 4_357_918);
    assert_eq!(artifact["runtime"]["package_engine"], ">=22");
    assert_eq!(
        artifact["runtime"]["observed_worker_runtime"]["version"],
        "22.23.2"
    );
    assert_eq!(identity["source_identity"]["git_commit"], Value::Null);
    assert_eq!(claim.id().as_str(), "command-code.headless-window-1");
    assert_eq!(claim.baseline().as_str(), "1.65.0");
    assert_eq!(claim.latest_qualified().as_str(), "1.79.1");
    assert_eq!(
        claim.newer_version_posture(),
        swallowtail_core::InterfaceNewerVersionPosture::QualifiedOnly
    );
    assert_eq!(
        claim
            .milestones()
            .filter(|segment| {
                segment.support_status() == swallowtail_core::InterfaceSupportStatus::Maintained
            })
            .count(),
        RELEASES.len()
    );

    for release in RELEASES {
        let version = InterfaceVersion::new(*release).expect("release version");
        let segment = claim
            .milestones()
            .find(|segment| segment.minimum() == &version && segment.maximum() == &version)
            .expect("published exact point is retained");
        let expected = if *release < "1.73.0" {
            "command-code.agent-event-ndjson-v1"
        } else {
            "command-code.agent-event-ndjson-v1-model-selection-v2"
        };
        assert_eq!(segment.behavior_revision().as_str(), expected, "{release}");
        assert_eq!(
            segment.support_status(),
            swallowtail_core::InterfaceSupportStatus::Maintained
        );
    }
    for gap in UNPUBLISHED_PATCHES {
        assert!(
            !claim.permits(&InterfaceVersion::new(*gap).unwrap()),
            "{gap}"
        );
    }
    assert!(!claim.permits(&InterfaceVersion::new("1.64.9").unwrap()));
    assert_eq!(
        claim.assess(&InterfaceVersion::new("1.79.2").unwrap()),
        InterfaceCompatibilityAssessment::Incompatible
    );
    assert!(!claim.permits(&InterfaceVersion::new("1.79.2").unwrap()));
}

#[test]
fn complete_tree_and_every_hop_have_mutation_sensitive_ledgers() {
    let inventory: Value = serde_json::from_str(INVENTORY).expect("inventory fixture");
    let artifacts = inventory["artifacts"].as_object().expect("artifact trees");
    let compared = inventory["compared"]
        .as_array()
        .expect("ordered stable releases")
        .iter()
        .map(|version| version.as_str().expect("version text"))
        .collect::<Vec<_>>();
    assert_eq!(compared, RELEASES);
    assert_eq!(artifacts.len(), RELEASES.len());
    assert_eq!(
        inventory["package_file_counts"].as_object().unwrap().len(),
        RELEASES.len()
    );
    assert_eq!(
        inventory["package_unpacked_sizes"]
            .as_object()
            .unwrap()
            .len(),
        RELEASES.len()
    );
    assert_eq!(inventory["tree_inventory_hash"]["algorithm"], "sha256");

    let expected_paths: BTreeSet<&str> = artifacts[RELEASES[0]]["files"]
        .as_object()
        .unwrap()
        .keys()
        .map(String::as_str)
        .collect();
    assert_eq!(expected_paths.len(), 72);
    for release in RELEASES {
        let artifact = &artifacts[*release];
        let files = artifact["files"].as_object().expect("full tree hashes");
        let paths = files.keys().map(String::as_str).collect::<BTreeSet<_>>();
        assert_eq!(paths, expected_paths, "{release} path set");
        assert_eq!(files.len(), 72, "{release} file count");
        assert_eq!(inventory["package_file_counts"][*release], 72);
        assert_eq!(
            inventory["package_unpacked_sizes"][*release],
            artifact["unpacked_size"]
        );
        assert_eq!(
            inventory["tree_inventory_hash"]["per_version"][*release],
            artifact["file_inventory_sha256"]
        );
    }

    let hops = inventory["from_hop_to_hop"]
        .as_object()
        .expect("complete adjacent-hop ledger");
    assert_eq!(hops.len(), RELEASES.len() - 1);
    for pair in RELEASES.windows(2) {
        let hop_key = format!("{}..{}", pair[0], pair[1]);
        let diff = hops.get(&hop_key).expect("each adjacent stable hop");
        let before = artifacts[pair[0]]["files"].as_object().unwrap();
        let after = artifacts[pair[1]]["files"].as_object().unwrap();
        let before_paths = before.keys().map(String::as_str).collect::<BTreeSet<_>>();
        let after_paths = after.keys().map(String::as_str).collect::<BTreeSet<_>>();
        let added = after_paths
            .difference(&before_paths)
            .copied()
            .collect::<BTreeSet<_>>();
        let removed = before_paths
            .difference(&after_paths)
            .copied()
            .collect::<BTreeSet<_>>();
        let mut changed = BTreeSet::new();
        let mut identical = BTreeSet::new();
        for path in before_paths.intersection(&after_paths) {
            if before[*path] == after[*path] {
                identical.insert(*path);
            } else {
                changed.insert(*path);
            }
        }
        assert_eq!(string_set(&diff["added"]), added, "{hop_key} additions");
        assert_eq!(string_set(&diff["removed"]), removed, "{hop_key} removals");
        assert_eq!(string_set(&diff["changed"]), changed, "{hop_key} changes");
        assert_eq!(
            string_set(&diff["identical"]),
            identical,
            "{hop_key} unchanged"
        );

        let classified = diff["classification"]
            .as_object()
            .expect("every changed path has a classification");
        let expected_classified = added
            .union(&removed)
            .copied()
            .chain(changed.iter().copied())
            .collect::<BTreeSet<_>>();
        assert_eq!(
            classified
                .keys()
                .map(String::as_str)
                .collect::<BTreeSet<_>>(),
            expected_classified,
            "{hop_key} exact classification keys"
        );
        assert!(classified.values().all(|entry| {
            entry["category"]
                .as_str()
                .is_some_and(|category| !category.is_empty())
                && entry["basis"]
                    .as_str()
                    .is_some_and(|basis| !basis.is_empty())
        }));
    }
    for release in RELEASES {
        assert_eq!(
            artifacts[*release]["files"]["dist/index.mjs"],
            artifacts["1.65.0"]["files"]["dist/index.mjs"],
            "{release} selected executable"
        );
    }
}

#[test]
fn selected_model_lane_and_debug_evidence_are_bounded_and_exact() {
    let inventory: Value = serde_json::from_str(INVENTORY).expect("inventory fixture");
    let protocol: Value = serde_json::from_str(PROTOCOL).expect("protocol fixture");
    let model: Value = serde_json::from_str(MODEL_FIXTURE).expect("conflicting-model fixture");
    let markers = inventory["selected_static_markers"]
        .as_object()
        .expect("marker map");
    assert_eq!(markers.len(), RELEASES.len());
    for release in RELEASES {
        let marker = &markers[*release];
        assert_eq!(marker["model_request_start_model_field"], true, "{release}");
        assert_eq!(marker["json_event_serializer"], true, "{release}");
        let has_lane = *release >= "1.73.0";
        assert_eq!(
            marker["configured_planning_model_setting"], has_lane,
            "{release}"
        );
        assert_eq!(marker["model_lane_resolver"], has_lane, "{release}");
    }
    assert_eq!(protocol["route_id"], "command-code.headless");
    assert_eq!(protocol["axis"], "command-code.npm");
    assert_eq!(protocol["artifact_revision"], "1.79.1");
    assert_eq!(
        string_set(&protocol["mapped_debug_event_types"]),
        BTreeSet::from(["model_request_start"])
    );
    assert!(!string_set(&protocol["projected_event_types"]).contains("model_request_start"));
    assert_eq!(
        string_set(&protocol["ignored_lifecycle_event_types"]),
        BTreeSet::from([
            "message_end",
            "message_start",
            "message_update",
            "model_request_start",
            "model_trace",
            "run_end",
            "turn_end",
            "turn_start",
        ])
    );
    assert_eq!(
        protocol["model_selection_evidence"]["first_lane_release"],
        "1.73.0"
    );
    assert_eq!(
        protocol["model_selection_evidence"]["debug_observation"]["kind"],
        "InterfaceVersion"
    );
    assert_eq!(
        string_set(&protocol["model_selection_evidence"]["debug_observation"]["detail_keys"]),
        BTreeSet::from([
            "effective_model_id",
            "effective_scope",
            "effective_source",
            "requested_model_id",
        ])
    );
    assert_eq!(model["route_id"], "command-code.headless");
    assert_eq!(model["permission_mode"], "plan");
    assert_eq!(model["requested_model_id"], "fixture-model");
    assert_eq!(
        model["configured_feature_models"]["planning"],
        "configured/planning-model"
    );
    let model_request = model["stdout_ndjson"]
        .as_array()
        .unwrap()
        .iter()
        .find(|record| record["event"]["type"] == "model_request_start")
        .expect("model request start event");
    assert_eq!(
        model_request["event"]["model"],
        model["configured_feature_models"]["planning"]
    );
    assert_eq!(model["persistent_settings_write"], false);
    assert_eq!(model["provider_artifact_executed"], false);
    assert_eq!(
        protocol["model_selection_evidence"]["effective_model_scope"],
        "CLI-selected model immediately before the SDK request; it is not evidence of which remote provider/backend ultimately served the request."
    );
}

fn string_set(value: &Value) -> BTreeSet<&str> {
    value
        .as_array()
        .expect("string array")
        .iter()
        .map(|item| item.as_str().expect("string value"))
        .collect()
}

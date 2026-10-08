use serde_json::Value;
use swallowtail_adapter_ollama::{
    OLLAMA_BASELINE_VERSION, OLLAMA_LATEST_QUALIFIED_VERSION, ollama_runtime_binding,
    ollama_runtime_claim,
    protocol::{ObservationBinding, Response, parse_model_detail},
};
use swallowtail_core::{
    AttachedModelTag, CatalogTimestamp, ConfiguredInstanceId, InterfaceCompatibilityAssessment,
    InterfaceSupportStatus, InterfaceVersion, ModelManifestDigest,
};

const IDENTITY: &str = include_str!("fixtures/ollama-0.40.0/identity.json");
const PROTOCOL: &str = include_str!("fixtures/ollama-0.40.0/protocol.json");
const INVENTORY: &str = include_str!("fixtures/ollama-0.40.0/dist-inventory.json");

#[test]
fn identity_freezes_official_hops_and_complete_trees_before_the_0400_stop() {
    let identity: Value = serde_json::from_str(IDENTITY).expect("identity corpus parses");
    let inventory: Value = serde_json::from_str(INVENTORY).expect("tree inventory parses");

    assert_eq!(identity["axis"], "ollama.runtime");
    assert_eq!(identity["package"], "swallowtail-adapter-ollama");
    assert_eq!(identity["host_observation"]["client_version"], "0.33.3");
    assert_eq!(identity["host_observation"]["no_runtime_reachable"], true);
    assert_eq!(identity["host_observation"]["host_not_mutated"], true);
    assert_eq!(identity["selected_channel"]["latest_version"], "0.40.0");
    assert_eq!(identity["selected_channel"]["draft"], false);
    assert_eq!(identity["selected_channel"]["prerelease"], false);
    assert_eq!(
        identity["selected_channel"]["tag_commit"],
        "0d0720e51fb2fd9aa58781c3d720c06d720c2e7b"
    );
    assert_eq!(
        identity["selected_channel"]["tree"],
        "d65785382b4d8b6dd98b25b09c59a4ec5b22d168"
    );
    assert_eq!(
        identity["selected_channel"]["source_archive_sha256"],
        "9fe3b69d3e539bd0168dfc1cbb7af9f15bac654d31744f96560f13f69672516f"
    );
    assert!(identity["selected_channel"]["timestamp_note"].is_string());
    assert_eq!(
        identity["matching_unreleased_prerelease_tag"],
        "v0.40.1-rc0"
    );
    assert_eq!(
        identity["published_stables_after_previous_ceiling"]
            .as_array()
            .unwrap()
            .iter()
            .map(|release| release["version"].as_str().unwrap())
            .collect::<Vec<_>>(),
        ["0.35.0", "0.35.1", "0.40.0"]
    );
    for release in identity["published_stables_after_previous_ceiling"]
        .as_array()
        .unwrap()
    {
        let version = release["version"].as_str().unwrap();
        let point = inventory["points"]
            .as_array()
            .unwrap()
            .iter()
            .find(|point| point["version"] == version)
            .unwrap_or_else(|| panic!("tree inventory omits release {version}"));
        assert_eq!(release["commit"], point["commit"], "commit at {version}");
        assert_eq!(release["tree"], point["tree"], "tree at {version}");
        assert_eq!(
            release["source_archive_sha256"], point["source_archive_sha256"],
            "archive digest at {version}"
        );
    }

    assert_eq!(inventory["schema"], 1);
    assert_eq!(
        inventory["points"]
            .as_array()
            .unwrap()
            .iter()
            .map(|point| point["version"].as_str().unwrap())
            .collect::<Vec<_>>(),
        ["0.34.4", "0.35.0", "0.35.1", "0.40.0"]
    );
    for (point, expected_files, expected_entries, expected_tree) in [
        (
            "0.34.4",
            1310,
            1492,
            "525c37066242cc8feb9f7d909927270b168c6a7c",
        ),
        (
            "0.35.0",
            1327,
            1510,
            "d663e3c722adf4ddde99352c02d3ffee583cab65",
        ),
        (
            "0.35.1",
            1335,
            1519,
            "e9516fd9467dfe72505fb2abde9b26e0c5652a01",
        ),
        (
            "0.40.0",
            1431,
            1621,
            "d65785382b4d8b6dd98b25b09c59a4ec5b22d168",
        ),
    ] {
        let entry = inventory["points"]
            .as_array()
            .unwrap()
            .iter()
            .find(|entry| entry["version"] == point)
            .unwrap_or_else(|| panic!("inventory omits {point}"));
        let entries = entry["entries"]
            .as_array()
            .expect("tree entries are complete");
        assert_eq!(entries.len(), expected_entries, "entry count at {point}");
        assert_eq!(entry["tree_entry_count"], expected_entries);
        assert_eq!(entry["file_count"], expected_files);
        assert_eq!(entry["tree"], expected_tree);
        assert_eq!(
            entries.iter().filter(|row| row[2] == "blob").count(),
            expected_files
        );
        let paths: Vec<_> = entries
            .iter()
            .map(|row| row[0].as_str().expect("tree path is text"))
            .collect();
        let mut sorted = paths.clone();
        sorted.sort_unstable();
        assert_eq!(paths, sorted, "tree inventory order at {point}");
        assert_eq!(
            paths
                .iter()
                .collect::<std::collections::BTreeSet<_>>()
                .len(),
            paths.len()
        );
    }

    for (from, to, added, removed, changed, unchanged) in [
        ("0.34.4", "0.35.0", 25, 8, 41, 1261),
        ("0.35.0", "0.35.1", 8, 0, 38, 1289),
        ("0.35.1", "0.40.0", 96, 0, 110, 1225),
    ] {
        let hop = inventory["version_ordered_hops"]
            .as_array()
            .unwrap()
            .iter()
            .find(|hop| hop["from"] == from && hop["to"] == to)
            .unwrap_or_else(|| panic!("missing source hop {from} to {to}"));
        assert_eq!(hop["added_file_count"], added);
        assert_eq!(hop["removed_file_count"], removed);
        assert_eq!(hop["changed_file_count"], changed);
        assert_eq!(hop["unchanged_file_count"], unchanged);
        assert_eq!(hop["added"].as_array().unwrap().len(), added);
        assert_eq!(hop["removed"].as_array().unwrap().len(), removed);
        assert_eq!(hop["changed"].as_array().unwrap().len(), changed);
        assert!(hop["changed"].as_array().unwrap().iter().all(|file| {
            file["path"].is_string()
                && file["before_blob"].is_string()
                && file["after_blob"].is_string()
                && file["classification"].is_string()
                && file["reason"].is_string()
        }));
    }

    assert_eq!(
        identity["decision"]["shape"],
        "compatible-extension-through-0.35.1-with-0.40.0-stop"
    );
    assert_eq!(
        identity["decision"]["qualified_hops"],
        serde_json::json!(["0.35.0", "0.35.1"])
    );
    assert!(identity["decision"]["operator_ruling_needed"].is_string());
    assert_eq!(identity["decision"]["provider_prompt_sent"], false);
    assert_eq!(identity["decision"]["attached_server_started"], false);
    assert_eq!(identity["decision"]["model_downloaded"], false);
    assert_eq!(identity["decision"]["live_inference_performed"], false);
}

#[test]
fn claim_adds_only_qualified_stable_points_and_keeps_unpublished_gaps() {
    assert_eq!(OLLAMA_BASELINE_VERSION, "0.14.0");
    assert_eq!(OLLAMA_LATEST_QUALIFIED_VERSION, "0.35.1");
    let claim = ollama_runtime_claim();

    for version in ["0.14.0", "0.34.4", "0.35.0", "0.35.1"] {
        assert!(matches!(
            claim.assess(&InterfaceVersion::new(version).unwrap()),
            InterfaceCompatibilityAssessment::Qualified(point)
                if point.support_status() == InterfaceSupportStatus::Maintained
                    && point.behavior_revision().as_str() == "ollama.native-text-v1"
        ));
    }
    for version in ["0.32.2", "0.32.10", "0.34.5"] {
        assert!(!claim.permits(&InterfaceVersion::new(version).unwrap()));
    }
    for version in ["0.35.2", "0.39.0", "0.40.0"] {
        assert!(matches!(
            claim.assess(&InterfaceVersion::new(version).unwrap()),
            InterfaceCompatibilityAssessment::UnverifiedNewer(_)
        ));
    }
    assert_eq!(claim.id().as_str(), "ollama.native-runtime-window-2");
}

#[test]
fn decision_capability_stays_closed_and_additive_runner_fields_stay_unmapped() {
    let decision = parse_model_detail(
        &response(
            br#"{"capabilities":["decision"],"details":{"format":"gguf","family":"fixture"}}"#,
        ),
        &observation_binding(),
        model_tag(),
        digest(),
    )
    .expect_err("decision-only models stay outside the text capability mapping");
    assert_eq!(
        decision.diagnostic().code(),
        "swallowtail.ollama.semantics_unsupported"
    );

    let ordinary = parse_model_detail(
        &response(
            br#"{"capabilities":["completion"],"details":{"format":"gguf","family":"fixture"},"runner":"mlx","manifests":[{"runner":"mlx"}]}"#,
        ),
        &observation_binding(),
        model_tag(),
        digest(),
    )
    .expect("additive response fields stay outside the selected detail mapping");
    assert!(
        ordinary.supports(swallowtail_adapter_ollama::protocol::OllamaModelCapability::Completion)
    );
}

#[test]
fn current_official_stop_and_adapter_request_shape_are_frozen() {
    let protocol: Value = serde_json::from_str(PROTOCOL).expect("protocol corpus parses");
    assert_eq!(protocol["unverified_current_stable"], "0.40.0");
    assert_eq!(
        protocol["selected_routes_registered_at_all_four_frozen_points"],
        true
    );
    assert_eq!(
        protocol["selected_request_shapes"]["runner_selector_sent"],
        false
    );
    assert_eq!(
        protocol["selected_request_shapes"]["all_manifests_sent"],
        false
    );
    assert_eq!(protocol["manifest_list_and_runner_mapping_added"], false);
    assert_eq!(protocol["new_public_swallowtail_operation"], false);
    assert_eq!(protocol["new_public_swallowtail_capability"], false);
    assert_eq!(
        protocol["selected_handler_hashes"]["chat_handler"]["v0.40.0"],
        "aa088d54ff7bb800ac3e28430c3292b82588d6b0b21b7222653977c6343e2756"
    );
    assert_eq!(
        protocol["selected_handler_hashes"]["chat_handler"]["v0.34.4_through_v0.35.1"],
        "12314badb6cb4bc0d91459dc48d0a065d32d636c8724d750d811ee51b713adf4"
    );
    assert!(
        protocol["selected_hop_classification"]["0.35.1_to_0.40.0"]["selected_chat_lifecycle"]
            .as_str()
            .unwrap()
            .contains("manifest-list")
    );

    let inventory: Value = serde_json::from_str(INVENTORY).expect("inventory parses");
    let latest = inventory["selected_path_fingerprints"]
        .as_array()
        .unwrap()
        .iter()
        .find(|point| point["version"] == "0.40.0")
        .unwrap();
    assert!(latest["files"]["server/routes.go"]["blob"].is_string());
    assert!(latest["files"]["compatmigrate/migrate.go"]["blob"].is_string());
    assert!(latest["files"]["compatmigrate/manifest.go"]["blob"].is_string());
}

fn response(body: &[u8]) -> Response {
    Response {
        status: 200,
        body: body.to_vec(),
    }
}

fn model_tag() -> AttachedModelTag {
    AttachedModelTag::new("fixture-model:8b").expect("tag is valid")
}

fn digest() -> ModelManifestDigest {
    ModelManifestDigest::new(
        "sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
    )
    .expect("digest is valid")
}

fn observation_binding() -> ObservationBinding {
    ObservationBinding {
        instance_id: ConfiguredInstanceId::new("fixture.ollama").expect("instance id is valid"),
        execution_host_id: swallowtail_core::ExecutionHostId::new("fixture.host")
            .expect("host id is valid"),
        runtime_version: ollama_runtime_binding("0.35.1").expect("version binds"),
        observed_at: CatalogTimestamp::new(1_700_000_000, 0).expect("timestamp is valid"),
    }
}

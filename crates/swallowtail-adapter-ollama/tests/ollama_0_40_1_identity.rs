use serde_json::Value;
use swallowtail_adapter_ollama::{
    OLLAMA_BASELINE_VERSION, OLLAMA_LATEST_QUALIFIED_VERSION, ollama_runtime_binding,
    ollama_runtime_claim,
    protocol::{ObservationBinding, Response, parse_inventory, parse_model_detail},
};
use swallowtail_core::{
    AttachedModelObservationScope, AttachedModelTag, CatalogTimestamp, ConfiguredInstanceId,
    InterfaceCompatibilityAssessment, InterfaceSupportStatus, InterfaceVersion,
    ModelManifestDigest,
};

const IDENTITY: &str = include_str!("fixtures/ollama-0.40.1/identity.json");
const PROTOCOL: &str = include_str!("fixtures/ollama-0.40.1/protocol.json");
const INVENTORY: &str = include_str!("fixtures/ollama-0.40.1/dist-inventory.json");
const HISTORICAL: &str = include_str!("fixtures/ollama-0.40.0/identity.json");

#[test]
fn identity_freezes_official_hops_from_the_0351_ceiling_through_0401() {
    let identity: Value = serde_json::from_str(IDENTITY).expect("identity corpus parses");
    let inventory: Value = serde_json::from_str(INVENTORY).expect("tree inventory parses");
    let historical: Value = serde_json::from_str(HISTORICAL).expect("historical stop parses");

    assert_eq!(identity["axis"], "ollama.runtime");
    assert_eq!(identity["package"], "swallowtail-adapter-ollama");
    assert_eq!(identity["host_observation"]["no_runtime_reachable"], true);
    assert_eq!(identity["host_observation"]["host_not_mutated"], true);
    assert_eq!(identity["selected_channel"]["latest_version"], "0.40.1");
    assert_eq!(identity["selected_channel"]["draft"], false);
    assert_eq!(identity["selected_channel"]["prerelease"], false);
    assert_eq!(
        identity["selected_channel"]["tag_commit"],
        "cf2a313a298066d572c36812e5ad30a21c0db13b"
    );
    assert_eq!(
        identity["selected_channel"]["tree"],
        "6ae219abaf2a32dc26c8dc14d71357b92532dec2"
    );
    assert_eq!(
        identity["selected_channel"]["source_archive_sha256"],
        "675cfca761a964d3b40d1331c006fd8a630bdb0b5a9b605ace3494e7d47c3e88"
    );
    assert_eq!(
        identity["published_stables_after_previous_ceiling"]
            .as_array()
            .unwrap()
            .iter()
            .map(|release| release["version"].as_str().unwrap())
            .collect::<Vec<_>>(),
        ["0.40.0", "0.40.1"]
    );
    assert_eq!(
        historical["decision"]["shape"],
        "compatible-extension-through-0.35.1-with-0.40.0-stop"
    );
    assert_eq!(historical["selected_channel"]["latest_version"], "0.40.0");

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

    assert_eq!(
        inventory["points"]
            .as_array()
            .unwrap()
            .iter()
            .map(|point| point["version"].as_str().unwrap())
            .collect::<Vec<_>>(),
        ["0.40.0", "0.40.1"]
    );
    for (point, expected_files, expected_entries, expected_tree) in [
        (
            "0.40.0",
            1431,
            1621,
            "d65785382b4d8b6dd98b25b09c59a4ec5b22d168",
        ),
        (
            "0.40.1",
            1433,
            1622,
            "6ae219abaf2a32dc26c8dc14d71357b92532dec2",
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
    }

    let hop = inventory["version_ordered_hops"]
        .as_array()
        .unwrap()
        .iter()
        .find(|hop| hop["from"] == "0.40.0" && hop["to"] == "0.40.1")
        .expect("missing 0.40.0 to 0.40.1 hop");
    assert_eq!(hop["added_file_count"], 3);
    assert_eq!(hop["removed_file_count"], 1);
    assert_eq!(hop["changed_file_count"], 17);
    assert_eq!(hop["unchanged_file_count"], 1413);
}

#[test]
fn claim_qualifies_0400_and_0401_and_keeps_interior_gaps() {
    assert_eq!(OLLAMA_BASELINE_VERSION, "0.14.0");
    assert_eq!(OLLAMA_LATEST_QUALIFIED_VERSION, "0.40.1");
    let claim = ollama_runtime_claim();
    for version in ["0.14.0", "0.34.4", "0.35.0", "0.35.1", "0.40.0", "0.40.1"] {
        assert!(matches!(
            claim.assess(&InterfaceVersion::new(version).unwrap()),
            InterfaceCompatibilityAssessment::Qualified(point)
                if point.support_status() == InterfaceSupportStatus::Maintained
                    && point.behavior_revision().as_str() == "ollama.native-text-v1"
        ));
    }
    for version in ["0.32.2", "0.32.10", "0.34.5", "0.35.2", "0.39.0"] {
        assert!(!claim.permits(&InterfaceVersion::new(version).unwrap()));
    }
    assert!(matches!(
        claim.assess(&InterfaceVersion::new("0.41.0").unwrap()),
        InterfaceCompatibilityAssessment::UnverifiedNewer(_)
    ));
    assert_eq!(claim.id().as_str(), "ollama.native-runtime-window-2");
}

#[test]
fn selected_path_blobs_and_request_shape_are_frozen() {
    let protocol: Value = serde_json::from_str(PROTOCOL).expect("protocol corpus parses");
    assert_eq!(
        protocol["compatible_points"],
        serde_json::json!(["0.40.0", "0.40.1"])
    );
    assert_eq!(protocol["unverified_newer_synthetic"], "0.41.0");
    assert_eq!(protocol["manifest_list_and_runner_mapping_added"], true);
    assert_eq!(
        protocol["selected_request_shapes"]["runner_selector_sent_when_matching_row_has_runner"],
        true
    );
    assert_eq!(
        protocol["selected_request_shapes"]["runner_selector_omitted_when_row_has_no_runner"],
        true
    );
    assert_eq!(
        protocol["selected_request_shapes"]["all_manifests_sent"],
        false
    );
    assert_eq!(protocol["new_public_swallowtail_operation"], false);
    assert_eq!(protocol["public_runner_type_exported"], false);
    assert_eq!(
        protocol["selected_path_blobs"]["api/types.go_0.40.0_and_0.40.1"],
        "e19de536fdaf350af615f24b52a2af9fedf44e54"
    );
    assert_eq!(
        protocol["selected_path_blobs"]["server/routes.go"]["v0.40.1"],
        "addc6c189b2455554a00b1a08f906a04945b4551"
    );

    let inventory: Value = serde_json::from_str(INVENTORY).expect("inventory parses");
    for version in ["0.40.0", "0.40.1"] {
        let point = inventory["selected_path_fingerprints"]
            .as_array()
            .unwrap()
            .iter()
            .find(|point| point["version"] == version)
            .unwrap_or_else(|| panic!("missing fingerprints at {version}"));
        assert_eq!(
            point["files"]["api/types.go"]["blob"],
            "e19de536fdaf350af615f24b52a2af9fedf44e54"
        );
        assert_eq!(
            point["files"]["compatmigrate/migrate.go"]["blob"],
            "d6a5ce939218e4f6092b30322cb39b1fc9dad35e"
        );
    }
}

#[test]
fn http_fixtures_cover_manifest_list_drift_and_unmapped_siblings() {
    let binding = ObservationBinding {
        instance_id: ConfiguredInstanceId::new("fixture.ollama").expect("instance id is valid"),
        execution_host_id: swallowtail_core::ExecutionHostId::new("fixture.host")
            .expect("host id is valid"),
        runtime_version: ollama_runtime_binding("0.40.1").expect("version binds"),
        observed_at: CatalogTimestamp::new(1_700_000_000, 0).expect("timestamp is valid"),
    };
    let installed = parse_inventory(
        &response(include_bytes!(
            "fixtures/ollama-0.40.1/tags-manifest-list.json"
        )),
        AttachedModelObservationScope::InstalledInventory,
        &binding,
    )
    .expect("manifest list maps both gguf rows");
    assert_eq!(installed.len(), 2);
    assert_eq!(installed[0].model_tag().as_str(), "fixture-model:8b");
    assert_eq!(
        installed[0].manifest_digest().unwrap().as_str(),
        "sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
    );
    assert_eq!(
        installed[1].manifest_digest().unwrap().as_str(),
        "sha256:bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb"
    );

    let unmapped = parse_inventory(
        &response(include_bytes!(
            "fixtures/ollama-0.40.1/tags-unmapped-sibling.json"
        )),
        AttachedModelObservationScope::InstalledInventory,
        &binding,
    )
    .expect("mlx sibling is skipped");
    assert_eq!(unmapped.len(), 1);

    let decision = parse_model_detail(
        &response(
            br#"{"capabilities":["decision"],"details":{"format":"gguf","family":"fixture"}}"#,
        ),
        &binding,
        AttachedModelTag::new("fixture-model:8b").expect("tag is valid"),
        ModelManifestDigest::new(
            "sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        )
        .expect("digest is valid"),
    )
    .expect_err("decision-only models stay outside the text capability mapping");
    assert_eq!(
        decision.diagnostic().code(),
        "swallowtail.ollama.semantics_unsupported"
    );
}

fn response(body: &[u8]) -> Response {
    Response {
        status: 200,
        body: body.to_vec(),
    }
}

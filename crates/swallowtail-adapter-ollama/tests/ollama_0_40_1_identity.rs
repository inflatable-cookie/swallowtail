use serde_json::Value;
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
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
    assert_eq!(
        hop_path_class_blob(&hop["added"], "blob"),
        BTreeSet::from([
            (
                "docs/api/balance.mdx",
                "unselected-cloud-path",
                "2d6c40faa05e3b07b788afaa3937560ad7af5a7f",
            ),
            (
                "docs/api/cloud-usage.mdx",
                "unselected-cloud-path",
                "9fd3cf9b01fd013876cf05a8fc1c11e4bf38c4a4",
            ),
            (
                "server/routes_reporting_test.go",
                "unselected-cloud-path",
                "a420ebbb50f5d99025ab8bb10255b9b33caf3578",
            ),
        ])
    );
    assert_eq!(
        hop_path_class_blob(&hop["removed"], "blob"),
        BTreeSet::from([(
            "mlx/compat/mlx/0001-metal-residency-refresh.patch",
            "unmapped-runner-or-model-family",
            "42ce5299994c7920351d78bf25d4ea7fcfc086f2",
        )])
    );
    assert_eq!(
        hop_changed_rows(&hop["changed"]),
        BTreeSet::from([
            (
                "MLX_VERSION",
                "unmapped-runner-or-model-family",
                "36bc0faa37a70b50d7a352c72b60026de0374285",
                "0bbe3060cda31c15bd641bf8dc7183e7f8767f6f",
            ),
            (
                "README.md",
                "unselected-provider-source",
                "e511fbe3fd3e73fc6d7cfb5393d827cf70f0a2f6",
                "ae6ff10f7e67ae41998e27e80f08647441a5502b",
            ),
            (
                "app/README.md",
                "unselected-provider-source",
                "95afcace6ffa048b07a8a6569e8dd43906197176",
                "756ea2b12cc43589e752a5be913219fc7a9f45bb",
            ),
            (
                "cmd/cmd.go",
                "unselected-provider-source",
                "f316a430de2a66d25ed1e11480af32ee737c9762",
                "9cd6186131c4ce9e320894c649184eaf347e9363",
            ),
            (
                "cmd/tui/welcome.go",
                "unselected-provider-source",
                "b08e5447ad33478339a9721658e6f280e4c9d9eb",
                "071133969eccdb993a00734cfce3b5d0ea68dac2",
            ),
            (
                "cmd/tui/welcome_test.go",
                "unselected-provider-source",
                "38e3fdce5f3578bc0e60c76c65ad21e76f0a2d66",
                "63160c560c03e3d118e806e3cf88f38d7732082e",
            ),
            (
                "cmd/welcome.go",
                "unselected-provider-source",
                "4c015de38ec594f908d005174a79290676b75111",
                "4d7a8296b4d30557021a44d467bba54ad4d3948d",
            ),
            (
                "cmd/welcome_test.go",
                "unselected-provider-source",
                "641d5852620abc5967ca6246f9769d3083ca7042",
                "540db63433f52f891baf7b40382ac44a59025a47",
            ),
            (
                "docs/docs.json",
                "unselected-provider-source",
                "44a655bcd9eeeddbdad3aa817ebac3aecc520c40",
                "ed96b71673618b3cce1e763a7148f656d019ebea",
            ),
            (
                "docs/openapi.yaml",
                "unselected-provider-source",
                "c66ffaa408f804ef6620d767c6c76a5f6b6ab248",
                "675c8bb59da99c32aabee4900f358ac074999c16",
            ),
            (
                "llama/clef/clef.cpp",
                "unselected-provider-source",
                "f5ad2e6febe8a749bb317e3d1be6d6fe2a10507c",
                "ef3b251c9b403a5091141781a0106911150868ed",
            ),
            (
                "manifest/manifest.go",
                "selected-store-lock-and-windows-copy",
                "61c808324cf40a5da23baa9fe259cd3af8cb4e43",
                "f476743b4f1c9367707d8438fd048df8510a8cf7",
            ),
            (
                "manifest/manifest_test.go",
                "unselected-provider-source",
                "0fa72b5155837d4008bf32dcd9f0d7c4ac0c6dc6",
                "1c54c1dda3d8da5373378f15096193f48702060f",
            ),
            (
                "manifest/paths.go",
                "selected-store-path-windows-copy",
                "79af2e7f4baf0eed9df71af25c1e6aef8565a388",
                "cf3bfa2569bf1a3fbc69ef12455566e259515a2d",
            ),
            (
                "server/cloud_proxy.go",
                "unselected-cloud-path",
                "922df796ce86af98a00ca9715842d0470fe1d697",
                "51db28d0d9763ae5e14a39e47fdea5f9ed31e95f",
            ),
            (
                "server/cloud_proxy_test.go",
                "unselected-cloud-path",
                "950ec2bc2f732113c71a76d764ee7d2062ef7e64",
                "05788bad4e33d5341d1479bf0b6e386d1714bbad",
            ),
            (
                "server/routes.go",
                "unselected-cloud-balance-and-usage-routes",
                "90ed6789a49f58221cefaa6b9c1a92347b3aba48",
                "addc6c189b2455554a00b1a08f906a04945b4551",
            ),
        ])
    );

    let before = blob_map(&inventory, "0.40.0");
    let after = blob_map(&inventory, "0.40.1");
    let derived_added = after
        .keys()
        .filter(|path| !before.contains_key(*path))
        .cloned()
        .collect::<BTreeSet<_>>();
    let derived_removed = before
        .keys()
        .filter(|path| !after.contains_key(*path))
        .cloned()
        .collect::<BTreeSet<_>>();
    let derived_changed = before
        .iter()
        .filter(|(path, blob)| {
            after
                .get(path.as_str())
                .is_some_and(|after_blob| after_blob != *blob)
        })
        .map(|(path, _)| path.clone())
        .collect::<BTreeSet<_>>();
    let derived_unchanged = before
        .iter()
        .filter(|(path, blob)| after.get(path.as_str()) == Some(*blob))
        .map(|(path, _)| path.clone())
        .collect::<BTreeSet<_>>();
    assert_eq!(
        hop_paths(&hop["added"]),
        derived_added,
        "added paths must match the complete 0.40.0/0.40.1 trees"
    );
    assert_eq!(
        hop_paths(&hop["removed"]),
        derived_removed,
        "removed paths must match the complete 0.40.0/0.40.1 trees"
    );
    assert_eq!(
        hop_paths(&hop["changed"]),
        derived_changed,
        "changed paths must match the complete 0.40.0/0.40.1 trees"
    );
    assert_eq!(derived_unchanged.len(), 1413);
    assert_eq!(
        hop["unchanged_path_list_sha256"],
        path_list_sha256(&derived_unchanged),
        "unchanged paths must match the frozen hop digest"
    );
    assert_eq!(hop["added_file_count"], derived_added.len());
    assert_eq!(hop["removed_file_count"], derived_removed.len());
    assert_eq!(hop["changed_file_count"], derived_changed.len());
    assert_eq!(hop["unchanged_file_count"], derived_unchanged.len());
    assert!(derived_unchanged.is_disjoint(&derived_added));
    assert!(derived_unchanged.is_disjoint(&derived_removed));
    assert!(derived_unchanged.is_disjoint(&derived_changed));
    assert_eq!(
        before.len(),
        derived_removed.len() + derived_changed.len() + derived_unchanged.len()
    );
    assert_eq!(
        after.len(),
        derived_added.len() + derived_changed.len() + derived_unchanged.len()
    );
}

#[test]
fn claim_qualifies_0400_and_0401_and_keeps_interior_gaps() {
    assert_eq!(OLLAMA_BASELINE_VERSION, "0.14.0");
    assert_eq!(OLLAMA_LATEST_QUALIFIED_VERSION, "0.40.1");
    let identity: Value = serde_json::from_str(IDENTITY).expect("identity corpus parses");
    assert_eq!(
        identity["decision"]["milestone_behavior_revision"],
        "ollama.native-text-v1.manifest-list-runner"
    );
    assert_eq!(
        identity["decision"]["retain_prior_behavior_revision"],
        "ollama.native-text-v1"
    );
    let claim = ollama_runtime_claim();
    for version in ["0.14.0", "0.34.4", "0.35.0", "0.35.1"] {
        assert!(matches!(
            claim.assess(&InterfaceVersion::new(version).unwrap()),
            InterfaceCompatibilityAssessment::Qualified(point)
                if point.support_status() == InterfaceSupportStatus::Deprecated
                    && point.behavior_revision().as_str() == "ollama.native-text-v1"
        ));
    }
    for version in ["0.40.0", "0.40.1"] {
        assert!(matches!(
            claim.assess(&InterfaceVersion::new(version).unwrap()),
            InterfaceCompatibilityAssessment::Qualified(point)
                if point.support_status() == InterfaceSupportStatus::Maintained
                    && point.behavior_revision().as_str()
                        == "ollama.native-text-v1.manifest-list-runner"
        ));
    }
    for version in ["0.32.2", "0.32.10", "0.34.5", "0.35.2", "0.39.0"] {
        assert!(!claim.permits(&InterfaceVersion::new(version).unwrap()));
    }
    assert!(matches!(
        claim.assess(&InterfaceVersion::new("0.41.0").unwrap()),
        InterfaceCompatibilityAssessment::UnverifiedNewer(newer)
            if newer.behavior_revision().as_str()
                == "ollama.native-text-v1.manifest-list-runner"
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
    assert_eq!(
        protocol["behavior_revision"],
        "ollama.native-text-v1.manifest-list-runner"
    );
    assert_eq!(protocol["prior_behavior_revision"], "ollama.native-text-v1");
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

    let empty_family = parse_inventory(
        &response(include_bytes!(
            "fixtures/ollama-0.40.1/tags-unrelated-empty-family.json"
        )),
        AttachedModelObservationScope::InstalledInventory,
        &binding,
    )
    .expect("unrelated empty-family GGUF row does not fail the catalogue");
    assert_eq!(empty_family.len(), 1);
    assert_eq!(empty_family[0].model_tag().as_str(), "fixture-model:8b");
    assert_eq!(
        empty_family[0].manifest_digest().unwrap().as_str(),
        "sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
    );

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

fn blob_map(inventory: &Value, version: &str) -> BTreeMap<String, String> {
    inventory["points"]
        .as_array()
        .unwrap()
        .iter()
        .find(|point| point["version"] == version)
        .unwrap_or_else(|| panic!("inventory omits {version}"))["entries"]
        .as_array()
        .expect("tree entries are complete")
        .iter()
        .filter(|row| row[2] == "blob")
        .map(|row| {
            (
                row[0].as_str().expect("path is text").to_owned(),
                row[3].as_str().expect("blob is text").to_owned(),
            )
        })
        .collect()
}

fn hop_paths(rows: &Value) -> BTreeSet<String> {
    rows.as_array()
        .expect("hop rows are an array")
        .iter()
        .map(|row| row["path"].as_str().expect("path is text").to_owned())
        .collect()
}

fn path_list_sha256(paths: &BTreeSet<String>) -> String {
    let mut bytes = paths
        .iter()
        .map(String::as_str)
        .collect::<Vec<_>>()
        .join("\n")
        .into_bytes();
    bytes.push(b'\n');
    Sha256::digest(bytes)
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

fn hop_path_class_blob<'a>(
    rows: &'a Value,
    blob_key: &str,
) -> BTreeSet<(&'a str, &'a str, &'a str)> {
    rows.as_array()
        .expect("hop rows are an array")
        .iter()
        .map(|row| {
            (
                row["path"].as_str().expect("path is text"),
                row["classification"]
                    .as_str()
                    .expect("classification is text"),
                row[blob_key].as_str().expect("blob is text"),
            )
        })
        .collect()
}

fn hop_changed_rows(rows: &Value) -> BTreeSet<(&str, &str, &str, &str)> {
    rows.as_array()
        .expect("changed rows are an array")
        .iter()
        .map(|row| {
            (
                row["path"].as_str().expect("path is text"),
                row["classification"]
                    .as_str()
                    .expect("classification is text"),
                row["before_blob"].as_str().expect("before blob is text"),
                row["after_blob"].as_str().expect("after blob is text"),
            )
        })
        .collect()
}

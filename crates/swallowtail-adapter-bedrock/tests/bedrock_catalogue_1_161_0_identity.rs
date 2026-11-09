use serde_json::Value;
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};

const IDENTITY: &str = include_str!("fixtures/bedrock-control-plane-1.161.0/identity.json");
const PROTOCOL: &str = include_str!("fixtures/bedrock-control-plane-1.161.0/protocol.json");
const DIST_INVENTORY: &str =
    include_str!("fixtures/bedrock-control-plane-1.161.0/dist-inventory.json");
const IDENTITY_SHA256: &str = "ae32e36da0e27a9533f8af8c48d91dba021dfbc42f0c860f343e3dfb542a0862";
const PROTOCOL_SHA256: &str = "582ea3a3db7874cbf46046b52a2c6a2c3bf8d207cdca77f73b47cadf1fc339b5";
const DIST_INVENTORY_SHA256: &str =
    "86e0862891472ad5cf421ad723e3b51703f4470ea1d08643661580020d5dd58b";

const VERSIONS: &[&str] = &[
    "1.150.0", "1.151.0", "1.152.0", "1.153.0", "1.154.0", "1.155.0", "1.156.0", "1.157.0",
    "1.158.0", "1.159.0", "1.160.0", "1.161.0",
];
const CRATE_SHA256S: &[&str] = &[
    "81186bb96a4e98ff93f7b4336deec0afc1f90ca282099fc5395fcf5de6c0389d",
    "2cb8e2aff3ed7af0df6c54961d38a4526b581bb00f93bee6e6a12581d05b1de2",
    "1779d9b47d6a549caa8cde4dd7811f3dfc2111308c0d696d202807a7f5892bda",
    "ad5175fd0be54231cc4f9d0a5cbb0a6384548fd6aa5c846c1d6945960d35d840",
    "4422d8bd0834d25f5c35b60ad77efea0b552e309691b30c975c0eef7dfd222b0",
    "42c7a359ec7c2904bd8f541d7931015902a73e380ee68760ce1e409f69809e8c",
    "d01490add1001bfe6529a3f36a84ca51f1d3d5ecc59f2ebd959ac76c04e2bc95",
    "347e5d5ea47f43415003507fbd8aacecc8f7ccd09a2455f2b5e376921905e48f",
    "0024daf189f4c0026c7aa2580d6450af163525c9afb7f84b019666661f7e8a6b",
    "42e438d72b1c900df6926ef6cfd4c378432ccb1ab2639dc943921a08f048f3c4",
    "fe7904fdf1267ae7c37d42b6ab231b58ccd42d50adbdcad446be243586a6a67d",
    "254169f7bd61c2189a0142067000ed89ab15a032b8b8cebf426fabeb425ad611",
];
const SOURCE_COMMITS: &[&str] = &[
    "4412bdcfc85b94c30e3b158970ca2fc693632ca7",
    "e3a8e3db1bd094e96c0e707f15e6f38baf92fe07",
    "3c6d526c9d4775f41a8ef1ed2ef574d1b14481db",
    "feff37ab0d80eef00a798cbbb740cf3b15453900",
    "e2d4cb15aab1e2a68d3d0ee6d5828f0333839ef0",
    "653085fbbc4a50138cf955445b65431bea3599d4",
    "82b4cf5c430a4177f7675a8c78986a9f3c74c8a7",
    "7d6ed1a3296cf582c7efeefcd35998975b188e09",
    "dd4d62780ee72db9adae16134e5dac47a9b6393e",
    "2ea89feffd8b76d5b4f1d9030f38f956ec4b6957",
    "d3d147c208ab27790dfded9cdc724aa9b7bed6b6",
    "7101aefb7632e44cce586886a1df595151409b5f",
];
const EXPECTED_CHANGED_COUNTS: &[usize] = &[104, 6, 7, 6, 116, 11, 11, 7, 222, 6, 6];
const EXPECTED_IDENTICAL_COUNTS: &[usize] = &[
    1366, 1464, 1463, 1464, 1354, 1459, 1459, 1463, 1248, 1464, 1464,
];

fn digest(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

fn json(value: &str) -> Value {
    serde_json::from_str(value).expect("frozen JSON is valid")
}

fn string_set(value: &Value) -> BTreeSet<String> {
    value
        .as_array()
        .expect("fixture field is an array")
        .iter()
        .map(|item| item.as_str().expect("fixture member is text").to_owned())
        .collect()
}

fn manifest_digest(manifest: &BTreeMap<String, String>) -> String {
    let bytes = manifest
        .iter()
        .map(|(path, hash)| format!("{path}\t{hash}\n"))
        .collect::<String>();
    digest(bytes.as_bytes())
}

fn classified_paths(value: &Value, from: &str, to: &str) -> BTreeSet<String> {
    value
        .as_array()
        .expect("classifications are an array")
        .iter()
        .filter(|item| item["from"] == from && item["to"] == to)
        .map(|item| item["path"].as_str().expect("path is text").to_owned())
        .collect()
}

#[test]
fn crates_io_freezes_every_published_catalogue_sdk_artifact() {
    assert_eq!(digest(IDENTITY.as_bytes()), IDENTITY_SHA256);
    let identity = json(IDENTITY);
    assert_eq!(identity["axis"], "amazon-bedrock.control-plane-rust-sdk");
    assert_eq!(identity["package"], "aws-sdk-bedrock");
    assert_eq!(
        identity["channel"]["official_channel"],
        "crates.io stable releases, maximum non-yanked semver for aws-sdk-bedrock"
    );
    assert_eq!(
        identity["channel"]["observations"][0]["official_latest_stable"],
        "1.161.0"
    );
    assert_eq!(identity["channel"]["observations"][0]["yanked"], false);
    assert_eq!(
        identity["channel"]["synthetic_unpublished_later_stable"],
        "1.162.0"
    );
    assert_eq!(
        identity["channel"]["synthetic_unpublished_later_stable_absent"],
        true
    );
    assert_eq!(
        identity["claim_at_observation"]["adapter_sdk_version_constant"],
        "1.148.0"
    );
    assert_eq!(
        identity["claim_at_observation"]["workspace_cargo_pin"],
        "1.150.0"
    );
    assert_eq!(identity["claim_at_observation"]["scheme"], "Opaque");

    let releases = identity["published_stables_from_current_workspace_pin_through_official_latest"]
        .as_array()
        .expect("release ledger is an array");
    assert_eq!(releases.len(), VERSIONS.len());
    for (index, ((release, version), checksum)) in
        releases.iter().zip(VERSIONS).zip(CRATE_SHA256S).enumerate()
    {
        assert_eq!(release["version"], *version);
        assert_eq!(release["crate_sha256"], *checksum);
        assert_eq!(
            release["download_url"],
            format!("https://crates.io/api/v1/crates/aws-sdk-bedrock/{version}/download")
        );
        assert_eq!(
            release["source_repository"],
            "https://github.com/awslabs/aws-sdk-rust"
        );
        assert_eq!(release["source_path"], "sdk/bedrock");
        assert_eq!(release["source_commit"], SOURCE_COMMITS[index]);
        assert_eq!(release["yanked"], index == 6);
        assert!(
            !release["runtime_dependency_requirements"]
                .as_object()
                .unwrap()
                .is_empty()
        );
    }
    let old_gap = &identity["preserved_unqualified_published_points"][0];
    assert_eq!(old_gap["version"], "1.149.0");
    assert_eq!(old_gap["yanked"], false);
    assert_eq!(
        old_gap["crate_sha256"],
        "fa59af1ad47597402d3feb45b1ad99d4c93da3d2ed52ab349a3fa97972ce2a40"
    );
    assert_eq!(
        identity["identity_decision"]["semantic_maintained_segments"],
        json(r#"[["1.150.0","1.155.0"],["1.157.0","1.161.0"]]"#)
    );
    assert_eq!(
        identity["identity_decision"]["yanked_exclusions"],
        json(r#"["1.156.0"]"#)
    );
    assert_eq!(identity["evidence"]["artifact_execution"], false);
    assert_eq!(identity["evidence"]["provider_calls"], false);
    assert_eq!(identity["evidence"]["credentials_or_host_changes"], false);
}

#[test]
fn selected_catalogue_protocol_and_every_hop_keep_the_exact_boundary() {
    assert_eq!(digest(PROTOCOL.as_bytes()), PROTOCOL_SHA256);
    let protocol = json(PROTOCOL);
    assert_eq!(protocol["sdk_crate"], "aws-sdk-bedrock");
    assert_eq!(protocol["sdk_version"], "1.161.0");
    assert_eq!(protocol["source"]["commit"], SOURCE_COMMITS[11]);
    assert_eq!(
        protocol["service_api"],
        "Amazon Bedrock ListFoundationModels"
    );
    assert_eq!(
        protocol["service_api_revision"],
        "bedrock-list-foundation-models"
    );
    assert_eq!(protocol["crate_semver_is_not_service_model_date"], true);
    assert_eq!(protocol["request"]["operation"], "ListFoundationModels");
    assert_eq!(protocol["request"]["method"], "GET");
    assert_eq!(protocol["request"]["path"], "/foundation-models");
    assert_eq!(protocol["request"]["query_parameters"], json("[]"));
    assert_eq!(protocol["request"]["filters"], json("[]"));
    assert_eq!(protocol["request"]["pagination"], "none");
    assert_eq!(protocol["request"]["maximum_provider_requests"], 1);
    assert_eq!(protocol["request"]["maximum_sdk_attempts"], 1);
    assert_eq!(
        protocol["client_configuration"]["default_chain_loading"],
        false
    );
    assert_eq!(protocol["client_configuration"]["retry_max_attempts"], 1);
    assert_eq!(
        protocol["client_configuration"]["input_telemetry_capture"],
        false
    );
    assert_eq!(protocol["authentication"]["default_chain_allowed"], false);
    assert_eq!(
        protocol["authorization"]["iam_action"],
        "bedrock:ListFoundationModels"
    );
    assert_eq!(protocol["failure"]["automatic_retry"], false);
    assert_eq!(protocol["failure"]["provider_messages_retained"], false);
    assert_eq!(protocol["catalogue_presence_implies"], json("[]"));

    let expected_files = [
        "src/operation/list_foundation_models.rs",
        "src/protocol_serde/shape_list_foundation_models.rs",
        "src/protocol_serde/shape_foundation_model_summary_list.rs",
        "src/protocol_serde/shape_foundation_model_summary.rs",
        "src/protocol_serde/shape_foundation_model_lifecycle.rs",
        "src/protocol_serde/shape_model_modality_list.rs",
        "src/protocol_serde/shape_inference_type_list.rs",
        "src/protocol_serde/shape_model_customization_list.rs",
        "src/types/_foundation_model_summary.rs",
        "src/types/_foundation_model_lifecycle.rs",
        "src/types/_model_modality.rs",
        "src/types/_inference_type.rs",
        "src/types/_model_customization.rs",
        "src/types/error.rs",
        "src/types/error/_access_denied_exception.rs",
        "src/types/error/_internal_server_exception.rs",
        "src/types/error/_throttling_exception.rs",
        "src/types/error/_validation_exception.rs",
        "src/types/error/builders.rs",
        "src/client.rs",
        "src/config.rs",
        "src/config/endpoint.rs",
        "src/config/retry.rs",
        "src/operation.rs",
        "src/protocol_serde.rs",
    ];
    assert_eq!(
        string_set(&protocol["selected_source_files"]),
        expected_files.into_iter().map(str::to_owned).collect()
    );

    let hops = protocol["published_hop_review"].as_array().unwrap();
    assert_eq!(hops.len(), VERSIONS.len() - 1);
    let expected_changed_selected: &[&[&str]] = &[
        &["src/operation/list_foundation_models.rs", "src/config.rs"],
        &[],
        &[],
        &[],
        &["src/operation/list_foundation_models.rs", "src/config.rs"],
        &[],
        &[],
        &["src/config.rs"],
        &[
            "src/operation/list_foundation_models.rs",
            "src/protocol_serde/shape_list_foundation_models.rs",
        ],
        &[],
        &[],
    ];
    for (index, hop) in hops.iter().enumerate() {
        assert_eq!(hop["from"], VERSIONS[index]);
        assert_eq!(hop["to"], VERSIONS[index + 1]);
        assert_eq!(hop["selected_behavior_unchanged"], true);
        assert_eq!(
            string_set(&hop["changed_selected_files"]),
            expected_changed_selected[index]
                .iter()
                .map(|path| (*path).to_owned())
                .collect()
        );
        assert!(!hop["reason"].as_str().unwrap().is_empty());
    }
    assert_eq!(
        protocol["unmapped_sdk_export_hops"]
            .as_array()
            .unwrap()
            .len(),
        11
    );
    assert!(
        protocol["unmapped_sdk_export_hops"]
            .as_array()
            .unwrap()
            .iter()
            .all(|hop| hop["path"] == "src/lib.rs")
    );
}

#[test]
fn complete_package_trees_and_changed_file_classifications_reconstruct_every_hop() {
    assert_eq!(digest(DIST_INVENTORY.as_bytes()), DIST_INVENTORY_SHA256);
    let inventory = json(DIST_INVENTORY);
    assert_eq!(inventory["crate"], "aws-sdk-bedrock");
    let compared = inventory["compared"]
        .as_array()
        .unwrap()
        .iter()
        .map(|version| version.as_str().unwrap())
        .collect::<Vec<_>>();
    assert_eq!(compared, VERSIONS);
    assert_eq!(
        inventory["change_classifications"]
            .as_array()
            .unwrap()
            .len(),
        502
    );

    let mut manifest =
        serde_json::from_value::<BTreeMap<String, String>>(inventory["base_manifest"].clone())
            .expect("base tree inventory is valid");
    assert_eq!(inventory["base_manifest_version"], "1.150.0");
    assert_eq!(manifest.len(), 1470);
    assert_eq!(
        manifest_digest(&manifest),
        inventory["tree_manifest_sha256"]["1.150.0"]
    );
    let mut all_manifests = BTreeMap::from([("1.150.0", manifest.clone())]);

    for (index, pair) in VERSIONS.windows(2).enumerate() {
        let from = pair[0];
        let to = pair[1];
        let key = format!("from_{from}_to_{to}");
        let hop = &inventory[key.as_str()];
        let changed = string_set(&hop["changed"]);
        let identical = string_set(&hop["identical"]);
        let added = string_set(&hop["added"]);
        let removed = string_set(&hop["removed"]);
        assert!(added.is_empty());
        assert!(removed.is_empty());
        assert_eq!(changed.len(), EXPECTED_CHANGED_COUNTS[index]);
        assert_eq!(identical.len(), EXPECTED_IDENTICAL_COUNTS[index]);
        assert!(changed.is_disjoint(&identical));
        assert_eq!(changed.union(&identical).count(), 1470);

        let changed_hashes = hop["changed_file_sha256"].as_object().unwrap();
        assert_eq!(
            changed_hashes.keys().cloned().collect::<BTreeSet<_>>(),
            changed
        );
        let classified = classified_paths(&inventory["change_classifications"], from, to);
        assert_eq!(classified, changed);
        let classifications = hop["classifications"].as_array().unwrap();
        assert_eq!(classifications.len(), changed.len());
        for record in classifications {
            assert!(!record["category"].as_str().unwrap().is_empty());
            assert!(!record["reason"].as_str().unwrap().is_empty());
            assert_eq!(record["from"], from);
            assert_eq!(record["to"], to);
        }

        for path in &changed {
            let hashes = &hop["changed_file_sha256"][path];
            assert_eq!(hashes["from_sha256"], manifest[path.as_str()]);
            manifest.insert(
                path.clone(),
                hashes["to_sha256"].as_str().unwrap().to_owned(),
            );
        }
        assert_eq!(manifest.len(), 1470);
        assert_eq!(inventory["package_file_counts"][to], 1470);
        assert_eq!(
            manifest_digest(&manifest),
            inventory["tree_manifest_sha256"][to]
        );
        all_manifests.insert(to, manifest.clone());
    }

    let identical_through_all_hops = string_set(&inventory["identical_through_all_hops"]);
    let derived_identical = all_manifests["1.150.0"]
        .iter()
        .filter(|(path, hash)| all_manifests.values().all(|tree| tree[*path] == **hash))
        .map(|(path, _)| path.clone())
        .collect::<BTreeSet<_>>();
    assert_eq!(identical_through_all_hops, derived_identical);
    assert_eq!(identical_through_all_hops.len(), 1240);

    let changed_paths = inventory["changed_file_hashes"].as_object().unwrap();
    let all_changed = inventory["change_classifications"]
        .as_array()
        .unwrap()
        .iter()
        .map(|record| record["path"].as_str().unwrap().to_owned())
        .collect::<BTreeSet<_>>();
    assert_eq!(
        changed_paths.keys().cloned().collect::<BTreeSet<_>>(),
        all_changed
    );
    for (path, hashes) in changed_paths {
        for version in VERSIONS {
            if all_manifests[*version].contains_key(path) {
                assert_eq!(hashes[*version], all_manifests[*version][path.as_str()]);
            }
        }
    }

    let selected = inventory["selected_file_hashes_by_version"]
        .as_object()
        .unwrap();
    let protocol = json(PROTOCOL);
    assert_eq!(
        selected.keys().cloned().collect::<BTreeSet<_>>(),
        string_set(&protocol["selected_source_files"])
    );
    for (path, hashes) in selected {
        assert_eq!(
            hashes
                .as_object()
                .unwrap()
                .keys()
                .cloned()
                .collect::<BTreeSet<_>>(),
            VERSIONS.iter().map(|v| (*v).to_owned()).collect()
        );
        for version in VERSIONS {
            assert_eq!(hashes[*version], all_manifests[*version][path.as_str()]);
        }
    }
}

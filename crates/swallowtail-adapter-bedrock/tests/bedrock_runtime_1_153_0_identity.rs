use serde_json::Value;
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};

const IDENTITY: &str = include_str!("fixtures/bedrock-runtime-1.153.0/identity.json");
const PROTOCOL: &str = include_str!("fixtures/bedrock-runtime-1.153.0/protocol.json");
const DIST_INVENTORY: &str = include_str!("fixtures/bedrock-runtime-1.153.0/dist-inventory.json");
const IDENTITY_SHA256: &str = "33d004c5ec748b56f573ae5219fa4941a04e5da949265cf21eabe7b4db33ae34";
const PROTOCOL_SHA256: &str = "a8a9284bc92d4bb22f29af17182fd591fdb4e89f791e7ef9a925333309e63231";
const DIST_INVENTORY_SHA256: &str =
    "ff35cb6ee2752c35af7dc65dad0996b952138b0e52a1adefe18f190df10e0191";

const VERSIONS: &[&str] = &[
    "1.148.0", "1.149.0", "1.150.0", "1.151.0", "1.152.0", "1.153.0",
];
const CRATE_SHA256S: &[&str] = &[
    "3f2542c0f038223ba2f9aefe3bd38c7395ccc6899d8fc58a99b8ddfeab3354f5",
    "59f4e05517820d9e79c115695bd4ac9005a9ebbd3d6aff5f8d3d4731325b3b21",
    "c37e1cf8f963668fcb6d45313799b97616f65e431cf5e65e87194147e2b18bdd",
    "f008e2a7a47e6c8f4a61c3783def307c6817532c1e05ce417da51657088fbe02",
    "dd3ff1fc9e5e8a2ac61bcc6ac82b13a57d431d6804d4461f9570a3b788b9549a",
    "89b8aeb4b1b6d5f9d377264cff21b4e4d773e66ee3c5582b4099a8fbc702bdb3",
];
const SOURCE_COMMITS: &[&str] = &[
    "7101aefb7632e44cce586886a1df595151409b5f",
    "b964fc77de60b0a492ff788c40bb66524fd26a44",
    "133c93f9af3783be5d41163784163cc80068ae1a",
    "3a3f39bd932690cab6d2321b677e97a24b687753",
    "c42d314c972a95e7cfd9583eef5113a2e1d6dccb",
    "44d6380d22c14bd5c89c59ceafcda7bc4da5b6f7",
];
const EXPECTED_CHANGED_COUNTS: &[usize] = &[8, 6, 6, 6, 7];
const EXPECTED_IDENTICAL_COUNTS: &[usize] = &[536, 538, 538, 538, 537];

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
fn crates_io_freezes_each_published_runtime_sdk_hop_after_1_148_0() {
    assert_eq!(digest(IDENTITY.as_bytes()), IDENTITY_SHA256);
    let identity = json(IDENTITY);
    assert_eq!(identity["axis"], "amazon-bedrock.runtime-rust-sdk");
    assert_eq!(identity["package"], "aws-sdk-bedrockruntime");
    assert_eq!(identity["channel"]["official_latest_stable"], "1.153.0");
    assert_eq!(
        identity["channel"]["synthetic_unpublished_later_stable"],
        "1.154.0"
    );
    assert_eq!(
        identity["channel"]["synthetic_version_absent_at_observation"],
        true
    );
    assert_eq!(
        identity["claim_at_observation"]["adapter_sdk_version_constant"],
        "1.148.0"
    );
    assert_eq!(
        identity["claim_at_observation"]["workspace_cargo_pin"],
        "1.148.0"
    );

    let releases = identity["published_stables_from_current_workspace_pin_through_official_latest"]
        .as_array()
        .expect("release ledger is an array");
    assert_eq!(releases.len(), VERSIONS.len());
    for (index, (release, version)) in releases.iter().zip(VERSIONS).enumerate() {
        assert_eq!(release["version"], *version);
        assert_eq!(release["crate_sha256"], CRATE_SHA256S[index]);
        assert_eq!(release["source_commit"], SOURCE_COMMITS[index]);
        assert_eq!(
            release["download_url"],
            format!("https://crates.io/api/v1/crates/aws-sdk-bedrockruntime/{version}/download")
        );
        assert_eq!(release["source_path"], "sdk/bedrockruntime");
        assert_eq!(
            release["source_repository"],
            "https://github.com/awslabs/aws-sdk-rust"
        );
        assert_eq!(release["rust_version"], "1.94.1");
        assert_eq!(release["yanked"], false);
        assert!(
            !release["runtime_dependency_minimums"]
                .as_object()
                .expect("declared minimums are an object")
                .is_empty()
        );
    }
    assert_eq!(releases[0]["version"], "1.148.0");
    assert_eq!(releases[5]["version"], "1.153.0");
    assert_eq!(
        identity["decision"]["extended_qualified_segment"]["minimum"],
        "1.139.0"
    );
    assert_eq!(
        identity["decision"]["extended_qualified_segment"]["maximum"],
        "1.153.0"
    );
    assert_eq!(
        identity["decision"]["extended_qualified_segment"]["previously_qualified_through"],
        "1.148.0"
    );
    assert_eq!(
        identity["decision"]["unqualified_published_gaps_preserved"],
        json(r#"["1.137.0","1.138.0"]"#)
    );
    assert_eq!(
        identity["decision"]["yanked_exclusions"],
        json(r#"["1.144.0"]"#)
    );
    let pin_utils_note = identity["decision"]["smithy_dependency_transition"]["pin_utils"]
        .as_str()
        .unwrap();
    assert!(pin_utils_note.contains("pin-utils"));
    assert!(pin_utils_note.contains("0.64.3"));
    assert!(pin_utils_note.contains("1.16.2"));
    assert!(pin_utils_note.contains("1.9.3"));
    // The resolved workspace closure this qualification ships with, asserted
    // as exact literals; the catalogue identity test cross-checks the same
    // versions against Cargo.lock itself.
    let transition = &identity["decision"]["smithy_dependency_transition"];
    assert_eq!(transition["resolved_aws_smithy_schema"], "1.2.0");
    assert_eq!(transition["resolved_aws_smithy_types"], "1.9.3");
    assert_eq!(transition["resolved_aws_smithy_json"], "0.65.1");
    assert_eq!(transition["resolved_aws_smithy_runtime_api"], "1.20.0");
    assert_eq!(transition["resolved_aws_smithy_http_client"], "1.5.2");
    assert!(
        !transition["dependency_maps_are_declared_minimums"]
            .as_str()
            .unwrap()
            .is_empty()
    );
    assert_eq!(identity["evidence"]["artifact_execution"], false);
    assert_eq!(identity["evidence"]["provider_calls"], false);
    assert_eq!(identity["evidence"]["credentials_or_host_changes"], false);
}

#[test]
fn selected_runtime_protocol_fixture_freezes_request_stream_and_failure_bounds() {
    assert_eq!(digest(PROTOCOL.as_bytes()), PROTOCOL_SHA256);
    let protocol = json(PROTOCOL);
    assert_eq!(protocol["sdk_crate"], "aws-sdk-bedrockruntime");
    assert_eq!(protocol["sdk_version"], "1.153.0");
    assert_eq!(
        protocol["source"]["commit"],
        "44d6380d22c14bd5c89c59ceafcda7bc4da5b6f7"
    );
    assert_eq!(
        protocol["source"]["crate_sha256"],
        "89b8aeb4b1b6d5f9d377264cff21b4e4d773e66ee3c5582b4099a8fbc702bdb3"
    );
    assert_eq!(
        protocol["service_api"],
        "Amazon Bedrock Runtime ConverseStream"
    );
    assert_eq!(
        protocol["transport"],
        "in_process_rust_sdk_typed_eventstream"
    );
    assert_eq!(
        protocol["request"]["body"],
        json(
            r#"{"inferenceConfig":{"maxTokens":7},"messages":[{"content":[{"text":"hello"}],"role":"user"}]}"#
        )
    );
    assert_eq!(
        string_set(&protocol["request"]["optional_members_absent"]),
        [
            "additionalModelRequestFields",
            "additionalModelResponseFieldPaths",
            "guardrailConfig",
            "performanceConfig",
            "promptVariables",
            "requestMetadata",
            "serviceTier",
            "system",
            "toolConfig",
            "trace",
        ]
        .into_iter()
        .map(str::to_owned)
        .collect()
    );
    assert_eq!(protocol["client_configuration"]["retry_max_attempts"], 1);
    assert_eq!(protocol["client_configuration"]["telemetry_capture"], false);
    assert_eq!(
        protocol["stream"]["semantic_order"]
            .as_array()
            .unwrap()
            .len(),
        5
    );
    assert_eq!(protocol["stream"]["unknown_sdk_variant"], "fail_closed");
    assert_eq!(
        protocol["stream"]["usage_fields"].as_array().unwrap().len(),
        3
    );
    assert_eq!(protocol["failure"]["provider_messages_retained"], false);
    assert_eq!(protocol["failure"]["automatic_retry"], false);
}

#[test]
fn complete_tree_deltas_and_classifications_reconstruct_every_published_hop() {
    assert_eq!(digest(DIST_INVENTORY.as_bytes()), DIST_INVENTORY_SHA256);
    let inventory = json(DIST_INVENTORY);
    let compared = inventory["compared"]
        .as_array()
        .expect("compared versions are an array")
        .iter()
        .map(|version| version.as_str().expect("version is text"))
        .collect::<Vec<_>>();
    assert_eq!(compared, VERSIONS);
    assert_eq!(
        inventory["change_classifications"]
            .as_array()
            .unwrap()
            .len(),
        33
    );

    let mut manifest =
        serde_json::from_value::<BTreeMap<String, String>>(inventory["base_manifest"].clone())
            .expect("base file manifest is valid");
    assert_eq!(manifest.len(), 544);
    assert_eq!(inventory["base_manifest_version"], "1.148.0");
    assert_eq!(
        manifest_digest(&manifest),
        inventory["tree_manifest_sha256"]["1.148.0"]
    );

    let mut all_manifests = BTreeMap::from([("1.148.0", manifest.clone())]);
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
        assert_eq!(changed.union(&identical).count(), manifest.len());
        let changed_hash_paths = hop["changed_file_sha256"]
            .as_object()
            .expect("changed file hashes are an object")
            .keys()
            .cloned()
            .collect::<BTreeSet<_>>();
        assert_eq!(changed_hash_paths, changed);
        assert_eq!(
            classified_paths(&inventory["change_classifications"], from, to),
            changed
        );

        for path in &changed {
            let hashes = &hop["changed_file_sha256"][path];
            assert_eq!(hashes["from_sha256"], manifest[path.as_str()]);
            manifest.insert(
                path.clone(),
                hashes["to_sha256"]
                    .as_str()
                    .expect("digest is text")
                    .to_owned(),
            );
        }
        assert_eq!(manifest.len(), 544);
        assert_eq!(inventory["package_file_counts"][to], 544);
        assert_eq!(
            manifest_digest(&manifest),
            inventory["tree_manifest_sha256"][to]
        );
        all_manifests.insert(to, manifest.clone());
    }

    let identical_through_all_hops = string_set(&inventory["identical_through_all_hops"]);
    let derived_identical = all_manifests["1.148.0"]
        .iter()
        .filter(|(path, hash)| all_manifests.values().all(|tree| tree[*path] == **hash))
        .map(|(path, _)| path.clone())
        .collect::<BTreeSet<_>>();
    assert_eq!(identical_through_all_hops, derived_identical);
    assert_eq!(identical_through_all_hops.len(), 535);

    let changed_paths = inventory["changed_file_hashes"]
        .as_object()
        .expect("changed file hashes are an object");
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
            if all_manifests[*version].contains_key(path.as_str()) {
                assert_eq!(hashes[*version], all_manifests[*version][path.as_str()]);
            }
        }
    }

    let selected = inventory["selected_file_hashes_by_version"]
        .as_object()
        .expect("selected file hashes are an object");
    let unchanged_types = [
        "src/operation/converse_stream/_converse_stream_input.rs",
        "src/types/_message.rs",
        "src/types/_content_block.rs",
        "src/types/_conversation_role.rs",
        "src/types/_inference_configuration.rs",
        "src/types/_content_block_delta.rs",
        "src/types/_content_block_delta_event.rs",
        "src/types/_content_block_stop_event.rs",
        "src/types/_converse_stream_metadata_event.rs",
        "src/types/_converse_stream_metrics.rs",
        "src/types/_message_start_event.rs",
        "src/types/_message_stop_event.rs",
        "src/types/_stop_reason.rs",
        "src/types/_token_usage.rs",
        "src/types/error.rs",
    ];
    for path in unchanged_types {
        let hashes = selected[path]
            .as_object()
            .expect("version hashes are an object");
        let first = &hashes[VERSIONS[0]];
        assert!(VERSIONS.iter().all(|version| hashes[*version] == *first));
    }
}

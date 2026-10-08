use serde_json::Value;
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};

const IDENTITY: &str = include_str!("fixtures/bedrock-runtime-1.148.0/identity.json");
const PROTOCOL: &str = include_str!("fixtures/bedrock-runtime-1.148.0/protocol.json");
const DIST_INVENTORY: &str = include_str!("fixtures/bedrock-runtime-1.148.0/dist-inventory.json");
const IDENTITY_SHA256: &str = "386e9afe95621f8b273f07029069513c1af1766044d7bb63c6ec459752982637";
const PROTOCOL_SHA256: &str = "40dc300cac6abfe141ff9a78dfd7ad93621be1effd66303e0b36280ff4fd36fc";
const DIST_INVENTORY_SHA256: &str =
    "f5a648bfff2c795e2234176b7580a970c8703f20107d3a65f968c49ff650aa2d";

const VERSIONS: &[&str] = &[
    "1.139.0", "1.140.0", "1.141.0", "1.142.0", "1.143.0", "1.144.0", "1.145.0", "1.146.0",
    "1.147.0", "1.148.0",
];
const CRATE_SHA256S: &[&str] = &[
    "8dffe4bd3da45048fa0d30b50febaedf093e93a25d832cc21f2845a6e0c72616",
    "5bca4d762965c0b7f5fb88ec0a4b48cfa1a22502e1a0283e5fd5d9498dae52b6",
    "6418b1f5feb3a8cc0fa10fb9af5693c88135ee03bae3ff48c8cdbf6a918daa07",
    "4844547a354fb4e41895f4c9c423e7a7de13e3b2adc3f3493a2f2d8aa397c294",
    "492a63a8e903a7b8f7c77537ef05fc75d71ba268cf23a0eee41fb6f0da605029",
    "a2a05220aa89f7db7883abc73cbcaf5f96929f4cdec577c50e3fadedeb574871",
    "aed4de4a98241f9dd95ddad4223755b7afc3327d0a148a2a29286c95ef64db5e",
    "8aab5c776d9d4b5d7bc476368394a8f683ae280c7afcf19fb83e9ed6840408c7",
    "36e7de5d63ec9b699713b498b458720c5219ad7eb66b861e65fe51a21ab043df",
    "3f2542c0f038223ba2f9aefe3bd38c7395ccc6899d8fc58a99b8ddfeab3354f5",
];
const EXPECTED_CHANGED_COUNTS: &[usize] = &[20, 6, 6, 18, 13, 13, 7, 30, 6];
const EXPECTED_IDENTICAL_COUNTS: &[usize] = &[524, 538, 538, 526, 531, 531, 537, 514, 538];

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
fn crates_io_freezes_each_published_runtime_sdk_hop_and_the_yanked_point() {
    assert_eq!(digest(IDENTITY.as_bytes()), IDENTITY_SHA256);
    let identity = json(IDENTITY);
    assert_eq!(identity["axis"], "amazon-bedrock.runtime-rust-sdk");
    assert_eq!(identity["package"], "aws-sdk-bedrockruntime");
    assert_eq!(identity["channel"]["official_latest_stable"], "1.148.0");
    assert_eq!(
        identity["channel"]["synthetic_unpublished_later_stable"],
        "1.149.0"
    );
    assert_eq!(
        identity["channel"]["synthetic_version_absent_at_observation"],
        true
    );
    assert_eq!(
        identity["claim_at_observation"]["adapter_sdk_version_constant"],
        "1.136.0"
    );
    assert_eq!(
        identity["claim_at_observation"]["workspace_cargo_pin"],
        "1.139.0"
    );

    let releases = identity["published_stables_from_current_workspace_pin_through_official_latest"]
        .as_array()
        .expect("release ledger is an array");
    assert_eq!(releases.len(), VERSIONS.len());
    for ((release, version), expected_digest) in releases.iter().zip(VERSIONS).zip(CRATE_SHA256S) {
        assert_eq!(release["version"], *version);
        assert_eq!(release["crate_sha256"], *expected_digest);
        assert_eq!(
            release["download_url"],
            format!("https://crates.io/api/v1/crates/aws-sdk-bedrockruntime/{version}/download")
        );
        assert_eq!(release["source_path"], "sdk/bedrockruntime");
        assert_eq!(
            release["source_repository"],
            "https://github.com/awslabs/aws-sdk-rust"
        );
    }
    assert_eq!(releases[5]["yanked"], true);
    assert!(
        releases
            .iter()
            .enumerate()
            .all(|(index, release)| release["yanked"] == (index == 5))
    );
    assert_eq!(releases[0]["version"], "1.139.0");
    assert_eq!(releases[9]["version"], "1.148.0");
}

#[test]
fn selected_runtime_protocol_fixture_freezes_request_stream_and_failure_bounds() {
    assert_eq!(digest(PROTOCOL.as_bytes()), PROTOCOL_SHA256);
    let protocol = json(PROTOCOL);
    assert_eq!(protocol["sdk_crate"], "aws-sdk-bedrockruntime");
    assert_eq!(protocol["sdk_version"], "1.148.0");
    assert_eq!(
        protocol["source"]["commit"],
        "7101aefb7632e44cce586886a1df595151409b5f"
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
        119
    );

    let mut manifest =
        serde_json::from_value::<BTreeMap<String, String>>(inventory["base_manifest"].clone())
            .expect("base file manifest is valid");
    assert_eq!(manifest.len(), 544);
    assert_eq!(inventory["base_manifest_version"], "1.139.0");
    assert_eq!(
        manifest_digest(&manifest),
        inventory["tree_manifest_sha256"]["1.139.0"]
    );

    let mut all_manifests = BTreeMap::from([("1.139.0", manifest.clone())]);
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
    let derived_identical = all_manifests["1.139.0"]
        .iter()
        .filter(|(path, hash)| all_manifests.values().all(|tree| tree[*path] == **hash))
        .map(|(path, _)| path.clone())
        .collect::<BTreeSet<_>>();
    assert_eq!(identical_through_all_hops, derived_identical);
    assert_eq!(identical_through_all_hops.len(), 503);

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

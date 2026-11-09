use serde_json::Value;
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};

const IDENTITY: &str = include_str!("fixtures/bedrock-control-plane-1.166.0/identity.json");
const PROTOCOL: &str = include_str!("fixtures/bedrock-control-plane-1.166.0/protocol.json");
const DIST_INVENTORY: &str =
    include_str!("fixtures/bedrock-control-plane-1.166.0/dist-inventory.json");
const ADAPTER_MANIFEST: &str = include_str!("../Cargo.toml");
const WORKSPACE_LOCK: &str = include_str!("../../../Cargo.lock");
const IDENTITY_SHA256: &str = "831eeaecca79bd3c4c5abc5bad1dc7b53399e1aa13d48d3c686a9627c4f31fd2";
const PROTOCOL_SHA256: &str = "76a05f3ffa304b03af5dc5e78328b60e55a6ae2195ba4426c5c664673e34067f";
const DIST_INVENTORY_SHA256: &str =
    "526f38c1793f5f3a27736c2255e712170046bcec2fa939b8f6f68648d36b9278";

const VERSIONS: &[&str] = &[
    "1.161.0", "1.162.0", "1.163.0", "1.164.0", "1.165.0", "1.166.0",
];
const CRATE_SHA256S: &[&str] = &[
    "254169f7bd61c2189a0142067000ed89ab15a032b8b8cebf426fabeb425ad611",
    "b0d44e3919d31d8d1b12798c304df84e93fa2490341e32e5fb1f33d58d5763d7",
    "ec6a0e49550d3bbcdcc51538a4e776a61f0cb1958dde09c7630cb396bfed7956",
    "1dc81814f1abbb184e1da290cc982aaffefb5c1be145cb761668b5f5fc0255c4",
    "086d90d6919e0d36b1ed5d83b3773e15a98f8cfa6d30b264936b0f7f1215dec6",
    "6a2046512ecd16913d858634bfa07fe2769b794f852989940f2d5fcab7877968",
];
const SOURCE_COMMITS: &[&str] = &[
    "7101aefb7632e44cce586886a1df595151409b5f",
    "b964fc77de60b0a492ff788c40bb66524fd26a44",
    "133c93f9af3783be5d41163784163cc80068ae1a",
    "3a3f39bd932690cab6d2321b677e97a24b687753",
    "c42d314c972a95e7cfd9583eef5113a2e1d6dccb",
    "44d6380d22c14bd5c89c59ceafcda7bc4da5b6f7",
];
const EXPECTED_CHANGED_COUNTS: &[usize] = &[14, 6, 6, 6, 7];
const EXPECTED_ADDED_COUNTS: &[usize] = &[5, 0, 0, 0, 0];
const EXPECTED_IDENTICAL_COUNTS: &[usize] = &[1456, 1469, 1469, 1469, 1468];

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

fn lock_package(name: &str, version: &str) -> &'static str {
    let marker = format!("[[package]]\nname = \"{name}\"\nversion = \"{version}\"\n");
    let start = WORKSPACE_LOCK
        .find(&marker)
        .unwrap_or_else(|| panic!("Cargo.lock has no {name} {version} package"));
    let remaining = &WORKSPACE_LOCK[start..];
    let end = remaining.find("\n[[package]]").unwrap_or(remaining.len());
    &remaining[..end]
}

fn lock_field<'a>(package: &'a str, field: &str) -> &'a str {
    let prefix = format!("{field} = \"");
    package
        .lines()
        .find_map(|line| line.strip_prefix(&prefix)?.strip_suffix('"'))
        .unwrap_or_else(|| panic!("Cargo.lock package has no {field} field"))
}

fn lock_dependencies(package: &str) -> BTreeSet<String> {
    let dependencies = package
        .split_once("dependencies = [")
        .expect("Cargo.lock package has dependencies")
        .1
        .split_once(']')
        .expect("Cargo.lock dependencies are closed")
        .0;
    dependencies
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(|line| line.trim_end_matches(',').trim_matches('"').to_owned())
        .collect()
}

#[test]
fn crates_io_freezes_every_published_catalogue_sdk_artifact_after_1_161_0() {
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
        "1.166.0"
    );
    assert_eq!(identity["channel"]["observations"][0]["yanked"], false);
    assert_eq!(
        identity["channel"]["synthetic_unpublished_later_stable"],
        "1.167.0"
    );
    assert_eq!(
        identity["channel"]["synthetic_unpublished_later_stable_absent"],
        true
    );
    assert_eq!(
        identity["claim_at_observation"]["adapter_sdk_version_constant"],
        "1.161.0"
    );
    assert_eq!(
        identity["claim_at_observation"]["workspace_cargo_pin"],
        "1.161.0"
    );

    let releases = identity["published_stables_from_current_workspace_pin_through_official_latest"]
        .as_array()
        .expect("release ledger is an array");
    assert_eq!(releases.len(), VERSIONS.len());
    for (index, (release, version)) in releases.iter().zip(VERSIONS).enumerate() {
        assert_eq!(release["version"], *version);
        assert_eq!(release["crate_sha256"], CRATE_SHA256S[index]);
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
        assert_eq!(release["yanked"], false);
        assert_eq!(release["rust_version"], "1.94.1");
        assert!(
            !release["runtime_dependency_requirements"]
                .as_object()
                .unwrap()
                .is_empty()
        );
    }
    let preserved = &identity["preserved_unqualified_published_points"][0];
    assert_eq!(preserved["version"], "1.149.0");
    assert_eq!(preserved["yanked"], false);
    assert_eq!(
        preserved["crate_sha256"],
        "fa59af1ad47597402d3feb45b1ad99d4c93da3d2ed52ab349a3fa97972ce2a40"
    );
    assert_eq!(
        identity["identity_decision"]["extended_qualified_segment"]["minimum"],
        "1.157.0"
    );
    assert_eq!(
        identity["identity_decision"]["extended_qualified_segment"]["maximum"],
        "1.166.0"
    );
    assert_eq!(
        identity["identity_decision"]["extended_qualified_segment"]["previously_qualified_through"],
        "1.161.0"
    );
    assert_eq!(
        identity["identity_decision"]["unqualified_published_gaps"],
        json(r#"["1.149.0"]"#)
    );
    assert_eq!(
        identity["identity_decision"]["yanked_exclusions"],
        json(r#"["1.156.0"]"#)
    );
    assert_eq!(
        identity["identity_decision"]["smithy_dependency_transition"]["pin_utils"]
            .as_str()
            .unwrap()
            .contains("pin-utils"),
        true
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
    assert_eq!(protocol["sdk_version"], "1.166.0");
    assert_eq!(protocol["source"]["commit"], SOURCE_COMMITS[5]);
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
        &["src/protocol_serde.rs"],
        &[],
        &[],
        &[],
        &["src/config/endpoint.rs"],
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
        5
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
fn workspace_lock_freezes_the_corrected_sdk_runtime_dependency_identity() {
    assert!(ADAPTER_MANIFEST.lines().any(|line| {
        line.trim()
            == "aws-sdk-bedrock = { version = \"=1.166.0\", default-features = false, features = [\"default-https-client\", \"rt-tokio\"] }"
    }));
    assert!(ADAPTER_MANIFEST.lines().any(|line| {
        line.trim()
            == "aws-sdk-bedrockruntime = { version = \"=1.153.0\", default-features = false, features = [\"default-https-client\", \"rt-tokio\"] }"
    }));
    assert!(ADAPTER_MANIFEST.lines().any(|line| {
        line.trim()
            == "aws-smithy-http-client = { version = \"=1.5.2\", features = [\"test-util\"] }"
    }));
    assert_eq!(
        WORKSPACE_LOCK
            .lines()
            .any(|line| line.trim() == "\"pin-utils\""),
        false,
        "the corrected workspace graph must not retain pin-utils"
    );

    let sdk = lock_package("aws-sdk-bedrock", "1.166.0");
    assert_eq!(
        lock_field(sdk, "source"),
        "registry+https://github.com/rust-lang/crates.io-index"
    );
    assert_eq!(
        lock_field(sdk, "checksum"),
        "6a2046512ecd16913d858634bfa07fe2769b794f852989940f2d5fcab7877968"
    );
    assert_eq!(
        lock_dependencies(sdk),
        [
            "arc-swap",
            "aws-credential-types",
            "aws-runtime",
            "aws-smithy-async",
            "aws-smithy-http",
            "aws-smithy-json",
            "aws-smithy-observability",
            "aws-smithy-runtime",
            "aws-smithy-runtime-api",
            "aws-smithy-schema",
            "aws-smithy-types",
            "aws-types",
            "bytes",
            "fastrand",
            "http 1.5.0",
            "regex-lite",
            "tracing",
        ]
        .into_iter()
        .map(str::to_owned)
        .collect()
    );

    let dependencies = [
        (
            "arc-swap",
            "1.9.2",
            "c049c0be4daef0b145cb3555416b3b8ef5b7888a38aea1a3a155801fe7b0810b",
        ),
        (
            "aws-credential-types",
            "1.3.1",
            "576e8da35751dc85409ec1cd15999f692c277a1fbe9336780fe6f15e49dd637d",
        ),
        (
            "aws-runtime",
            "1.10.2",
            "9a0cc2144962cc5e358467b16e3d82981b1d424a7c3d34602be0036103814a1a",
        ),
        (
            "aws-smithy-async",
            "1.3.1",
            "de681a9df7ff79823b759ee17061076ec049cd61a8fa28ece4293d89c9f6964e",
        ),
        (
            "aws-smithy-http",
            "0.64.3",
            "19af609dd5edf8b056cd0bb001c706c8b3b1458055328b334ec13d7f46f6aaa8",
        ),
        (
            "aws-smithy-http-client",
            "1.5.2",
            "859a81364f8185349c584fb41cd8765a400cf982f447adada590cf008a9f3b85",
        ),
        (
            "aws-smithy-json",
            "0.65.1",
            "b45f9fc77ffe92c92c341165a01a565bd30c39ecf5aa8c0719b41719009604f9",
        ),
        (
            "aws-smithy-observability",
            "0.3.0",
            "8e86338c869539a581bf161247762a6e87f92c5c075060057b5ed6d06632ed0c",
        ),
        (
            "aws-smithy-runtime",
            "1.16.2",
            "4cb80fb19c084da5fca9d797845735e184445e5b958599440d67529bf6ef1b64",
        ),
        (
            "aws-smithy-runtime-api",
            "1.20.0",
            "d06a8615310f03c73ea8eea607ee1a9cb17b7ebe055b9152feda1d86831c215c",
        ),
        (
            "aws-smithy-schema",
            "1.2.0",
            "f7ef30d39e131651359b60a1c560de9a9502a4c3754172ab2d6e8734fe21bcba",
        ),
        (
            "aws-smithy-types",
            "1.9.3",
            "6c3a040d8b963838a01d7d239b0118357388d0ab881857bb5fc2dd780bb87957",
        ),
        (
            "aws-types",
            "1.7.0",
            "6ba0d970d3947bb064d2ae1d823efa186c1dbde88f88afa8f6549946e5d8deee",
        ),
        (
            "bytes",
            "1.12.1",
            "fc652a48c352aef3ea3aed32080501cf3ef6ed5da78602a020c991775b0aff04",
        ),
        (
            "fastrand",
            "2.5.0",
            "da7c62ceae207dd37ea5b845da6a0696c799f85e97da1ab5b7910be3c1c80223",
        ),
        (
            "http",
            "1.5.0",
            "918d3568bebf352712bc2ef3d46a8bcf1a75b373be6539de198e9105cbbf9ce0",
        ),
        (
            "regex-lite",
            "0.1.9",
            "cab834c73d247e67f4fae452806d17d3c7501756d98c8808d7c9c7aa7d18f973",
        ),
        (
            "tracing",
            "0.1.44",
            "63e71662fa4b2a2c3a26f570f037eb95bb1f85397f3cd8076caed2f026a6d100",
        ),
    ];
    for (name, version, checksum) in dependencies {
        let package = lock_package(name, version);
        assert_eq!(
            lock_field(package, "source"),
            "registry+https://github.com/rust-lang/crates.io-index"
        );
        assert_eq!(lock_field(package, "checksum"), checksum);
    }
    assert!(
        lock_package("aws-smithy-schema", "1.2.0").contains("aws-smithy-schema"),
        "the workspace resolves exactly one aws-smithy-schema package on the 1.x line"
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
        44
    );

    let mut manifest =
        serde_json::from_value::<BTreeMap<String, String>>(inventory["base_manifest"].clone())
            .expect("base tree inventory is valid");
    assert_eq!(inventory["base_manifest_version"], "1.161.0");
    assert_eq!(manifest.len(), 1470);
    assert_eq!(
        manifest_digest(&manifest),
        inventory["tree_manifest_sha256"]["1.161.0"]
    );
    let mut all_manifests = BTreeMap::from([("1.161.0", manifest.clone())]);

    for (index, pair) in VERSIONS.windows(2).enumerate() {
        let from = pair[0];
        let to = pair[1];
        let key = format!("from_{from}_to_{to}");
        let hop = &inventory[key.as_str()];
        let changed = string_set(&hop["changed"]);
        let identical = string_set(&hop["identical"]);
        let added = string_set(&hop["added"]);
        let removed = string_set(&hop["removed"]);
        assert!(removed.is_empty());
        assert_eq!(added.len(), EXPECTED_ADDED_COUNTS[index]);
        assert_eq!(changed.len(), EXPECTED_CHANGED_COUNTS[index]);
        assert_eq!(identical.len(), EXPECTED_IDENTICAL_COUNTS[index]);
        assert!(changed.is_disjoint(&identical));
        assert_eq!(
            changed.union(&identical).count() + added.len(),
            inventory["package_file_counts"][to].as_u64().unwrap() as usize
        );

        let changed_hashes = hop["changed_file_sha256"].as_object().unwrap();
        assert_eq!(
            changed_hashes.keys().cloned().collect::<BTreeSet<_>>(),
            changed
        );
        let classified = classified_paths(&inventory["change_classifications"], from, to);
        assert_eq!(classified, changed.union(&added).cloned().collect());
        let classifications = hop["classifications"].as_array().unwrap();
        assert_eq!(classifications.len(), changed.len() + added.len());
        for record in classifications {
            assert!(!record["category"].as_str().unwrap().is_empty());
            assert!(!record["reason"].as_str().unwrap().is_empty());
            assert_eq!(record["from"], from);
            assert_eq!(record["to"], to);
        }

        for path in &added {
            manifest.insert(
                path.clone(),
                inventory["changed_file_hashes"][path.as_str()][to]
                    .as_str()
                    .expect("added path digest is text")
                    .to_owned(),
            );
        }
        for path in &changed {
            let hashes = &hop["changed_file_sha256"][path];
            assert_eq!(hashes["from_sha256"], manifest[path.as_str()]);
            manifest.insert(
                path.clone(),
                hashes["to_sha256"].as_str().unwrap().to_owned(),
            );
        }
        assert_eq!(
            inventory["package_file_counts"][to].as_u64().unwrap() as usize,
            manifest.len()
        );
        assert_eq!(
            manifest_digest(&manifest),
            inventory["tree_manifest_sha256"][to]
        );
        all_manifests.insert(to, manifest.clone());
    }

    let identical_through_all_hops = string_set(&inventory["identical_through_all_hops"]);
    let derived_identical = all_manifests["1.161.0"]
        .iter()
        .filter(|(path, hash)| all_manifests.values().all(|tree| tree[*path] == **hash))
        .map(|(path, _)| path.clone())
        .collect::<BTreeSet<_>>();
    assert_eq!(identical_through_all_hops, derived_identical);
    assert_eq!(identical_through_all_hops.len(), 1455);

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
            if all_manifests[*version].contains_key(path.as_str()) {
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

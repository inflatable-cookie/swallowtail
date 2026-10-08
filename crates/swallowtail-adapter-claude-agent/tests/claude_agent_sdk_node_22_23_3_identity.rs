//! Frozen Node 22.23.3 runtime identity and selected-hop classification for
//! the `claude-agent.sdk.node` axis.
//!
//! The official Node artifact is represented by its signed archive identity
//! and complete extracted-tree ledger. Sidecar behavior is exercised in the
//! separate fake-backed sidecar and driver binaries.

use serde_json::Value;
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;
use swallowtail_adapter_claude_agent::sdk::{
    CLAUDE_AGENT_SDK_BEHAVIOR, CLAUDE_AGENT_SDK_NATIVE_VERSION, CLAUDE_AGENT_SDK_NODE_RUNTIME,
    CLAUDE_AGENT_SDK_VERSION, CLAUDE_AGENT_SDK_WIRE, claude_agent_sdk_node_binding,
    claude_agent_sdk_node_claim,
};
use swallowtail_core::InterfaceVersion;

const IDENTITY: &str = include_str!("fixtures/claude-agent-sdk-node-22.23.3/identity.json");
const PROTOCOL: &str = include_str!("fixtures/claude-agent-sdk-node-22.23.3/protocol.json");
const DIST_INVENTORY: &[u8] =
    include_bytes!("fixtures/claude-agent-sdk-node-22.23.3/dist-inventory.json");
const SOURCE_DELTA: &[u8] =
    include_bytes!("fixtures/claude-agent-sdk-node-22.23.3/source-delta.json");
const DIST_INVENTORY_SHA256: &str =
    "dcf2aa0d6d313587bdcb6ccbe4b404ec0922683068fd11702184d11dfa6e1d84";
const SOURCE_DELTA_SHA256: &str =
    "91173b432f0589721cd8d9f68d37bcbdb8e608aca1d7360c38115c25dc5dc180";

fn json(bytes: &[u8]) -> Value {
    serde_json::from_slice(bytes).expect("frozen Node identity JSON is valid")
}

fn object_keys(value: &Value) -> BTreeSet<&str> {
    value
        .as_object()
        .expect("fixture value is an object")
        .keys()
        .map(String::as_str)
        .collect()
}

fn assert_sha256(bytes: &[u8], expected: &str) {
    let actual = format!("{:x}", Sha256::digest(bytes));
    assert_eq!(actual, expected);
}

#[test]
fn official_node22_channel_and_signed_runtime_artifacts_are_frozen() {
    let identity = json(IDENTITY.as_bytes());
    assert_eq!(
        object_keys(&identity),
        BTreeSet::from([
            "artifact_identities",
            "axis",
            "current_claim_before",
            "decision",
            "distribution_inventory",
            "execution_limits",
            "official_channel",
            "observed_at",
            "package",
            "platform",
            "record",
            "retained_live_gates",
            "runtime_observation",
            "schema",
            "selected_tuple",
            "upstream_source",
        ])
    );
    assert_eq!(identity["record"], "Research 387");
    assert_eq!(identity["axis"], "claude-agent.sdk.node");
    assert_eq!(identity["package"], "swallowtail-adapter-claude-agent");
    assert_eq!(identity["schema"], 1);
    let channel = &identity["official_channel"];
    assert_eq!(
        object_keys(channel),
        BTreeSet::from([
            "channel_rows",
            "index_url",
            "name",
            "official_latest_stable",
            "published_hops_after_ceiling",
            "qualified_ceiling_before",
            "reprobed_before_artifact_selection",
            "selection",
            "unpublished_later_absent_at_observation",
            "unpublished_later_stable",
        ])
    );
    assert_eq!(channel["index_url"], "https://nodejs.org/dist/index.json");
    assert_eq!(channel["name"], "Node.js 22 distribution index");
    assert_eq!(
        channel["selection"],
        "newest published stable release in major line 22"
    );
    assert_eq!(channel["qualified_ceiling_before"], "22.23.2");
    assert_eq!(channel["official_latest_stable"], "22.23.3");
    assert_eq!(
        channel["published_hops_after_ceiling"],
        serde_json::json!(["22.23.3"])
    );
    assert_eq!(channel["unpublished_later_stable"], "22.23.4");
    assert_eq!(channel["unpublished_later_absent_at_observation"], true);
    assert_eq!(channel["reprobed_before_artifact_selection"], true);

    let artifacts = identity["artifact_identities"]
        .as_array()
        .expect("both boundary archives are frozen");
    assert_eq!(artifacts.len(), 2);
    assert_eq!(
        object_keys(&artifacts[0]),
        BTreeSet::from([
            "archive_sha256",
            "archive_size_bytes",
            "checksums_manifest",
            "checksums_manifest_sha256",
            "detached_signature",
            "detached_signature_sha256",
            "runtime_binary_path",
            "runtime_binary_sha256",
            "signature_verified_with_official_release_keyring",
            "signer_fingerprint",
            "source_commit",
            "source_tag_object",
            "url",
            "version",
        ])
    );
    assert_eq!(artifacts[0]["version"], "22.23.2");
    assert_eq!(artifacts[0]["archive_size_bytes"], 25_950_400);
    assert_eq!(
        artifacts[0]["archive_sha256"],
        "5eff7a9011895aae3f29d06f167b84a62b028a591370c7cafb59103559fd26e1"
    );
    assert_eq!(
        artifacts[0]["signature_verified_with_official_release_keyring"],
        true
    );
    assert_eq!(
        artifacts[0]["signer_fingerprint"],
        "CC68F5A3106FF448322E48ED27F5E38D5B0A215F"
    );
    assert_eq!(
        artifacts[0]["checksums_manifest_sha256"],
        "778ac5b2fcdbd68d9c0ae9f4310674faa3af0910bd0d18e7f6597787c40a3e39"
    );
    assert_eq!(
        artifacts[0]["detached_signature_sha256"],
        "169f1452c14cd653247408352f1534b9f31e3d13f9c6399c3977368095e11eda"
    );
    assert_eq!(
        artifacts[0]["runtime_binary_sha256"],
        "18e387c90ab8a8400183e8bdd396376e1e875b91b4c874b894dcade7b35bf572"
    );
    assert_eq!(
        artifacts[0]["source_tag_object"],
        "490a9fef8f8adcda5a95bd6f96035b05cb43fe5b"
    );
    assert_eq!(
        artifacts[0]["source_commit"],
        "aa4c77582be995286fc6e00aaf530dc7ade102a9"
    );
    assert_eq!(
        artifacts[0]["url"],
        "https://nodejs.org/dist/v22.23.2/node-v22.23.2-darwin-arm64.tar.xz"
    );
    assert_eq!(artifacts[1]["version"], "22.23.3");
    assert_eq!(artifacts[1]["archive_size_bytes"], 25_876_024);
    assert_eq!(
        artifacts[1]["archive_sha256"],
        "72d5d8832b41c9d9646197af614ffd751406ea4d215060eb91b98864e1919a3e"
    );
    assert_eq!(
        artifacts[1]["signature_verified_with_official_release_keyring"],
        true
    );
    assert_eq!(
        artifacts[1]["signer_fingerprint"],
        "5BE8A3F6C8A5C01D106C0AD820B1A390B168D356"
    );
    assert_eq!(
        artifacts[1]["checksums_manifest_sha256"],
        "4fe99a2ba9d552a6f51c13ed68fb11104cfa5df601aec616be689253a8139e7a"
    );
    assert_eq!(
        artifacts[1]["detached_signature_sha256"],
        "0aafa311177794108f9f15cf7baebc747ba21983e999b20eee94738c651469e0"
    );
    assert_eq!(artifacts[1]["runtime_binary_path"], "bin/node");
    assert_eq!(
        artifacts[1]["runtime_binary_sha256"],
        "68f4d07ca49e0500cc135c7e0a445093e228e42e126ac22306d045f0a8c2636b"
    );
    assert_eq!(
        artifacts[1]["source_tag_object"],
        "9ff018de597f54877b72bde69eb37de4e47c15c4"
    );
    assert_eq!(
        artifacts[1]["source_commit"],
        "80dc632040e6bada37aac1220dde9c79581c9c22"
    );
    assert_eq!(
        artifacts[1]["url"],
        "https://nodejs.org/dist/v22.23.3/node-v22.23.3-darwin-arm64.tar.xz"
    );
    assert_eq!(identity["platform"], "darwin-arm64");
    assert_eq!(
        identity["runtime_observation"]["candidate_output"],
        "v22.23.3"
    );
    assert_eq!(
        identity["runtime_observation"]["installation_or_host_update"],
        false
    );
    assert_eq!(
        identity["runtime_observation"]["provider_calls_or_credentials"],
        false
    );
    assert_eq!(
        object_keys(&identity["selected_tuple"]),
        BTreeSet::from([
            "behavior_revision",
            "native_version",
            "node_axis",
            "sdk_package",
            "sdk_version",
            "sidecar_source_tag",
            "wire",
        ])
    );
    assert_eq!(
        identity["selected_tuple"]["sdk_package"],
        "@anthropic-ai/claude-agent-sdk"
    );
    assert_eq!(identity["selected_tuple"]["sdk_version"], "0.3.284");
    assert_eq!(identity["selected_tuple"]["native_version"], "2.1.284");
    assert_eq!(
        identity["selected_tuple"]["wire"],
        "swallowtail-claude-agent-sdk-jsonl-v1"
    );
    assert_eq!(
        identity["selected_tuple"]["sidecar_source_tag"],
        "swallowtail-claude-agent-sdk-sidecar@0.5.1"
    );
    assert_eq!(
        identity["selected_tuple"]["behavior_revision"],
        "claude-agent.sdk-v1"
    );

    let upstream = &identity["upstream_source"];
    assert_eq!(upstream["repository"], "https://github.com/nodejs/node");
    assert_eq!(
        upstream["from_tag_object"],
        "490a9fef8f8adcda5a95bd6f96035b05cb43fe5b"
    );
    assert_eq!(
        upstream["from_peeled_commit"],
        "aa4c77582be995286fc6e00aaf530dc7ade102a9"
    );
    assert_eq!(
        upstream["to_tag_object"],
        "9ff018de597f54877b72bde69eb37de4e47c15c4"
    );
    assert_eq!(
        upstream["to_peeled_commit"],
        "80dc632040e6bada37aac1220dde9c79581c9c22"
    );
    assert_eq!(upstream["source_changed_path_count"], 755);
}

#[test]
fn node_window_extends_the_existing_claim_without_transferring_other_axes() {
    let claim = claude_agent_sdk_node_claim();
    assert_eq!(claim.id().as_str(), "claude-agent.sdk.node-window-1");
    assert_eq!(claim.axis().as_str(), "claude-agent.sdk.node");
    assert_eq!(
        claim.newer_version_posture(),
        swallowtail_core::InterfaceNewerVersionPosture::QualifiedOnly
    );
    assert_eq!(claim.milestones().len(), 1);
    assert_eq!(claim.baseline(), &InterfaceVersion::new("22.23.2").unwrap());
    assert_eq!(
        claim.latest_qualified().as_str(),
        CLAUDE_AGENT_SDK_NODE_RUNTIME
    );
    assert_eq!(CLAUDE_AGENT_SDK_NODE_RUNTIME, "22.23.3");
    assert_eq!(
        claim
            .milestones()
            .next()
            .expect("Node claim has its qualified segment")
            .behavior_revision()
            .as_str(),
        CLAUDE_AGENT_SDK_BEHAVIOR
    );
    assert_eq!(claim.exclusions().len(), 0);
    for version in ["22.23.2", "22.23.3"] {
        assert!(
            claim.permits(&InterfaceVersion::new(version).unwrap()),
            "{version}"
        );
        assert!(claude_agent_sdk_node_binding(version).is_some());
    }
    for version in ["22.23.1", "22.23.4", "23.0.0", "26.11.1"] {
        assert!(
            !claim.permits(&InterfaceVersion::new(version).unwrap()),
            "{version}"
        );
    }
    assert_eq!(CLAUDE_AGENT_SDK_VERSION, "0.3.293");
    assert_eq!(CLAUDE_AGENT_SDK_NATIVE_VERSION, "2.1.293");
    assert_eq!(
        CLAUDE_AGENT_SDK_WIRE,
        "swallowtail-claude-agent-sdk-jsonl-v1"
    );
}

#[test]
fn complete_distribution_trees_and_every_changed_path_are_integrity_bound() {
    let identity = json(IDENTITY.as_bytes());
    let inventory_digest = identity["distribution_inventory"]["sha256"]
        .as_str()
        .expect("inventory digest is recorded");
    assert_eq!(inventory_digest, DIST_INVENTORY_SHA256);
    assert_sha256(DIST_INVENTORY, inventory_digest);
    let inventory = json(DIST_INVENTORY);
    assert_eq!(
        object_keys(&inventory),
        BTreeSet::from(["base", "candidate", "hop", "schema"])
    );
    assert_eq!(inventory["base"]["entries"].as_array().unwrap().len(), 5865);
    assert_eq!(
        inventory["candidate"]["entries"].as_array().unwrap().len(),
        5865
    );
    assert_eq!(
        inventory["base"]["canonical_sha256"],
        "13f3f6716b51144605fc2deff341d8d76cfc49896b81d41ac175119ef276bce5"
    );
    assert_eq!(
        inventory["candidate"]["canonical_sha256"],
        "50e45afc486c881797754ffe363ad2d6a13b86bd43ad064ec180406499f13d9d"
    );
    assert_eq!(inventory["hop"]["from"], "22.23.2");
    assert_eq!(inventory["hop"]["to"], "22.23.3");
    assert_eq!(inventory["hop"]["added"].as_array().unwrap().len(), 0);
    assert_eq!(inventory["hop"]["removed"].as_array().unwrap().len(), 0);
    assert_eq!(inventory["hop"]["changed"].as_array().unwrap().len(), 393);
    assert_eq!(
        inventory["hop"]["selected_runtime_paths"],
        serde_json::json!(["bin/node"])
    );
    let distribution_classes = &inventory["hop"]["classified_changed_paths"];
    assert_eq!(
        object_keys(distribution_classes),
        BTreeSet::from([
            "documentation-or-license",
            "selected-runtime-executable",
            "unselected-development-headers",
            "unselected-npm-or-corepack",
        ])
    );
    let mut classified_distribution_paths = BTreeSet::new();
    for (classification, expected_count) in [
        ("documentation-or-license", 2),
        ("selected-runtime-executable", 1),
        ("unselected-development-headers", 176),
        ("unselected-npm-or-corepack", 214),
    ] {
        let paths = distribution_classes[classification]
            .as_array()
            .expect("classification paths are an array");
        assert_eq!(paths.len(), expected_count, "{classification} path count");
        for path in paths {
            assert!(
                classified_distribution_paths
                    .insert(path.as_str().expect("classified path is text").to_owned()),
                "duplicate distribution classification path: {path}"
            );
        }
    }
    let changed_distribution_paths = inventory["hop"]["changed"]
        .as_array()
        .unwrap()
        .iter()
        .map(|row| {
            assert_eq!(
                object_keys(row),
                BTreeSet::from(["after", "before", "classification", "path"])
            );
            row["path"].as_str().unwrap().to_owned()
        })
        .collect::<BTreeSet<_>>();
    assert_eq!(changed_distribution_paths.len(), 393);
    assert_eq!(classified_distribution_paths, changed_distribution_paths);
    assert_eq!(
        distribution_classes["selected-runtime-executable"],
        serde_json::json!(["bin/node"])
    );
    let node = inventory["hop"]["changed"]
        .as_array()
        .unwrap()
        .iter()
        .find(|row| row["path"] == "bin/node")
        .expect("runtime executable delta is explicit");
    assert_eq!(
        node["before"]["sha256"],
        "18e387c90ab8a8400183e8bdd396376e1e875b91b4c874b894dcade7b35bf572"
    );
    assert_eq!(
        node["after"]["sha256"],
        "68f4d07ca49e0500cc135c7e0a445093e228e42e126ac22306d045f0a8c2636b"
    );
}

#[test]
fn source_hop_classifies_selected_loader_and_keeps_other_runtime_deltas_bounded() {
    let identity = json(IDENTITY.as_bytes());
    let source_digest = identity["upstream_source"]["source_changes_sha256"]
        .as_str()
        .expect("source diff digest is recorded");
    assert_eq!(source_digest, SOURCE_DELTA_SHA256);
    assert_sha256(SOURCE_DELTA, source_digest);
    let source = json(SOURCE_DELTA);
    assert_eq!(
        object_keys(&source),
        BTreeSet::from([
            "changes",
            "classification_reasons",
            "classifications",
            "from_tag",
            "hop_summary",
            "repository",
            "schema",
            "to_tag",
        ])
    );
    assert_eq!(source["hop_summary"]["changed_path_count"], 755);
    assert_eq!(source["hop_summary"]["modified"], 750);
    assert_eq!(source["hop_summary"]["added"], 4);
    assert_eq!(source["hop_summary"]["deleted"], 1);
    let expected_source_classes = BTreeSet::from([
        "runtime-callsite-formatting",
        "runtime-dns",
        "runtime-global-http-client",
        "runtime-http2",
        "runtime-js-engine",
        "runtime-locale-time-data",
        "runtime-node-api",
        "runtime-tls-trust-store",
        "runtime-url-setter",
        "runtime-version-metadata",
        "runtime-windows-task-runner",
        "selected-esm-loader-fs-patchability",
        "unselected-corepack",
        "unselected-npm",
        "unshipped-ci-or-test",
        "unshipped-docs-or-build-tooling",
    ]);
    assert_eq!(
        object_keys(&source["classifications"]),
        expected_source_classes
    );
    assert_eq!(
        object_keys(&source["classification_reasons"]),
        expected_source_classes
    );
    let expected_source_counts = [
        ("runtime-callsite-formatting", 1),
        ("runtime-dns", 65),
        ("runtime-global-http-client", 9),
        ("runtime-http2", 2),
        ("runtime-js-engine", 1),
        ("runtime-locale-time-data", 5),
        ("runtime-node-api", 3),
        ("runtime-tls-trust-store", 401),
        ("runtime-url-setter", 1),
        ("runtime-version-metadata", 1),
        ("runtime-windows-task-runner", 1),
        ("selected-esm-loader-fs-patchability", 3),
        ("unselected-corepack", 4),
        ("unselected-npm", 210),
        ("unshipped-ci-or-test", 32),
        ("unshipped-docs-or-build-tooling", 16),
    ];
    let mut classified_source_paths = BTreeSet::new();
    for (classification, expected_count) in expected_source_counts {
        let class = &source["classifications"][classification];
        assert_eq!(object_keys(class), BTreeSet::from(["path_count", "paths"]));
        let paths = class["paths"]
            .as_array()
            .expect("source paths are an array");
        assert_eq!(paths.len(), expected_count, "{classification} path count");
        assert_eq!(class["path_count"], expected_count);
        for path in paths {
            assert!(
                classified_source_paths
                    .insert(path.as_str().expect("classified path is text").to_owned()),
                "duplicate source classification path: {path}"
            );
        }
    }
    let mut changed_source_paths = BTreeSet::new();
    assert_eq!(
        source["classifications"]["selected-esm-loader-fs-patchability"]["paths"],
        serde_json::json!([
            "lib/internal/modules/esm/load.js",
            "lib/internal/modules/esm/resolve.js",
            "lib/internal/modules/esm/translators.js"
        ])
    );
    assert_eq!(
        source["classifications"]["runtime-http2"]["paths"],
        serde_json::json!(["src/node_http2.cc", "src/node_http2.h"])
    );
    for row in source["changes"].as_array().unwrap() {
        assert_eq!(
            object_keys(row),
            BTreeSet::from([
                "classification",
                "from_blob",
                "from_mode",
                "path",
                "status",
                "to_blob",
                "to_mode",
            ])
        );
        assert!(row["classification"].as_str().is_some());
        assert!(row["from_blob"].as_str().is_some());
        assert!(row["to_blob"].as_str().is_some());
        assert!(
            changed_source_paths.insert(row["path"].as_str().unwrap().to_owned()),
            "duplicate upstream source path: {}",
            row["path"]
        );
        assert!(classified_source_paths.contains(row["path"].as_str().unwrap()));
        assert!(
            source["classifications"][row["classification"].as_str().unwrap()]["paths"]
                .as_array()
                .unwrap()
                .contains(&row["path"]),
            "source path has exactly its recorded classification"
        );
    }
    assert_eq!(changed_source_paths.len(), 755);
    assert_eq!(classified_source_paths, changed_source_paths);
    let protocol = json(PROTOCOL.as_bytes());
    assert_eq!(
        protocol["tuple"]["node_runtime"],
        CLAUDE_AGENT_SDK_NODE_RUNTIME
    );
    assert_eq!(protocol["tuple"]["sdk_version"], "0.3.284");
    assert_eq!(protocol["tuple"]["native_version"], "2.1.284");
    assert_eq!(
        protocol["tuple"]["sidecar_source_tag"],
        "swallowtail-claude-agent-sdk-sidecar@0.5.1"
    );
    assert_eq!(
        protocol["tuple"]["wire"],
        "swallowtail-claude-agent-sdk-jsonl-v1"
    );
    assert_eq!(
        protocol["tuple"]["behavior_revision"],
        "claude-agent.sdk-v1"
    );
    assert_eq!(
        protocol["selected_behavior"]["sdk_native_wire_permission_usage_configuration_and_tool_mappings_unchanged"],
        true
    );
    assert!(
        protocol["unmapped_runtime_deltas"]["tls_dns_global_http_http2"]
            .as_str()
            .unwrap()
            .contains("no live provider/HTTP-MCP evidence transfers")
    );
}

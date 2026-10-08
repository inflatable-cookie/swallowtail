use std::collections::BTreeSet;

use serde_json::{Value, json};
use sha2::{Digest, Sha256};

const IDENTITY: &str = include_str!("fixtures/pi-sdk-sidecar-1.1.0/identity.json");
const INVENTORY: &str = include_str!("fixtures/pi-sdk-sidecar-1.1.0/dist-inventory.json");
const PROTOCOL: &str = include_str!("fixtures/pi-sdk-sidecar-1.1.0/protocol.json");
const DEPENDENCIES: &str = include_str!("fixtures/pi-sdk-sidecar-1.1.0/dependency-surface.json");

const QUALIFIED_POINTS: [&str; 18] = [
    "0.84.2", "0.84.3", "0.84.4", "0.85.0", "0.85.1", "0.86.0", "0.86.1", "0.87.0", "0.87.1",
    "0.99.0", "0.99.1", "0.99.2", "1.0.0", "1.0.1", "1.0.2", "1.0.3", "1.0.4", "1.1.0",
];

const ROOT_TARBALL_SHA256: [&str; 18] = [
    "95b899cd7b1a0c1f0174c7bf33ab427435e3553a7d1f4756661aa9c7f1a68ffa",
    "d07dc417f78a14dac376a878b6556b51961f118f79771ee375333dc51356bc75",
    "5bce766d19c3ceba18f3fbaad91c449c9f9d73981f9e3400ecef932006f06968",
    "a0895f70a9efd9dde2a69b9cee04cb3b7c5aab68f5d47aad92b63f27a4ca13c8",
    "1f498729649bdce647d1160993b4d92bf3c614cc819213bee2f91dd34f2a7af4",
    "3f0f502f497c44888ddbd68a11897651b739eaabdf4b235e3a7ed9735353b0cd",
    "8dff93e6fa03e0d498e72a78d2c7bb5f094f5e06ee268e6abd000ba2984a0b6a",
    "9a6733c0e6a31d592b53dc60df43dd0c26fa793ccf2384ed123fc17b0448866a",
    "1423ee3c61e7c96464e1cbf3c8dc24d3056cb3410995c3671a98c3ecc527540f",
    "19a8dbd8d697a599ea972a8ba307c109a814bb9122604b2553c62006b9617724",
    "6686592adaea19092c85c94f5d40323dbf3db141e90eb3ede9e9e87302abdd1d",
    "5bb197bed8e46b5352a7a940ddc868c358725b214f27f3ad4d33e77ee9832558",
    "638ed3abbe54ef70cbf8673ae4bc531e791613756aac04644cfcdcc4af0fafaf",
    "99c2e1958ac6d4c6a36e7f1c3690ae38778bb397c63e9d611d2c09521be735c5",
    "eda5ae7875343bd902ffe55718fb65b2406d7b03abecc5cb89d8e4bb09ceeda2",
    "106eadb1f823f72f012c08f23bd36e435f9e62f6c81e98a5d8f70c8a9543dd05",
    "04910bdae661a6529e9d6869b04006f6aad01186398b04a16c6d9793667b96c5",
    "09cd8a0a43dbb1d81a67346b09400b439ce71d818caba4e963ea958846a1aed4",
];

const EXPECTED_MANIFESTS: [(&str, usize, &str); 18] = [
    (
        "0.84.2",
        972,
        "612303b0e49bb2b3c86c0c1a6e99dcc8263957ccefc7d2e6fc936516ae9d459f",
    ),
    (
        "0.84.3",
        1044,
        "af463ad048bcc55280bc4ade1bf4c76d7dc07ec1de8507063a2c6ce4f5c4c68a",
    ),
    (
        "0.84.4",
        1044,
        "bc44cbb135890605431fa7b34e44542d883b845d474dda0d3d589dd9cac06e1c",
    ),
    (
        "0.85.0",
        1249,
        "210be5fdd953056a6f45a52c58b83ad8bcb2865bd72e90cd2c9d71596a328678",
    ),
    (
        "0.85.1",
        1056,
        "ecae935670bfed3deafae10d796c20badc56a30438d5e3cb208b0a4ce8ca77e8",
    ),
    (
        "0.86.0",
        1094,
        "bff7fedd633cc4fde55a3e872303d3610e0483cb8af1e93c290abfa38eb54581",
    ),
    (
        "0.86.1",
        1100,
        "38920eb7b8cf3232eae59063a4f81b1c8fd6d7a6a2600fdab6206c79e2779ce5",
    ),
    (
        "0.87.0",
        1100,
        "3312cce2863588249ebbea6c4608d05a814fbb5ca2a39de86f7a655c5d8c7969",
    ),
    (
        "0.87.1",
        1108,
        "d6f039d6a5ba209287537c9f2ae5d71cc00acda15060c98ad496361b6fc17217",
    ),
    (
        "0.99.0",
        1226,
        "7ddfa796b9c5b70dcbe08d60119ede96d19ba11a9e1e7860bead859f1c00c052",
    ),
    (
        "0.99.1",
        1227,
        "913bf8cdb5db68249233691f66b15cffb147c3db39487abd6bea3838c157596c",
    ),
    (
        "0.99.2",
        1228,
        "4e2034d74e6d2f9ea9a49bee3b73e6b761c6dc1ce5922907eb05d79c01ef5586",
    ),
    (
        "1.0.0",
        1243,
        "14313b8e739381719bc844df12dc648dcf531924252e10551cd969e019488134",
    ),
    (
        "1.0.1",
        1243,
        "e0a3267a0269ba4a009ff7faeb42bf00e251ed45661c768e78b6ceb424dcef03",
    ),
    (
        "1.0.2",
        1243,
        "a31b4fbf5ee8aae33dd4ee46a0637e850f1f5ddf8114d02a667ed5e38453f662",
    ),
    (
        "1.0.3",
        1248,
        "4a47c39bc26dc237e97779c44ad2f77698c255d4b4438b7fc52a8455c857180c",
    ),
    (
        "1.0.4",
        1248,
        "3d433e7858f53a48b6a693cb0791e38abe54c5e942b8c1ca1976553581403ab8",
    ),
    (
        "1.1.0",
        1254,
        "62fb9de5c48a64d72f8f8fb2ad7a1f7aae15ca389dc52348b7d973cc9bdf82c8",
    ),
];

const EXPECTED_HOP_COUNTS: [(usize, usize, usize, usize); 17] = [
    (72, 0, 209, 763),
    (7, 7, 94, 943),
    (246, 41, 207, 796),
    (20, 213, 48, 988),
    (61, 23, 233, 800),
    (10, 4, 41, 1049),
    (4, 4, 109, 987),
    (14, 6, 60, 1034),
    (139, 21, 570, 517),
    (4, 3, 17, 1206),
    (29, 28, 109, 1090),
    (30, 15, 106, 1107),
    (30, 30, 115, 1098),
    (14, 14, 24, 1205),
    (35, 30, 42, 1171),
    (14, 14, 85, 1149),
    (40, 34, 142, 1072),
];

const SELECTED_HOP_FILE_SETS: [(usize, &str); 17] = [
    (
        30,
        "7e1cdc23e1dd1b3b8d4fa497ac09c828103b3d0cee0d28c3555c53f9df916e07",
    ),
    (
        11,
        "8115a6aa97842ea806712ff55ec78c149ceb5d6030b5e06446afb274d7989c25",
    ),
    (
        27,
        "76fd7e3a7172927b4229e5d56671011f18d0aa47738b242a2e513f87145e310b",
    ),
    (
        0,
        "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
    ),
    (
        32,
        "504fed523c5216d5da07c8c732e73d074797058dbd660620e21f4bfdbc26cc7b",
    ),
    (
        2,
        "82852f4fa3e1b1ca6e5490e62aab6653d53972d9e1e1790183b841de7550e431",
    ),
    (
        23,
        "492f326a0cb2f3123d21bb71f04b5096c5b3e8d0db628d5c0477126096543bfc",
    ),
    (
        2,
        "82852f4fa3e1b1ca6e5490e62aab6653d53972d9e1e1790183b841de7550e431",
    ),
    (
        37,
        "8a3da7bfd3e02c1c4ec3bc44557e6f4b4d0a9a801d92e580b199da709897307e",
    ),
    (
        2,
        "82852f4fa3e1b1ca6e5490e62aab6653d53972d9e1e1790183b841de7550e431",
    ),
    (
        10,
        "ef374cff51d5ba08fb4d01c1458fd70e25524c137dd69fa857f123bf41fd2d13",
    ),
    (
        7,
        "d365a01e57f65c61c9ce6d425e72ff318142afa950dea74360ff535f3e703f6d",
    ),
    (
        10,
        "b3c037111e8804150350e7823b4390e9024d0e11dae8f314792b4d6cbc9e8aaf",
    ),
    (
        4,
        "7160e7e699113e8d7c2dffbeb5af3711cc2e35ae34f9e7a7164b99906851bfed",
    ),
    (
        4,
        "9aabfb9dc87dfb565609c8200f55f6c03570d9afc140c9942b328b675386576a",
    ),
    (
        10,
        "4ede3cbf66b19be1a54b92fbd24c9a4ae5bf8523f126386d3b3f0a8f7d78577e",
    ),
    (
        8,
        "6429fa3c2ee1848c0dacd794c7e5690ba048c8c7bb85bd83d2e343b9e001c61a",
    ),
];

const SELECTED_SOURCES: [&str; 29] = [
    "core/agent-session-runtime.ts",
    "core/agent-session-services.ts",
    "core/agent-session.ts",
    "core/cache-warmer.ts",
    "core/defaults.ts",
    "core/extensions/index.ts",
    "core/extensions/loader.ts",
    "core/extensions/runner.ts",
    "core/messages.ts",
    "core/model-config.ts",
    "core/model-resolver.ts",
    "core/model-runtime.ts",
    "core/models-store.ts",
    "core/provider-composer.ts",
    "core/remote-catalog-provider.ts",
    "core/resource-loader.ts",
    "core/sdk.ts",
    "core/session-manager.ts",
    "core/settings-manager.ts",
    "core/system-prompt.ts",
    "core/tools/find.ts",
    "core/tools/grep.ts",
    "core/tools/index.ts",
    "core/tools/ls.ts",
    "core/tools/output-accumulator.ts",
    "core/tools/read.ts",
    "core/tools/truncate.ts",
    "core/usage-totals.ts",
    "index.ts",
];

fn sha256(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

fn adjacent_hops() -> Vec<String> {
    QUALIFIED_POINTS
        .windows(2)
        .map(|pair| format!("{}->{}", pair[0], pair[1]))
        .collect()
}

fn key_set(value: &Value) -> BTreeSet<&str> {
    value
        .as_object()
        .expect("frozen value is an object")
        .keys()
        .map(String::as_str)
        .collect()
}

#[test]
fn exact_npm_artifact_identities_and_current_channels_are_frozen() {
    assert_eq!(
        sha256(IDENTITY.as_bytes()),
        "9f3d0599902dfb2a802c4a3062d31fa7fcb620016e1a2184526a907ef2a10768"
    );
    let identity: Value = serde_json::from_str(IDENTITY).expect("identity fixture is valid JSON");
    assert_eq!(identity["route"], "pi.sdk-sidecar");
    assert_eq!(identity["npm_package"], "@earendil-works/pi-coding-agent");
    assert_eq!(identity["official_channels"]["npm_dist_tag"], "latest");
    assert_eq!(identity["official_channels"]["npm_latest"], "1.1.0");
    assert_eq!(identity["official_channels"]["github_latest_tag"], "v1.1.0");
    assert_eq!(
        identity["official_channels"]["github_latest_commit"],
        "abe508e1b89912adde45528136c3221eb69acdd7"
    );
    assert_eq!(identity["official_channels"]["latest_channels_agree"], true);
    assert_eq!(
        identity["previous_qualification"]["latest_qualified"],
        "0.84.2"
    );
    assert_eq!(identity["runtime_identity"]["node_runtime"], "22.23.2");
    assert_eq!(
        identity["runtime_identity"]["node_engine_requirement"],
        ">=22.19.0"
    );
    assert_eq!(
        identity["runtime_identity"]["official_artifacts_executed"],
        false
    );

    let artifacts = identity["official_artifacts"]
        .as_array()
        .expect("one frozen artifact identity per selected point");
    assert_eq!(artifacts.len(), QUALIFIED_POINTS.len());
    for (index, (artifact, expected_version)) in artifacts.iter().zip(QUALIFIED_POINTS).enumerate()
    {
        assert_eq!(artifact["version"], expected_version);
        assert_eq!(artifact["tarball_sha256"], ROOT_TARBALL_SHA256[index]);
    }
    assert_eq!(
        identity["official_channels"]["published_stables_from_previous_ceiling"],
        json!(QUALIFIED_POINTS[1..])
    );
}

#[test]
fn complete_package_trees_and_every_published_hop_are_mutation_sensitive() {
    assert_eq!(
        sha256(INVENTORY.as_bytes()),
        "530be30b3d00af21ec91c916eea56e67ff2422f68558f14105d95ba8f53a1564"
    );
    let inventory: Value =
        serde_json::from_str(INVENTORY).expect("complete package inventory is valid JSON");
    assert_eq!(inventory["package"], "@earendil-works/pi-coding-agent");
    assert_eq!(
        inventory["manifest_scope"],
        "Every regular file in each official npm package tarball, package-relative path and SHA-256"
    );
    assert_eq!(inventory["compared"], json!(QUALIFIED_POINTS));

    let manifests = inventory["manifests"]
        .as_object()
        .expect("every selected artifact has a complete manifest");
    let expected_manifest_keys: BTreeSet<&str> = QUALIFIED_POINTS.into_iter().collect();
    let actual_manifest_keys: BTreeSet<&str> = manifests.keys().map(String::as_str).collect();
    assert_eq!(actual_manifest_keys, expected_manifest_keys);
    for (version, file_count, tree_sha256) in EXPECTED_MANIFESTS {
        assert_eq!(manifests[version]["file_count"], file_count);
        assert_eq!(manifests[version]["tree_sha256"], tree_sha256);
    }

    let file_hashes = inventory["hashes"]
        .as_object()
        .expect("file/hash matrix contains every published package path");
    assert_eq!(file_hashes.len(), 1741);
    for (version, file_count, _) in EXPECTED_MANIFESTS {
        let observed_paths = file_hashes
            .values()
            .filter(|versions| versions.get(version).is_some())
            .count();
        assert_eq!(
            observed_paths, file_count,
            "complete path set for {version}"
        );
    }

    let hops = inventory["hops"]
        .as_object()
        .expect("each adjacent published point has a delta ledger");
    let expected_hops = adjacent_hops();
    assert_eq!(
        hops.keys().map(String::as_str).collect::<Vec<_>>(),
        expected_hops.iter().map(String::as_str).collect::<Vec<_>>()
    );
    for (index, hop) in expected_hops.iter().enumerate() {
        let ledger = &hops[hop];
        let (added, removed, changed, identical) = EXPECTED_HOP_COUNTS[index];
        for (name, count) in [("added", added), ("removed", removed), ("changed", changed)] {
            let paths = ledger[name].as_array().expect("path delta is an array");
            assert_eq!(paths.len(), count, "{name} path count at {hop}");
            let values: Vec<&str> = paths
                .iter()
                .map(|path| path.as_str().expect("artifact path is text"))
                .collect();
            let sorted: Vec<&str> = {
                let mut sorted = values.clone();
                sorted.sort_unstable();
                sorted
            };
            assert_eq!(
                values, sorted,
                "{name} paths are deterministically sorted at {hop}"
            );
            let digest = if values.is_empty() {
                String::new()
            } else {
                format!("{}\n", values.join("\n"))
            };
            let expected_digest = ledger
                .get(format!("{name}_path_list_sha256").as_str())
                .and_then(Value::as_str)
                .expect("path-set digest accompanies each delta");
            assert_eq!(sha256(digest.as_bytes()), expected_digest);
        }
        assert_eq!(ledger["identical_count"], identical);
    }
}

#[test]
fn selected_sdk_files_facade_and_dependency_surface_are_exact() {
    assert_eq!(
        sha256(PROTOCOL.as_bytes()),
        "9eafade02f959ddb7f50e9a6d9bfb0c3c63d84b8310507288f8a2c2a9c052830"
    );
    assert_eq!(
        sha256(DEPENDENCIES.as_bytes()),
        "aaa5e47bd188dd41b9160b11055119d4be8b153ebc1d8fcc147ac4065f0ecef8"
    );
    let protocol: Value =
        serde_json::from_str(PROTOCOL).expect("selected SDK source ledger is valid JSON");
    assert_eq!(protocol["previous_qualified_ceiling"], "0.84.2");
    assert_eq!(protocol["current_official_stable"], "1.1.0");
    assert_eq!(
        protocol["source_entrypoint"],
        "@earendil-works/pi-coding-agent/dist/index.js"
    );
    assert_eq!(
        key_set(&protocol["hops"]),
        adjacent_hops().iter().map(String::as_str).collect()
    );
    assert_eq!(
        key_set(&protocol["selected_source_classifications"]),
        SELECTED_SOURCES.into_iter().collect()
    );
    assert_eq!(
        key_set(&protocol["selected_source_hashes"]),
        QUALIFIED_POINTS.into_iter().collect()
    );

    let expected_facade = &protocol["selected_facade"];
    assert_eq!(
        expected_facade["operations"],
        json!([
            "catalogue",
            "new_session",
            "load_session",
            "resume_session",
            "prompt",
            "steer",
            "follow_up",
            "abort",
            "state",
            "close"
        ])
    );
    assert_eq!(
        expected_facade["tools"],
        json!(["read", "grep", "find", "ls"])
    );
    assert_eq!(
        expected_facade["usage_projection"],
        json!(["input", "output", "cacheRead", "cacheWrite"])
    );
    assert_eq!(expected_facade["node_runtime"], "22.23.2");
    assert_eq!(
        protocol["adaptation"]["cache_warming"]["introduced"],
        "0.86.0"
    );
    assert_eq!(
        protocol["adaptation"]["cache_warming"]["upstream_default"],
        "streaming"
    );
    assert_eq!(
        protocol["adaptation"]["agent_settled_aborted"]["introduced"],
        "1.1.0"
    );
    assert!(
        protocol["adaptation"]["agent_settled_aborted"]["mapping"]
            .as_str()
            .expect("mapping classification is text")
            .contains("without projecting")
    );

    let source_classes = protocol["selected_source_classifications"]
        .as_object()
        .expect("selected source files have classifications");
    let protocol_hops = protocol["hops"]
        .as_object()
        .expect("per-hop source file sets exist");
    let expected_hops = adjacent_hops();
    for (index, hop) in expected_hops.iter().enumerate() {
        let files = protocol_hops[hop]["selected_source_files"]
            .as_array()
            .expect("selected changed files are an exact array");
        let (expected_count, expected_digest) = SELECTED_HOP_FILE_SETS[index];
        assert_eq!(files.len(), expected_count, "selected file count at {hop}");
        let mut paths = Vec::with_capacity(files.len());
        let mut unique_paths = BTreeSet::new();
        for file in files {
            let path = file["path"]
                .as_str()
                .expect("compiled artifact path is text");
            let source = file["source"].as_str().expect("source map source is text");
            assert_eq!(
                file["classification"], source_classes[source],
                "classification for {path} at {hop}"
            );
            assert!(
                unique_paths.insert(path),
                "duplicate selected file {path} at {hop}"
            );
            paths.push(path);
        }
        paths.sort_unstable();
        let path_list = if paths.is_empty() {
            String::new()
        } else {
            format!("{}\n", paths.join("\n"))
        };
        assert_eq!(
            sha256(path_list.as_bytes()),
            expected_digest,
            "file set at {hop}"
        );
    }

    let dependencies: Value =
        serde_json::from_str(DEPENDENCIES).expect("dependency artifact ledger is valid JSON");
    assert_eq!(
        protocol["dependency_scope"]["selected_direct_dependencies"],
        json!(["@earendil-works/pi-ai", "@earendil-works/pi-agent-core"])
    );
    let model_versions = dependencies["packages"]["@earendil-works/pi-ai"]
        .as_array()
        .expect("pi-ai artifact is frozen at each exact point");
    assert_eq!(model_versions.len(), QUALIFIED_POINTS.len());
    for (index, (row, point)) in model_versions.iter().zip(QUALIFIED_POINTS).enumerate() {
        assert_eq!(row["version"], point);
        assert_eq!(
            row["root_declared_range"],
            format!("^{point}"),
            "root package declaration at {point}"
        );
        let model = &row["claude_opus_4_5_model"];
        assert_eq!(model["id"], "claude-opus-4-5");
        assert_eq!(model["provider"], "anthropic");
        assert_eq!(model["api"], "anthropic-messages");
        assert_eq!(model["reasoning"], true);
        assert_eq!(
            row["claude_opus_4_5_registry_key"],
            if index < 9 {
                "claude-opus-4-5"
            } else {
                "chat:claude-opus-4-5"
            }
        );
    }
    assert_eq!(
        protocol["dependency_scope"]["root_package_dependencies"]
            .as_str()
            .expect("dependency limitation is recorded")
            .contains("host-resolved transitive tree is not asserted"),
        true
    );
}

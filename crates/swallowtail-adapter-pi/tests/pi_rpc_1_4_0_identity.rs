use serde_json::Value;
use sha2::{Digest, Sha256};
use swallowtail_adapter_pi::{
    PI_PACKAGE_BASELINE_VERSION, PI_PACKAGE_LATEST_QUALIFIED_VERSION, pi_rpc_claim,
};
use swallowtail_core::{
    InterfaceCompatibilityAssessment, InterfaceSupportStatus, InterfaceVersion,
};

const IDENTITY: &str = include_str!("fixtures/pi-rpc-1.4.0/identity.json");
const PROTOCOL: &str = include_str!("fixtures/pi-rpc-1.4.0/protocol.json");
const INVENTORY: &str = include_str!("fixtures/pi-rpc-1.4.0/dist-inventory.json");

const VERSIONS: [&str; 4] = ["1.1.0", "1.2.0", "1.3.0", "1.4.0"];

const ARTIFACTS: [(&str, usize, &str, &str); 4] = [
    (
        "1.1.0",
        1254,
        "62fb9de5c48a64d72f8f8fb2ad7a1f7aae15ca389dc52348b7d973cc9bdf82c8",
        "09cd8a0a43dbb1d81a67346b09400b439ce71d818caba4e963ea958846a1aed4",
    ),
    (
        "1.2.0",
        1275,
        "7748aa457a31ebd6183bf445e0b1a8e78f41bc03f7becb49ebd22fd8ee64622e",
        "1dfed308f46a69aec06bf9dc20700ddc6fe2370b1898f01334b7898397541e66",
    ),
    (
        "1.3.0",
        1276,
        "c12dbb0f8483e29f4e1f418fcfc1a3139d076cbae05d49d26867abbf7a9b7e6e",
        "ae2785e870b81a8c8e1d44510a57b5ee0bfc9e974679ac2faaada4aabee3d3c0",
    ),
    (
        "1.4.0",
        1279,
        "fb51c1573be247d23c53a214ec773bb0669236b67d9be055a94d697fd5ffe13f",
        "d155c16ef6fa00960fbf58b2911536e80b05f33700e0b14fbd08967ede0c7428",
    ),
];

const HOP_PATH_HASHES: [(&str, &str, &str, &str, &str); 3] = [
    (
        "1.1.0->1.2.0",
        "e5b4a727abc0fd3aa309dccd898cad1cdc71d0bcab8562c7070c01b4b1480155",
        "185065bf3f39e17f199585c870d23d3b08829f31c5b189380118a04a2bde6aac",
        "8367c6eb9e3173565c477486be4d439ff952d2ab686cb3553a733751cb9502d6",
        "6ffc8cdaa1f6eff4ac06a4bf74e4a7deed6c3a2fd70dbfabba1c39f9a4d33dd1",
    ),
    (
        "1.2.0->1.3.0",
        "db76bdadcfbee5382271ef98c61b8a7bd050ee2247965f80e2d689ee722b08c0",
        "841f5a7b815edd0e7ab007e736145d8cc06d38c35238679440cb5b294c8d0f4a",
        "2856829b4363ee8d5fdcca7b2450c39134893bfa08472704c210b0e70b803799",
        "e65d1204c9709f156d3f645bd40aeae14c1f141a8e298f37099283cc2ebda3e1",
    ),
    (
        "1.3.0->1.4.0",
        "a4273d090934cecfd50d03689ee6ea93b8b0ff9205ee8bb7d3f52f2c4992e8bc",
        "77bfeacd395240a8863c21686371340d7c7ec3dea1702437f92c9308daee049b",
        "ab695d54ca1d20828b88b3af202688147e2deb42812557c5cf05c4d14059127b",
        "72d41a31e67f49445d19209c5cc5dd062e0d9527bb58b2d19b572874456a2c19",
    ),
];

const HOP_COUNTS: [(&str, usize, usize, usize, usize); 3] = [
    ("1.1.0->1.2.0", 62, 41, 153, 1060),
    ("1.2.0->1.3.0", 19, 18, 88, 1169),
    ("1.3.0->1.4.0", 59, 56, 95, 1125),
];

const SOURCE_RPC_MODE: &str = "packages/coding-agent/src/modes/rpc/rpc-mode.ts";
const SOURCE_AGENT_SESSION: &str = "packages/coding-agent/src/core/agent-session.ts";
const SOURCE_SETTINGS_MANAGER: &str = "packages/coding-agent/src/core/settings-manager.ts";
const SOURCE_RESOURCE_LOADER: &str = "packages/coding-agent/src/core/resource-loader.ts";
const SOURCE_ARGS: &str = "packages/coding-agent/src/cli/args.ts";
const SOURCE_GREP: &str = "packages/coding-agent/src/core/tools/grep.ts";
const SOURCE_SETTINGS_DEFAULTS: &str = "packages/coding-agent/src/core/settings-defaults.ts";
const SOURCE_SETTINGS_SCHEMA: &str = "packages/coding-agent/src/core/settings-schema.ts";

const RPC_MODE_SHA256: &str = "d534e0fa1484844a097963d5f4fb4df9397c5a1b5e29e42aed7575ab6f67989b";
const ARGS_SHA256: &str = "ef7b54f02939d2ebc8ec2defcf4c2d9be90fe979c2c74675d970a4dd14aadad3";
const GREP_SHA256_THROUGH_1_3_0: &str =
    "88be3d00217d1a1caf8a9e7bb2b8d2e6d96edfa4b790c2cf76c746ab36b8841b";
const GREP_SHA256_1_4_0: &str = "37a906d99672bd043d6f729e0dc09589f46ffc87bdf3d2944fc24331c0ebc2a0";
const SETTINGS_DEFAULTS_SHA256_1_2_0: &str =
    "6faa340bfdd818d3ec779cbd3be4626b0cbad976d39a11eb3f549a0af63b9a44";
const SETTINGS_DEFAULTS_SHA256_1_3_0: &str =
    "6fb94031cfd462e357723a35aab328e9a1ed5109c2aa59549d8291345875d5a9";
const SETTINGS_SCHEMA_SHA256_1_2_0: &str =
    "7c7c19e791b16c5bf47d08b1a919a4e15c56a54c47a55a09f29f9be0c9e5ebe7";
const SETTINGS_SCHEMA_SHA256_1_3_0: &str =
    "1679b3f28b4ad7941e7b891f9c8d6a9db8e3a5c8902f8bc9eea86b00f9622ca9";

const HOP_SOURCE_DIGESTS: [(&str, &[(&str, &str)]); 4] = [
    (
        "1.1.0",
        &[
            (SOURCE_RPC_MODE, RPC_MODE_SHA256),
            (
                SOURCE_AGENT_SESSION,
                "19186bf8a79876131a296fe17333c6276a671f8f48401df238c9814eeba0f5df",
            ),
            (
                SOURCE_SETTINGS_MANAGER,
                "d916365ae21bce0f9a4f3ad15690e290ceca0aad958013e3280779b0cde0b074",
            ),
            (
                SOURCE_RESOURCE_LOADER,
                "dbe87c11983b8dac949377904f4cf0511155771498ba88d2fefb0cd0d5dd9e28",
            ),
            (SOURCE_ARGS, ARGS_SHA256),
            (SOURCE_GREP, GREP_SHA256_THROUGH_1_3_0),
        ],
    ),
    (
        "1.2.0",
        &[
            (SOURCE_RPC_MODE, RPC_MODE_SHA256),
            (
                SOURCE_AGENT_SESSION,
                "e2bbf746779fbed5a8c36f023842bf271252d1fd6f17f5a32f70a0d1c63f0b65",
            ),
            (
                SOURCE_SETTINGS_MANAGER,
                "754fc0ee6289fda207136f9b5a389cab6ef4de2e35cc69c8010967ca9d371624",
            ),
            (
                SOURCE_RESOURCE_LOADER,
                "ccc798224d070e7a18c918f74be48c15eacc67f76aaa66b9809c015327b555d4",
            ),
            (SOURCE_ARGS, ARGS_SHA256),
            (SOURCE_GREP, GREP_SHA256_THROUGH_1_3_0),
            (SOURCE_SETTINGS_DEFAULTS, SETTINGS_DEFAULTS_SHA256_1_2_0),
            (SOURCE_SETTINGS_SCHEMA, SETTINGS_SCHEMA_SHA256_1_2_0),
        ],
    ),
    (
        "1.3.0",
        &[
            (SOURCE_RPC_MODE, RPC_MODE_SHA256),
            (
                SOURCE_AGENT_SESSION,
                "e0fa206e8c49a98c024708068df39c0e4944f4251633425fc083b2fbb5ec0ac6",
            ),
            (
                SOURCE_SETTINGS_MANAGER,
                "aef8fea51000f291c1c51d81fd9b30f7a2bdd0e68370ad84b969e5fd15667e98",
            ),
            (
                SOURCE_RESOURCE_LOADER,
                "ccc798224d070e7a18c918f74be48c15eacc67f76aaa66b9809c015327b555d4",
            ),
            (SOURCE_ARGS, ARGS_SHA256),
            (SOURCE_GREP, GREP_SHA256_THROUGH_1_3_0),
            (SOURCE_SETTINGS_DEFAULTS, SETTINGS_DEFAULTS_SHA256_1_3_0),
            (SOURCE_SETTINGS_SCHEMA, SETTINGS_SCHEMA_SHA256_1_3_0),
        ],
    ),
    (
        "1.4.0",
        &[
            (SOURCE_RPC_MODE, RPC_MODE_SHA256),
            (
                SOURCE_AGENT_SESSION,
                "e0a6e3e7b662afff9763a6297e0bb600da75e8eff4df443912f2e215bdc6a8e2",
            ),
            (
                SOURCE_SETTINGS_MANAGER,
                "aef8fea51000f291c1c51d81fd9b30f7a2bdd0e68370ad84b969e5fd15667e98",
            ),
            (
                SOURCE_RESOURCE_LOADER,
                "ccc798224d070e7a18c918f74be48c15eacc67f76aaa66b9809c015327b555d4",
            ),
            (SOURCE_ARGS, ARGS_SHA256),
            (SOURCE_GREP, GREP_SHA256_1_4_0),
            (SOURCE_SETTINGS_DEFAULTS, SETTINGS_DEFAULTS_SHA256_1_3_0),
            (SOURCE_SETTINGS_SCHEMA, SETTINGS_SCHEMA_SHA256_1_3_0),
        ],
    ),
];

const SELECTED_CORE_PATHS: &[&str] = &[
    "dist/core/agent-session.js",
    "dist/core/agent-session.js.map",
    "dist/core/auth-storage.js",
    "dist/core/auth-storage.js.map",
    "dist/core/client-identity.js",
    "dist/core/client-identity.js.map",
    "dist/core/compaction/compaction.js",
    "dist/core/compaction/compaction.js.map",
    "dist/core/compaction/utils.js",
    "dist/core/compaction/utils.js.map",
    "dist/core/mcp-servers.js",
    "dist/core/mcp-servers.js.map",
    "dist/core/model-config.js",
    "dist/core/model-config.js.map",
    "dist/core/model-resolver.js",
    "dist/core/model-resolver.js.map",
    "dist/core/model-runtime.js",
    "dist/core/model-runtime.js.map",
    "dist/core/provider-attribution.js",
    "dist/core/provider-attribution.js.map",
    "dist/core/resource-loader.js",
    "dist/core/resource-loader.js.map",
    "dist/core/sdk.js",
    "dist/core/sdk.js.map",
    "dist/core/settings-defaults.js",
    "dist/core/settings-defaults.js.map",
    "dist/core/settings-manager.js",
    "dist/core/settings-manager.js.map",
    "dist/core/settings-schema.js",
    "dist/core/settings-schema.js.map",
    "dist/core/tools/grep.js",
    "dist/core/tools/grep.js.map",
    "dist/cli/args.js",
    "dist/cli/args.js.map",
    "dist/cli/setup.js",
    "dist/cli/setup.js.map",
    "dist/extensions/index.js",
    "dist/extensions/index.js.map",
];

const RELEASE_IDENTITIES: [(&str, &str, &str, &str, u64); 4] = [
    (
        "1.1.0",
        "sha512-SeEi/4hdcHNgA9UWlefZl7ZZpm3dzi2OoxNjDHsBJ9o298LNOtbL4DGKgitlEj6uCTccvtw6f2hlCkTPVJ2RXg==",
        "abe508e1b89912adde45528136c3221eb69acdd7",
        "0ebd064bf1342f812f9676dde85b2f0b3e02b7e6",
        406210577,
    ),
    (
        "1.2.0",
        "sha512-h/4WaVWv1VlCaY8x8qEN5U+7jbF/e7+Ck6moPeEOFn5juMTWwNAXjwgYM7gRgbBYh8Zppwt6qWvJDzNTf4GNEQ==",
        "06a466935536f8c9bcacb2d5efc0fc021be1bdff",
        "896fb3ad2b050190ac0f0f8acce7fb648487cda1",
        411055414,
    ),
    (
        "1.3.0",
        "sha512-V/WoCjA2hQSe3QLWIUB24cTRdtEdeLiIzkDqU5RMXV6VWuD+KGZnzVBMhglwdnQTySkehZsjt8a3g6Jd2g4nnw==",
        "71023aec1546d6d677a65669c5665c0927ce0dd0",
        "a52ed0ef8ba4fd03de221ec52b0996855bf5bf4d",
        414165830,
    ),
    (
        "1.4.0",
        "sha512-Z41LpZ3yYHSC/UUps6Mr39UMFPwH8elhJBG9DY1P7kuzsE5qSK/6eMj2EHzO5evJRQWLbKSqUMw+4nCNtIyiQg==",
        "4a080252f06caf63a525d7873c5d94d3c23887e0",
        "7f5dd2a4f7fa46ed252465ac4d341831896b1fc2",
        420119608,
    ),
];

#[test]
fn official_identity_preserves_exact_channels_and_artifact_digests() {
    let identity = json(IDENTITY);
    assert_eq!(identity["axis"], "pi.package");
    assert_eq!(identity["selected_route"], "pi.rpc");
    assert_eq!(identity["npm_package"], "@earendil-works/pi-coding-agent");
    assert_eq!(identity["official_channels"]["npm_latest"], "1.4.0");
    assert_eq!(identity["official_channels"]["github_latest_tag"], "v1.4.0");
    assert_eq!(
        identity["official_channels"]["github_latest_commit"],
        "4a080252f06caf63a525d7873c5d94d3c23887e0"
    );
    assert_eq!(identity["official_channels"]["latest_channels_agree"], true);
    assert_eq!(
        identity["previous_qualification"]["latest_qualified"],
        "1.1.0"
    );
    assert_eq!(
        identity["claim_at_observation"]["latest_qualified"],
        "1.1.0"
    );
    assert_eq!(
        identity["claim_at_observation"]["classification_of_1_4_0"],
        "unverified_newer"
    );
    assert_eq!(
        identity["runtime_identity"]["npm_engine_requirement"],
        ">=22.19.0"
    );
    assert_eq!(identity["runtime_identity"]["host_pi_on_path"], false);
    assert_eq!(
        identity["runtime_identity"]["official_artifact_executed"],
        false
    );
    assert_eq!(identity["host_observation"]["pi_on_path"], false);
    assert_eq!(identity["host_observation"]["node_version"], "v22.23.4");
    assert_eq!(identity["execution_limits"]["provider_prompt_sent"], false);
    assert_eq!(identity["execution_limits"]["live_rpc_session"], false);
    assert_eq!(identity["execution_limits"]["credentials_used"], false);
    assert_eq!(identity["execution_limits"]["package_installation"], false);
    assert_eq!(identity["execution_limits"]["host_mutation"], false);
    assert_eq!(
        strings(&identity["official_channels"]["published_stables_after_previous_ceiling"]),
        ["1.2.0", "1.3.0", "1.4.0"]
    );

    let artifacts = identity["official_artifacts"]
        .as_array()
        .expect("official artifacts");
    assert_eq!(artifacts.len(), ARTIFACTS.len());
    for (artifact, (version, count, tree_sha256, tarball_sha256)) in artifacts.iter().zip(ARTIFACTS)
    {
        assert_exact_keys(
            artifact,
            &[
                "complete_tree_sha256",
                "github_release_id",
                "github_release_published_at",
                "github_tag",
                "github_tag_commit",
                "npm_files",
                "npm_git_head",
                "npm_git_head_matches_tag",
                "npm_integrity",
                "npm_published_at",
                "npm_shasum",
                "npm_tarball_sha256",
                "version",
            ],
        );
        assert_eq!(artifact["version"], version);
        assert_eq!(artifact["npm_files"], count);
        assert_eq!(artifact["complete_tree_sha256"], tree_sha256);
        assert_eq!(artifact["npm_tarball_sha256"], tarball_sha256);
        assert_eq!(artifact["github_tag"], format!("v{version}"));
        assert_eq!(artifact["npm_git_head"], Value::Null);
        assert_eq!(artifact["npm_git_head_matches_tag"], Value::Null);
    }
    for (artifact, (version, integrity, tag_commit, shasum, release_id)) in
        artifacts.iter().zip(RELEASE_IDENTITIES)
    {
        assert_eq!(artifact["version"], version);
        assert_eq!(artifact["npm_integrity"], integrity);
        assert_eq!(artifact["npm_shasum"], shasum);
        assert_eq!(artifact["github_release_id"], release_id);
        assert_eq!(artifact["github_tag_commit"], tag_commit);
    }
    let checks = identity["latest_source_correlation"]["checks"]
        .as_array()
        .expect("latest tagged source checks");
    let (_, latest_expected) = HOP_SOURCE_DIGESTS[3];
    assert_eq!(checks.len(), latest_expected.len());
    for (check, (source, digest)) in checks.iter().zip(latest_expected) {
        assert_eq!(check["source"], *source);
        assert_eq!(check["sha256"], *digest);
        assert_eq!(check["matches_tag"], "v1.4.0");
    }
    assert_eq!(
        identity["latest_source_correlation"]["npm_sourcemap_sources_content_matches_tagged_source"],
        true
    );
    assert!(
        identity["latest_source_correlation"]["executed_rpc_mode"]
            .as_str()
            .is_some_and(|value| value.contains("bundled CLI chunk")
                && value.contains("rpc-mode.ts sourcesContent"))
    );
    let hops = identity["latest_source_correlation"]["every_hop"]
        .as_array()
        .expect("per-hop source correlation");
    assert_eq!(hops.len(), HOP_SOURCE_DIGESTS.len());
    for (hop, (version, expected)) in hops.iter().zip(HOP_SOURCE_DIGESTS) {
        assert_eq!(hop["version"], version);
        let hop_checks = hop["checks"].as_array().expect("hop source checks");
        assert_eq!(hop_checks.len(), expected.len(), "{version} source checks");
        for (check, (source, digest)) in hop_checks.iter().zip(expected.iter()) {
            assert_eq!(check["source"], *source, "{version} source path");
            assert_eq!(check["sha256"], *digest, "{version} {source}");
            assert_eq!(check["matches_tag"], format!("v{version}"));
        }
    }
}

#[test]
fn complete_package_trees_and_each_published_hop_are_hash_checked() {
    let identity = json(IDENTITY);
    let inventory = json(INVENTORY);
    assert_eq!(inventory["package"], "@earendil-works/pi-coding-agent");
    assert_eq!(strings(&inventory["compared"]), VERSIONS);
    let hashes = inventory["hashes"]
        .as_object()
        .expect("package file hashes");

    for (version, count, expected_tree, _) in ARTIFACTS {
        let mut rows = hashes
            .iter()
            .filter_map(|(path, versions)| {
                versions
                    .get(version)
                    .and_then(Value::as_str)
                    .map(|digest| (path.as_str(), digest))
            })
            .collect::<Vec<_>>();
        rows.sort_unstable_by_key(|(path, _)| *path);
        assert_eq!(rows.len(), count, "{version} complete file count");
        let mut tree = Vec::new();
        for (path, digest) in rows {
            tree.extend_from_slice(path.as_bytes());
            tree.push(b'\t');
            tree.extend_from_slice(digest.as_bytes());
            tree.push(b'\n');
        }
        assert_eq!(sha256_hex(&tree), expected_tree, "{version} recursive tree");
        assert_eq!(inventory["manifests"][version]["file_count"], count);
        assert_eq!(
            inventory["manifests"][version]["tree_sha256"],
            expected_tree
        );
        let artifact = identity["official_artifacts"]
            .as_array()
            .expect("official artifacts")
            .iter()
            .find(|artifact| artifact["version"] == version)
            .expect("matching official artifact");
        assert_eq!(artifact["complete_tree_sha256"], expected_tree);
    }

    let hop_records = inventory["hops"].as_object().expect("published hops");
    assert_eq!(hop_records.len(), HOP_PATH_HASHES.len());
    for (
        (hop_name, added_sha256, removed_sha256, changed_sha256, identical_sha256),
        (_, added_count, removed_count, changed_count, identical_count),
    ) in HOP_PATH_HASHES.into_iter().zip(HOP_COUNTS)
    {
        let hop = &inventory["hops"][hop_name];
        assert_exact_keys(
            hop,
            &[
                "added",
                "added_path_list_sha256",
                "changed",
                "changed_path_list_sha256",
                "identical_count",
                "identical_path_list_sha256",
                "removed",
                "removed_path_list_sha256",
            ],
        );
        assert_path_list_hash(&hop["added"], added_sha256);
        assert_path_list_hash(&hop["removed"], removed_sha256);
        assert_path_list_hash(&hop["changed"], changed_sha256);
        assert_eq!(hop["added_path_list_sha256"], added_sha256);
        assert_eq!(hop["removed_path_list_sha256"], removed_sha256);
        assert_eq!(hop["changed_path_list_sha256"], changed_sha256);
        assert_eq!(hop["identical_path_list_sha256"], identical_sha256);

        let (before, after) = hop_name.split_once("->").expect("hop version pair");
        let added = strings(&hop["added"]);
        let removed = strings(&hop["removed"]);
        let changed = strings(&hop["changed"]);
        assert_eq!(added.len(), added_count, "{hop_name} additions");
        assert_eq!(removed.len(), removed_count, "{hop_name} removals");
        assert_eq!(changed.len(), changed_count, "{hop_name} changes");
        let changed_paths = added
            .iter()
            .chain(removed.iter())
            .chain(changed.iter())
            .copied()
            .collect::<std::collections::BTreeSet<_>>();
        let mut identical = hashes
            .iter()
            .filter_map(|(path, versions)| {
                let old = versions.get(before).and_then(Value::as_str)?;
                let new = versions.get(after).and_then(Value::as_str)?;
                (old == new && !changed_paths.contains(path.as_str())).then_some(path.as_str())
            })
            .collect::<Vec<_>>();
        identical.sort_unstable();
        assert_eq!(
            identical.len(),
            identical_count,
            "{hop_name} unchanged paths"
        );
        assert_eq!(hop["identical_count"], identical.len());
        assert_eq!(sha256_hex(&path_list_bytes(&identical)), identical_sha256);
    }
}

#[test]
fn selected_rpc_hops_extend_the_aborted_settled_revision() {
    let identity = json(IDENTITY);
    let protocol = json(PROTOCOL);
    let inventory = json(INVENTORY);
    assert_eq!(
        protocol["scope"],
        "pi.rpc only; no Pi SDK sidecar, Oh My Pi, provider API or public operation claim"
    );
    assert_eq!(protocol["previous_qualified_ceiling"], "1.1.0");
    assert_eq!(protocol["current_official_stable"], "1.4.0");
    assert_eq!(protocol["selected_invocation"]["selected_mode"], "rpc");
    assert_eq!(
        protocol["selected_invocation"]["upstream_rpc_command_count"],
        33
    );
    assert_eq!(
        protocol["selected_invocation"]["extension_paths"],
        serde_json::json!([])
    );
    assert_eq!(
        strings(&protocol["selected_invocation"]["adapter_selected_rpc_commands"]),
        [
            "prompt",
            "steer",
            "follow_up",
            "abort",
            "get_state",
            "get_available_models",
            "set_auto_compaction",
            "set_auto_retry",
            "set_steering_mode",
            "set_follow_up_mode",
        ]
    );
    let commands = strings(&protocol["selected_invocation"]["upstream_rpc_commands"]);
    assert_eq!(commands.len(), 33);
    for selected in strings(&protocol["selected_invocation"]["adapter_selected_rpc_commands"]) {
        assert!(
            commands.contains(&selected),
            "missing selected command {selected}"
        );
    }
    assert_eq!(
        strings(&protocol["selected_invocation"]["adapter_selected_rpc_absences"]),
        [
            "bash",
            "switch_session",
            "fork",
            "clone",
            "extensions",
            "clear_queue",
            "compact",
        ]
    );
    for absent in [
        "bash",
        "switch_session",
        "fork",
        "clone",
        "clear_queue",
        "compact",
    ] {
        assert!(
            commands.contains(&absent),
            "missing upstream command {absent}"
        );
        assert!(
            !strings(&protocol["selected_invocation"]["adapter_selected_rpc_commands"])
                .contains(&absent)
        );
    }
    assert!(
        !commands.contains(&"extensions"),
        "extensions is a launch flag, not an RPC command"
    );
    let rpc_files = protocol["mapped_behavior"]["rpc_tree_byte_identical"]["files"]
        .as_object()
        .expect("byte-identical rpc files");
    assert_eq!(rpc_files.len(), 16);
    for (path, digest) in rpc_files {
        let versions = inventory["hashes"][path]
            .as_object()
            .expect("rpc file hashes");
        let values = VERSIONS
            .into_iter()
            .map(|version| versions[version].as_str().expect("rpc digest"))
            .collect::<Vec<_>>();
        assert!(values.iter().all(|value| *value == values[0]));
        assert_eq!(digest, values[0]);
    }
    assert_eq!(
        protocol["mapped_behavior"]["command_dispositions"]["unchanged_through"],
        "1.4.0"
    );
    assert_eq!(
        protocol["mapped_behavior"]["agent_settled_aborted"]["unchanged_through"],
        "1.4.0"
    );
    assert_eq!(
        identity["identity_decision"]["shape"],
        "compatible-extension"
    );
    assert_eq!(identity["identity_decision"]["latest_qualified"], "1.4.0");
    assert_eq!(
        strings(&identity["identity_decision"]["maintained_points"]),
        ["1.1.0", "1.2.0", "1.3.0", "1.4.0"]
    );
    assert_eq!(
        identity["identity_decision"]["same_revision_segments_maintained"],
        true
    );
    assert!(identity["identity_decision"]["demote_1_1_0_to_deprecated"].is_null());
    assert_eq!(
        identity["identity_decision"]["new_behavior_revision"],
        false
    );
    assert_eq!(
        identity["identity_decision"]["new_public_operations"],
        false
    );
    assert_eq!(
        identity["identity_decision"]["public_lifecycle_contract_changed"],
        false
    );
    assert_eq!(
        identity["identity_decision"]["sdk_sidecar_or_oh_my_pi_qualification_changed"],
        false
    );
    assert_eq!(
        identity["identity_decision"]["preserve_behavior_revision"],
        "pi.rpc.strict-lf-v1.1.0-aborted-settled"
    );

    let hop_records = protocol["per_hop_classifications"]
        .as_array()
        .expect("per-hop classifications");
    assert_eq!(hop_records.len(), HOP_PATH_HASHES.len());
    for (index, hop) in hop_records.iter().enumerate() {
        let (expected_name, _, _, _, _) = HOP_PATH_HASHES[index];
        let (before, after) = expected_name.split_once("->").expect("hop pair");
        assert_eq!(hop["from"], before);
        assert_eq!(hop["to"], after);
        let runtime_transition = &hop["compiled_runtime_transition"];
        assert_eq!(runtime_transition["from"]["version"], before);
        assert_eq!(runtime_transition["to"]["version"], after);
        for runtime in [&runtime_transition["from"], &runtime_transition["to"]] {
            let version = runtime["version"].as_str().expect("runtime version");
            let chunk = runtime["main_chunk"].as_str().expect("runtime chunk path");
            assert_eq!(
                inventory["hashes"][chunk][version], runtime["main_chunk_sha256"],
                "{version} compiled runtime chunk"
            );
            assert_eq!(
                inventory["hashes"]["dist/bundle/cli-runtime.js"][version],
                runtime["runtime_import_sha256"]
            );
            assert_eq!(
                inventory["hashes"]["dist/bundle/cli.js"][version],
                runtime["launcher_sha256"]
            );
            assert_eq!(runtime["mcp_option_present"], true);
            for marker in [
                "agent_settled",
                "--mode",
                "--no-session",
                "--offline",
                "--provider",
                "--model",
                "--tools",
                "--no-extensions",
                "--no-context-files",
            ] {
                assert_eq!(
                    runtime["selected_markers"][marker], true,
                    "{version} {marker}"
                );
            }
            assert_eq!(
                runtime["selected_markers"]["--no-config"],
                version == "1.4.0",
                "{version} --no-config"
            );
        }

        let recorded = hop["selected_source_files"]
            .as_array()
            .expect("selected changed source files");
        for entry in recorded {
            assert_exact_keys(entry, &["change", "classification", "path"]);
        }
        let mut recorded_paths = recorded
            .iter()
            .map(|entry| {
                (
                    entry["change"].as_str().expect("change kind"),
                    entry["path"].as_str().expect("source path"),
                )
            })
            .collect::<Vec<_>>();
        recorded_paths.sort_unstable();
        let changed = &inventory["hops"][expected_name];
        let mut inventory_paths = Vec::new();
        for change in ["added", "removed", "changed"] {
            for path in strings(&changed[change]) {
                if selected_source_path(path) {
                    inventory_paths.push((change, path));
                }
            }
        }
        inventory_paths.sort_unstable();
        assert_eq!(
            recorded_paths, inventory_paths,
            "{expected_name} selected file set"
        );
        assert!(recorded.iter().all(|entry| {
            entry["classification"]
                .as_str()
                .is_some_and(|value| !value.is_empty())
        }));
        match expected_name {
            "1.1.0->1.2.0" => {
                assert_classification_contains(
                    recorded,
                    "dist/core/agent-session.js",
                    "consumer cancel can land inside preparation",
                );
                assert_classification_contains(
                    recorded,
                    "dist/core/settings-defaults.js",
                    "sets steering, follow-up, compaction, and retry over RPC before any prompt",
                );
            }
            "1.2.0->1.3.0" => {
                assert_classification_contains(
                    recorded,
                    "dist/core/settings-defaults.js",
                    "sessionShareDestination default auto",
                );
                assert_classification_contains(
                    recorded,
                    "dist/core/settings-schema.js",
                    "/share is not a selected RPC operation",
                );
                assert_classification_contains(
                    recorded,
                    "dist/core/agent-session.js",
                    "plain --tools",
                );
                assert_classification_contains(
                    recorded,
                    "dist/core/model-resolver.js",
                    "--provider/--model",
                );
            }
            "1.3.0->1.4.0" => {
                assert_classification_contains(recorded, "dist/core/tools/grep.js", "--no-config");
                assert_classification_contains(
                    recorded,
                    "dist/core/agent-session.js",
                    "set_auto_compaction enabled:false",
                );
                assert_classification_contains(
                    recorded,
                    "dist/core/agent-session.js",
                    "plain --tools",
                );
                assert_classification_contains(
                    recorded,
                    "dist/core/model-resolver.js",
                    "--provider/--model",
                );
                assert_classification_contains(recorded, "dist/core/sdk.js", "usesDefaultTools");
                assert_classification_contains(
                    recorded,
                    "dist/core/client-identity.js",
                    "request-identity",
                );
                assert_classification_contains(
                    recorded,
                    "dist/core/provider-attribution.js",
                    "Removed",
                );
            }
            other => panic!("unexpected hop {other}"),
        }
    }

    assert_eq!(PI_PACKAGE_BASELINE_VERSION, "0.80.10");
    assert_eq!(PI_PACKAGE_LATEST_QUALIFIED_VERSION, "1.4.0");
    let claim = pi_rpc_claim();
    assert_eq!(claim.id().as_str(), "pi.rpc.package-window-2");
    for (version, status) in [
        ("1.1.0", InterfaceSupportStatus::Maintained),
        ("1.2.0", InterfaceSupportStatus::Maintained),
        ("1.3.0", InterfaceSupportStatus::Maintained),
        ("1.4.0", InterfaceSupportStatus::Maintained),
    ] {
        assert!(matches!(
            claim.assess(&version_value(version)),
            InterfaceCompatibilityAssessment::Qualified(matched)
                if matched.behavior_revision().as_str()
                    == "pi.rpc.strict-lf-v1.1.0-aborted-settled"
                    && matched.support_status() == status
        ));
    }
    for gap in ["1.1.1", "1.2.1", "1.3.1", "1.0.5", "0.99.3"] {
        assert!(
            matches!(
                claim.assess(&version_value(gap)),
                InterfaceCompatibilityAssessment::Incompatible
            ),
            "{gap}"
        );
    }
    let InterfaceCompatibilityAssessment::UnverifiedNewer(newer) =
        claim.assess(&version_value("1.4.1"))
    else {
        panic!("stable after 1.4.0 remains unverified");
    };
    assert_eq!(
        newer.behavior_revision().as_str(),
        "pi.rpc.strict-lf-v1.1.0-aborted-settled"
    );
    assert!(!claim.permits(&version_value("1.4.1-rc.1")));
}

fn json(value: &str) -> Value {
    serde_json::from_str(value).expect("frozen identity JSON is valid")
}

fn strings(value: &Value) -> Vec<&str> {
    value
        .as_array()
        .expect("value is an array")
        .iter()
        .map(|value| value.as_str().expect("array value is text"))
        .collect()
}

fn assert_path_list_hash(value: &Value, expected: &str) {
    let paths = strings(value);
    assert!(paths.windows(2).all(|pair| pair[0] < pair[1]));
    assert_eq!(sha256_hex(&path_list_bytes(&paths)), expected);
}

fn path_list_bytes(paths: &[&str]) -> Vec<u8> {
    let mut bytes = paths.join("\n").into_bytes();
    bytes.push(b'\n');
    bytes
}

fn sha256_hex(value: &[u8]) -> String {
    Sha256::digest(value)
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

fn assert_exact_keys(value: &Value, expected: &[&str]) {
    let mut keys = value
        .as_object()
        .expect("JSON value is an object")
        .keys()
        .map(String::as_str)
        .collect::<Vec<_>>();
    keys.sort_unstable();
    let mut expected = expected.to_vec();
    expected.sort_unstable();
    assert_eq!(keys, expected);
}

fn selected_source_path(path: &str) -> bool {
    path.starts_with("dist/modes/rpc/")
        || SELECTED_CORE_PATHS.contains(&path)
        || path.starts_with("dist/extensions/mcp/")
        || path.starts_with("dist/extensions/codemode/")
        || path.starts_with("dist/extensions/tool-search/")
}

fn assert_classification_contains(recorded: &[Value], path: &str, needle: &str) {
    let classification = recorded
        .iter()
        .find(|entry| entry["path"] == path)
        .and_then(|entry| entry["classification"].as_str())
        .unwrap_or_else(|| panic!("missing selected classification for {path}"));
    assert!(
        classification.contains(needle),
        "{path} classification missing {needle:?}: {classification}"
    );
}

fn version_value(value: &str) -> InterfaceVersion {
    InterfaceVersion::new(value).expect("version fixture is valid")
}

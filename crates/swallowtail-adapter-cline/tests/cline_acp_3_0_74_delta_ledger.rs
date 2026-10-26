use std::collections::{BTreeMap, BTreeSet};

use base64::{Engine, engine::general_purpose::STANDARD};
use serde_json::Value;
use sha2::{Digest, Sha256};
use swallowtail_adapter_cline::{CLINE_PACKAGE_VERSION, cline_acp_claim, cline_headless_claim};
use swallowtail_core::{
    InterfaceCompatibilityAssessment, InterfaceSupportStatus, InterfaceVersion,
};

const IDENTITY: &str = include_str!("fixtures/cline-acp-3.0.74/release-identity.json");
const DIST: &str = include_str!("fixtures/cline-acp-3.0.74/dist-inventory.json");
const PROVENANCE: &str = include_str!("fixtures/cline-acp-3.0.74/provenance.json");
const SOURCE_TREE: &str = include_str!("fixtures/cline-acp-3.0.74/source-tree-inventory.json");
const PROTOCOL: &str = include_str!("fixtures/cline-acp-3.0.74/protocol.json");
const PRIOR_DIST: &str = include_str!("fixtures/cline-acp-3.0.70/dist-inventory.json");

const VERSIONS: &[&str] = &["3.0.70", "3.0.71", "3.0.72", "3.0.73", "3.0.74"];
const EARLIER_QUALIFIED: &[&str] = &[
    "3.0.55", "3.0.56", "3.0.57", "3.0.58", "3.0.60", "3.0.61", "3.0.62", "3.0.63", "3.0.64",
    "3.0.65", "3.0.66", "3.0.67", "3.0.68", "3.0.69",
];
const PLATFORM_PACKAGES: &[&str] = &[
    "@cline/cli-linux-x64",
    "@cline/cli-darwin-x64",
    "@cline/cli-linux-arm64",
    "@cline/cli-windows-x64",
    "@cline/cli-darwin-arm64",
    "@cline/cli-windows-arm64",
];
const DIST_PACKAGES: &[&str] = &["cline", "@cline/cli-darwin-arm64"];

#[test]
fn official_latest_extends_only_the_existing_acp_window() {
    assert_eq!(
        sha256(IDENTITY),
        "fe8e53dde3957f1b2ff76dda8b21fbd2d1cccd912d910a24cacc493720e51c4e"
    );
    assert_eq!(
        sha256(DIST),
        "40feb32cc1329b031c377c4a2ea9f1e8a031b76b0f0279e209da5edb880d4cec"
    );
    assert_eq!(
        sha256(PROVENANCE),
        "4d4a4da15fd5622600d81241531557c5b3fbea2bae5e3ffdea16149c1750e4fa"
    );
    assert_eq!(
        sha256(SOURCE_TREE),
        "cb97b02ff3a3fb504d74a3f2b8e5b35829258f0bd5b06367ae25113f7ba7b387"
    );
    assert_eq!(
        sha256(PROTOCOL),
        "5f4db0fd045dae7a7705d5c987e095f11ef5045f8898c6545b2bfe63cb40e750"
    );

    let identity = json(IDENTITY);
    let channel = &identity["official_channel"];
    assert_eq!(identity["route"], "cline.acp");
    assert_eq!(identity["axis"], "cline.package");
    assert_eq!(channel["registry"], "https://registry.npmjs.org/cline");
    assert_eq!(channel["selection"], "dist-tags.latest");
    assert_eq!(channel["latest"], "3.0.74");
    assert_eq!(channel["latest_published_at"], "2026-10-26T04:04:22.234Z");
    assert_eq!(channel["reprobed_after_source_review"], true);
    assert_eq!(channel["previous_qualified_ceiling"], "3.0.70");
    assert_exact_strings(&channel["stable_published_versions_in_scope"], VERSIONS);
    assert_eq!(channel["unpublished_next_stable"]["version"], "3.0.75");
    assert_eq!(
        channel["unpublished_next_stable"]["registry_version_exists"],
        false
    );
    assert_eq!(channel["github_releases_latest"]["tag"], "v4.1.26");
    assert!(
        channel["github_releases_latest"]["disposition"]
            .as_str()
            .unwrap()
            .contains("Editor product")
    );

    let dependency_lock = &identity["dependency_lock"];
    assert_eq!(dependency_lock["package_manager"], "Bun");
    assert_eq!(dependency_lock["lock_file"], "bun.lock");
    assert_eq!(
        dependency_lock["acp_sdk_package"],
        "@agentclientprotocol/sdk"
    );
    assert_eq!(dependency_lock["app_manifest_declaration"], "^0.16.1");
    assert_exact_object_keys(
        &dependency_lock["resolved_version_by_published_hop"],
        VERSIONS,
    );
    for version in VERSIONS {
        assert_eq!(
            dependency_lock["resolved_version_by_published_hop"][*version], "0.16.1"
        );
    }

    let hole = &channel["unpublished_hole"];
    assert_eq!(hole["version"], "3.0.59");
    assert_eq!(hole["registry_version_exists"], false);
    assert_eq!(hole["disposition"], "excluded");

    let claim = cline_acp_claim();
    assert_eq!(claim.baseline().as_str(), CLINE_PACKAGE_VERSION);
    assert_eq!(claim.latest_qualified().as_str(), "3.0.74");
    assert_eq!(claim.id().as_str(), "cline.acp.package-window-1");
    for version in EARLIER_QUALIFIED.iter().chain(VERSIONS) {
        let version = InterfaceVersion::new(*version).expect("published stable version");
        let matched = claim
            .classify(&version)
            .expect("published point stays qualified");
        assert_eq!(matched.support_status(), InterfaceSupportStatus::Maintained);
        assert_eq!(matched.behavior_revision().as_str(), "cline.acp.stdio-v1");
    }
    let excluded = InterfaceVersion::new("3.0.59").expect("unpublished stable version");
    assert!(!claim.assess(&excluded).is_permitted());
    let unverified = InterfaceVersion::new("3.0.75").expect("next stable version");
    assert!(matches!(
        claim.assess(&unverified),
        InterfaceCompatibilityAssessment::UnverifiedNewer(_)
    ));

    let headless = cline_headless_claim();
    assert_eq!(headless.latest_qualified().as_str(), "3.0.70");
    assert!(headless.permits(&InterfaceVersion::new("3.0.70").expect("headless ceiling")));
    assert!(!headless.permits(&excluded));
    for version in ["3.0.71", "3.0.72", "3.0.73", "3.0.74", "3.0.75"] {
        assert!(
            !headless.permits(&InterfaceVersion::new(version).expect("above headless pin")),
            "{version} stays outside headless"
        );
    }
}

#[test]
fn every_selected_npm_tree_and_hop_set_is_reproducible() {
    let inventory = json(DIST);
    let prior = json(PRIOR_DIST);
    assert_eq!(inventory["latest"], "3.0.74");
    assert_exact_object_keys(&inventory["packages"], DIST_PACKAGES);
    for package in DIST_PACKAGES {
        assert_eq!(
            inventory["packages"][*package]["versions"]["3.0.70"]["tree_manifest_sha256"],
            prior["packages"][*package]["versions"]["3.0.70"]["tree_manifest_sha256"],
            "{package} 3.0.70 tree continues the frozen corpus"
        );
        assert_eq!(
            inventory["packages"][*package]["versions"]["3.0.70"]["tarball_sha256"],
            prior["packages"][*package]["versions"]["3.0.70"]["tarball_sha256"]
        );
        let package_record = &inventory["packages"][*package];
        assert_exact_object_keys(&package_record["versions"], VERSIONS);
        let hop_names = VERSIONS
            .windows(2)
            .map(|pair| format!("{}_to_{}", pair[0], pair[1]))
            .collect::<Vec<_>>();
        assert_exact_object_keys(&package_record["hops"], &hop_names);

        let mut trees = BTreeMap::<&str, BTreeMap<String, String>>::new();
        for version in VERSIONS {
            let artifact = &package_record["versions"][*version];
            assert_eq!(artifact["integrity_matches_download"], true);
            assert_eq!(artifact["shasum_matches_download"], true);
            assert_eq!(
                artifact["file_count"].as_u64(),
                u64::try_from(artifact["files"].as_array().unwrap().len()).ok()
            );
            assert_eq!(
                artifact["unpacked_size_registry"],
                artifact["unpacked_size_inventory"]
            );
            assert_sha256(&artifact["tarball_sha256"]);
            assert_sha1(&artifact["shasum"]);
            let files = artifact["files"].as_array().expect("complete package tree");
            let mut tree = BTreeMap::new();
            let mut previous = None;
            for file in files {
                assert_exact_object_keys(file, &["path", "size", "sha256"]);
                let path = file["path"].as_str().expect("file path").to_owned();
                assert!(previous.as_ref().is_none_or(|old: &String| old < &path));
                previous = Some(path.clone());
                assert_sha256(&file["sha256"]);
                assert!(
                    tree.insert(path, file["sha256"].as_str().unwrap().to_owned())
                        .is_none()
                );
            }
            assert_eq!(
                tree_manifest_digest(files),
                artifact["tree_manifest_sha256"].as_str().unwrap()
            );
            trees.insert(version, tree);
        }
        for pair in VERSIONS.windows(2) {
            let hop_name = format!("{}_to_{}", pair[0], pair[1]);
            let expected = tree_delta(&trees[pair[0]], &trees[pair[1]]);
            let actual = &package_record["hops"][hop_name];
            for category in ["added", "removed", "changed", "identical"] {
                assert_exact_strings(&actual[category], &expected[category]);
            }
        }
    }

    let darwin = &inventory["packages"]["@cline/cli-darwin-arm64"];
    assert_eq!(darwin["versions"]["3.0.73"]["file_count"].as_u64(), Some(142));
    assert_eq!(darwin["versions"]["3.0.74"]["file_count"].as_u64(), Some(2));
    assert_exact_strings(
        &darwin["hops"]["3.0.73_to_3.0.74"]["changed"],
        &["bin/cline", "package.json"],
    );
    assert!(
        darwin["hops"]["3.0.73_to_3.0.74"]["added"]
            .as_array()
            .unwrap()
            .is_empty()
    );
    let removed = strings(&darwin["hops"]["3.0.73_to_3.0.74"]["removed"]);
    assert_eq!(removed.len(), 140);
    assert!(removed.iter().any(|path| path == "extensions/plugin-sandbox-bootstrap.js"));
    assert!(removed.iter().any(|path| path.starts_with("cline-hub/")));
    assert_exact_strings(
        &inventory["packages"]["cline"]["hops"]["3.0.73_to_3.0.74"]["changed"],
        &["README.md", "package.json"],
    );
    for hop in ["3.0.70_to_3.0.71", "3.0.71_to_3.0.72", "3.0.72_to_3.0.73"] {
        assert_exact_strings(
            &inventory["packages"]["cline"]["hops"][hop]["changed"],
            &["package.json"],
        );
    }
}

#[test]
fn platform_file_counts_and_provenance_share_each_wrapper_source_commit() {
    let identity = json(IDENTITY);
    let provenance = json(PROVENANCE);
    let artifact_inventory = json(DIST);
    assert_exact_strings(&provenance["versions"], VERSIONS);
    assert_exact_strings(&provenance["platform_packages"], PLATFORM_PACKAGES);
    assert_eq!(
        provenance["artifacts"].as_array().unwrap().len(),
        VERSIONS.len() * 7
    );
    let counts = &identity["official_channel"]["platform_registry_file_counts"];
    assert_eq!(counts["3.0.73"]["cline"].as_u64(), Some(6));
    assert_eq!(counts["3.0.74"]["cline"].as_u64(), Some(6));
    for package in PLATFORM_PACKAGES {
        assert_eq!(counts["3.0.73"][*package].as_u64(), Some(142));
        assert_eq!(counts["3.0.74"][*package].as_u64(), Some(2));
    }
    assert_eq!(
        identity["official_channel"]["runtime_packaging"]["retained_paths"],
        json(r#"["bin/cline","package.json"]"#)
    );

    let mut rows = BTreeMap::<(&str, &str), &Value>::new();
    for artifact in provenance["artifacts"].as_array().unwrap() {
        let package = artifact["package"].as_str().unwrap();
        let version = artifact["version"].as_str().unwrap();
        assert!(rows.insert((package, version), artifact).is_none());
    }
    let source_commits = identity["source_identity"]["source_commits"]
        .as_object()
        .expect("source commit map");
    assert_exact_object_keys(&identity["source_identity"]["source_commits"], VERSIONS);
    assert_exact_object_keys(&identity["source_identity"]["cli_tags"], VERSIONS);
    for version in VERSIONS {
        let wrapper = rows[&("cline", *version)];
        let source_commit = wrapper["slsa"][0]["source_commit"].as_str().unwrap();
        assert_eq!(source_commits[*version], source_commit);
        assert_eq!(
            identity["source_identity"]["cli_tags"][*version]["commit"],
            source_commit
        );
        assert_eq!(
            identity["source_identity"]["cli_tags"][*version]["tag"],
            format!("cli-v{version}")
        );
        assert_eq!(wrapper["slsa"][0]["repository"], "https://github.com/cline/cline");
        assert_eq!(
            wrapper["slsa"][0]["workflow"],
            ".github/workflows/cli-publish.yml"
        );
        assert_eq!(wrapper["slsa"][0]["workflow_ref"], "refs/heads/main");
        assert_eq!(wrapper["registry_sha512_matches_slsa_subject"], true);
        assert_slsa_digest_matches_registry(wrapper);
        for package in PLATFORM_PACKAGES {
            let artifact = rows[&(*package, *version)];
            assert_eq!(artifact["registry_sha512_matches_slsa_subject"], true);
            assert_eq!(artifact["slsa"][0]["source_commit"], source_commit);
            assert_eq!(
                artifact["slsa"][0]["repository"],
                "https://github.com/cline/cline"
            );
            assert_slsa_digest_matches_registry(artifact);
            let frozen = &identity["source_identity"]["platform_attestations"][*version][*package];
            assert_eq!(frozen["source_commit"], source_commit);
            assert_eq!(frozen["registry_integrity"], artifact["registry_integrity"]);
            if DIST_PACKAGES.contains(package) {
                assert_eq!(
                    artifact["registry_integrity"],
                    artifact_inventory["packages"][*package]["versions"][*version]["integrity"]
                );
            }
        }
    }
}

#[test]
fn selected_source_hops_and_route_classifications_match_exact_file_sets() {
    let identity = json(IDENTITY);
    let source = json(SOURCE_TREE);
    let protocol = json(PROTOCOL);
    assert_eq!(source["repository"], "https://github.com/cline/cline");
    assert_exact_strings(&source["versions"], VERSIONS);
    assert_eq!(protocol["baseline"], CLINE_PACKAGE_VERSION);
    assert_eq!(protocol["qualified_through"], "3.0.74");
    assert_eq!(protocol["previous_qualified_ceiling"], "3.0.70");
    assert_exact_strings(
        &protocol["selected_methods"],
        &[
            "initialize",
            "session/new",
            "session/set_config_option",
            "session/prompt",
            "session/cancel",
        ],
    );
    assert_exact_strings(&protocol["selected_command"], &["cline", "--acp"]);
    assert!(
        protocol["preserved_unmapped_methods"]
            .as_array()
            .unwrap()
            .iter()
            .any(|method| method == "session/load")
    );
    assert_eq!(protocol["provider_work"]["provider_prompt_sent"], false);
    assert_eq!(protocol["provider_work"]["live_acp_initialize"], false);
    assert_eq!(protocol["provider_work"]["artifacts_executed"], false);
    assert_eq!(
        identity["bounded_unmapped"]["mcp_client"]["from_git_blob_sha1"],
        "16849bb01963bc415a3de06572d308b1afd7767f"
    );
    assert_eq!(
        identity["bounded_unmapped"]["mcp_client"]["to_git_blob_sha1"],
        "76f82111bb5f8356dabda10a07d2fd8970f7454c"
    );

    let mut source_trees = BTreeMap::<&str, BTreeMap<String, String>>::new();
    for version in VERSIONS {
        let files = source["files_by_version"][*version]
            .as_object()
            .expect("complete selected source tree");
        for required in [
            "apps/cli/package.json",
            "apps/cli/src/acp/acpAgent.ts",
            "apps/cli/src/acp/permissions.ts",
            "bun.lock",
        ] {
            assert!(files.contains_key(required), "{version} contains {required}");
        }
        let mut tree = BTreeMap::new();
        for (path, metadata) in files {
            assert_exact_object_keys(metadata, &["git_blob_sha1"]);
            assert_sha1(&metadata["git_blob_sha1"]);
            assert!(
                tree.insert(
                    path.clone(),
                    metadata["git_blob_sha1"].as_str().unwrap().to_owned()
                )
                .is_none()
            );
        }
        assert_eq!(
            source_tree_digest(&tree),
            source["tree_manifest_sha256"][*version].as_str().unwrap()
        );
        source_trees.insert(version, tree);
        assert_eq!(
            identity["source_identity"]["source_commits"][*version],
            source["source_commits"][*version]
        );
    }
    assert_eq!(source_trees["3.0.70"].len(), 49);
    assert_eq!(source_trees["3.0.74"].len(), 52);
    assert!(!source_trees["3.0.70"].contains_key("apps/cli/src/acp/config-options.ts"));
    assert!(source_trees["3.0.74"].contains_key("apps/cli/src/acp/config-options.ts"));
    assert!(source_trees["3.0.71"].contains_key("apps/cli/src/acp/reasoning.test.ts"));

    let source_hops = source["hops"].as_array().expect("source hop ledger");
    let protocol_hops = protocol["published_hops"].as_array().expect("classified hops");
    assert_eq!(source_hops.len(), VERSIONS.len() - 1);
    assert_eq!(protocol_hops.len(), source_hops.len());
    for (index, (source_hop, protocol_hop)) in source_hops.iter().zip(protocol_hops).enumerate() {
        assert_eq!(source_hop["from"], VERSIONS[index]);
        assert_eq!(source_hop["to"], VERSIONS[index + 1]);
        assert_eq!(protocol_hop["from"], source_hop["from"]);
        assert_eq!(protocol_hop["to"], source_hop["to"]);
        assert_eq!(
            protocol_hop["from_source_commit"],
            source_hop["from_source_commit"]
        );
        assert_eq!(
            protocol_hop["to_source_commit"],
            source_hop["to_source_commit"]
        );
        let expected = tree_delta(
            &source_trees[VERSIONS[index]],
            &source_trees[VERSIONS[index + 1]],
        );
        for category in ["added", "removed", "changed", "identical"] {
            assert_exact_strings(&source_hop[category], &expected[category]);
            assert_eq!(protocol_hop[category], source_hop[category]);
        }
        assert!(
            !protocol_hop["selected_route_classification"]
                .as_str()
                .unwrap()
                .is_empty()
        );
        let delta_paths = ["added", "removed", "changed"]
            .into_iter()
            .flat_map(|category| strings(&source_hop[category]))
            .collect::<BTreeSet<_>>();
        let classifications = protocol_hop["file_classifications"]
            .as_object()
            .expect("every changed path has a classification");
        assert_eq!(
            classifications.keys().cloned().collect::<BTreeSet<_>>(),
            delta_paths
        );
        for classification in classifications.values() {
            assert!(classification.as_str().is_some_and(|value| !value.is_empty()));
        }
    }
}

fn json(source: &str) -> Value {
    serde_json::from_str(source).expect("valid frozen JSON fixture")
}

fn assert_exact_object_keys<S: AsRef<str>>(value: &Value, expected: &[S]) {
    let actual = value
        .as_object()
        .expect("JSON object")
        .keys()
        .cloned()
        .collect::<BTreeSet<_>>();
    let expected = expected.iter().map(|key| key.as_ref().to_owned()).collect();
    assert_eq!(actual, expected);
}

fn assert_exact_strings<S: AsRef<str>>(value: &Value, expected: &[S]) {
    let actual = strings(value);
    let expected = expected
        .iter()
        .map(|entry| entry.as_ref().to_owned())
        .collect::<Vec<_>>();
    assert_eq!(actual, expected);
}

fn strings(value: &Value) -> Vec<String> {
    value
        .as_array()
        .expect("string array")
        .iter()
        .map(|entry| entry.as_str().expect("string item").to_owned())
        .collect()
}

fn assert_sha256(value: &Value) {
    assert_hex_digest(value, 64);
}

fn assert_sha1(value: &Value) {
    assert_hex_digest(value, 40);
}

fn assert_hex_digest(value: &Value, length: usize) {
    let digest = value.as_str().expect("hex digest");
    assert_eq!(digest.len(), length);
    assert!(digest.bytes().all(|byte| byte.is_ascii_hexdigit()));
}

fn assert_slsa_digest_matches_registry(artifact: &Value) {
    let registry_sri = artifact["registry_integrity"].as_str().unwrap();
    let registry_digest = registry_sri.strip_prefix("sha512-").expect("sha512 SRI");
    let registry_bytes = STANDARD.decode(registry_digest).expect("base64 SHA-512");
    assert_eq!(registry_bytes.len(), 64);
    assert_eq!(
        artifact["slsa"][0]["subject"][0]["digest"]["sha512"],
        hex(&registry_bytes)
    );
}

fn tree_manifest_digest(files: &[Value]) -> String {
    let mut canonical = String::new();
    for file in files {
        canonical.push_str(file["path"].as_str().expect("path"));
        canonical.push('\0');
        canonical.push_str(&file["size"].as_u64().expect("file size").to_string());
        canonical.push('\0');
        canonical.push_str(file["sha256"].as_str().expect("file SHA-256"));
        canonical.push('\n');
    }
    sha256(&canonical)
}

fn source_tree_digest(tree: &BTreeMap<String, String>) -> String {
    let mut canonical = String::new();
    for (path, blob) in tree {
        canonical.push_str(path);
        canonical.push('\0');
        canonical.push_str(blob);
        canonical.push('\n');
    }
    sha256(&canonical)
}

fn tree_delta(
    before: &BTreeMap<String, String>,
    after: &BTreeMap<String, String>,
) -> BTreeMap<&'static str, Vec<String>> {
    let before_paths = before.keys().cloned().collect::<BTreeSet<_>>();
    let after_paths = after.keys().cloned().collect::<BTreeSet<_>>();
    let added = after_paths.difference(&before_paths).cloned().collect();
    let removed = before_paths.difference(&after_paths).cloned().collect();
    let mut changed = Vec::new();
    let mut identical = Vec::new();
    for path in before_paths.intersection(&after_paths) {
        if before[path] == after[path] {
            identical.push(path.clone());
        } else {
            changed.push(path.clone());
        }
    }
    BTreeMap::from([
        ("added", added),
        ("removed", removed),
        ("changed", changed),
        ("identical", identical),
    ])
}

fn sha256(value: &str) -> String {
    format!("{:x}", Sha256::digest(value.as_bytes()))
}

fn hex(bytes: &[u8]) -> String {
    let mut output = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        use std::fmt::Write as _;
        write!(&mut output, "{byte:02x}").expect("writing to a string is infallible");
    }
    output
}

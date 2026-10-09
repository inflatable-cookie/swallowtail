use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};
use swallowtail_adapter_cline::{CLINE_PACKAGE_VERSION, cline_acp_claim, cline_headless_claim};
use swallowtail_core::{InterfaceSupportStatus, InterfaceVersion};

const IDENTITY: &str = include_str!("fixtures/cline-headless-3.0.70/identity.json");
const WRAPPER_TREE: &str =
    include_str!("fixtures/cline-headless-3.0.70/npm-wrapper-tree-inventory.json");
const DARWIN_TREE: &str =
    include_str!("fixtures/cline-headless-3.0.70/darwin-arm64-runtime-tree-inventory.json");
const SOURCE_TREE: &str =
    include_str!("fixtures/cline-headless-3.0.70/selected-source-tree-inventory.json");
const HOP_LEDGER: &str = include_str!("fixtures/cline-headless-3.0.70/hop-ledger.json");

const RELEASES: [&str; 15] = [
    "3.0.55", "3.0.56", "3.0.57", "3.0.58", "3.0.60", "3.0.61", "3.0.62", "3.0.63", "3.0.64",
    "3.0.65", "3.0.66", "3.0.67", "3.0.68", "3.0.69", "3.0.70",
];
const SELECTED_SOURCE_PATHS: [&str; 12] = [
    "apps/cli/src/commands/program.ts",
    "apps/cli/src/main.ts",
    "apps/cli/src/runtime/defaults.ts",
    "apps/cli/src/runtime/run-agent.ts",
    "apps/cli/src/runtime/session-events.ts",
    "apps/cli/src/runtime/tool-policies.ts",
    "apps/cli/src/utils/events.ts",
    "apps/cli/src/utils/helpers.ts",
    "apps/cli/src/utils/output.ts",
    "apps/cli/src/utils/startup-settings.ts",
    "sdk/packages/agents/src/agent-runtime.ts",
    "sdk/packages/shared/src/agents/types.ts",
];
const PLATFORM_ARTIFACTS: [&str; 6] = [
    "@cline/cli-darwin-arm64",
    "@cline/cli-darwin-x64",
    "@cline/cli-linux-arm64",
    "@cline/cli-linux-x64",
    "@cline/cli-windows-arm64",
    "@cline/cli-windows-x64",
];
const DEPENDENCY_ARTIFACTS: [&str; 5] = [
    "@cline/agents",
    "@cline/core",
    "@cline/llms",
    "@cline/sdk",
    "@cline/shared",
];

fn fixture(body: &str, name: &str) -> Value {
    serde_json::from_str(body).unwrap_or_else(|error| panic!("{name}: {error}"))
}

fn object<'a>(value: &'a Value, name: &str) -> &'a serde_json::Map<String, Value> {
    value
        .as_object()
        .unwrap_or_else(|| panic!("{name} is an object"))
}

fn strings(value: &Value, name: &str) -> Vec<String> {
    value
        .as_array()
        .unwrap_or_else(|| panic!("{name} is a list"))
        .iter()
        .map(|entry| {
            entry
                .as_str()
                .unwrap_or_else(|| panic!("{name} entries are text"))
                .to_owned()
        })
        .collect()
}

fn string_set(value: &Value, name: &str) -> BTreeSet<String> {
    strings(value, name).into_iter().collect()
}

fn file_map(value: &Value, name: &str) -> BTreeMap<String, (u64, String)> {
    let rows = value
        .as_array()
        .unwrap_or_else(|| panic!("{name} is a file list"));
    let mut map = BTreeMap::new();
    let mut ordered_paths = Vec::with_capacity(rows.len());
    for row in rows {
        let path = row["path"]
            .as_str()
            .unwrap_or_else(|| panic!("{name} paths are text"))
            .to_owned();
        let size = row["size"]
            .as_u64()
            .unwrap_or_else(|| panic!("{name} sizes are integers"));
        let hash = row["sha256"]
            .as_str()
            .unwrap_or_else(|| panic!("{name} digests are text"))
            .to_owned();
        assert_eq!(hash.len(), 64, "{name} {path} has a SHA-256 digest");
        ordered_paths.push(path.clone());
        assert!(
            map.insert(path.clone(), (size, hash)).is_none(),
            "{name} repeats {path}"
        );
    }
    let mut sorted_paths = ordered_paths.clone();
    sorted_paths.sort_unstable();
    assert_eq!(ordered_paths, sorted_paths, "{name} paths are sorted");
    map
}

fn assert_hop(
    from: &BTreeMap<String, (u64, String)>,
    to: &BTreeMap<String, (u64, String)>,
    record: &Value,
    name: &str,
) {
    let from_paths: BTreeSet<_> = from.keys().cloned().collect();
    let to_paths: BTreeSet<_> = to.keys().cloned().collect();
    let shared: BTreeSet<_> = from_paths.intersection(&to_paths).cloned().collect();
    let expected = [
        ("added", to_paths.difference(&from_paths).cloned().collect()),
        (
            "removed",
            from_paths.difference(&to_paths).cloned().collect(),
        ),
        (
            "changed",
            shared
                .iter()
                .filter(|path| from[*path].1 != to[*path].1)
                .cloned()
                .collect(),
        ),
        (
            "identical",
            shared
                .iter()
                .filter(|path| from[*path].1 == to[*path].1)
                .cloned()
                .collect(),
        ),
    ];
    for (field, paths) in expected {
        assert_eq!(
            string_set(&record[field], &format!("{name}.{field}")),
            paths,
            "{name}.{field} reproduces the complete file-tree diff"
        );
        let rows = strings(&record[field], &format!("{name}.{field}"));
        let mut sorted = rows.clone();
        sorted.sort_unstable();
        sorted.dedup();
        assert_eq!(rows, sorted, "{name}.{field} is sorted and unique");
    }
}

#[test]
fn official_stable_points_and_headless_claim_preserve_the_only_gap() {
    let identity = fixture(IDENTITY, "identity");
    assert_eq!(
        identity["fixture_schema"],
        "cline-headless-currentness-identity/v1"
    );
    assert_eq!(identity["observed_at"], "2026-10-08");
    assert_eq!(identity["axis"], "cline.package");
    assert_eq!(identity["family"], "cline.headless");
    assert_eq!(identity["selected_channel"]["package"], "cline");
    assert_eq!(identity["selected_channel"]["observed_latest"], "3.0.70");
    assert_eq!(
        identity["previous_qualified_ceiling"],
        CLINE_PACKAGE_VERSION
    );
    assert_eq!(identity["qualified_target"], "3.0.70");
    assert_eq!(identity["unpublished_next_stable"]["version"], "3.0.71");
    assert_eq!(identity["unpublished_next_stable"]["npm_published"], false);
    assert_eq!(
        identity["unpublished_next_stable"]["github_tag_exists"],
        false
    );
    assert_eq!(
        identity["claim_at_observation"]["latest_qualified"],
        "3.0.55"
    );
    assert_eq!(identity["identity_decision"]["provider_prompt_sent"], false);
    assert_eq!(
        identity["identity_decision"]["live_catalogue_or_session"],
        false
    );
    assert_eq!(identity["identity_decision"]["credentials_used"], false);
    assert_eq!(identity["identity_decision"]["host_install_changed"], false);
    assert_eq!(identity["identity_decision"]["artifact_executed"], false);

    let published = strings(
        &identity["published_stables_after_previous_ceiling"],
        "published stable hops",
    );
    assert_eq!(
        published,
        RELEASES[1..]
            .iter()
            .map(|version| (*version).to_owned())
            .collect::<Vec<_>>()
    );
    assert_eq!(
        identity["unpublished_interior_gaps"][0]["version"],
        "3.0.59"
    );
    assert_eq!(
        identity["unpublished_interior_gaps"][0]["npm_published"],
        false
    );
    assert_eq!(
        identity["unpublished_interior_gaps"][0]["github_tag_exists"],
        true
    );

    let claim = cline_headless_claim();
    assert_eq!(claim.id().as_str(), "cline.headless.package-window-1");
    assert_eq!(claim.baseline().as_str(), "3.0.55");
    assert_eq!(claim.latest_qualified().as_str(), "3.0.70");
    assert_eq!(claim.milestones().len(), 1);
    assert_eq!(
        claim
            .exclusions()
            .map(InterfaceVersion::as_str)
            .collect::<Vec<_>>(),
        ["3.0.59"]
    );
    for version in RELEASES {
        let version = InterfaceVersion::new(version).expect("published semver");
        let matched = claim
            .classify(&version)
            .expect("published point is qualified");
        assert_eq!(matched.support_status(), InterfaceSupportStatus::Maintained);
        assert_eq!(
            matched.behavior_revision().as_str(),
            "cline.headless.stdio-json-v1"
        );
    }
    for excluded in ["3.0.54", "3.0.59", "3.0.71"] {
        assert!(!claim.permits(&InterfaceVersion::new(excluded).expect("semver")));
    }
    assert!(cline_acp_claim().permits(&InterfaceVersion::new("3.0.55").expect("baseline")));
    assert!(cline_acp_claim().permits(&InterfaceVersion::new("3.0.70").expect("latest ACP")));
}

#[test]
fn npm_source_and_runtime_identities_are_complete_for_every_published_hop() {
    let identity = fixture(IDENTITY, "identity");
    let wrappers = fixture(WRAPPER_TREE, "wrapper trees");
    let native = fixture(DARWIN_TREE, "darwin arm64 trees");
    let sources = fixture(SOURCE_TREE, "selected source trees");
    let versions = identity["versions"].as_array().expect("identity versions");
    assert_eq!(versions.len(), RELEASES.len());
    assert_eq!(wrappers["schema"], "cline-root-wrapper-tree/v1");
    assert_eq!(native["schema"], "cline-darwin-arm64-tree/v1");
    assert_eq!(sources["schema"], "cline-selected-source-tree/v1");

    let wrapper_versions = object(&wrappers["versions"], "wrapper versions");
    let native_versions = object(&native["versions"], "native versions");
    let source_versions = object(&sources["versions"], "source versions");
    let selected_paths = strings(&sources["paths"], "selected source paths");
    assert_eq!(
        strings(
            &identity["selected_mapped_source_paths"],
            "identity selected source paths"
        ),
        SELECTED_SOURCE_PATHS
            .iter()
            .map(|path| (*path).to_owned())
            .collect::<Vec<_>>()
    );
    assert_eq!(
        selected_paths,
        SELECTED_SOURCE_PATHS
            .iter()
            .map(|path| (*path).to_owned())
            .collect::<Vec<_>>()
    );
    assert!(selected_paths.windows(2).all(|pair| pair[0] < pair[1]));
    for (index, version) in RELEASES.iter().enumerate() {
        let identity_point = &versions[index];
        assert_eq!(identity_point["version"], *version);
        let wrapper = &identity_point["npm_wrapper"];
        let wrapper_meta = wrapper_versions[*version]
            .as_array()
            .expect("wrapper manifest");
        let wrapper_files = file_map(&Value::Array(wrapper_meta.clone()), "wrapper manifest");
        assert_eq!(
            wrapper["file_count"].as_u64(),
            Some(wrapper_files.len() as u64)
        );
        assert_eq!(
            wrapper["unpacked_size"].as_u64(),
            Some(wrapper_files.values().map(|(size, _)| size).sum())
        );
        assert!(
            wrapper["integrity"]
                .as_str()
                .unwrap()
                .starts_with("sha512-")
        );

        let native_meta = native_versions[*version]
            .as_array()
            .expect("native manifest");
        let native_files = file_map(&Value::Array(native_meta.clone()), "native manifest");
        let native_summary = &native["summaries"][*version];
        assert_eq!(
            native_summary["file_count"].as_u64(),
            Some(native_files.len() as u64)
        );
        assert_eq!(
            native_summary["unpacked_size"].as_u64(),
            Some(native_files.values().map(|(size, _)| size).sum())
        );
        assert_eq!(
            native_summary["bin_cline_sha256"],
            native_files["bin/cline"].1
        );

        let source_rows = object(&source_versions[*version], "source version");
        assert_eq!(
            source_rows.keys().cloned().collect::<Vec<_>>(),
            selected_paths
        );
        for (path, row) in source_rows {
            assert_eq!(
                row["sha256"].as_str().unwrap().len(),
                64,
                "{version} {path}"
            );
            assert!(
                row["bytes"].as_u64().is_some(),
                "{version} {path} byte count"
            );
        }

        assert_eq!(
            identity_point["github_source"]["tag"],
            format!("cli-v{version}")
        );
        let commit = identity_point["github_source"]["peeled_commit"]
            .as_str()
            .expect("peeled source commit");
        assert_eq!(commit.len(), 40);
        assert!(commit.bytes().all(|byte| byte.is_ascii_hexdigit()));
        assert_eq!(
            identity_point["platform_artifacts"]
                .as_object()
                .unwrap()
                .len(),
            6
        );
        assert_eq!(
            identity_point["dependency_artifacts"]
                .as_object()
                .unwrap()
                .len(),
            5
        );
        assert_eq!(
            object(&identity_point["platform_artifacts"], "platform artifacts")
                .keys()
                .map(String::as_str)
                .collect::<Vec<_>>(),
            PLATFORM_ARTIFACTS.to_vec()
        );
        assert_eq!(
            object(
                &identity_point["dependency_artifacts"],
                "dependency artifacts"
            )
            .keys()
            .map(String::as_str)
            .collect::<Vec<_>>(),
            DEPENDENCY_ARTIFACTS.to_vec()
        );
        assert_eq!(
            object(&wrapper["optional_dependencies"], "optional dependencies")
                .keys()
                .map(String::as_str)
                .collect::<Vec<_>>(),
            PLATFORM_ARTIFACTS.to_vec()
        );
        assert_eq!(
            object(&wrapper["dependencies"], "root dependencies")
                .keys()
                .map(String::as_str)
                .collect::<Vec<_>>(),
            DEPENDENCY_ARTIFACTS.to_vec()
        );
        for artifact in identity_point["platform_artifacts"]
            .as_object()
            .unwrap()
            .values()
        {
            assert_eq!(artifact["version"], *version);
            assert!(
                artifact["integrity"]
                    .as_str()
                    .unwrap()
                    .starts_with("sha512-")
            );
        }
    }

    assert_eq!(identity["versions"][13]["version"], "3.0.69");
    assert_eq!(
        identity["versions"][13]["npm_wrapper"]["integrity"],
        "sha512-W5WJGbtwB2raJoCo5KhjIOQiBL3nEw9+F2tUm2b+woI0jDJGx1pey2maTDYCwYwyGDY36m/SjxpRWI8z1ROQpw=="
    );
    assert_eq!(
        identity["versions"][13]["github_source"]["peeled_commit"],
        "ef9430ffb4ceae9a7ab0b95d27cad8133ea7c576"
    );
    assert_eq!(identity["versions"][14]["version"], "3.0.70");
    assert_eq!(
        identity["versions"][14]["npm_wrapper"]["integrity"],
        "sha512-ekeGJ7YdVKL9p//KF0F5nxFXMvgSuhwPxvCUkS+SaBDJJoOUt+HMeJS+J+hkF3ezPekbSvpK1wCPGraZbAEO8A=="
    );
    assert_eq!(
        identity["versions"][14]["github_source"]["peeled_commit"],
        "0322bc5d510000a33ef5eadc3b4c84df7fcef285"
    );
    assert_eq!(
        identity["versions"][14]["platform_artifacts"]["@cline/cli-darwin-arm64"]["integrity"],
        "sha512-c+GHUmyL2exX4iC3BOhqQVfaToUJNMUPvuRDJGr3adILhhVL0II53XokBAWDrgpHAoyoQkiVs2nSGgzQhSXz5g=="
    );
    assert_eq!(
        identity["versions"][14]["npm_wrapper"]["tarball_sha256"],
        "4081fba9ec0867e32267b3875fbbe322edff8d55b7a11c1dcd0ff3066c88760b"
    );
    assert_eq!(
        identity["versions"][14]["npm_wrapper"]["tree_manifest_sha256"],
        "e34201854e1d814d6b099fe2a57afde456d9fcce61348fb34c6d62221cc87a94"
    );
    assert_eq!(
        identity["versions"][14]["npm_wrapper"]["shasum"],
        "837f68cd31378bd5fcd7a38510625bdc9c33adbb"
    );
    assert_eq!(
        source_versions["3.0.70"]["apps/cli/src/runtime/defaults.ts"]["sha256"],
        "d85a609d98501f3949765416a5891451ceae38549cf9771b548a6409f414486d"
    );
    assert_eq!(
        native["summaries"]["3.0.70"]["tree_manifest_sha256"],
        "e4ef11780c7ac862353e572378ab57f1265a8161d71be57b3c8e5dbd010ddd85"
    );
    assert_eq!(
        native["summaries"]["3.0.70"]["bin_cline_sha256"],
        "3ae76234a92f4de4fe6f9bf22635ac656994b29af83ea6d5c144b7eeea5a9ae7"
    );
}

#[test]
fn each_published_hop_classifies_every_wrapper_runtime_and_selected_source_path() {
    let wrappers = fixture(WRAPPER_TREE, "wrapper trees");
    let native = fixture(DARWIN_TREE, "darwin arm64 trees");
    let sources = fixture(SOURCE_TREE, "selected source trees");
    let ledger = fixture(HOP_LEDGER, "hop ledger");
    assert_eq!(ledger["versions"], serde_json::json!(RELEASES));
    for (tree_key, ledger_key) in [
        ("versions", "wrapper_hops"),
        ("versions", "darwin_arm64_runtime_hops"),
    ] {
        let tree = if ledger_key == "wrapper_hops" {
            &wrappers
        } else {
            &native
        };
        let manifests = object(&tree[tree_key], "tree manifests");
        let hops = ledger[ledger_key].as_array().expect("hop records");
        assert_eq!(hops.len(), RELEASES.len() - 1);
        for (index, hop) in hops.iter().enumerate() {
            assert_eq!(hop["from"], RELEASES[index]);
            assert_eq!(hop["to"], RELEASES[index + 1]);
            let before = file_map(&manifests[RELEASES[index]], "before manifest");
            let after = file_map(&manifests[RELEASES[index + 1]], "after manifest");
            assert_hop(&before, &after, hop, ledger_key);
        }
    }

    let source_versions = object(&sources["versions"], "source versions");
    let source_hops = ledger["selected_source_hops"]
        .as_array()
        .expect("source hops");
    assert_eq!(source_hops.len(), RELEASES.len() - 1);
    for (index, hop) in source_hops.iter().enumerate() {
        let before = object(&source_versions[RELEASES[index]], "source before");
        let after = object(&source_versions[RELEASES[index + 1]], "source after");
        let expected: BTreeSet<_> = before
            .iter()
            .filter(|(path, row)| row["sha256"] != after[*path]["sha256"])
            .map(|(path, _)| path.clone())
            .collect();
        assert_eq!(hop["from"], RELEASES[index]);
        assert_eq!(hop["to"], RELEASES[index + 1]);
        let changed = hop["changed"].as_array().expect("changed source files");
        let actual: BTreeSet<_> = changed
            .iter()
            .map(|row| row["path"].as_str().expect("source path").to_owned())
            .collect();
        assert_eq!(actual, expected, "selected-source hop {index}");
        for row in changed {
            let path = row["path"].as_str().expect("source path");
            assert_eq!(row["from_sha256"], before[path]["sha256"]);
            assert_eq!(row["to_sha256"], after[path]["sha256"]);
            assert!(!row["classification"].as_str().unwrap().trim().is_empty());
        }
    }

    let wrapper_latest = &ledger["wrapper_hops"][13];
    assert_eq!(
        strings(&wrapper_latest["changed"], "3.0.70 wrapper changes"),
        ["README.md".to_owned(), "package.json".to_owned()]
    );
    assert!(strings(&wrapper_latest["added"], "3.0.70 wrapper additions").is_empty());
    assert!(strings(&wrapper_latest["removed"], "3.0.70 wrapper removals").is_empty());

    let runtime_latest = &ledger["darwin_arm64_runtime_hops"][13];
    assert_eq!(
        strings(&runtime_latest["changed"], "3.0.70 runtime changes"),
        ["bin/cline".to_owned(), "package.json".to_owned()]
    );
    assert!(strings(&runtime_latest["added"], "3.0.70 runtime additions").is_empty());
    assert!(strings(&runtime_latest["removed"], "3.0.70 runtime removals").is_empty());

    let source_latest = source_hops[13]["changed"]
        .as_array()
        .expect("3.0.70 selected source changes");
    let source_changes = source_latest
        .iter()
        .map(|row| row["path"].as_str().expect("source path"))
        .collect::<Vec<_>>();
    assert_eq!(
        source_changes,
        [
            "apps/cli/src/commands/program.ts",
            "apps/cli/src/main.ts",
            "apps/cli/src/runtime/defaults.ts",
        ]
    );
}

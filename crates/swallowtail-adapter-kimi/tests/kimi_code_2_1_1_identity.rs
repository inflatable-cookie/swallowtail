//! Research 403: Kimi Code headless 2.1.1 identity and security-hop evidence.
//!
//! The evidence is frozen before changing the production claim. The 2.1.0
//! filesystem-access change needs an operator ruling, so the claim remains at
//! its previous 0.43.0 ceiling in this task state.

use serde_json::Value;
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use swallowtail_adapter_kimi::{KIMI_HEADLESS_LATEST_QUALIFIED_VERSION, kimi_headless_claim};
use swallowtail_core::{InterfaceCompatibilityAssessment, InterfaceVersion};

const IDENTITY: &str = include_str!("fixtures/kimi-code-2.1.1/identity.json");
const ARTIFACT_TREE: &str = include_str!("fixtures/kimi-code-2.1.1/artifact-tree.json");
const SOURCE_TREE: &str = include_str!("fixtures/kimi-code-2.1.1/source-tree.json");
const BUNDLE_ORACLES: &str = include_str!("fixtures/kimi-code-2.1.1/bundle-oracles.json");
const PROTOCOL: &str = include_str!("fixtures/kimi-code-2.1.1/protocol.json");

const VERSIONS: [&str; 7] = [
    "0.43.0", "0.43.1", "2.0.0", "2.0.1", "2.0.2", "2.1.0", "2.1.1",
];

fn json(source: &str) -> Value {
    serde_json::from_str(source).expect("Kimi Code currentness fixture is valid JSON")
}

fn sha256_hex(source: &str) -> String {
    Sha256::digest(source.as_bytes())
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect::<Vec<_>>()
        .join("")
}

fn strings(value: &Value) -> Vec<String> {
    value
        .as_array()
        .expect("value is an array")
        .iter()
        .map(|entry| entry.as_str().expect("array entry is text").to_owned())
        .collect()
}

fn tree_files(tree: &Value) -> BTreeMap<String, String> {
    let rows = tree["files"].as_array().expect("files are an array");
    let mut files = BTreeMap::new();
    let mut previous: Option<&str> = None;
    for row in rows {
        let fields = row.as_array().expect("file row is an array");
        let path = fields[0].as_str().expect("file path is text");
        assert!(
            previous.is_none_or(|previous| previous < path),
            "{path} is sorted and unique"
        );
        previous = Some(path);
        let signature = fields[1..]
            .iter()
            .map(Value::to_string)
            .collect::<Vec<_>>()
            .join("|");
        assert!(files.insert(path.to_owned(), signature).is_none());
    }
    assert_eq!(tree["file_count"].as_u64(), Some(files.len() as u64));
    files
}

fn assert_hop(inventory: &Value, from: &str, to: &str) {
    let left = tree_files(&inventory["trees"][from]);
    let right = tree_files(&inventory["trees"][to]);
    let delta = &inventory["hop_deltas"][format!("{from}_to_{to}")];
    let added = right
        .keys()
        .filter(|path| !left.contains_key(*path))
        .cloned()
        .collect::<Vec<_>>();
    let removed = left
        .keys()
        .filter(|path| !right.contains_key(*path))
        .cloned()
        .collect::<Vec<_>>();
    let changed = left
        .iter()
        .filter_map(|(path, before)| {
            right
                .get(path)
                .filter(|after| *after != before)
                .map(|_| path.clone())
        })
        .collect::<Vec<_>>();
    let identical = left.len() - removed.len() - changed.len();

    for (name, actual) in [("added", added), ("removed", removed), ("changed", changed)] {
        let actual_count = actual.len() as u64;
        assert_eq!(strings(&delta[name]), actual, "{from} → {to}: {name}");
        assert_eq!(delta["counts"][name].as_u64(), Some(actual_count));
    }
    assert_eq!(delta["identical_count"].as_u64(), Some(identical as u64));
}

#[test]
fn official_identity_freezes_the_complete_published_chain() {
    assert_eq!(
        sha256_hex(IDENTITY),
        "0c253a9e735b19a86c069054250786bd43f0ef2a4931207bdaccce3f4fc7de36"
    );
    let identity = json(IDENTITY);
    assert_eq!(identity["axis"], "kimi-code.executable");
    assert_eq!(identity["qualified_route"], "kimi-code.headless");
    assert_eq!(identity["previous_ceiling"], "0.43.0");
    assert_eq!(identity["npm_latest"], "2.1.1");
    assert_eq!(identity["github_latest"], "@moonshot-ai/kimi-code@2.1.1");
    assert_eq!(
        identity["stable_hops"],
        serde_json::json!(["0.43.1", "2.0.0", "2.0.1", "2.0.2", "2.1.0", "2.1.1"])
    );
    let points = &identity["points"];
    assert_eq!(
        points.as_object().expect("points map").len(),
        VERSIONS.len()
    );
    for version in VERSIONS {
        let point = &points[version];
        let npm = &point["npm"];
        let source = &point["source"];
        assert_eq!(
            source["github_tag"],
            format!("@moonshot-ai/kimi-code@{version}")
        );
        assert_eq!(source["package_version"], version);
        assert_eq!(source["prerelease"], false);
        assert_eq!(source["draft"], false);
        assert_eq!(npm["runtime_entry"], "dist/main.mjs");
        assert_eq!(npm["bin"]["kimi"], "dist/main.mjs");
        assert_eq!(npm["node_engine"], ">=22.19.0");
        assert_eq!(npm["git_head"], Value::Null);
        assert!(npm["integrity"].as_str().unwrap().starts_with("sha512-"));
        assert_eq!(npm["shasum"].as_str().unwrap().len(), 40);
        assert_eq!(npm["tarball_sha256"].as_str().unwrap().len(), 64);
        assert_eq!(npm["runtime_entry_sha256"].as_str().unwrap().len(), 64);
        assert!(npm["file_count"].as_u64().unwrap() > 500);
        assert_eq!(source["tag_object"].as_str().unwrap().len(), 40);
        assert_eq!(source["commit"].as_str().unwrap().len(), 40);
        assert_eq!(source["tree"].as_str().unwrap().len(), 40);
    }
    assert_eq!(
        identity["points"]["2.1.1"]["npm"]["published_at"],
        "2026-09-24T07:27:15.480Z"
    );
    assert_eq!(
        identity["points"]["2.1.1"]["source"]["published_at"],
        "2026-09-24T07:24:08Z"
    );
    assert_eq!(identity["host"]["version"], "0.34.0");
    assert_eq!(identity["host"]["host_updated"], false);
    assert_eq!(
        identity["artifact_method"]["npm_integrity_and_shasum_reproduced"],
        true
    );
    assert_eq!(identity["artifact_method"]["npm_bundles_executed"], false);
    assert_eq!(identity["artifact_method"]["credentials_used"], false);
}

#[test]
fn complete_package_and_source_trees_reproduce_every_hop_delta() {
    assert_eq!(
        sha256_hex(ARTIFACT_TREE),
        "e7d3ad45cff2891683cbd36a86944d09cdfd6e51320b07bf85e3756d96c9c878"
    );
    assert_eq!(
        sha256_hex(SOURCE_TREE),
        "6bc334f5ca1c23099528af379a78b30fbff6c51030b6d05dbb678ba5d646cd5e"
    );
    let artifacts = json(ARTIFACT_TREE);
    let sources = json(SOURCE_TREE);
    for inventory in [&artifacts, &sources] {
        assert_eq!(
            inventory["versions"],
            serde_json::json!([
                "0.43.0", "0.43.1", "2.0.0", "2.0.1", "2.0.2", "2.1.0", "2.1.1"
            ])
        );
        for pair in VERSIONS.windows(2) {
            assert_hop(inventory, pair[0], pair[1]);
        }
    }
    assert_eq!(artifacts["trees"]["2.1.1"]["file_count"], 541);
    assert_eq!(sources["trees"]["2.1.1"]["file_count"], 4410);
}

#[test]
fn current_claim_stays_at_0_43_0_while_the_security_hop_needs_a_ruling() {
    assert_eq!(
        sha256_hex(BUNDLE_ORACLES),
        "8825a3e3711ed9912656ecbcad6f621cabfe8b37e964b65018f0eaf31020f952"
    );
    assert_eq!(
        sha256_hex(PROTOCOL),
        "227a8227701518b79a357b4277a26eff39b22c7a6d47ec8631b9b9452d0f0c95"
    );
    assert_eq!(KIMI_HEADLESS_LATEST_QUALIFIED_VERSION, "0.43.0");
    let claim = kimi_headless_claim();
    let InterfaceCompatibilityAssessment::UnverifiedNewer(newer) =
        claim.assess(&InterfaceVersion::new("2.1.1").expect("valid release"))
    else {
        panic!("the frozen claim keeps 2.1.1 visible as unverified newer");
    };
    assert_eq!(newer.latest_qualified().as_str(), "0.43.0");

    let protocol = json(PROTOCOL);
    let source_ledger = &protocol["selected_source_ledger"];
    let auto_policy = &source_ledger["packages/agent-core-v2/src/agent/permissionPolicy/policies/auto-mode-approve.ts"];
    let policy_service = &source_ledger["packages/agent-core-v2/src/agent/permissionPolicy/permissionPolicyService.ts"];
    assert_eq!(auto_policy["0.43.0"], auto_policy["2.1.1"]);
    assert_eq!(policy_service["0.43.0"], policy_service["2.1.1"]);

    let realpath = &source_ledger["packages/agent-core-v2/src/tool/realpath-access.ts"];
    assert_eq!(realpath["2.0.2"], Value::Null);
    assert_ne!(realpath["2.1.0"], Value::Null);
    assert_eq!(realpath["2.1.1"], Value::Null);
    for path in [
        "packages/agent-core-v2/src/agent/tools/edit/editTool.ts",
        "packages/agent-core-v2/src/agent/tools/os/glob/globTool.ts",
        "packages/agent-core-v2/src/agent/tools/os/grep/grepTool.ts",
        "packages/agent-core-v2/src/agent/tools/os/read/readTool.ts",
        "packages/agent-core-v2/src/agent/tools/os/write/writeTool.ts",
        "packages/agent-core-v2/src/agent/tools/read-media-file/readMediaFileTool.ts",
    ] {
        assert_eq!(
            source_ledger[path]["2.0.2"], source_ledger[path]["2.1.1"],
            "{path}"
        );
        assert_ne!(
            source_ledger[path]["2.0.2"], source_ledger[path]["2.1.0"],
            "{path}"
        );
    }

    let markers = &protocol["npm_bundle_static_marker_counts"];
    for version in ["0.43.0", "0.43.1", "2.0.0", "2.0.1", "2.0.2", "2.1.1"] {
        assert_eq!(markers[version]["assertRealPathWithinWorkspace"], 0);
        assert_eq!(markers[version]["checkRealPathWriteTarget"], 0);
        assert_eq!(markers[version]["PATH_SYMLINK_ESCAPE"], 0);
    }
    assert_eq!(markers["2.1.0"]["assertRealPathWithinWorkspace"], 3);
    assert_eq!(markers["2.1.0"]["checkRealPathWriteTarget"], 3);
    assert_eq!(markers["2.1.0"]["PATH_SYMLINK_ESCAPE"], 4);
    assert_eq!(
        protocol["permission_path_proof"]["run_v2_print_sets_mode"],
        "auto"
    );
    assert_eq!(
        protocol["permission_path_proof"]["run_v2_print_sets_non_interactive"],
        true
    );
    assert_eq!(
        protocol["hop_classification"][4]["verdict"],
        "scope-stop-pending-operator-ruling"
    );
    assert_eq!(protocol["safety"]["downloaded_artifacts_executed"], false);
    assert_eq!(protocol["safety"]["credentials_used"], false);
}

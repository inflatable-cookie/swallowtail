//! Freeze the official Gemini CLI identity chain through 0.63.0 while keeping
//! `gemini-cli.headless` at its qualified 0.61.0 ceiling pending a ruling on
//! the selected Plan Mode permission and authority changes.
//!
//! The downloaded npm packages and tagged source archives were inspected as
//! data only. This corpus includes complete deterministic trees and checks
//! the selected mapped source set, each changed source path classification,
//! and both published hops. It makes no claim about live provider behavior.

use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};

const IDENTITY: &str = include_str!("fixtures/gemini-cli-0.63.0/identity.json");
const PROTOCOL: &str = include_str!("fixtures/gemini-cli-0.63.0/protocol.json");
const NPM_TREE: &str = include_str!("fixtures/gemini-cli-0.63.0/npm-tree-inventory.json");
const SOURCE_TREE: &str = include_str!("fixtures/gemini-cli-0.63.0/source-tree-inventory.json");

const VERSIONS: [&str; 3] = ["0.61.0", "0.62.0", "0.63.0"];
const MAPPED_SOURCE_PATHS: [&str; 23] = [
    "packages/cli/src/config/config.ts",
    "packages/cli/src/nonInteractiveCli.ts",
    "packages/core/src/core/geminiChat.ts",
    "packages/core/src/core/turn.ts",
    "packages/core/src/output/types.ts",
    "packages/core/src/output/stream-json-formatter.ts",
    "packages/core/src/utils/exitCodes.ts",
    "packages/cli/src/utils/sessions.ts",
    "packages/core/src/utils/sessionOperations.ts",
    "packages/cli/src/utils/sessionCleanup.ts",
    "packages/cli/src/gemini.tsx",
    "packages/core/src/prompts/snippets.ts",
    "packages/core/src/policy/policy-engine.ts",
    "packages/core/src/safety/built-in.ts",
    "packages/core/src/tools/read-file.ts",
    "packages/core/src/tools/shell.ts",
    "packages/core/src/tools/write-file.ts",
    "packages/core/src/tools/edit.ts",
    "packages/core/src/tools/tool-registry.ts",
    "packages/core/src/scheduler/tool-executor.ts",
    "packages/core/src/agents/local-executor.ts",
    "packages/core/src/core/client.ts",
    "packages/core/src/policy/config.ts",
];

fn fixture(body: &str, name: &str) -> Value {
    serde_json::from_str(body).unwrap_or_else(|error| panic!("{name}: {error}"))
}

fn key_set(value: &Value, name: &str) -> BTreeSet<String> {
    value
        .as_object()
        .unwrap_or_else(|| panic!("{name} is an object"))
        .keys()
        .cloned()
        .collect()
}

fn string_list(value: &Value, name: &str) -> Vec<String> {
    value
        .as_array()
        .unwrap_or_else(|| panic!("{name} is a list"))
        .iter()
        .map(|item| item.as_str().expect("entry is text").to_owned())
        .collect()
}

fn string_set(value: &Value, name: &str) -> BTreeSet<String> {
    string_list(value, name).into_iter().collect()
}

fn manifest(value: &Value, name: &str) -> BTreeMap<String, (String, String)> {
    value
        .as_object()
        .unwrap_or_else(|| panic!("{name} is an object"))
        .iter()
        .map(|(path, entry)| {
            let kind = entry["kind"]
                .as_str()
                .unwrap_or_else(|| panic!("{name} {path} kind is text"));
            let hash = entry["sha256"]
                .as_str()
                .unwrap_or_else(|| panic!("{name} {path} hash is text"));
            assert_eq!(hash.len(), 64, "{name} {path} is SHA-256");
            (path.clone(), (kind.to_owned(), hash.to_owned()))
        })
        .collect()
}

fn npm_manifest(value: &Value, name: &str) -> BTreeMap<String, (String, String)> {
    value
        .as_array()
        .unwrap_or_else(|| panic!("{name} is a list"))
        .iter()
        .map(|entry| {
            let path = entry["path"]
                .as_str()
                .unwrap_or_else(|| panic!("{name} path is text"));
            let hash = entry["sha256"]
                .as_str()
                .unwrap_or_else(|| panic!("{name} {path} hash is text"));
            assert_eq!(hash.len(), 64, "{name} {path} is SHA-256");
            (path.to_owned(), ("file".to_owned(), hash.to_owned()))
        })
        .collect()
}

fn tree_manifest_sha256(files: &BTreeMap<String, (String, String)>) -> String {
    let canonical = files
        .iter()
        .map(|(path, (kind, hash))| format!("{path}\0{kind}:{hash}\n"))
        .collect::<String>();
    Sha256::digest(canonical.as_bytes())
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

fn assert_inventory_hop(
    from: &BTreeMap<String, (String, String)>,
    to: &BTreeMap<String, (String, String)>,
    record: &Value,
    name: &str,
) {
    let from_paths: BTreeSet<_> = from.keys().cloned().collect();
    let to_paths: BTreeSet<_> = to.keys().cloned().collect();
    let added: BTreeSet<_> = to_paths.difference(&from_paths).cloned().collect();
    let removed: BTreeSet<_> = from_paths.difference(&to_paths).cloned().collect();
    let shared: BTreeSet<_> = from_paths.intersection(&to_paths).cloned().collect();
    let changed: BTreeSet<_> = shared
        .iter()
        .filter(|path| from[*path] != to[*path])
        .cloned()
        .collect();
    let identical: BTreeSet<_> = shared.difference(&changed).cloned().collect();

    for (field, expected) in [
        ("added", added),
        ("removed", removed),
        ("changed", changed),
        ("identical", identical),
    ] {
        assert_eq!(
            string_set(&record[field], &format!("{name}.{field}")),
            expected,
            "{name}.{field} must reproduce the complete tree diff"
        );
        let rows = string_list(&record[field], &format!("{name}.{field}"));
        let mut sorted = rows.clone();
        sorted.sort_unstable();
        sorted.dedup();
        assert_eq!(rows, sorted, "{name}.{field} is sorted and unique");
    }
}

#[test]
fn official_channel_and_identity_chain_stop_at_the_observed_ruling_gate() {
    let identity = fixture(IDENTITY, "identity");
    assert_eq!(
        key_set(&identity, "identity"),
        [
            "axis",
            "canonical_main_at_number_allocation",
            "channels_agree",
            "claim_at_observation",
            "compared_points",
            "family",
            "fixture_schema",
            "github_latest_stable_at_observation",
            "host",
            "identity_decision",
            "ignored_channels",
            "mapped_source_sha256",
            "npm_latest_at_observation",
            "npm_package",
            "observed_at",
            "official",
            "published_artifact_executed",
            "published_stables_after_previous_ceiling",
            "unpublished_interior_gaps",
            "unpublished_later_stable",
        ]
        .map(str::to_owned)
        .into(),
        "identity top-level fields"
    );
    assert_eq!(identity["family"], "gemini-cli");
    assert_eq!(identity["axis"], "gemini-cli.headless-stream-json");
    assert_eq!(identity["npm_package"], "@google/gemini-cli");
    assert_eq!(identity["npm_latest_at_observation"], "0.63.0");
    assert_eq!(identity["github_latest_stable_at_observation"], "v0.63.0");
    assert_eq!(identity["channels_agree"], true);
    assert_eq!(identity["observed_at"], "2026-10-07");
    assert_eq!(identity["published_artifact_executed"], false);
    assert_eq!(
        identity["published_stables_after_previous_ceiling"],
        json!(["0.62.0", "0.63.0"])
    );
    assert_eq!(
        identity["unpublished_interior_gaps"],
        json!(["0.56.1", "0.59.1"])
    );
    assert_eq!(identity["unpublished_later_stable"], "0.63.1");
    assert_eq!(
        identity["ignored_channels"],
        json!({
            "npm_preview": "0.64.0-preview.0",
            "npm_nightly": "0.65.0-nightly.20261007.gef59c532f"
        })
    );

    let official = &identity["official"];
    assert_eq!(official["version"], "0.63.0");
    assert_eq!(official["npm_published_at"], "2026-10-06T20:58:43.642Z");
    assert_eq!(
        official["github_release_published_at"],
        "2026-10-06T20:38:49Z"
    );
    assert_eq!(
        official["npm_integrity"],
        "sha512-mmhqMmAdsoplCzrLQQC4yJBqr4uSKXghnrLspgUU6N9drmsaq7pi7oF09skRYzEq2dYK6eMT4UpFQP6rG2o/ew=="
    );
    assert_eq!(
        official["npm_shasum"],
        "0ce9cd94c6b3c490af9460fdf9f5e69bc274a84c"
    );
    assert_eq!(
        official["npm_tarball_sha256"],
        "97a6edfc10645463b517f0518d46a8c72efbdc12558a9a948607f726284a0420"
    );
    assert_eq!(
        official["npm_package_json_sha256"],
        "3dea1781827df5d62db33a7f970d595e09e27373e277abcd1ab8d1652fa05a8a"
    );
    assert_eq!(official["npm_bin_entry"], "bundle/gemini.js");
    assert_eq!(
        official["npm_bin_entry_sha256"],
        "5aee9ecb65b0b821e36a12f6e9e837301636de1a849a5a8bcc9c2b2620bbd72e"
    );
    assert_eq!(official["npm_file_count"], 449);
    assert_eq!(official["github_tag"], "v0.63.0");
    assert_eq!(
        official["github_commit"],
        "573846625af9e93b3b968e0e0b86bb093a4c9b16"
    );
    assert_eq!(
        official["github_tree"],
        "a6a023123538ba75eb8412528b9942d0de8917e3"
    );
    assert_eq!(
        official["github_source_archive_sha256"],
        "75903470f15719bc061df6c2792cc55274494f7c25efdb7a864c50f5913bf917"
    );
    assert_eq!(official["source_file_count"], 3019);

    let points = identity["compared_points"]
        .as_array()
        .expect("compared points are a list");
    assert_eq!(points.len(), 3);
    for (
        point,
        version,
        published,
        npm_integrity,
        npm_shasum,
        package_json,
        bin_entry,
        release_published,
        commit,
        tree,
        tarball,
        source_archive,
        source_files,
    ) in [
        (
            &points[0],
            "0.61.0",
            "2026-09-24T00:04:53.021Z",
            "sha512-dbQ9A0qBtFJNi6XBkHvfZ6Azpn6PNgH/P8h2MZ67RLlX8hSAVjup39CRWqdRxZv7YXIuhrFaytr8y0jKnkoxnQ==",
            "d4880a2b42aa6786cf626184303b2fdf13bde88f",
            "8461a3167a919dafdc3dbbee7a8dba273d4236bafe61050949fa4927215f4e65",
            "0b6e283ae88682b0e27e8ef85a608ab74807a1513dc0c053e5aa80d5b80b29ab",
            "2026-09-23T23:59:15Z",
            "bb523741c7429a44d03e964bc124c7c92df59d5f",
            "c4226f654bb0b35109c4f5ce34535c5addecda47",
            "bc4efa5c925c4430105b552820ed3164bbeffa9dc227990fc922f954733bcd7d",
            "917e0ac08eb3ef2048910ecc84d4da672ad0c58d82bb2eaa1d6f9e18af80241d",
            3005,
        ),
        (
            &points[1],
            "0.62.0",
            "2026-09-29T21:25:44.188Z",
            "sha512-A1rw0Tf2sHLpGncfYdaq5WaJIufKAP8il4BmHD5Yw4ewmB/Wo0vRQb2bEvx7OqyaPFPZCh0hVhcMKsICZyIBww==",
            "1c856e63fbf2650dd16acb25d13cb69c5a8a6962",
            "b6a087cf7fbfa394a5a010c2e9129545660a4cf40633ba396b2cb3885197671f",
            "ef1d1bd9ee5aaae37ebcfb601b56659e3b16c5b258b21c138109c541e487ade7",
            "2026-09-29T21:17:07Z",
            "b460678f3db508407554afd604cc9d6635becb2a",
            "65ae43a5a0bf3670294be027ffe40eb5b0c5f18c",
            "2276032b1c33d2b828b1cf197e52f48e74b0a395326763ff01a80d97d0fbc0c3",
            "18d3955d07457723e5f9b24ff2d7622081b855ea8591d00a977b89a2089626f0",
            3015,
        ),
        (
            &points[2],
            "0.63.0",
            "2026-10-06T20:58:43.642Z",
            "sha512-mmhqMmAdsoplCzrLQQC4yJBqr4uSKXghnrLspgUU6N9drmsaq7pi7oF09skRYzEq2dYK6eMT4UpFQP6rG2o/ew==",
            "0ce9cd94c6b3c490af9460fdf9f5e69bc274a84c",
            "3dea1781827df5d62db33a7f970d595e09e27373e277abcd1ab8d1652fa05a8a",
            "5aee9ecb65b0b821e36a12f6e9e837301636de1a849a5a8bcc9c2b2620bbd72e",
            "2026-10-06T20:38:49Z",
            "573846625af9e93b3b968e0e0b86bb093a4c9b16",
            "a6a023123538ba75eb8412528b9942d0de8917e3",
            "97a6edfc10645463b517f0518d46a8c72efbdc12558a9a948607f726284a0420",
            "75903470f15719bc061df6c2792cc55274494f7c25efdb7a864c50f5913bf917",
            3019,
        ),
    ] {
        assert_eq!(point["version"], version);
        assert_eq!(
            point["npm_published_at"], published,
            "{version} npm publish"
        );
        assert_eq!(
            point["npm_integrity"], npm_integrity,
            "{version} npm integrity"
        );
        assert_eq!(point["npm_shasum"], npm_shasum, "{version} npm shasum");
        assert_eq!(
            point["npm_package_json_sha256"], package_json,
            "{version} package.json"
        );
        assert_eq!(
            point["npm_bin_entry_sha256"], bin_entry,
            "{version} npm bin entry"
        );
        assert_eq!(
            point["github_release_published_at"], release_published,
            "{version} GitHub release"
        );
        assert_eq!(point["github_commit"], commit, "{version} commit");
        assert_eq!(point["github_tree"], tree, "{version} tree");
        assert_eq!(
            point["source_file_count"], source_files,
            "{version} source file count"
        );
        assert_eq!(
            point["npm_tarball_sha256"], tarball,
            "{version} npm tarball"
        );
        assert_eq!(
            point["github_source_archive_sha256"], source_archive,
            "{version} source archive"
        );
    }

    assert_eq!(
        identity["claim_at_observation"],
        json!({
            "baseline": "0.51.0",
            "latest_qualified": "0.61.0",
            "newer_posture": "allow_unverified",
            "0.62.0": "unverified-newer",
            "0.63.0": "unverified-newer"
        })
    );
    let decision = &identity["identity_decision"];
    assert_eq!(decision["shape"], "stop");
    assert_eq!(
        decision["stop_kind"],
        "selected-permission-authority-and-tool-output-change"
    );
    assert_eq!(decision["keep_existing_ceiling"], "0.61.0");
    assert_eq!(decision["raise_latest_qualified_to"], Value::Null);
    assert_eq!(decision["ruling_required"], true);
    assert_eq!(
        decision["trigger_sources"],
        json!([
            "packages/core/src/prompts/snippets.ts",
            "packages/core/src/policy/policy-engine.ts",
            "packages/core/src/tools/read-file.ts",
            "packages/core/src/safety/built-in.ts",
            "packages/core/src/scheduler/tool-executor.ts",
            "packages/core/src/agents/local-executor.ts"
        ])
    );
    for key in [
        "provider_prompt_sent",
        "live_session",
        "credentials_used",
        "host_install_changed",
    ] {
        assert_eq!(decision[key], false, "{key}");
    }
    assert_eq!(identity["host"]["on_path"], false);
    assert_eq!(identity["host"]["version"], Value::Null);
    assert_eq!(identity["host"]["host_install_changed"], false);
}

#[test]
fn selected_route_contract_and_mapped_source_changes_are_frozen() {
    let protocol = fixture(PROTOCOL, "protocol");
    assert_eq!(protocol["route"], "gemini-cli.headless");
    assert_eq!(protocol["axis"], "gemini-cli.headless-stream-json");
    assert_eq!(protocol["previous_ceiling"], "0.61.0");
    assert_eq!(protocol["official_latest"], "0.63.0");
    assert_eq!(protocol["selected_external_wire_shape_unchanged"], true);
    assert_eq!(
        protocol["selected_invocation"],
        json!([
            "gemini",
            "--output-format",
            "stream-json",
            "--model",
            "<MODEL>",
            "--approval-mode",
            "plan",
            "--extensions",
            "none",
            "--allowed-mcp-server-names",
            "",
            "--skip-trust",
            "--session-id",
            "<SESSION_ID>"
        ])
    );
    assert_eq!(
        protocol["selected_event_types"],
        json!([
            "init",
            "message",
            "tool_use",
            "tool_result",
            "error",
            "result"
        ])
    );
    assert_eq!(
        protocol["selected_options"],
        json!({
            "output_format": "stream-json",
            "approval_mode": "plan",
            "extensions": "none",
            "allowed_mcp_server_names": [],
            "skip_trust": true,
            "session_id": "caller-bound"
        })
    );
    assert_eq!(
        protocol["selected_terminal"],
        json!({
            "result_statuses": ["success", "error"],
            "native_exit_codes": [41, 42, 44, 52, 53, 54, 55, 130],
            "terminal_record_required": true
        })
    );
    assert_eq!(protocol["provider_prompt_sent"], false);
    assert_eq!(protocol["live_session"], false);
    assert_eq!(protocol["credential_used"], false);
    assert_eq!(protocol["host_install_changed"], false);

    let source_tree = fixture(SOURCE_TREE, "source tree");
    let source_files = &source_tree["files"];
    let source_hashes = &protocol["mapped_source_sha256"];
    assert_eq!(
        key_set(source_hashes, "mapped source versions"),
        VERSIONS.map(str::to_owned).into(),
    );
    let expected_paths: BTreeSet<String> = MAPPED_SOURCE_PATHS.map(str::to_owned).into();
    for version in VERSIONS {
        assert_eq!(
            key_set(&source_hashes[version], "mapped source paths"),
            expected_paths,
            "mapped source set at {version}"
        );
        for path in MAPPED_SOURCE_PATHS {
            assert_eq!(
                source_hashes[version][path], source_files[version][path]["sha256"],
                "mapped source digest {version} {path}"
            );
        }
    }
    for (from, to, changed) in [
        (
            "0.61.0",
            "0.62.0",
            [
                "packages/cli/src/gemini.tsx",
                "packages/core/src/tools/shell.ts",
                "packages/core/src/tools/tool-registry.ts",
            ]
            .map(str::to_owned)
            .into_iter()
            .collect::<BTreeSet<_>>(),
        ),
        (
            "0.62.0",
            "0.63.0",
            [
                "packages/core/src/prompts/snippets.ts",
                "packages/core/src/policy/policy-engine.ts",
                "packages/core/src/safety/built-in.ts",
                "packages/core/src/tools/read-file.ts",
                "packages/core/src/tools/shell.ts",
                "packages/core/src/scheduler/tool-executor.ts",
                "packages/core/src/agents/local-executor.ts",
                "packages/core/src/core/client.ts",
            ]
            .map(str::to_owned)
            .into_iter()
            .collect::<BTreeSet<_>>(),
        ),
    ] {
        let actual: BTreeSet<String> = MAPPED_SOURCE_PATHS
            .iter()
            .filter(|path| source_hashes[to][*path] != source_hashes[from][*path])
            .map(|path| (*path).to_owned())
            .collect();
        assert_eq!(actual, changed, "selected mapped source delta {from}..{to}");
    }

    let hops = &protocol["hop_classification"];
    assert_eq!(
        key_set(hops, "protocol hop classifications"),
        ["0.61.0..0.62.0", "0.62.0..0.63.0"]
            .map(str::to_owned)
            .into(),
    );
    assert_eq!(
        hops["0.62.0..0.63.0"]["selected_plan_permission"]["path"],
        "packages/core/src/prompts/snippets.ts"
    );
    assert_eq!(
        hops["0.62.0..0.63.0"]["selected_noninteractive_policy"]["path"],
        "packages/core/src/policy/policy-engine.ts"
    );
    assert_eq!(
        hops["0.62.0..0.63.0"]["selected_file_read"]["path"],
        "packages/core/src/tools/read-file.ts"
    );
    assert_eq!(
        hops["0.62.0..0.63.0"]["selected_file_safety"]["path"],
        "packages/core/src/safety/built-in.ts"
    );
    assert_eq!(
        hops["0.62.0..0.63.0"]["selected_tool_output"]["paths"],
        json!([
            "packages/core/src/scheduler/tool-executor.ts",
            "packages/core/src/agents/local-executor.ts"
        ])
    );
    assert_eq!(
        hops["0.62.0..0.63.0"]["unselected_resumed_session_recording"]["path"],
        "packages/core/src/core/client.ts"
    );
    assert_eq!(
        hops["0.61.0..0.62.0"]["selected_failure_lifecycle"]["path"],
        "packages/cli/src/gemini.tsx"
    );
    assert_eq!(
        hops["0.61.0..0.62.0"]["selected_tool_surface"]["path"],
        "packages/core/src/tools/tool-registry.ts"
    );
    assert_eq!(
        hops["0.62.0..0.63.0"]["external_stream"]["classification"],
        "selected invocation/options and stream-json formatter/types remain byte-identical; no output framing change identified"
    );
}

#[test]
fn npm_and_source_tree_diffs_reproduce_every_changed_path_and_classification() {
    let npm = fixture(NPM_TREE, "npm tree");
    assert_eq!(npm["artifact"], "official-npm-tarball");
    assert_eq!(npm["package"], "@google/gemini-cli");
    assert_eq!(npm["compared"], json!(VERSIONS));
    assert_eq!(
        npm["package_file_counts"],
        json!({"0.61.0": 449, "0.62.0": 449, "0.63.0": 449})
    );
    assert_eq!(
        npm["tree_manifest_sha256"],
        json!({
            "0.61.0": "0804807d120138636856247ccd632a60a8018a8f5a3ee277702e6e905198c242",
            "0.62.0": "8dd69d36dd30cca1a925dbfd960e5959d658e50fb81aa020d34546d18f5eefa4",
            "0.63.0": "4e1155bb97a72bebc281e3fe396b833168e6e3c33301083cfb217db8a7aa4981"
        })
    );
    let npm_files = &npm["files"];
    for version in VERSIONS {
        let rows = &npm_files[version];
        let rows = rows.as_array().expect("npm files are list");
        assert_eq!(rows.len(), 449);
        let keys: Vec<_> = rows
            .iter()
            .map(|entry| entry["path"].as_str().expect("npm path is text"))
            .collect();
        let mut sorted = keys.clone();
        sorted.sort_unstable();
        sorted.dedup();
        assert_eq!(keys, sorted, "npm inventory path order at {version}");
        for entry in rows {
            assert_eq!(
                entry["sha256"]
                    .as_str()
                    .expect("npm file hash is text")
                    .len(),
                64,
                "{version} {}",
                entry["path"]
            );
        }
        assert_eq!(
            tree_manifest_sha256(&npm_manifest(&npm_files[version], "npm files")),
            npm["tree_manifest_sha256"][version],
            "npm tree manifest digest at {version}"
        );
    }
    for (from, to, hop, added, removed, changed, identical) in [
        (
            "0.61.0",
            "0.62.0",
            "from_0.61.0_to_0.62.0",
            50,
            50,
            [
                "bundle/docs/changelogs/index.md",
                "bundle/docs/changelogs/latest.md",
                "bundle/docs/changelogs/preview.md",
                "bundle/docs/get-started/authentication.mdx",
                "bundle/gemini.js",
                "package.json",
            ]
            .map(str::to_owned)
            .into_iter()
            .collect::<BTreeSet<_>>(),
            393,
        ),
        (
            "0.62.0",
            "0.63.0",
            "from_0.62.0_to_0.63.0",
            48,
            48,
            [
                "bundle/docs/changelogs/index.md",
                "bundle/docs/changelogs/latest.md",
                "bundle/docs/changelogs/preview.md",
                "bundle/gemini.js",
                "package.json",
            ]
            .map(str::to_owned)
            .into_iter()
            .collect::<BTreeSet<_>>(),
            396,
        ),
    ] {
        let prior = npm_manifest(&npm_files[from], "prior npm files");
        let current = npm_manifest(&npm_files[to], "current npm files");
        let record = &npm[hop];
        assert_inventory_hop(&prior, &current, record, hop);
        assert_eq!(
            string_list(&record["added"], "added npm paths").len(),
            added
        );
        assert_eq!(
            string_list(&record["removed"], "removed npm paths").len(),
            removed
        );
        assert_eq!(
            string_list(&record["changed"], "changed npm paths").len(),
            changed.len()
        );
        assert_eq!(string_set(&record["changed"], "changed npm paths"), changed);
        assert_eq!(
            string_list(&record["identical"], "identical npm paths").len(),
            identical
        );
    }
    assert_eq!(
        npm["selected_runtime_chunks"],
        json!({
            "0.61.0": {"path":"bundle/chunk-JDPZ4CE3.js","sha256":"901004474485b55896feaed563d2e1f75bce021a9245ffbd50724028f7926ba6","loaded_from":"bundle/gemini.js"},
            "0.62.0": {"path":"bundle/chunk-MLY4WQFO.js","sha256":"c67737944afddc18a1cee05b556367c1beab2235bf608d5fdf57bb0cfc42cce0","loaded_from":"bundle/gemini.js"},
            "0.63.0": {"path":"bundle/chunk-RWBXO4OP.js","sha256":"3734d78897bc515eef42341316b37341144d07cd84ee76e5bd6c788dad6ec916","loaded_from":"bundle/gemini.js"}
        })
    );

    let source = fixture(SOURCE_TREE, "source tree");
    assert_eq!(source["artifact"], "official-github-source-archive");
    assert_eq!(source["repository"], "google-gemini/gemini-cli");
    assert_eq!(source["compared"], json!(VERSIONS));
    assert_eq!(
        source["file_counts"],
        json!({"0.61.0": 3005, "0.62.0": 3015, "0.63.0": 3019})
    );
    assert_eq!(
        source["tree_manifest_sha256"],
        json!({
            "0.61.0": "379f7c1948c5d191fcc84cd77cccb821ba8d6fc4fc3290a76df79fb232c5177c",
            "0.62.0": "3492f2ac3b9367c2e4f37813d648dff864d19fd2676c12868d2e5130f05bdd35",
            "0.63.0": "fbb5d78fd631e4a53e26a62284f8c15d5d90a1ef2743d4d79a5517218dfc6e51"
        })
    );
    let source_files = &source["files"];
    let source_hops = &source["hops"];
    assert_eq!(
        key_set(source_hops, "source hops"),
        ["0.61.0..0.62.0", "0.62.0..0.63.0"]
            .map(str::to_owned)
            .into(),
    );
    for version in VERSIONS {
        let rows = &source_files[version];
        let expected_count = match version {
            "0.61.0" => 3005,
            "0.62.0" => 3015,
            "0.63.0" => 3019,
            _ => unreachable!(),
        };
        assert_eq!(
            rows.as_object().expect("source files are object").len(),
            expected_count
        );
        let keys: Vec<_> = rows
            .as_object()
            .expect("source files are object")
            .keys()
            .collect();
        let mut sorted = keys.clone();
        sorted.sort_unstable();
        sorted.dedup();
        assert_eq!(keys, sorted, "source inventory path order at {version}");
        for (path, value) in rows.as_object().expect("source files are object") {
            let kind = value["kind"].as_str().expect("source kind is text");
            assert!(kind == "file" || kind == "symlink", "{version} {path} kind");
            assert_eq!(
                value["sha256"].as_str().expect("source hash is text").len(),
                64,
                "{version} {path}"
            );
            if kind == "symlink" {
                assert!(
                    value["target"].as_str().is_some(),
                    "{version} {path} link target"
                );
            }
        }
        assert_eq!(
            tree_manifest_sha256(&manifest(rows, "source files")),
            source["tree_manifest_sha256"][version],
            "source tree manifest digest at {version}"
        );
    }
    for (from, to, hop, expected_changed, categories) in [
        (
            "0.61.0",
            "0.62.0",
            "0.61.0..0.62.0",
            [
                "packages/cli/src/gemini.tsx",
                "packages/core/src/tools/shell.ts",
                "packages/core/src/tools/tool-registry.ts",
            ]
            .map(str::to_owned)
            .into_iter()
            .collect::<BTreeSet<_>>(),
            json!({
                "unmapped-provider-internal": 75,
                "selected-failure-lifecycle": 1,
                "post-plan-transition-behavior-unqualified": 1,
                "unselected-disabled-mcp-presentation": 1
            }),
        ),
        (
            "0.62.0",
            "0.63.0",
            "0.62.0..0.63.0",
            [
                "packages/core/src/prompts/snippets.ts",
                "packages/core/src/policy/policy-engine.ts",
                "packages/core/src/safety/built-in.ts",
                "packages/core/src/tools/read-file.ts",
                "packages/core/src/tools/shell.ts",
                "packages/core/src/scheduler/tool-executor.ts",
                "packages/core/src/agents/local-executor.ts",
                "packages/core/src/core/client.ts",
            ]
            .map(str::to_owned)
            .into_iter()
            .collect::<BTreeSet<_>>(),
            json!({
                "unmapped-provider-internal": 79,
                "selected-plan-authority-change": 1,
                "selected-noninteractive-permission-change": 1,
                "selected-file-safety-permission-change": 1,
                "selected-read-file-path-change": 1,
                "post-plan-transition-behavior-unqualified": 1,
                "selected-tool-output-and-context-change": 2,
                "unselected-resumed-session-recording-cleanup": 1
            }),
        ),
    ] {
        let prior = manifest(&source_files[from], "prior source files");
        let current = manifest(&source_files[to], "current source files");
        let record = &source_hops[hop];
        assert_inventory_hop(&prior, &current, record, hop);
        let actual_changed = string_set(&record["changed"], "source changed paths");
        assert_eq!(actual_changed.len(), if from == "0.61.0" { 78 } else { 87 });
        let classified = &record["classifications"];
        assert_eq!(
            key_set(classified, "source classifications"),
            actual_changed
        );
        assert_eq!(
            classified
                .as_object()
                .expect("classifications are object")
                .iter()
                .fold(
                    BTreeMap::<String, usize>::new(),
                    |mut counts, (_, value)| {
                        *counts
                            .entry(value.as_str().unwrap().to_owned())
                            .or_default() += 1;
                        counts
                    }
                ),
            categories
                .as_object()
                .unwrap()
                .iter()
                .map(|(k, v)| (k.clone(), v.as_u64().unwrap() as usize))
                .collect(),
            "all changed source paths have the expected bounded classification"
        );
        let mut selected = BTreeSet::new();
        for path in MAPPED_SOURCE_PATHS {
            if prior[path].1 != current[path].1 {
                selected.insert(path.to_owned());
            }
        }
        assert_eq!(
            selected, expected_changed,
            "mapped changed source set for {hop}"
        );
        let required_classes = match from {
            "0.61.0" => [
                ("packages/cli/src/gemini.tsx", "selected-failure-lifecycle"),
                (
                    "packages/core/src/tools/shell.ts",
                    "post-plan-transition-behavior-unqualified",
                ),
                (
                    "packages/core/src/tools/tool-registry.ts",
                    "unselected-disabled-mcp-presentation",
                ),
            ]
            .into_iter()
            .collect::<BTreeMap<_, _>>(),
            _ => [
                (
                    "packages/core/src/prompts/snippets.ts",
                    "selected-plan-authority-change",
                ),
                (
                    "packages/core/src/policy/policy-engine.ts",
                    "selected-noninteractive-permission-change",
                ),
                (
                    "packages/core/src/safety/built-in.ts",
                    "selected-file-safety-permission-change",
                ),
                (
                    "packages/core/src/tools/read-file.ts",
                    "selected-read-file-path-change",
                ),
                (
                    "packages/core/src/tools/shell.ts",
                    "post-plan-transition-behavior-unqualified",
                ),
                (
                    "packages/core/src/scheduler/tool-executor.ts",
                    "selected-tool-output-and-context-change",
                ),
                (
                    "packages/core/src/agents/local-executor.ts",
                    "selected-tool-output-and-context-change",
                ),
                (
                    "packages/core/src/core/client.ts",
                    "unselected-resumed-session-recording-cleanup",
                ),
            ]
            .into_iter()
            .collect::<BTreeMap<_, _>>(),
        };
        for (path, class) in &required_classes {
            assert_eq!(classified[*path], *class, "{path} classification in {hop}");
        }
        for (path, class) in classified.as_object().unwrap() {
            if !required_classes.contains_key(path.as_str()) {
                assert_eq!(class, "unmapped-provider-internal", "{path} in {hop}");
            }
        }
    }
}

//! Identity and compatible-extension evidence for official Gemini CLI
//! `0.59.0..=0.61.0` (Research 357, swallowtail#071).
//!
//! This corpus freezes the three compared official points, their npm registry
//! identity, GitHub tags, trees, and darwin-arm64 unsigned assets, plus one
//! deterministic tagged-source file inventory per point. It is
//! mutation-sensitive: the assertions fail if an official identity, a
//! selected mapped-file hash, the changed-path set, the ACP HTTP MCP mapping
//! sources, or the per-claim classification drifts.
//!
//! Nothing here executes a downloaded artifact. npm tarballs were hashed
//! against their registry `integrity`/`shasum`, tagged source archives were
//! hashed and extracted statically, and no prompt, login, credential,
//! installation, or host update occurred.

use serde_json::{Value, json};
use std::collections::BTreeSet;

const IDENTITY: &str = include_str!("fixtures/gemini-cli-0.61.0/identity.json");
const SURFACE: &str = include_str!("fixtures/gemini-cli-0.61.0/surface-ledger.json");
const PROTOCOL: &str = include_str!("fixtures/gemini-cli-0.61.0/protocol.json");
const PRIOR_IDENTITY: &str = include_str!("fixtures/gemini-cli-0.59.0/identity.json");
const PRIOR_SURFACE: &str = include_str!("fixtures/gemini-cli-0.59.0/surface-ledger.json");

const OFFICIAL: &str = "0.61.0";
const PRIOR_CEILING: &str = "0.59.0";
const BASELINE: &str = "0.51.0";
const COMPARED: [&str; 3] = ["0.59.0", "0.60.0", "0.61.0"];

fn fixture(body: &str, name: &str) -> Value {
    serde_json::from_str(body).unwrap_or_else(|error| panic!("{name}: {error}"))
}

fn string_set(value: &Value, name: &str) -> BTreeSet<String> {
    value
        .as_array()
        .unwrap_or_else(|| panic!("{name} is a list"))
        .iter()
        .map(|item| item.as_str().expect("entry is text").to_owned())
        .collect()
}

fn key_set(value: &Value, name: &str) -> BTreeSet<String> {
    value
        .as_object()
        .unwrap_or_else(|| panic!("{name} is an object"))
        .keys()
        .cloned()
        .collect()
}

#[test]
fn official_identity_reproduces_for_every_compared_point() {
    let identity = fixture(IDENTITY, "identity");
    let official = &identity["official"];
    assert_eq!(identity["family"], "gemini-cli");
    assert_eq!(identity["npm_package"], "@google/gemini-cli");
    assert_eq!(identity["npm_latest"], true);
    assert_eq!(official["version"], OFFICIAL);
    assert_eq!(official["npm_published_at"], "2026-09-24T00:04:53.021Z");
    assert_eq!(official["github_tag"], "v0.61.0");
    assert_eq!(
        official["github_commit"],
        "bb523741c7429a44d03e964bc124c7c92df59d5f"
    );
    assert_eq!(
        official["github_tree"],
        "c4226f654bb0b35109c4f5ce34535c5addecda47"
    );
    assert_eq!(
        official["npm_integrity"],
        "sha512-dbQ9A0qBtFJNi6XBkHvfZ6Azpn6PNgH/P8h2MZ67RLlX8hSAVjup39CRWqdRxZv7YXIuhrFaytr8y0jKnkoxnQ=="
    );
    assert_eq!(
        official["npm_shasum"],
        "d4880a2b42aa6786cf626184303b2fdf13bde88f"
    );
    assert_eq!(
        official["npm_tarball_sha256"],
        "bc4efa5c925c4430105b552820ed3164bbeffa9dc227990fc922f954733bcd7d"
    );
    assert_eq!(
        official["npm_bin_entry_sha256"],
        "0b6e283ae88682b0e27e8ef85a608ab74807a1513dc0c053e5aa80d5b80b29ab"
    );
    assert_eq!(
        official["github_source_archive_sha256"],
        "917e0ac08eb3ef2048910ecc84d4da672ad0c58d82bb2eaa1d6f9e18af80241d"
    );
    assert_eq!(
        official["darwin_arm64_asset_sha256"],
        "829738c00cab5a73b3ed01ce874c7ba79064fbe5869ad821517cba24f778dce2"
    );
    assert_eq!(
        official["darwin_arm64_extracted_sha256"],
        "93a9d76e77a4a7716eb64127b24907309fe2b0af1e2273fb97a8c44b793fc5d2"
    );
    assert_eq!(official["downloaded_artifact_executed"], false);

    let points: Vec<&str> = identity["compared_points"]
        .as_array()
        .expect("compared points are a list")
        .iter()
        .map(|point| point["version"].as_str().expect("version is text"))
        .collect();
    assert_eq!(points, COMPARED);
    assert_eq!(
        identity["compared_points"][1]["github_commit"],
        "733edcb597ce690ac2e2fe3b3b3690b60a4c8f27"
    );
    assert_eq!(
        identity["compared_points"][1]["npm_tarball_sha256"],
        "cecb24eabf2eb23f0f49bf131cddd640e65017336b6298da9297bdf9d213cc0b"
    );

    let published: Vec<&str> = identity["published_stables_from_previous_ceilings"]
        .as_array()
        .expect("published stables are a list")
        .iter()
        .map(|point| point["version"].as_str().expect("version is text"))
        .collect();
    assert_eq!(published, ["0.60.0", OFFICIAL]);
    assert_eq!(identity["unpublished_later_stable"], "0.61.1");
    assert_eq!(identity["ignored_preview"], "0.62.0-preview.0");
}

#[test]
fn prior_ceiling_reproduces_the_research_324_corpus() {
    let identity = fixture(IDENTITY, "identity");
    let prior = fixture(PRIOR_IDENTITY, "prior identity");
    let point = &identity["compared_points"][0];
    assert_eq!(point["version"], PRIOR_CEILING);
    for field in [
        "github_commit",
        "github_tree",
        "npm_integrity",
        "npm_shasum",
        "npm_tarball_sha256",
        "npm_package_json_sha256",
        "npm_bin_entry_sha256",
        "github_source_archive_sha256",
        "darwin_arm64_asset_sha256",
        "darwin_arm64_extracted_sha256",
    ] {
        assert_eq!(point[field], prior["official"][field], "{field}");
    }

    let surface = fixture(SURFACE, "surface-ledger");
    let prior_surface = fixture(PRIOR_SURFACE, "prior surface-ledger");
    for group in [
        "selected_acp_sources",
        "selected_headless_sources",
        "selected_option_literals_present",
    ] {
        assert_eq!(
            surface["mapped_files"][group][PRIOR_CEILING],
            prior_surface["mapped_files"][group][PRIOR_CEILING],
            "{group}"
        );
    }
}

#[test]
fn host_bundle_is_the_official_npm_bin_entry() {
    let identity = fixture(IDENTITY, "identity");
    let host = &identity["host"];
    assert_eq!(host["version"], OFFICIAL);
    assert_eq!(
        host["bundle_sha256"],
        identity["official"]["npm_bin_entry_sha256"]
    );
    assert_eq!(host["bundle_matches_official_npm_bin_entry"], true);
    assert_eq!(host["auto_updated_from"], "0.53.0");
    assert_eq!(host["host_install_changed"], false);
}

#[test]
fn both_claims_extend_to_0_61_0_without_a_new_milestone() {
    let identity = fixture(IDENTITY, "identity");
    for axis in ["acp", "headless"] {
        let before = &identity["claim_at_observation"][axis];
        assert_eq!(before["latest_qualified"], PRIOR_CEILING);
        assert_eq!(before["classification_of_0.61.0"], "unverified_newer");
    }
    for (axis, revision) in [
        ("acp", "gemini-cli.acp.v0.51.0"),
        ("headless", "gemini-cli.headless.stream-json.v1"),
    ] {
        let decision = &identity["identity_decision"][axis];
        assert_eq!(decision["shape"], "compatible-extension");
        assert_eq!(decision["reuse_behavior_revision"], revision);
        assert_eq!(decision["raise_latest_qualified_to"], OFFICIAL);
        assert_eq!(decision["keep_baseline"], BASELINE);
        assert_eq!(decision["new_milestone"], false);
        assert_eq!(decision["qualify_published_intermediates"], true);
    }
    let guards: BTreeSet<String> = [
        "new_public_operation",
        "new_public_flag",
        "map_flash_rollout_model_routing",
        "map_build_file_protection",
        "map_acp_model_advertisement",
        "map_mcp_stdio_env_sanitization",
        "map_mcp_oauth_issuer_check",
        "map_sandbox_changes",
        "map_browser_login",
        "map_individual_account_service",
        "flatten_onto_gemini_live_or_models",
        "provider_prompt_sent",
        "live_catalogue",
        "live_session",
        "host_install_changed",
    ]
    .into_iter()
    .map(str::to_owned)
    .collect();
    let decision_keys = key_set(&identity["identity_decision"], "identity decision");
    let mut expected_keys = guards.clone();
    expected_keys.extend(
        [
            "acp",
            "headless",
            "keep_unpublished_later_stable_unverified",
        ]
        .map(str::to_owned),
    );
    assert_eq!(decision_keys, expected_keys);
    for guard in &guards {
        assert_eq!(identity["identity_decision"][guard], false, "{guard}");
    }
    assert_eq!(
        identity["identity_decision"]["keep_unpublished_later_stable_unverified"],
        true
    );
}

#[test]
fn selected_sources_are_byte_identical_across_the_window() {
    let surface = fixture(SURFACE, "surface-ledger");
    let mapped = &surface["mapped_files"];
    for group in [
        "selected_acp_sources",
        "selected_headless_sources",
        "selected_retention_sources",
        "selected_option_literals_present",
    ] {
        let prior = &mapped[group][PRIOR_CEILING];
        assert!(
            prior.as_object().is_some_and(|files| !files.is_empty()),
            "{group} is mapped"
        );
        for point in COMPARED {
            assert_eq!(&mapped[group][point], prior, "{group} at {point}");
        }
    }
    for group in [
        "acp_selected_changed_files",
        "headless_selected_changed_files",
        "retention_selected_changed_files",
    ] {
        assert_eq!(mapped[group], json!([]), "{group}");
    }
    assert_eq!(
        mapped["acp_directory_changed_files"],
        json!(["packages/cli/src/acp/acpUtils.ts"])
    );
    assert_eq!(
        mapped["selected_headless_sources"][OFFICIAL]["geminiChat.ts"],
        "446b5cd55e91b4b79273d0c4a20bdac923ba6a6bf2cebc5016c74a49519e6717"
    );
    assert_eq!(
        mapped["selected_retention_sources"][OFFICIAL]["exitCodes.ts"],
        "3f540276cf593a187d5c5180a242457ee5de038df35524e2c6b9c811bab87839"
    );
}

#[test]
fn acp_http_mcp_mapping_is_unchanged_at_0_61_0() {
    let surface = fixture(SURFACE, "surface-ledger");
    let mcp = &surface["mapped_files"]["selected_http_mcp_sources"];
    assert_eq!(
        key_set(&mcp[OFFICIAL], "http mcp sources"),
        [
            "MCPServerConfig",
            "acpRpcDispatcher.ts",
            "acpSessionManager.ts",
            "mcp-client.ts"
        ]
        .map(str::to_owned)
        .into_iter()
        .collect()
    );
    for source in [
        "acpRpcDispatcher.ts",
        "acpSessionManager.ts",
        "MCPServerConfig",
    ] {
        for point in COMPARED {
            assert_eq!(
                mcp[point][source], mcp[PRIOR_CEILING][source],
                "{source} at {point}"
            );
        }
    }
    assert_eq!(
        mcp[PRIOR_CEILING]["mcp-client.ts"],
        "60daf577146fb938d2c939db4cbc22233aeeb28fe4d304948ddbdb20bc24370e"
    );
    assert_eq!(
        mcp["0.60.0"]["mcp-client.ts"],
        mcp[OFFICIAL]["mcp-client.ts"]
    );
    assert_eq!(
        surface["mapped_files"]["http_mcp_changed_files"],
        json!(["packages/core/src/tools/mcp-client.ts"])
    );

    let protocol = fixture(PROTOCOL, "protocol");
    let http = &protocol["acp"]["http_mcp"];
    assert_eq!(http["class"], "direct-http-candidate");
    assert_eq!(http["mcp_capabilities"], json!({"http": true, "sse": true}));
    assert_eq!(http["accepted_forms"], json!(["stdio", "http", "sse"]));
    assert_eq!(http["http_maps_to"], "httpUrl");
    assert_eq!(http["sse_maps_to"], "url");
    assert_eq!(http["source_sha256"], mcp[OFFICIAL]);
    assert_eq!(http["mapping_unchanged_through_0.61.0"], true);
    assert_eq!(http["live_honouring_proven"], false);
}

#[test]
fn deterministic_changed_path_ledger_classifies_every_hop() {
    let surface = fixture(SURFACE, "surface-ledger");
    assert_eq!(
        surface["package_file_counts"],
        json!({"0.59.0": 2997, "0.60.0": 3000, "0.61.0": 3004})
    );
    let hops = &surface["changed_paths"];
    assert_eq!(
        key_set(hops, "changed paths"),
        ["0.59.0..0.60.0", "0.60.0..0.61.0"]
            .map(str::to_owned)
            .into_iter()
            .collect()
    );
    for (hop, count) in [("0.59.0..0.60.0", 93), ("0.60.0..0.61.0", 89)] {
        assert_eq!(hops[hop]["count"], count, "{hop} count");
        let paths: Vec<&str> = hops[hop]["paths"]
            .as_array()
            .unwrap_or_else(|| panic!("{hop} has paths"))
            .iter()
            .map(|path| path.as_str().expect("path"))
            .collect();
        assert_eq!(paths.len(), count, "{hop} path count");
        let mut sorted = paths.clone();
        sorted.sort_unstable();
        sorted.dedup();
        assert_eq!(paths, sorted, "{hop} paths are sorted and unique");
    }

    let classification = &surface["mapped_files"]["classification"];
    let classified = key_set(classification, "classification");
    let mut expected: BTreeSet<String> = ["sandbox", "mcp_oauth"].map(str::to_owned).into();
    for (hop, file) in [
        ("0.60.0..0.61.0", "packages/cli/src/acp/acpUtils.ts"),
        ("0.59.0..0.60.0", "packages/core/src/tools/mcp-client.ts"),
        ("0.60.0..0.61.0", "packages/core/src/config/models.ts"),
        (
            "0.60.0..0.61.0",
            "packages/core/src/core/contentGenerator.ts",
        ),
        ("0.60.0..0.61.0", "packages/core/src/config/config.ts"),
        (
            "0.59.0..0.60.0",
            "packages/core/src/policy/policy-engine.ts",
        ),
        (
            "0.60.0..0.61.0",
            "packages/core/src/policy/policy-engine.ts",
        ),
        ("0.59.0..0.60.0", "packages/core/src/safety/built-in.ts"),
        ("0.60.0..0.61.0", "packages/core/src/tools/tools.ts"),
        ("0.60.0..0.61.0", "packages/core/src/tools/write-file.ts"),
        ("0.60.0..0.61.0", "packages/core/src/tools/edit.ts"),
        (
            "0.60.0..0.61.0",
            "packages/core/src/confirmation-bus/types.ts",
        ),
        ("0.60.0..0.61.0", "packages/core/src/config/storage.ts"),
        ("0.59.0..0.60.0", "packages/cli/src/config/settings.ts"),
    ] {
        let paths = string_set(&hops[hop]["paths"], hop);
        assert!(paths.contains(file), "{file} must be in the {hop} ledger");
        expected.insert(file.to_owned());
    }
    assert_eq!(classified, expected);
    for key in &classified {
        assert!(
            classification[key]
                .as_str()
                .is_some_and(|text| !text.is_empty()),
            "{key} needs a classification"
        );
    }
}

#[test]
fn protocol_corpus_keeps_both_axes_and_unmapped_deltas_explicit() {
    let protocol = fixture(PROTOCOL, "protocol");
    assert_eq!(protocol["official_version"], OFFICIAL);
    assert_eq!(protocol["acp"]["sdk"], "@agentclientprotocol/sdk@0.16.1");
    assert_eq!(protocol["acp"]["wire_version"], 1);
    assert_eq!(
        protocol["acp"]["profile_comparison"]["compared_releases"],
        json!(COMPARED)
    );
    assert_eq!(
        protocol["acp"]["profile_comparison"]["bounded_write"]["agent_mode_id"],
        "autoEdit"
    );
    assert_eq!(
        protocol["acp"]["selected_external_shapes_unchanged_through_0.61.0"],
        true
    );
    assert_eq!(
        protocol["headless"]["selected_external_shapes_unchanged_through_0.61.0"],
        true
    );
    assert_eq!(protocol["npm_bundle_identity"]["host_bundle_matches"], true);
    assert_eq!(protocol["npm_bundle_identity"]["executed"], false);
    assert_eq!(
        protocol["headless"]["selected_terminal"]["native_exit_codes"],
        json!([41, 42, 44, 52, 53, 54, 55, 130])
    );
    assert!(
        string_set(
            &protocol["headless"]["unmapped_additions"],
            "headless additions"
        )
        .iter()
        .any(|entry| entry.starts_with("Flash rollout model routing"))
    );
    assert!(
        string_set(&protocol["acp"]["unmapped_additions"], "acp additions")
            .iter()
            .any(|entry| entry.starts_with("build-file protection"))
    );
    for (route, guard) in [
        ("acp", "provider_prompt_sent"),
        ("acp", "live_session"),
        ("headless", "provider_prompt_sent"),
        ("headless", "live_session"),
    ] {
        assert_eq!(protocol[route][guard], false, "{route}.{guard}");
    }
    assert_eq!(protocol["access_boundary"]["browser_login"], false);
}

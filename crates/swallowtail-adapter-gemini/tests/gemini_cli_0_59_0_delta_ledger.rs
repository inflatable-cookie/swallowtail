//! Identity and compatible-extension evidence for official Gemini CLI
//! `0.56.0..=0.59.0` (g05.076, Research 308 family sixteen).
//!
//! This corpus freezes the four compared official points, their npm registry
//! identity, GitHub tags, trees, and darwin-arm64 unsigned assets, plus one
//! deterministic tagged-source file inventory per point. It is
//! mutation-sensitive: the assertions fail if an official identity, the
//! selected mapped-file hash, the changed-path set, or the per-claim
//! classification drifts.
//!
//! Nothing here executes a downloaded artifact. npm tarballs were hashed
//! against their registry `integrity`/`shasum`, tagged source archives were
//! hashed and extracted statically, and no prompt, login, credential,
//! installation, or host update occurred.

use serde_json::Value;
use std::collections::BTreeSet;

const IDENTITY: &str = include_str!("fixtures/gemini-cli-0.59.0/identity.json");
const SURFACE: &str = include_str!("fixtures/gemini-cli-0.59.0/surface-ledger.json");
const PROTOCOL: &str = include_str!("fixtures/gemini-cli-0.59.0/protocol.json");
const PRIOR_IDENTITY: &str = include_str!("fixtures/gemini-cli-0.56.0/identity.json");

const OFFICIAL: &str = "0.59.0";
const PRIOR_CEILING: &str = "0.56.0";
const BASELINE: &str = "0.51.0";
const COMPARED: [&str; 4] = ["0.56.0", "0.57.0", "0.58.0", "0.59.0"];

const ACP_SOURCES: [&str; 6] = [
    "acpCommandHandler.ts",
    "acpRpcDispatcher.ts",
    "acpSessionManager.ts",
    "acpSession.ts",
    "acpFileSystemService.ts",
    "acpStdioTransport.ts",
];

const HEADLESS_0_56_0: [(&str, &str); 6] = [
    (
        "config.ts",
        "5100bcd48f798d04b9463bd72680af7202f331de566321b1c29f5f8710c2c44c",
    ),
    (
        "nonInteractiveCli.ts",
        "fe569c4ac3436a851c991e0e916554b63bd9b9eb0bf5dee644f9258fad5ba298",
    ),
    (
        "geminiChat.ts",
        "509d03416ad093b15fdf1fa891ccc719591a4d0a02976f9086b5546033e64873",
    ),
    (
        "turn.ts",
        "d4f95a7d330b6065fc0c194b82375c7ed9db9ce9b1a126dae4250b668e5801aa",
    ),
    (
        "types.ts",
        "23f7ea24497c88a703e0e4f8b6deb8bda969c2c2a32ca213beacfae46d798341",
    ),
    (
        "stream-json-formatter.ts",
        "f78377bb9cbb56cfe3509655ff6ebfaef8873641942139562dc4ab7a3347e721",
    ),
];

const HEADLESS_0_59_0: [(&str, &str); 6] = [
    (
        "config.ts",
        "bdb490bbf0ea1803d74e8bc1748432096eff7063abec0447e5e7aa467e4187b8",
    ),
    (
        "nonInteractiveCli.ts",
        "fe569c4ac3436a851c991e0e916554b63bd9b9eb0bf5dee644f9258fad5ba298",
    ),
    (
        "geminiChat.ts",
        "446b5cd55e91b4b79273d0c4a20bdac923ba6a6bf2cebc5016c74a49519e6717",
    ),
    (
        "turn.ts",
        "d4f95a7d330b6065fc0c194b82375c7ed9db9ce9b1a126dae4250b668e5801aa",
    ),
    (
        "types.ts",
        "23f7ea24497c88a703e0e4f8b6deb8bda969c2c2a32ca213beacfae46d798341",
    ),
    (
        "stream-json-formatter.ts",
        "f78377bb9cbb56cfe3509655ff6ebfaef8873641942139562dc4ab7a3347e721",
    ),
];

fn fixture(body: &str, name: &str) -> Value {
    serde_json::from_str(body).unwrap_or_else(|error| panic!("{name}: {error}"))
}

#[test]
fn official_identity_reproduces_for_every_compared_point() {
    let identity = fixture(IDENTITY, "identity");
    assert_eq!(identity["family"], "gemini-cli");
    assert_eq!(identity["npm_package"], "@google/gemini-cli");
    assert_eq!(identity["npm_latest"], true);
    assert_eq!(identity["official"]["version"], OFFICIAL);
    assert_eq!(
        identity["official"]["npm_published_at"],
        "2026-09-08T21:19:17.301Z"
    );
    assert_eq!(identity["official"]["github_tag"], "v0.59.0");
    assert_eq!(
        identity["official"]["github_commit"],
        "fb0d535af931b27c51e87e5e6ade72905b1e8390"
    );
    assert_eq!(
        identity["official"]["github_tree"],
        "a080c8f15d68e93f65a29c6b177090206038dc97"
    );
    assert_eq!(
        identity["official"]["npm_integrity"],
        "sha512-RHcjpQEMwVkrWz75mEFsuMuM0JRVJgzLQzkGhEY6SE0KjwOWFA1wYKTUqpzDUHprcE2mgcTActyd4ihVdC2dpg=="
    );
    assert_eq!(
        identity["official"]["npm_shasum"],
        "d0e2cddc280c32520ae9596059be289df380599c"
    );
    assert_eq!(
        identity["official"]["npm_tarball_sha256"],
        "59dc2cdb098b3000d36e34a185fc873932df4fd9d00900e817f2b19cd349d98b"
    );
    assert_eq!(
        identity["official"]["npm_package_json_sha256"],
        "e79e13e7c1464a008e73b54fef263b042be488dfec35dc56823c865798be1eff"
    );
    assert_eq!(
        identity["official"]["npm_bin_entry_sha256"],
        "283a6044adc7837f9331fc2a5ea06ec4d52e9738f6fd202a94a4ae0c11efd53d"
    );
    assert_eq!(
        identity["official"]["github_source_archive_sha256"],
        "6e698510dcae4341f94efe93704447b2fe456062c138f64f95da1c30996d0f33"
    );
    assert_eq!(
        identity["official"]["darwin_arm64_asset_sha256"],
        "0c8938c68df7e46fd1de63583e4c0a1d7a452e1ecb1d33065f9ac12b8814876a"
    );
    assert_eq!(identity["official"]["darwin_arm64_asset_size"], 36512845);
    assert_eq!(
        identity["official"]["darwin_arm64_extracted_sha256"],
        "f78acf4241ae6b1c9b04c9a2cb201c6a876e9e79266a9d1078b88cf833edf36c"
    );
    assert_eq!(
        identity["official"]["darwin_arm64_extracted_size"],
        121528144
    );
    assert_eq!(identity["official"]["downloaded_artifact_executed"], false);

    let points: BTreeSet<&str> = identity["compared_points"]
        .as_array()
        .expect("compared points are a list")
        .iter()
        .map(|point| point["version"].as_str().expect("version is text"))
        .collect();
    assert_eq!(points, COMPARED.iter().copied().collect());

    assert_eq!(
        identity["published_stables_from_previous_ceilings"]
            .as_array()
            .map(Vec::len),
        Some(3)
    );
    assert_eq!(
        identity["published_stables_from_previous_ceilings"][2]["version"],
        OFFICIAL
    );
    assert_eq!(identity["unpublished_later_stable"], "0.59.1");
    assert_eq!(identity["ignored_preview"], "0.60.0-preview.0");
}

#[test]
fn host_stays_observation_only_at_the_research_182_digest() {
    let identity = fixture(IDENTITY, "identity");
    assert_eq!(identity["host"]["version"], "0.53.0");
    assert_eq!(
        identity["host"]["executable_sha256"],
        "4a8f99947eae4e1ff501269ba8b9ca2d1216db044fb75e01f4ee86fd1d8f175e"
    );
    assert_eq!(identity["host"]["host_install_changed"], false);
}

#[test]
fn both_claims_extend_to_0_59_0_without_a_new_milestone() {
    let identity = fixture(IDENTITY, "identity");
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
    for guard in [
        "new_public_operation",
        "new_public_flag",
        "map_provider_retry_nudge",
        "map_workspace_trust_hardening",
        "map_mcp_oauth_ssrf_repair",
        "map_sandbox_seatbelt_changes",
        "map_browser_login",
        "map_individual_account_service",
        "flatten_onto_gemini_live_or_models",
        "provider_prompt_sent",
        "live_catalogue",
        "live_session",
        "host_install_changed",
    ] {
        assert_eq!(identity["identity_decision"][guard], false, "{guard}");
    }
}

#[test]
fn selected_acp_sources_are_byte_identical_across_the_window() {
    let surface = fixture(SURFACE, "surface-ledger");
    let acp = &surface["mapped_files"]["selected_acp_sources"];
    for source in ACP_SOURCES {
        let expected = acp[PRIOR_CEILING][source]
            .as_str()
            .unwrap_or_else(|| panic!("{source} is mapped"));
        for point in COMPARED {
            assert_eq!(
                acp[point][source], expected,
                "{source} must not change at {point}"
            );
        }
    }
    assert_eq!(
        surface["mapped_files"]["acp_selected_changed_files"]
            .as_array()
            .map(Vec::len),
        Some(0)
    );
}

#[test]
fn selected_headless_sources_change_only_inside_the_provider_request_path() {
    let surface = fixture(SURFACE, "surface-ledger");
    let headless = &surface["mapped_files"]["selected_headless_sources"];
    for (source, expected) in HEADLESS_0_56_0 {
        assert_eq!(headless[PRIOR_CEILING][source], expected, "{source}");
    }
    for (source, expected) in HEADLESS_0_59_0 {
        assert_eq!(headless[OFFICIAL][source], expected, "{source}");
    }
    for source in [
        "nonInteractiveCli.ts",
        "turn.ts",
        "types.ts",
        "stream-json-formatter.ts",
    ] {
        assert_eq!(
            headless[PRIOR_CEILING][source], headless[OFFICIAL][source],
            "{source} is unchanged on the selected headless surface"
        );
    }
    assert_eq!(
        headless["0.57.0"]["geminiChat.ts"],
        "a9e22526436fa435a25ef655f2151aa90b66de2f761d9dd7dc3b2804b2734495"
    );
    assert_eq!(
        surface["mapped_files"]["headless_selected_changed_files"],
        serde_json::json!([
            "packages/cli/src/config/config.ts",
            "packages/core/src/core/geminiChat.ts"
        ])
    );
}

#[test]
fn selected_retention_and_option_sources_are_unchanged() {
    let surface = fixture(SURFACE, "surface-ledger");
    let retention = &surface["mapped_files"]["selected_retention_sources"];
    for source in [
        "sessions.ts",
        "sessionOperations.ts",
        "gemini.tsx",
        "sessionCleanup.ts",
    ] {
        let expected = retention[PRIOR_CEILING][source]
            .as_str()
            .unwrap_or_else(|| panic!("{source} is mapped"));
        for point in COMPARED {
            assert_eq!(retention[point][source], expected, "{source} at {point}");
        }
    }
    assert_eq!(
        surface["mapped_files"]["retention_selected_changed_files"]
            .as_array()
            .map(Vec::len),
        Some(0)
    );

    let literals = &surface["mapped_files"]["selected_option_literals_present"];
    for point in COMPARED {
        assert_eq!(literals[point]["acp"], 2);
        assert_eq!(literals[point]["approval-mode"], 1);
        assert_eq!(literals[point]["output-format"], 1);
        assert_eq!(literals[point]["model"], 1);
        assert_eq!(literals[point]["skip-trust"], 2);
        assert_eq!(literals[point]["session-id"], 2);
        assert_eq!(literals[point]["delete-session"], 1);
        assert_eq!(literals[point]["stream-json"], 2);
    }
}

#[test]
fn deterministic_changed_path_ledger_classifies_every_hop() {
    let surface = fixture(SURFACE, "surface-ledger");
    let hops = &surface["changed_paths"];
    assert_eq!(hops["0.56.0..0.57.0"]["count"], 89);
    assert_eq!(hops["0.57.0..0.58.0"]["count"], 36);
    assert_eq!(hops["0.58.0..0.59.0"]["count"], 24);

    for (hop, count) in [
        ("0.56.0..0.57.0", 89),
        ("0.57.0..0.58.0", 36),
        ("0.58.0..0.59.0", 24),
    ] {
        let paths = hops[hop]["paths"]
            .as_array()
            .unwrap_or_else(|| panic!("{hop} has paths"));
        assert_eq!(paths.len(), count, "{hop} path count");
        let sorted: Vec<&str> = paths.iter().map(|p| p.as_str().expect("path")).collect();
        let mut expected = sorted.clone();
        expected.sort_unstable();
        assert_eq!(sorted, expected, "{hop} paths are sorted and unique");
        assert!(
            !sorted.iter().any(|path| path.contains("/acp/")),
            "{hop} must not change any ACP source"
        );
    }

    let classification = &surface["mapped_files"]["classification"];
    for (hop, file) in [
        ("0.56.0..0.57.0", "packages/cli/src/config/config.ts"),
        ("0.56.0..0.57.0", "packages/core/src/core/geminiChat.ts"),
        ("0.57.0..0.58.0", "packages/core/src/core/geminiChat.ts"),
        ("0.58.0..0.59.0", "packages/core/src/config/config.ts"),
        ("0.58.0..0.59.0", "packages/core/src/utils/trust.ts"),
    ] {
        let paths = hops[hop]["paths"].as_array().expect("paths");
        assert!(
            paths.iter().any(|path| path == file),
            "{file} must be in the {hop} ledger"
        );
    }
    for file in [
        "packages/core/src/config/config.ts",
        "packages/core/src/policy/policy-engine.ts",
        "packages/core/src/utils/trust.ts",
        "packages/core/src/availability/modelAvailabilityService.ts",
    ] {
        assert!(
            classification[file]
                .as_str()
                .is_some_and(|text| !text.is_empty()),
            "{file} needs a classification"
        );
    }
}

#[test]
fn prior_0_56_0_ceiling_corpus_still_reproduces() {
    let prior = fixture(PRIOR_IDENTITY, "prior identity");
    assert_eq!(prior["official"]["version"], PRIOR_CEILING);
    assert_eq!(
        prior["official"]["github_tree"],
        "379c84605a494dd3adf650acb3cc2a6d82e82e53"
    );
    assert_eq!(
        prior["official"]["npm_tarball_sha256"],
        "e25443a59b22f0000d6418ce42c5c0710bc04d8f41b5567417e30e038a80120b"
    );
    assert_eq!(
        prior["official"]["darwin_arm64_extracted_sha256"],
        "fa84c229012862d3695775afafff6e07dcffaa6da22db4072a6dbe87e5265151"
    );
}

#[test]
fn protocol_corpus_keeps_both_axes_and_unmapped_deltas_explicit() {
    let protocol = fixture(PROTOCOL, "protocol");
    assert_eq!(protocol["official_version"], OFFICIAL);
    assert_eq!(protocol["acp"]["axis"], "gemini-cli.acp-agent");
    assert_eq!(
        protocol["headless"]["axis"],
        "gemini-cli.headless-stream-json"
    );
    assert_eq!(protocol["acp"]["sdk"], "@agentclientprotocol/sdk@0.16.1");
    assert_eq!(protocol["acp"]["wire_version"], 1);
    assert_eq!(
        protocol["acp"]["selected_external_shapes_unchanged_through_0.59.0"],
        true
    );
    assert_eq!(
        protocol["headless"]["selected_external_shapes_unchanged_through_0.59.0"],
        true
    );
    assert_eq!(protocol["npm_bundle_identity"]["executed"], false);
    assert_eq!(
        protocol["headless"]["selected_terminal"]["native_exit_codes"],
        serde_json::json!([41, 42, 44, 52, 53, 54, 55, 130])
    );
    assert_eq!(
        protocol["headless"]["selected_event_types"],
        serde_json::json!([
            "init",
            "message",
            "tool_use",
            "tool_result",
            "error",
            "result"
        ])
    );
    for (route, guard) in [
        ("acp", "provider_prompt_sent"),
        ("acp", "live_session"),
        ("headless", "provider_prompt_sent"),
        ("headless", "live_session"),
    ] {
        assert_eq!(protocol[route][guard], false, "{route}.{guard}");
    }
    assert!(
        !protocol["acp"]["unmapped_additions"]
            .as_array()
            .expect("acp additions")
            .is_empty()
    );
    assert!(
        !protocol["headless"]["unmapped_additions"]
            .as_array()
            .expect("headless additions")
            .is_empty()
    );
    assert_eq!(protocol["access_boundary"]["browser_login"], false);
    assert_eq!(
        protocol["access_boundary"]["individual_account_service"],
        false
    );
}

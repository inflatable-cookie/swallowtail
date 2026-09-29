//! Frozen identity ledger for the `0.3.270` → `0.3.284` Claude Agent SDK hops.
//!
//! These assertions are mutation-sensitive on purpose: exact string sets and
//! exact digests, never counts alone. If the frozen inventory or the classified
//! deltas are edited without re-deriving the evidence, this fails. Nothing here
//! executes a downloaded artifact, contacts a provider, or reads a credential.
//! This ledger names no range: the route stays exact one-point per axis, and
//! Research 301 live evidence stays bound to `0.3.259`/`2.1.259`.
//!
//! Every assertion below binds the frozen fixture to itself with literals, so
//! this test holds both before and after the production rebind lands. The live
//! constants are bound by the selection and sidecar suites, not here.

use serde_json::Value;
use std::collections::BTreeSet;

const IDENTITY: &str = include_str!("fixtures/claude-agent-sdk-0.3.284/identity.json");
const PROTOCOL: &str = include_str!("fixtures/claude-agent-sdk-0.3.284/protocol.json");
const INVENTORY: &str = include_str!("fixtures/claude-agent-sdk-0.3.284/dist-inventory.json");

fn json(text: &str) -> Value {
    serde_json::from_str(text).expect("frozen evidence is valid JSON")
}

fn exact_set(value: &Value, expected: &[&str]) -> bool {
    let actual: BTreeSet<&str> = value
        .as_array()
        .expect("frozen evidence array")
        .iter()
        .map(|entry| entry.as_str().expect("frozen evidence string"))
        .collect();
    actual == expected.iter().copied().collect::<BTreeSet<&str>>()
}

#[test]
fn the_official_point_is_exactly_the_frozen_0_3_284_artifact() {
    let identity = json(IDENTITY);
    assert_eq!(identity["official"]["version"], "0.3.284");
    assert_eq!(identity["native"]["0.3.284"]["version"], "2.1.284");
    assert_eq!(
        identity["native"]["0.3.284"]["commit"],
        "2b8ce618c24de26410e4bdfc4e1d592accd61f61"
    );
    assert_eq!(
        identity["official"]["tarball_sha256"],
        "4550e830246026133fc1802a2208dd0f3a785cae1eec83f261d114c33d797771"
    );
    assert_eq!(
        identity["official"]["tarball_sha1"],
        "28ab0fde5207c162d8dd1dbaaac5e0f5d5ec46c7"
    );
    assert_eq!(
        identity["official"]["published"],
        "2026-09-28T17:15:09.221Z"
    );
    assert!(exact_set(&identity["official"]["dist_tags"], &["latest"]));
    assert_eq!(identity["official"]["next_dist_tag"], "0.3.285");
    assert_eq!(
        identity["previous_ceiling"]["tarball_sha256"],
        "36e86fc13a1ddc8c026dcf5e5bcf72f4107bcbceaff1a45e7f81c5d44d703575"
    );
    assert_eq!(
        identity["previous_ceiling"]["corroborates_research_315"],
        true
    );
    let between: Vec<&str> = identity["published_stables_between"]
        .as_array()
        .expect("published stables array")
        .iter()
        .map(|entry| entry["version"].as_str().expect("hop version"))
        .collect();
    assert_eq!(
        between,
        vec![
            "0.3.271", "0.3.272", "0.3.273", "0.3.274", "0.3.275", "0.3.276", "0.3.277", "0.3.278",
            "0.3.280", "0.3.281", "0.3.282", "0.3.283",
        ]
    );
    assert!(exact_set(&identity["unpublished_gaps"], &["0.3.279"]));
    assert_eq!(identity["first_unpublished_later_stable"], "0.3.286");
    for entry in identity["published_stables_between"]
        .as_array()
        .expect("published stables array")
    {
        let wrapper = entry["version"].as_str().expect("hop version");
        let expected = format!("2.1.{}", &wrapper[4..]);
        assert_eq!(
            entry["coupled_native"].as_str().expect("coupled native"),
            expected.as_str(),
            "{wrapper} must carry its coupled native"
        );
    }
    for flag in [
        "no_prompt",
        "no_live_session",
        "no_login",
        "no_install",
        "nothing_executed",
    ] {
        assert_eq!(identity[flag], true, "{flag} must hold");
    }
}

#[test]
fn the_package_tree_gains_core_at_0_3_282_and_keeps_four_identical_files() {
    let inventory = json(INVENTORY);
    let compared = inventory["compared"].as_array().expect("compared versions");
    assert_eq!(compared.len(), 14);
    for version in compared {
        let version = version.as_str().expect("compared version");
        let expected = if version >= "0.3.282" { 19 } else { 15 };
        assert_eq!(
            inventory["package_file_counts"][version], expected,
            "{version} must ship exactly {expected} files"
        );
    }
    assert!(exact_set(
        &inventory["identical_through_0_3_270_to_0_3_284"],
        &[
            "LICENSE.md",
            "agentSdkTypes.d.ts",
            "extractFromBunfs.d.ts",
            "extractFromBunfs.js",
        ]
    ));
    let changed = [
        (
            "from_0_3_270_to_0_3_271",
            &[
                "bridge.mjs",
                "browser-sdk.d.ts",
                "browser-sdk.js",
                "manifest.json",
                "manifest.zst.json",
                "package.json",
                "sdk-tools.d.ts",
                "sdk.d.ts",
                "sdk.mjs",
            ][..],
        ),
        (
            "from_0_3_271_to_0_3_272",
            &[
                "bridge.mjs",
                "browser-sdk.js",
                "manifest.json",
                "manifest.zst.json",
                "package.json",
                "sdk.mjs",
            ],
        ),
        (
            "from_0_3_272_to_0_3_273",
            &[
                "bridge.mjs",
                "browser-sdk.d.ts",
                "browser-sdk.js",
                "manifest.json",
                "manifest.zst.json",
                "package.json",
                "sdk-tools.d.ts",
                "sdk.d.ts",
                "sdk.mjs",
            ],
        ),
        (
            "from_0_3_273_to_0_3_274",
            &[
                "bridge.mjs",
                "browser-sdk.js",
                "manifest.json",
                "manifest.zst.json",
                "package.json",
                "sdk-tools.d.ts",
                "sdk.d.ts",
                "sdk.mjs",
            ],
        ),
        (
            "from_0_3_274_to_0_3_275",
            &[
                "bridge.mjs",
                "browser-sdk.js",
                "manifest.json",
                "manifest.zst.json",
                "package.json",
                "sdk-tools.d.ts",
                "sdk.d.ts",
                "sdk.mjs",
            ],
        ),
        (
            "from_0_3_275_to_0_3_276",
            &[
                "bridge.mjs",
                "browser-sdk.js",
                "manifest.json",
                "manifest.zst.json",
                "package.json",
                "sdk.mjs",
            ],
        ),
        (
            "from_0_3_276_to_0_3_277",
            &[
                "bridge.d.ts",
                "bridge.mjs",
                "browser-sdk.js",
                "manifest.json",
                "manifest.zst.json",
                "package.json",
                "sdk-tools.d.ts",
                "sdk.d.ts",
                "sdk.mjs",
            ],
        ),
        (
            "from_0_3_277_to_0_3_278",
            &[
                "bridge.mjs",
                "browser-sdk.js",
                "manifest.json",
                "manifest.zst.json",
                "package.json",
                "sdk.mjs",
            ],
        ),
        (
            "from_0_3_278_to_0_3_280",
            &[
                "bridge.mjs",
                "browser-sdk.js",
                "manifest.json",
                "manifest.zst.json",
                "package.json",
                "sdk.d.ts",
                "sdk.mjs",
            ],
        ),
        (
            "from_0_3_280_to_0_3_281",
            &[
                "bridge.d.ts",
                "bridge.mjs",
                "browser-sdk.js",
                "manifest.json",
                "manifest.zst.json",
                "package.json",
                "sdk-tools.d.ts",
                "sdk.d.ts",
                "sdk.mjs",
            ],
        ),
        (
            "from_0_3_281_to_0_3_282",
            &[
                "README.md",
                "bridge.mjs",
                "browser-sdk.js",
                "manifest.json",
                "manifest.zst.json",
                "package.json",
                "sdk.d.ts",
                "sdk.mjs",
            ],
        ),
        (
            "from_0_3_282_to_0_3_283",
            &[
                "bridge.mjs",
                "browser-sdk.js",
                "core.mjs",
                "manifest.json",
                "manifest.zst.json",
                "package.json",
                "sdk.d.ts",
                "sdk.mjs",
            ],
        ),
        (
            "from_0_3_283_to_0_3_284",
            &[
                "bridge.mjs",
                "browser-sdk.js",
                "core.mjs",
                "manifest.json",
                "manifest.zst.json",
                "package.json",
                "sdk-tools.d.ts",
                "sdk.d.ts",
                "sdk.mjs",
            ],
        ),
    ];
    for (hop, files) in changed {
        assert!(
            exact_set(&inventory[hop]["changed"], files),
            "{hop} changed"
        );
    }
    for hop in [
        "from_0_3_270_to_0_3_271",
        "from_0_3_271_to_0_3_272",
        "from_0_3_272_to_0_3_273",
        "from_0_3_273_to_0_3_274",
        "from_0_3_274_to_0_3_275",
        "from_0_3_275_to_0_3_276",
        "from_0_3_276_to_0_3_277",
        "from_0_3_277_to_0_3_278",
        "from_0_3_278_to_0_3_280",
        "from_0_3_280_to_0_3_281",
    ] {
        assert!(exact_set(&inventory[hop]["added"], &[]), "{hop} added");
        assert!(exact_set(&inventory[hop]["removed"], &[]), "{hop} removed");
    }
    assert!(exact_set(
        &inventory["from_0_3_281_to_0_3_282"]["added"],
        &[
            "core-276encc2.mjs",
            "core-5drmq1nt.mjs",
            "core.d.ts",
            "core.mjs",
        ]
    ));
    assert!(exact_set(
        &inventory["from_0_3_281_to_0_3_282"]["removed"],
        &[]
    ));
    assert!(exact_set(
        &inventory["from_0_3_282_to_0_3_283"]["added"],
        &["core-3mr1yqt9.mjs", "core-rg0sgcv0.mjs"]
    ));
    assert!(exact_set(
        &inventory["from_0_3_282_to_0_3_283"]["removed"],
        &["core-276encc2.mjs", "core-5drmq1nt.mjs"]
    ));
    assert!(exact_set(
        &inventory["from_0_3_283_to_0_3_284"]["added"],
        &["core-3ctbshc7.mjs", "core-eg2e19h0.mjs"]
    ));
    assert!(exact_set(
        &inventory["from_0_3_283_to_0_3_284"]["removed"],
        &["core-3mr1yqt9.mjs", "core-rg0sgcv0.mjs"]
    ));

    assert_eq!(
        inventory["hashes"]["sdk.d.ts"]["0.3.284"],
        "048ae2e6c796cc2aa3c423afaad59a08972cb48c271ffcc9847d910ff65f61b2"
    );
    assert_eq!(
        inventory["hashes"]["sdk.mjs"]["0.3.284"],
        "32d062c37b03e10870fbf839f54694545ee01bc0ec719e47078fbed76e30ef71"
    );
    let bridge = &inventory["hashes"]["bridge.d.ts"];
    for version in [
        "0.3.271", "0.3.272", "0.3.273", "0.3.274", "0.3.275", "0.3.276",
    ] {
        assert_eq!(
            bridge[version], bridge["0.3.270"],
            "bridge.d.ts must stay byte-identical through {version}"
        );
    }
    assert_ne!(
        bridge["0.3.277"], bridge["0.3.276"],
        "bridge.d.ts must move at 0.3.277"
    );
    for version in ["0.3.278", "0.3.280"] {
        assert_eq!(
            bridge[version], bridge["0.3.277"],
            "bridge.d.ts must not move again before 0.3.281"
        );
    }
    assert_ne!(
        bridge["0.3.281"], bridge["0.3.280"],
        "bridge.d.ts must move at 0.3.281"
    );
    for version in ["0.3.282", "0.3.283", "0.3.284"] {
        assert_eq!(
            bridge[version], bridge["0.3.281"],
            "bridge.d.ts must not move after 0.3.281"
        );
    }
}

#[test]
fn every_declaration_delta_is_classified_and_none_is_mapped() {
    let protocol = json(PROTOCOL);
    assert_eq!(protocol["mapped_subset_unchanged"], true);
    let deltas = protocol["declaration_deltas"]
        .as_array()
        .expect("classified deltas");
    let symbols: BTreeSet<&str> = deltas
        .iter()
        .map(|delta| delta["symbol"].as_str().expect("delta symbol"))
        .collect();
    assert_eq!(
        symbols,
        BTreeSet::from([
            "CanUseTool.options.mcpServer",
            "McpServerProvenance / SDKStartupFailureReason / SDKUsageReport",
            "McpServerStatus.source",
            "McpServerStatus.tools[]._meta",
            "McpStdioServerConfig.bareElicitationCapability",
            "Query.readMcpResource",
            "bridge.d.ts thinkingDisplay highlights and verdict return (0.3.277, 0.3.281)",
            "exports['./core'] tree refactor at 0.3.282",
            "in-process bash process.kill(-pid) left sdk.mjs at 0.3.281",
            "prewarm / SpareProcess / ClaimOptions",
            "sdk-tools skill/tool deltas",
            "setMaxThinkingTokens thinkingDisplay highlights",
            "updateSettings source userSettings",
        ])
    );
    for delta in deltas {
        assert_eq!(
            delta["mapped"], false,
            "{} is classified as mapped and needs a behavior revision",
            delta["symbol"]
        );
        let reason = delta["why_unmapped"].as_str().expect("unmapped reason");
        assert!(
            reason.len() > 40,
            "{} needs a real reason, not a label",
            delta["symbol"]
        );
    }
}

#[test]
fn the_lifecycle_and_credential_invariants_survived_every_hop() {
    let protocol = json(PROTOCOL);
    let invariants = &protocol["implementation_invariants_unchanged"];
    for key in [
        "can_use_tool_gating",
        "mcp_stdio_forwarding",
        "mcp_status_passthrough",
        "spawn_callback",
        "account_projection",
        "no_joined_stop",
        "stderr_drain",
        "permission_prompts_default",
        "plugin_delivery_default",
        "query_close",
        "mcp_wire_constants",
    ] {
        assert!(
            invariants[key]
                .as_str()
                .is_some_and(|value| !value.is_empty()),
            "{key} invariant must be recorded"
        );
    }
    let credential = &protocol["credential_non_custody"];
    assert_eq!(credential["ten_pattern_hits_in_default_entry"], 3);
    assert_eq!(credential["hits_are_prose_only"], true);
    assert_eq!(credential["exported_functions_0_3_284"], 18);
    assert_eq!(credential["added_export"], "prewarm");
    assert_eq!(credential["login_or_oauth_exports"], 0);
    assert_eq!(credential["entry_points_dot_and_extract_unchanged"], true);
    assert_eq!(credential["new_core_export_unmapped"], true);

    let native = &protocol["native_artifact_rotation"];
    assert_eq!(
        native["all_eight_platform_binaries_rotated_every_hop"],
        true
    );
    assert_eq!(native["harness_schema_unchanged"], true);
    assert_eq!(
        native["darwin_arm64_0_3_284_checksum"],
        "50a14c2f50f56668380fdda490167f1d3630d5cc18fb8aed3073c2c7ea7314fe"
    );

    let allowlist = &protocol["sidecar_allowlist"];
    assert_eq!(allowlist["source_is_discarded"], true);
    assert!(exact_set(
        &allowlist["declared_keys_0_3_284"],
        &[
            "name",
            "status",
            "serverInfo",
            "error",
            "config",
            "scope",
            "source",
            "tools",
        ]
    ));
}

#[test]
fn the_route_sets_no_new_optional_selector() {
    // `permissionPrompts: 'none'` would stop `canUseTool` being called at all;
    // `pluginDelivery: 'initialize'` would change how plugins reach the native
    // process. New optional selectors on this hop stay unmapped: the sidecar
    // never names them, and the frozen classification keeps saying why.
    let source = swallowtail_adapter_claude_agent::sdk::CLAUDE_AGENT_SDK_SIDECAR_SOURCE;
    for forbidden in [
        "permissionPrompts",
        "pluginDelivery",
        "sdkMcpServerManifests",
        "bareElicitationCapability",
        "readMcpResource",
        "prewarm",
    ] {
        assert!(
            !source.contains(forbidden),
            "the sidecar must not set {forbidden}"
        );
    }
    let protocol = json(PROTOCOL);
    let deltas = protocol["declaration_deltas"]
        .as_array()
        .expect("classified deltas");
    for symbol in [
        "CanUseTool.options.mcpServer",
        "McpStdioServerConfig.bareElicitationCapability",
        "Query.readMcpResource",
        "prewarm / SpareProcess / ClaimOptions",
        "exports['./core'] tree refactor at 0.3.282",
    ] {
        let delta = deltas
            .iter()
            .find(|delta| delta["symbol"] == symbol)
            .unwrap_or_else(|| panic!("{symbol} is classified"));
        assert_eq!(delta["mapped"], false);
    }
}

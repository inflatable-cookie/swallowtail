//! Frozen identity ledger for the `0.3.259` → `0.3.270` Claude Agent SDK hops.
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

const IDENTITY: &str = include_str!("fixtures/claude-agent-sdk-0.3.270/identity.json");
const PROTOCOL: &str = include_str!("fixtures/claude-agent-sdk-0.3.270/protocol.json");
const INVENTORY: &str = include_str!("fixtures/claude-agent-sdk-0.3.270/dist-inventory.json");

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
fn the_official_point_is_exactly_the_frozen_0_3_270_artifact() {
    let identity = json(IDENTITY);
    assert_eq!(identity["official"]["version"], "0.3.270");
    assert_eq!(identity["native"]["0.3.270"]["version"], "2.1.270");
    assert_eq!(
        identity["native"]["0.3.270"]["commit"],
        "97ecbf7abeb4170dcfd26c4d4b397afd9015030e"
    );
    // The npm digest is the sole artifact identity for this family, so it is
    // asserted exactly rather than by size or file count.
    assert_eq!(
        identity["official"]["tarball_sha256"],
        "36e86fc13a1ddc8c026dcf5e5bcf72f4107bcbceaff1a45e7f81c5d44d703575"
    );
    assert_eq!(
        identity["official"]["tarball_sha1"],
        "2a1d5fd4e265320a64e62607ae87567bd7b955d8"
    );
    assert_eq!(
        identity["official"]["published"],
        "2026-09-12T18:53:13.002Z"
    );
    assert!(exact_set(
        &identity["official"]["dist_tags"],
        &["latest", "next"]
    ));
    // The previous ceiling must still match Research 280 byte for byte, or the
    // ledger is not chained to the frozen point.
    assert_eq!(
        identity["previous_ceiling"]["tarball_sha256"],
        "0c5740e44a536ab6fd32f2a7de0d508b75d34782ebc219b87aa8d834449a3f7e"
    );
    assert_eq!(
        identity["previous_ceiling"]["corroborates_research_280"],
        true
    );
    // Every published stable after the ceiling is named; the two gaps are kept.
    let between: Vec<&str> = identity["published_stables_between"]
        .as_array()
        .expect("published stables array")
        .iter()
        .map(|entry| entry["version"].as_str().expect("hop version"))
        .collect();
    assert_eq!(
        between,
        vec![
            "0.3.260", "0.3.261", "0.3.263", "0.3.265", "0.3.266", "0.3.267", "0.3.268", "0.3.269",
        ]
    );
    assert!(exact_set(
        &identity["unpublished_gaps"],
        &["0.3.262", "0.3.264"]
    ));
    assert_eq!(identity["first_unpublished_later_stable"], "0.3.271");
    // Wrapper and native stay coupled one-to-one on every hop.
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
fn the_package_tree_delta_is_exactly_six_identical_through_every_hop() {
    let inventory = json(INVENTORY);
    let compared = inventory["compared"].as_array().expect("compared versions");
    assert_eq!(compared.len(), 10);
    for version in compared {
        let version = version.as_str().expect("compared version");
        assert_eq!(
            inventory["package_file_counts"][version], 15,
            "{version} must ship exactly 15 files"
        );
    }
    // Exact set: a file silently moving out of this bucket fails here.
    assert!(exact_set(
        &inventory["identical_through_0_3_259_to_0_3_270"],
        &[
            "LICENSE.md",
            "README.md",
            "agentSdkTypes.d.ts",
            "bridge.d.ts",
            "extractFromBunfs.d.ts",
            "extractFromBunfs.js",
        ]
    ));
    // Per-hop changed sets, in order. Three hops carry zero declaration
    // changes: pure native rotation and metadata.
    let expected: &[(&str, &[&str])] = &[
        (
            "from_0_3_259_to_0_3_260",
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
            "from_0_3_260_to_0_3_261",
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
            "from_0_3_261_to_0_3_263",
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
            "from_0_3_263_to_0_3_265",
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
            "from_0_3_265_to_0_3_266",
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
            "from_0_3_266_to_0_3_267",
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
            "from_0_3_267_to_0_3_268",
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
            "from_0_3_268_to_0_3_269",
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
            "from_0_3_269_to_0_3_270",
            &[
                "bridge.mjs",
                "browser-sdk.js",
                "manifest.json",
                "manifest.zst.json",
                "package.json",
                "sdk.mjs",
            ],
        ),
    ];
    for (hop, changed) in expected {
        let hop_value = &inventory[*hop];
        assert!(exact_set(&hop_value["changed"], changed));
        assert!(exact_set(&hop_value["added"], &[]));
        assert!(exact_set(&hop_value["removed"], &[]));
    }

    // The two entry points this route actually loads, pinned by digest.
    assert_eq!(
        inventory["hashes"]["sdk.d.ts"]["0.3.270"],
        "851c62ed778ee56625537a608ef93fbb53445a687eac9314ae6c758c94f1ce44"
    );
    assert_eq!(
        inventory["hashes"]["sdk.mjs"]["0.3.270"],
        "ad98da1735760c234efb02a1bc6aaa3b25911a773d8334fd1a7a1e6cf46e8b31"
    );
    // The credential-bearing bridge declaration did not move on any hop.
    let bridge = &inventory["hashes"]["bridge.d.ts"];
    for version in [
        "0.3.260", "0.3.261", "0.3.263", "0.3.265", "0.3.266", "0.3.267", "0.3.268", "0.3.269",
        "0.3.270",
    ] {
        assert_eq!(
            bridge[version], bridge["0.3.259"],
            "bridge.d.ts must stay byte-identical through {version}"
        );
    }
    // The browser declaration moved exactly once, at 0.3.266 -> 0.3.267.
    let browser = &inventory["hashes"]["browser-sdk.d.ts"];
    for version in ["0.3.260", "0.3.261", "0.3.263", "0.3.265", "0.3.266"] {
        assert_eq!(
            browser[version], browser["0.3.259"],
            "browser-sdk.d.ts must not move before 0.3.267"
        );
    }
    assert_ne!(
        browser["0.3.267"], browser["0.3.266"],
        "browser-sdk.d.ts must move exactly at 0.3.267"
    );
    for version in ["0.3.268", "0.3.269", "0.3.270"] {
        assert_eq!(
            browser[version], browser["0.3.267"],
            "browser-sdk.d.ts must not move after 0.3.267"
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
    // The named surfaces this ledger had to cover, each with a reason.
    let symbols: BTreeSet<&str> = deltas
        .iter()
        .map(|delta| delta["symbol"].as_str().expect("delta symbol"))
        .collect();
    assert_eq!(
        symbols,
        BTreeSet::from([
            "Options.pluginDelivery",
            "Query.reloadOutputStyles",
            "SDKAssistantMessageError verification_required/cloud_credential_error",
            "SDKControlGetHooksListingRequest/Response",
            "SDKControlGetUsageRequest.skip_behaviors",
            "SDKControlListPermissionRules + rule/workspace types",
            "SDKControlMcpCall STAGED prose",
            "SDKControlReloadOutputStylesRequest/Response",
            "SDKControlReloadPluginsRequest.hold_on_cache_impact",
            "SDKControlRenameSessionRequest.source/session_id",
            "SDKControlRequestInner +3 members",
            "SDKPermissionDeniedMessage prose",
            "Settings bashEditDiffEnabled/bashOutputMaxChars/taskOutputMaxChars",
            "Settings gatewayInternalNetworks/parentSettingsBehavior",
            "Settings keybindingFlavor deprecation",
            "Settings managedSourcesBehavior prose",
            "Settings maxEffortLevel (+per-model)",
            "Settings prependPlugins/appendPlugins/extraKnownMarketplaces",
            "Transport.markDelivered",
            "ambient prose x3",
            "api-retry no_response",
            "applyFlagSettings / effortLevel max prose",
            "ask-dialog defaultToNo / suppressAlwaysAllowRule",
            "browser-sdk.d.ts SSE resume cursors (0.3.266->0.3.267)",
            "context-usage row kind",
            "initialize plugins[]",
            "initialize sdkMcpServerManifests",
            "initialize-response models / plugins_applied",
            "interrupt cancel_queued/still_queued prose",
            "pending_permission_requests / pending_user_dialog_requests prose",
            "reloadPlugins holdOnCacheImpact + held/cache_impact",
            "resume_reason",
            "resumed_from_incomplete_thinking",
            "result resume_reason/result_index/local_command/timings",
            "sdk-tools skill/tool deltas",
            "spend-limit limitScope",
            "system-message betas",
            "systemPromptSnapshot default prose",
            "thinking_tokens user_message_uuid",
            "usage model_scoped prose",
            "usage_EXPERIMENTAL skipBehaviors",
            "user_message_uuid / user_message_uuids prose",
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
        "spawn_callback",
        "account_projection",
        "no_joined_stop",
        "stderr_drain",
        "permission_prompts_default",
        "plugin_delivery_default",
        "system_prompt_snapshot",
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
    assert_eq!(credential["exported_functions"], 17);
    assert_eq!(credential["login_or_oauth_exports"], 0);
    assert_eq!(credential["entry_points_unchanged"], true);
    assert_eq!(protocol["bridge_declaration_byte_identical_all_hops"], true);

    let native = &protocol["native_artifact_rotation"];
    assert_eq!(
        native["all_eight_platform_binaries_rotated_every_hop"],
        true
    );
    // The wrapper-to-native protocol schema is what would force a behavior
    // revision; the rotation alone does not.
    assert_eq!(native["harness_schema_unchanged"], true);
    assert_eq!(
        native["darwin_arm64_0_3_270_checksum"],
        "a506b6d970a4cf44f6abdb53a81ddcd5d3b0ce042a95c502fe9d1f946bdb8807"
    );
}

#[test]
fn the_route_sets_no_new_optional_selector() {
    // `permissionPrompts: 'none'` would stop `canUseTool` being called at all;
    // `pluginDelivery: 'initialize'` would change how plugins reach the native
    // process. The shipped asset must never name either, and the frozen
    // classification must keep saying why.
    let source = swallowtail_adapter_claude_agent::sdk::CLAUDE_AGENT_SDK_SIDECAR_SOURCE;
    for forbidden in [
        "permissionPrompts",
        "pluginDelivery",
        "sdkMcpServerManifests",
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
        "Options.pluginDelivery",
        "ask-dialog defaultToNo / suppressAlwaysAllowRule",
    ] {
        let delta = deltas
            .iter()
            .find(|delta| delta["symbol"] == symbol)
            .unwrap_or_else(|| panic!("{symbol} is classified"));
        assert_eq!(delta["mapped"], false);
    }
}

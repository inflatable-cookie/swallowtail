//! Frozen identity ledger for the `0.3.258` → `0.3.259` Claude Agent SDK hop.
//!
//! These assertions are mutation-sensitive on purpose: exact string sets and
//! exact digests, never counts alone. If the frozen inventory or the classified
//! deltas are edited without re-deriving the evidence, this fails. Nothing here
//! executes a downloaded artifact, contacts a provider, or reads a credential.

use serde_json::Value;
use std::collections::BTreeSet;
use swallowtail_adapter_claude_agent::sdk::{
    CLAUDE_AGENT_SDK_NATIVE_VERSION, CLAUDE_AGENT_SDK_VERSION,
};

const IDENTITY: &str = include_str!("fixtures/claude-agent-sdk-0.3.259/identity.json");
const PROTOCOL: &str = include_str!("fixtures/claude-agent-sdk-0.3.259/protocol.json");
const INVENTORY: &str = include_str!("fixtures/claude-agent-sdk-0.3.259/dist-inventory.json");

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
fn the_bound_points_are_exactly_the_frozen_official_artifact() {
    let identity = json(IDENTITY);
    assert_eq!(identity["official"]["version"], CLAUDE_AGENT_SDK_VERSION);
    assert_eq!(
        identity["native"]["0.3.259"]["version"],
        CLAUDE_AGENT_SDK_NATIVE_VERSION
    );
    // The npm digest is the sole artifact identity for this family, so it is
    // asserted exactly rather than by size or file count.
    assert_eq!(
        identity["official"]["tarball_sha256"],
        "0c5740e44a536ab6fd32f2a7de0d508b75d34782ebc219b87aa8d834449a3f7e"
    );
    assert_eq!(
        identity["official"]["tarball_sha1"],
        "daf465f8231392ab99e1c7fc7f1e14c3d25ea012"
    );
    assert_eq!(
        identity["official"]["published"],
        "2026-09-02T21:22:40.857Z"
    );
    assert!(exact_set(
        &identity["official"]["dist_tags"],
        &["latest", "next"]
    ));
    // The previous ceiling must still match Research 278 byte for byte, or the
    // hop is not the hop this ledger describes.
    assert_eq!(
        identity["previous_ceiling"]["tarball_sha256"],
        "656cf237bc567cb172a007a0fd5b3958cf960d154c03ab390a755d2c3bdbb398"
    );
    assert_eq!(
        identity["previous_ceiling"]["corroborates_research_278"],
        true
    );
    // No intermediate published stable hides inside the hop.
    assert_eq!(
        identity["published_stables_between"]
            .as_array()
            .expect("published stables array")
            .len(),
        0
    );
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
fn the_package_tree_delta_is_exactly_seven_identical_and_eight_changed_files() {
    let inventory = json(INVENTORY);
    assert_eq!(inventory["package_file_counts"]["0.3.258"], 15);
    assert_eq!(inventory["package_file_counts"]["0.3.259"], 15);
    let hop = &inventory["from_0_3_258_to_0_3_259"];

    // Exact sets: a file silently moving between these buckets fails here.
    assert!(exact_set(
        &hop["identical"],
        &[
            "LICENSE.md",
            "README.md",
            "agentSdkTypes.d.ts",
            "bridge.d.ts",
            "browser-sdk.d.ts",
            "extractFromBunfs.d.ts",
            "extractFromBunfs.js",
        ]
    ));
    assert!(exact_set(
        &hop["changed"],
        &[
            "bridge.mjs",
            "browser-sdk.js",
            "manifest.json",
            "manifest.zst.json",
            "package.json",
            "sdk-tools.d.ts",
            "sdk.d.ts",
            "sdk.mjs",
        ]
    ));
    assert!(exact_set(&hop["added"], &[]));
    assert!(exact_set(&hop["removed"], &[]));

    // The two entry points this route actually loads, pinned by digest.
    assert_eq!(
        inventory["hashes"]["sdk.d.ts"]["0.3.259"],
        "f76aa847ddf433a2d7ff9c28935c0764467f00d31f35044226382ddf86ad7d81"
    );
    assert_eq!(
        inventory["hashes"]["sdk.mjs"]["0.3.259"],
        "7fa7c212361864544e775e7551519e790515f95d4bb6a4831b0b05f5b368a0c5"
    );
    // The credential-bearing subpath declarations did not move at all.
    for declaration in ["bridge.d.ts", "browser-sdk.d.ts"] {
        assert_eq!(
            inventory["hashes"][declaration]["0.3.258"],
            inventory["hashes"][declaration]["0.3.259"],
            "{declaration} must stay byte-identical"
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
    // The named surfaces this refresh had to cover, each with a reason.
    let symbols: BTreeSet<&str> = deltas
        .iter()
        .map(|delta| delta["symbol"].as_str().expect("delta symbol"))
        .collect();
    assert_eq!(
        symbols,
        BTreeSet::from([
            "Options.permissionPrompts",
            "Settings.allowedMcpServers / managedSourcesBehavior",
            "Settings.managedMcpServers",
            "sdk-tools skill publish fields",
            "task summary",
            "user_message_uuids",
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
fn the_lifecycle_and_credential_invariants_survived_the_hop() {
    let protocol = json(PROTOCOL);
    let invariants = &protocol["implementation_invariants_unchanged"];
    for key in [
        "can_use_tool_gating",
        "spawn_callback",
        "no_joined_stop",
        "stderr_drain",
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
    assert_eq!(
        credential["bridge_and_browser_declarations_byte_identical"],
        true
    );

    let native = &protocol["native_artifact_rotation"];
    assert_eq!(native["all_eight_platform_binaries_rotated"], true);
    // The wrapper-to-native protocol schema is what would force a behavior
    // revision; the rotation alone does not.
    assert_eq!(native["harness_schema_unchanged"], true);
}

#[test]
fn the_route_never_sets_the_new_permission_prompt_selector() {
    // `permissionPrompts: 'none'` would stop `canUseTool` being called at all.
    // The shipped asset must never name it, and the frozen classification must
    // keep saying why.
    assert!(
        !swallowtail_adapter_claude_agent::sdk::CLAUDE_AGENT_SDK_SIDECAR_SOURCE
            .contains("permissionPrompts"),
        "the sidecar must not set permissionPrompts"
    );
    let protocol = json(PROTOCOL);
    let delta = protocol["declaration_deltas"]
        .as_array()
        .expect("classified deltas")
        .iter()
        .find(|delta| delta["symbol"] == "Options.permissionPrompts")
        .expect("permissionPrompts is classified");
    assert_eq!(delta["kind"], "added_optional_input");
    assert!(
        delta["why_unmapped"]
            .as_str()
            .expect("reason")
            .contains("canUseTool"),
        "the reason must name the admission path it protects"
    );
}

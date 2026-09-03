//! Exact identity, credential non-custody, and route-distinctness proofs for
//! the `claude-agent.sdk` route.
//!
//! Nothing here executes the downloaded official artifact, opens a session, or
//! contacts a provider: every assertion reads the shipped adapter asset and
//! the adapter's own declarations.

use swallowtail_adapter_claude_agent::sdk::{
    CLAUDE_AGENT_SDK_ADDABLE_ROUTE_ID, CLAUDE_AGENT_SDK_BEHAVIOR, CLAUDE_AGENT_SDK_NATIVE_VERSION,
    CLAUDE_AGENT_SDK_NODE_RUNTIME, CLAUDE_AGENT_SDK_PACKAGE, CLAUDE_AGENT_SDK_SIDECAR_ENTRY_FILE,
    CLAUDE_AGENT_SDK_SIDECAR_SOURCE, CLAUDE_AGENT_SDK_SIDECAR_SOURCE_TAG, CLAUDE_AGENT_SDK_VERSION,
    CLAUDE_AGENT_SDK_WIRE, claude_agent_sdk_addable_route_descriptor, claude_agent_sdk_descriptor,
    claude_agent_sdk_tool_admission_namespace,
};
use swallowtail_core::ExecutionHostId;
use swallowtail_runtime::HostServices;

const PROTOCOL: &str = include_str!("fixtures/claude-agent-sdk-v1/protocol.json");

fn protocol() -> serde_json::Value {
    serde_json::from_str(PROTOCOL).expect("frozen corpus identity is valid JSON")
}

#[test]
fn the_route_binds_five_independent_exact_identities() {
    let descriptor = claude_agent_sdk_descriptor();
    assert_eq!(
        descriptor.identity().id().as_str(),
        "swallowtail.claude-agent.sdk"
    );
    assert_eq!(
        descriptor.transport_family().as_str(),
        CLAUDE_AGENT_SDK_WIRE
    );
    for (axis, point) in [
        ("claude-agent.sdk.package", CLAUDE_AGENT_SDK_VERSION),
        ("claude-agent.sdk.native", CLAUDE_AGENT_SDK_NATIVE_VERSION),
        ("claude-agent.sdk.node", CLAUDE_AGENT_SDK_NODE_RUNTIME),
        ("claude-agent.sdk.wire", CLAUDE_AGENT_SDK_WIRE),
        (
            "claude-agent.sdk.sidecar",
            CLAUDE_AGENT_SDK_SIDECAR_SOURCE_TAG,
        ),
    ] {
        let axis = swallowtail_core::InterfaceVersionAxis::new(axis).expect("valid axis");
        let claim = descriptor
            .interface_compatibility(&axis)
            .unwrap_or_else(|| panic!("axis {axis:?} is bound"));
        assert_eq!(
            claim.newer_version_posture(),
            swallowtail_core::InterfaceNewerVersionPosture::QualifiedOnly,
            "no axis may inherit an unverified-newer posture"
        );
        assert_eq!(claim.milestones().len(), 1, "each axis starts at one point");
        let point = swallowtail_core::InterfaceVersion::new(point).expect("valid version");
        assert_eq!(claim.baseline(), &point);
        assert_eq!(claim.latest_qualified(), &point);
        assert_eq!(claim.exclusions().len(), 0);
    }
    // The SDK wrapper and the native binary it delivers are coupled but never
    // equal, and neither is the Claude Code axis.
    assert_ne!(CLAUDE_AGENT_SDK_VERSION, CLAUDE_AGENT_SDK_NATIVE_VERSION);
    assert_eq!(
        swallowtail_adapter_claude_agent::CLAUDE_CODE_HEADLESS_LATEST_QUALIFIED_VERSION,
        "2.1.257",
        "the Claude Code window is observed separately and does not transfer"
    );
}

#[test]
fn the_shipped_asset_matches_the_frozen_identity() {
    let protocol = protocol();
    assert_eq!(protocol["sdk_package"], CLAUDE_AGENT_SDK_PACKAGE);
    assert_eq!(protocol["sdk_version"], CLAUDE_AGENT_SDK_VERSION);
    assert_eq!(protocol["native_version"], CLAUDE_AGENT_SDK_NATIVE_VERSION);
    assert_eq!(protocol["node_runtime"], CLAUDE_AGENT_SDK_NODE_RUNTIME);
    assert_eq!(protocol["behavior_revision"], CLAUDE_AGENT_SDK_BEHAVIOR);
    assert_eq!(
        protocol["sidecar_entry_file"],
        CLAUDE_AGENT_SDK_SIDECAR_ENTRY_FILE
    );
    assert!(
        CLAUDE_AGENT_SDK_SIDECAR_SOURCE_TAG.starts_with("swallowtail-claude-agent-sdk-sidecar@")
    );
    assert!(CLAUDE_AGENT_SDK_SIDECAR_SOURCE.contains(CLAUDE_AGENT_SDK_WIRE));
    assert!(CLAUDE_AGENT_SDK_SIDECAR_SOURCE.contains(CLAUDE_AGENT_SDK_VERSION));
    assert!(CLAUDE_AGENT_SDK_SIDECAR_SOURCE.contains(CLAUDE_AGENT_SDK_NATIVE_VERSION));
}

#[test]
fn the_sidecar_asset_can_never_reach_a_credential_bearing_surface() {
    // Mechanical falsifier from the frozen evidence: the `.` entry point only.
    // `/bridge` and `/browser` declare raw access tokens, minted worker
    // credentials, and OAuth credential messages.
    let forbidden = protocol();
    let forbidden = forbidden["forbidden_specifiers"]
        .as_array()
        .expect("the frozen corpus lists forbidden specifiers");
    assert!(!forbidden.is_empty());
    for specifier in forbidden {
        let specifier = specifier.as_str().expect("specifier is text");
        for line in CLAUDE_AGENT_SDK_SIDECAR_SOURCE.lines() {
            let code = line.trim_start();
            if code.starts_with("//") {
                continue;
            }
            assert!(
                !code.contains(specifier),
                "sidecar code must never reference {specifier}"
            );
        }
    }
    // Explicit environment on every launch: omission would inherit the parent
    // environment and could silently select API-key authentication.
    assert!(CLAUDE_AGENT_SDK_SIDECAR_SOURCE.contains("env: {}"));
    assert!(CLAUDE_AGENT_SDK_SIDECAR_SOURCE.contains("settingSources: []"));
    assert!(CLAUDE_AGENT_SDK_SIDECAR_SOURCE.contains("skills: []"));
    assert!(CLAUDE_AGENT_SDK_SIDECAR_SOURCE.contains("persistSession: false"));
}

#[test]
fn the_sidecar_holds_an_independently_joinable_native_handle() {
    // The SDK supplies a discarded bounded wait, not a join, so the sidecar
    // must retain its own handle and report only what it observed of it.
    assert!(CLAUDE_AGENT_SDK_SIDECAR_SOURCE.contains("spawnClaudeCodeProcess"));
    assert!(CLAUDE_AGENT_SDK_SIDECAR_SOURCE.contains("class NativeChild"));
    assert!(CLAUDE_AGENT_SDK_SIDECAR_SOURCE.contains("detached: false"));
    for observation in ["exited", "survivor"] {
        assert!(CLAUDE_AGENT_SDK_SIDECAR_SOURCE.contains(observation));
    }
    // The sidecar owns no cleanup vocabulary: it cannot escalate, speak for
    // the owned tree, or call anything clean.
    for reserved in ["OwnedTreeEmpty", "CleanupOutcome", "escalated"] {
        assert!(
            !CLAUDE_AGENT_SDK_SIDECAR_SOURCE.contains(reserved),
            "the sidecar must not carry the host's {reserved} vocabulary"
        );
    }
    let protocol = protocol();
    assert_eq!(
        protocol["sidecar_native_join_observations"]
            .as_array()
            .expect("native join observations are listed")
            .len(),
        2
    );
    // Route cleanup outcomes stay Rust-side and evidence-keyed.
    assert_eq!(
        protocol["route_cleanup_outcomes"]
            .as_array()
            .expect("route cleanup outcomes are listed")
            .len(),
        3
    );
}

#[test]
fn the_addable_route_exposes_references_and_no_sign_in_action() {
    let host = ExecutionHostId::new("claude-agent-sdk.fixture.addable").expect("valid host");
    let services = HostServices::new(host);
    let unavailable = claude_agent_sdk_addable_route_descriptor(&services);
    assert_eq!(
        unavailable.availability(),
        swallowtail_core::AddableRouteAvailability::Unavailable(
            swallowtail_core::AddableRouteMissingRequirement::HostService
        )
    );
    assert_eq!(unavailable.id().as_str(), CLAUDE_AGENT_SDK_ADDABLE_ROUTE_ID);
    assert_eq!(
        unavailable.topology(),
        swallowtail_core::RouteTopology::Installed
    );
    let fields: Vec<&str> = unavailable
        .config_fields()
        .map(|field| field.id().as_str())
        .collect();
    assert_eq!(fields, ["environment", "launch_recipe"]);
    let credentials: Vec<&str> = unavailable
        .credential_fields()
        .map(|field| field.id().as_str())
        .collect();
    assert_eq!(credentials, ["delegated_subscription"]);
    assert_eq!(unavailable.sign_in_actions().count(), 0);
}

#[test]
fn tool_admission_uses_a_route_local_namespace() {
    // Research 279 leaves shared vocabulary to orchestrator integration, so
    // this route does not reuse the ACP permission namespace.
    assert_eq!(
        claude_agent_sdk_tool_admission_namespace().as_str(),
        "claude-agent-sdk/can-use-tool"
    );
    assert_ne!(
        claude_agent_sdk_tool_admission_namespace(),
        swallowtail_adapter_claude_agent::claude_agent_permission_namespace()
    );
}

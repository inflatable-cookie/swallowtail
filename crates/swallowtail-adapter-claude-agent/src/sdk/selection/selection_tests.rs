use super::{
    CLAUDE_AGENT_SDK_NATIVE_AXIS, CLAUDE_AGENT_SDK_NODE_AXIS, CLAUDE_AGENT_SDK_PACKAGE_AXIS,
    CLAUDE_AGENT_SDK_SIDECAR_AXIS, CLAUDE_AGENT_SDK_WIRE_AXIS, claude_agent_sdk_native_binding,
    claude_agent_sdk_native_claim, claude_agent_sdk_node_binding, claude_agent_sdk_node_claim,
    claude_agent_sdk_package_binding, claude_agent_sdk_package_claim,
    claude_agent_sdk_sidecar_binding, claude_agent_sdk_sidecar_claim,
    claude_agent_sdk_wire_binding, claude_agent_sdk_wire_claim,
};
use crate::sdk::{
    CLAUDE_AGENT_SDK_BEHAVIOR, CLAUDE_AGENT_SDK_NATIVE_VERSION, CLAUDE_AGENT_SDK_NODE_RUNTIME,
    CLAUDE_AGENT_SDK_SIDECAR_SOURCE_TAG, CLAUDE_AGENT_SDK_VERSION, CLAUDE_AGENT_SDK_WIRE,
};
use swallowtail_core::InterfaceVersion;

fn version(value: &str) -> InterfaceVersion {
    InterfaceVersion::new(value).expect("fixture version is valid")
}

#[test]
fn package_claim_qualifies_only_the_exact_sdk_point() {
    let claim = claude_agent_sdk_package_claim();
    assert_eq!(claim.axis().as_str(), CLAUDE_AGENT_SDK_PACKAGE_AXIS);
    let assessment = claim.assess(&version(CLAUDE_AGENT_SDK_VERSION));
    assert!(assessment.is_permitted());
    assert_eq!(
        assessment.behavior_revision().unwrap().as_str(),
        CLAUDE_AGENT_SDK_BEHAVIOR
    );
    // The publication cadence is roughly daily; neighbouring points are not
    // qualified and never become visible unverified newer on this route.
    for rejected in ["0.3.257", "0.3.259", "0.3.258-rc.1", "0.3.252"] {
        assert!(
            !claim.permits(&version(rejected)),
            "unqualified point {rejected} must be rejected"
        );
    }
    assert!(claude_agent_sdk_package_binding(CLAUDE_AGENT_SDK_VERSION).is_some());
    for value in ["", " 0.3.258", "latest", "0.3.258 "] {
        assert!(claude_agent_sdk_package_binding(value).is_none());
    }
}

#[test]
fn native_and_node_axes_stay_separate_from_the_wrapper_axis() {
    let native = claude_agent_sdk_native_claim();
    assert_eq!(native.axis().as_str(), CLAUDE_AGENT_SDK_NATIVE_AXIS);
    assert!(native.permits(&version(CLAUDE_AGENT_SDK_NATIVE_VERSION)));
    // The Claude Code routes sit on the same native version family. Their
    // qualification never transfers here and this one never transfers back.
    for rejected in ["2.1.257", "2.1.259", "0.3.258"] {
        assert!(!native.permits(&version(rejected)));
    }
    assert!(claude_agent_sdk_native_binding(CLAUDE_AGENT_SDK_NATIVE_VERSION).is_some());
    assert!(claude_agent_sdk_native_binding("2.1").is_none());

    let node = claude_agent_sdk_node_claim();
    assert_eq!(node.axis().as_str(), CLAUDE_AGENT_SDK_NODE_AXIS);
    assert!(node.permits(&version(CLAUDE_AGENT_SDK_NODE_RUNTIME)));
    for rejected in ["22.23.1", "22.23.3", "18.0.0", "23.0.0"] {
        assert!(!node.permits(&version(rejected)));
    }
    assert!(claude_agent_sdk_node_binding(CLAUDE_AGENT_SDK_NODE_RUNTIME).is_some());
    assert!(claude_agent_sdk_node_binding("22.x").is_none());
}

#[test]
fn opaque_claims_qualify_only_the_exact_wire_and_sidecar_points() {
    let wire = claude_agent_sdk_wire_claim();
    assert_eq!(wire.axis().as_str(), CLAUDE_AGENT_SDK_WIRE_AXIS);
    assert!(wire.permits(&version(CLAUDE_AGENT_SDK_WIRE)));
    assert!(!wire.permits(&version("swallowtail-claude-agent-sdk-jsonl-v2")));
    // The ACP route's wire identity is a different axis entirely.
    assert!(claude_agent_sdk_wire_binding("acp-v1").is_none());
    assert!(claude_agent_sdk_wire_binding(CLAUDE_AGENT_SDK_WIRE).is_some());

    let sidecar = claude_agent_sdk_sidecar_claim();
    assert_eq!(sidecar.axis().as_str(), CLAUDE_AGENT_SDK_SIDECAR_AXIS);
    assert!(sidecar.permits(&version(CLAUDE_AGENT_SDK_SIDECAR_SOURCE_TAG)));
    assert!(!sidecar.permits(&version("swallowtail-claude-agent-sdk-sidecar@0.0.0")));
    assert!(claude_agent_sdk_sidecar_binding(CLAUDE_AGENT_SDK_SIDECAR_SOURCE_TAG).is_some());
    assert!(claude_agent_sdk_sidecar_binding("").is_none());
}

#[test]
fn every_axis_is_distinct() {
    let axes = [
        CLAUDE_AGENT_SDK_PACKAGE_AXIS,
        CLAUDE_AGENT_SDK_NATIVE_AXIS,
        CLAUDE_AGENT_SDK_NODE_AXIS,
        CLAUDE_AGENT_SDK_WIRE_AXIS,
        CLAUDE_AGENT_SDK_SIDECAR_AXIS,
    ];
    let unique: std::collections::BTreeSet<&str> = axes.into_iter().collect();
    assert_eq!(unique.len(), axes.len());
    for axis in axes {
        assert!(axis.starts_with("claude-agent.sdk."));
        assert_ne!(axis, crate::CLAUDE_AGENT_ACP_AXIS);
        assert_ne!(axis, crate::CLAUDE_CODE_HEADLESS_AXIS);
        assert_ne!(axis, crate::CLAUDE_CODE_RESPONSE_ONLY_AXIS);
    }
}

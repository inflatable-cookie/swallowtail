//! Interface-version selection for the Claude Agent SDK sidecar route.
//!
//! Five axes bind the exact SDK wrapper package, the exact native binary
//! version the shipped manifest declares, the exact approved Node runtime,
//! the private sidecar wire, and the source-tagged sidecar revision. Every
//! claim is a qualified-only one-point segment. None inherits the
//! `claude-agent.acp` window or either Claude Code window: the wrapper and
//! native axes are coupled but not equal, so a Claude Code qualification
//! never transfers here and this route's qualification never transfers back.

use super::{
    CLAUDE_AGENT_SDK_BEHAVIOR, CLAUDE_AGENT_SDK_NATIVE_VERSION, CLAUDE_AGENT_SDK_NODE_RUNTIME,
    CLAUDE_AGENT_SDK_SIDECAR_SOURCE_TAG, CLAUDE_AGENT_SDK_VERSION, CLAUDE_AGENT_SDK_WIRE,
};
use crate::sdk::failure::failure;
use swallowtail_core::{
    InterfaceBehaviorRevision, InterfaceCompatibilityClaim, InterfaceCompatibilityClaimId,
    InterfaceNewerVersionPosture, InterfaceSupportStatus, InterfaceVersion, InterfaceVersionAxis,
    InterfaceVersionBinding, InterfaceVersionScheme, PreflightPlan,
};
use swallowtail_runtime::RuntimeFailure;

/// Semantic-version axis for the exact SDK wrapper package.
pub const CLAUDE_AGENT_SDK_PACKAGE_AXIS: &str = "claude-agent.sdk.package";
/// Semantic-version axis for the exact native binary the wrapper delivers.
pub const CLAUDE_AGENT_SDK_NATIVE_AXIS: &str = "claude-agent.sdk.native";
/// Semantic-version axis for the exact approved Node runtime.
pub const CLAUDE_AGENT_SDK_NODE_AXIS: &str = "claude-agent.sdk.node";
/// Opaque axis for the private sidecar wire identity.
pub const CLAUDE_AGENT_SDK_WIRE_AXIS: &str = "claude-agent.sdk.wire";
/// Opaque axis for the source-tagged sidecar revision.
pub const CLAUDE_AGENT_SDK_SIDECAR_AXIS: &str = "claude-agent.sdk.sidecar";

/// Parses one exact SDK wrapper package semantic-version binding.
#[must_use]
pub fn claude_agent_sdk_package_binding(value: &str) -> Option<InterfaceVersionBinding> {
    swallowtail_runtime::parse_semantic_version_binding(
        &InterfaceVersionAxis::new(CLAUDE_AGENT_SDK_PACKAGE_AXIS)
            .expect("static SDK sidecar axis is valid"),
        value,
    )
}

/// Parses one exact native binary semantic-version binding.
#[must_use]
pub fn claude_agent_sdk_native_binding(value: &str) -> Option<InterfaceVersionBinding> {
    swallowtail_runtime::parse_semantic_version_binding(
        &InterfaceVersionAxis::new(CLAUDE_AGENT_SDK_NATIVE_AXIS)
            .expect("static SDK sidecar axis is valid"),
        value,
    )
}

/// Parses one exact Node runtime semantic-version binding.
#[must_use]
pub fn claude_agent_sdk_node_binding(value: &str) -> Option<InterfaceVersionBinding> {
    swallowtail_runtime::parse_semantic_version_binding(
        &InterfaceVersionAxis::new(CLAUDE_AGENT_SDK_NODE_AXIS)
            .expect("static SDK sidecar axis is valid"),
        value,
    )
}

/// Binds the exact opaque sidecar wire identity.
#[must_use]
pub fn claude_agent_sdk_wire_binding(value: &str) -> Option<InterfaceVersionBinding> {
    if value != CLAUDE_AGENT_SDK_WIRE {
        return None;
    }
    Some(InterfaceVersionBinding::new(
        InterfaceVersionAxis::new(CLAUDE_AGENT_SDK_WIRE_AXIS)
            .expect("static SDK sidecar axis is valid"),
        InterfaceVersion::new(CLAUDE_AGENT_SDK_WIRE).ok()?,
    ))
}

/// Binds the exact opaque sidecar source tag.
#[must_use]
pub fn claude_agent_sdk_sidecar_binding(value: &str) -> Option<InterfaceVersionBinding> {
    if value != CLAUDE_AGENT_SDK_SIDECAR_SOURCE_TAG {
        return None;
    }
    Some(InterfaceVersionBinding::new(
        InterfaceVersionAxis::new(CLAUDE_AGENT_SDK_SIDECAR_AXIS)
            .expect("static SDK sidecar axis is valid"),
        InterfaceVersion::new(CLAUDE_AGENT_SDK_SIDECAR_SOURCE_TAG).ok()?,
    ))
}

/// Returns the qualified-only one-point SDK wrapper package claim.
#[must_use]
pub fn claude_agent_sdk_package_claim() -> InterfaceCompatibilityClaim {
    claim(
        "claude-agent.sdk.package-window-1",
        InterfaceVersionAxis::new(CLAUDE_AGENT_SDK_PACKAGE_AXIS)
            .expect("static SDK sidecar axis is valid"),
        InterfaceVersionScheme::Semantic,
        InterfaceVersion::new(CLAUDE_AGENT_SDK_VERSION)
            .expect("static SDK sidecar version is valid"),
    )
}

/// Returns the qualified-only one-point native binary claim.
#[must_use]
pub fn claude_agent_sdk_native_claim() -> InterfaceCompatibilityClaim {
    claim(
        "claude-agent.sdk.native-window-1",
        InterfaceVersionAxis::new(CLAUDE_AGENT_SDK_NATIVE_AXIS)
            .expect("static SDK sidecar axis is valid"),
        InterfaceVersionScheme::Semantic,
        InterfaceVersion::new(CLAUDE_AGENT_SDK_NATIVE_VERSION)
            .expect("static SDK sidecar version is valid"),
    )
}

/// Returns the qualified-only one-point Node runtime claim.
#[must_use]
pub fn claude_agent_sdk_node_claim() -> InterfaceCompatibilityClaim {
    claim(
        "claude-agent.sdk.node-window-1",
        InterfaceVersionAxis::new(CLAUDE_AGENT_SDK_NODE_AXIS)
            .expect("static SDK sidecar axis is valid"),
        InterfaceVersionScheme::Semantic,
        InterfaceVersion::new(CLAUDE_AGENT_SDK_NODE_RUNTIME)
            .expect("static SDK sidecar version is valid"),
    )
}

/// Returns the qualified-only one-point sidecar wire claim.
#[must_use]
pub fn claude_agent_sdk_wire_claim() -> InterfaceCompatibilityClaim {
    claim(
        "claude-agent.sdk.wire-v1",
        InterfaceVersionAxis::new(CLAUDE_AGENT_SDK_WIRE_AXIS)
            .expect("static SDK sidecar axis is valid"),
        InterfaceVersionScheme::Opaque,
        InterfaceVersion::new(CLAUDE_AGENT_SDK_WIRE).expect("static SDK sidecar version is valid"),
    )
}

/// Returns the qualified-only one-point sidecar source-tag claim.
#[must_use]
pub fn claude_agent_sdk_sidecar_claim() -> InterfaceCompatibilityClaim {
    claim(
        "claude-agent.sdk.sidecar-v1",
        InterfaceVersionAxis::new(CLAUDE_AGENT_SDK_SIDECAR_AXIS)
            .expect("static SDK sidecar axis is valid"),
        InterfaceVersionScheme::Opaque,
        InterfaceVersion::new(CLAUDE_AGENT_SDK_SIDECAR_SOURCE_TAG)
            .expect("static SDK sidecar version is valid"),
    )
}

pub(crate) fn validate_claude_agent_sdk_plan_versions(
    plan: &PreflightPlan,
) -> Result<(), RuntimeFailure> {
    for claim in [
        claude_agent_sdk_package_claim(),
        claude_agent_sdk_native_claim(),
        claude_agent_sdk_node_claim(),
        claude_agent_sdk_wire_claim(),
        claude_agent_sdk_sidecar_claim(),
    ] {
        validate_axis(plan, &claim)?;
    }
    Ok(())
}

fn validate_axis(
    plan: &PreflightPlan,
    claim: &InterfaceCompatibilityClaim,
) -> Result<(), RuntimeFailure> {
    let mut bindings = plan
        .interface_versions()
        .filter(|binding| binding.axis() == claim.axis());
    let binding = bindings.next().ok_or_else(|| {
        failure(
            "swallowtail.claude-agent.sdk.version_missing",
            "Claude Agent SDK sidecar plan is missing an exact bound interface version",
        )
    })?;
    if bindings.next().is_some() {
        return Err(failure(
            "swallowtail.claude-agent.sdk.version_ambiguous",
            "Claude Agent SDK sidecar plan contains more than one version on one axis",
        ));
    }
    let assessment = claim.assess(binding.version());
    if assessment != plan.assess_interface_version(binding)
        || !assessment.is_permitted()
        || assessment
            .behavior_revision()
            .is_none_or(|revision| revision.as_str() != CLAUDE_AGENT_SDK_BEHAVIOR)
    {
        return Err(failure(
            "swallowtail.claude-agent.sdk.version_incompatible",
            "Claude Agent SDK sidecar bound version is incompatible with this driver",
        ));
    }
    Ok(())
}

fn claim(
    id: &str,
    axis: InterfaceVersionAxis,
    scheme: InterfaceVersionScheme,
    version: InterfaceVersion,
) -> InterfaceCompatibilityClaim {
    InterfaceCompatibilityClaim::new(
        InterfaceCompatibilityClaimId::new(id).expect("static SDK sidecar claim id is valid"),
        axis,
        scheme,
        InterfaceNewerVersionPosture::QualifiedOnly,
        [swallowtail_core::InterfaceVersionSegment::exact(
            version,
            InterfaceBehaviorRevision::new(CLAUDE_AGENT_SDK_BEHAVIOR)
                .expect("static SDK sidecar behavior revision is valid"),
            InterfaceSupportStatus::Maintained,
        )],
        [],
    )
    .expect("static SDK sidecar compatibility claim is valid")
}

#[cfg(test)]
mod selection_tests;

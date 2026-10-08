//! Interface-version selection for the Claude Agent SDK sidecar route.
//!
//! Five axes bind the SDK wrapper, the native binary declared by its shipped
//! manifest, the approved Node runtime, the private sidecar wire, and the
//! source-tagged sidecar revision. The package/native axes retain their
//! baselines and claim IDs while extending across published hops `0.3.285`–
//! `0.3.293`; the independent Node axis is maintained through `22.23.3` (Research
//! 387). Wire and sidecar source stay exact. Package/native versions are checked
//! together; no Claude Code or ACP qualification transfers here.

use super::{
    CLAUDE_AGENT_SDK_BASELINE_VERSION, CLAUDE_AGENT_SDK_BEHAVIOR,
    CLAUDE_AGENT_SDK_NATIVE_BASELINE_VERSION, CLAUDE_AGENT_SDK_NATIVE_VERSION,
    CLAUDE_AGENT_SDK_NODE_RUNTIME, CLAUDE_AGENT_SDK_SIDECAR_SOURCE_TAG, CLAUDE_AGENT_SDK_VERSION,
    CLAUDE_AGENT_SDK_WIRE,
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

const CLAUDE_AGENT_SDK_NODE_BASELINE: &str = "22.23.2";

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

/// Returns the qualified-only maintained SDK wrapper package segment.
#[must_use]
pub fn claude_agent_sdk_package_claim() -> InterfaceCompatibilityClaim {
    window_claim(
        "claude-agent.sdk.package-window-1",
        InterfaceVersionAxis::new(CLAUDE_AGENT_SDK_PACKAGE_AXIS)
            .expect("static SDK sidecar axis is valid"),
        CLAUDE_AGENT_SDK_BASELINE_VERSION,
        InterfaceVersion::new(CLAUDE_AGENT_SDK_VERSION)
            .expect("static SDK sidecar version is valid"),
    )
}

/// Returns the qualified-only maintained native binary segment.
#[must_use]
pub fn claude_agent_sdk_native_claim() -> InterfaceCompatibilityClaim {
    window_claim(
        "claude-agent.sdk.native-window-1",
        InterfaceVersionAxis::new(CLAUDE_AGENT_SDK_NATIVE_AXIS)
            .expect("static SDK sidecar axis is valid"),
        CLAUDE_AGENT_SDK_NATIVE_BASELINE_VERSION,
        InterfaceVersion::new(CLAUDE_AGENT_SDK_NATIVE_VERSION)
            .expect("static SDK sidecar version is valid"),
    )
}

/// Returns the qualified-only Node runtime claim through current Node 22.
#[must_use]
pub fn claude_agent_sdk_node_claim() -> InterfaceCompatibilityClaim {
    InterfaceCompatibilityClaim::new(
        InterfaceCompatibilityClaimId::new("claude-agent.sdk.node-window-1")
            .expect("static SDK sidecar claim id is valid"),
        InterfaceVersionAxis::new(CLAUDE_AGENT_SDK_NODE_AXIS)
            .expect("static SDK sidecar axis is valid"),
        InterfaceVersionScheme::Semantic,
        InterfaceNewerVersionPosture::QualifiedOnly,
        [swallowtail_core::InterfaceVersionSegment::new(
            InterfaceVersion::new(CLAUDE_AGENT_SDK_NODE_BASELINE)
                .expect("static SDK baseline version is valid"),
            InterfaceVersion::new(CLAUDE_AGENT_SDK_NODE_RUNTIME)
                .expect("static SDK sidecar version is valid"),
            InterfaceBehaviorRevision::new(CLAUDE_AGENT_SDK_BEHAVIOR)
                .expect("static SDK sidecar behavior revision is valid"),
            InterfaceSupportStatus::Maintained,
        )],
        [],
    )
    .expect("static SDK sidecar compatibility claim is valid")
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
    let package = plan
        .interface_versions()
        .find(|binding| binding.axis().as_str() == CLAUDE_AGENT_SDK_PACKAGE_AXIS)
        .expect("validated package axis is present");
    let native = plan
        .interface_versions()
        .find(|binding| binding.axis().as_str() == CLAUDE_AGENT_SDK_NATIVE_AXIS)
        .expect("validated native axis is present");
    if !package_native_pair_matches(package.version().as_str(), native.version().as_str()) {
        return Err(failure(
            "swallowtail.claude-agent.sdk.version_incompatible",
            "Claude Agent SDK wrapper and embedded native versions must be a qualified manifest pair",
        ));
    }
    Ok(())
}

fn package_native_pair_matches(package: &str, native: &str) -> bool {
    package.strip_prefix("0.3.").is_some_and(|package_patch| {
        native
            .strip_prefix("2.1.")
            .is_some_and(|native_patch| native_patch == package_patch)
    })
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
    window_claim_versions(id, axis, scheme, version.clone(), version)
}

fn window_claim(
    id: &str,
    axis: InterfaceVersionAxis,
    baseline: &str,
    current: InterfaceVersion,
) -> InterfaceCompatibilityClaim {
    window_claim_versions(
        id,
        axis,
        InterfaceVersionScheme::Semantic,
        InterfaceVersion::new(baseline).expect("static SDK baseline is valid"),
        current,
    )
}

fn window_claim_versions(
    id: &str,
    axis: InterfaceVersionAxis,
    scheme: InterfaceVersionScheme,
    baseline: InterfaceVersion,
    current: InterfaceVersion,
) -> InterfaceCompatibilityClaim {
    InterfaceCompatibilityClaim::new(
        InterfaceCompatibilityClaimId::new(id).expect("static SDK sidecar claim id is valid"),
        axis,
        scheme,
        InterfaceNewerVersionPosture::QualifiedOnly,
        [swallowtail_core::InterfaceVersionSegment::new(
            baseline,
            current,
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

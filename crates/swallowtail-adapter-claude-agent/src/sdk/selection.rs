//! Interface-version selection for the Claude Agent SDK sidecar route.
//!
//! Five axes bind the SDK wrapper, the native binary declared by its shipped
//! manifest, the approved Node runtime, the private sidecar wire, and the
//! source-tagged sidecar revision. The package/native axes retain their
//! baselines and claim IDs while extending across every published hop through
//! `0.3.317` (the unpublished `0.3.297`, `0.3.298`, and `0.3.313` stay outside
//! the maintained pair set); the independent Node axis is maintained through
//! `22.23.3` (Research 387). Wire and sidecar source stay exact. Package/native
//! versions are checked together; no Claude Code or ACP qualification
//! transfers here.

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

/// One exact package/native pair qualified for the maintained SDK segment.
///
/// The package and native axes are independently bound in the preflight plan,
/// while this closed set ensures that only an artifact-backed coupled pair
/// can be selected for preparation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ClaudeAgentSdkPackageNativePair {
    /// SDK package `0.3.284` with native `2.1.284`.
    V0_3_284Native2_1_284,
    /// SDK package `0.3.285` with native `2.1.285`.
    V0_3_285Native2_1_285,
    /// SDK package `0.3.286` with native `2.1.286`.
    V0_3_286Native2_1_286,
    /// SDK package `0.3.287` with native `2.1.287`.
    V0_3_287Native2_1_287,
    /// SDK package `0.3.288` with native `2.1.288`.
    V0_3_288Native2_1_288,
    /// SDK package `0.3.289` with native `2.1.289`.
    V0_3_289Native2_1_289,
    /// SDK package `0.3.290` with native `2.1.290`.
    V0_3_290Native2_1_290,
    /// SDK package `0.3.291` with native `2.1.291`.
    V0_3_291Native2_1_291,
    /// SDK package `0.3.292` with native `2.1.292`.
    V0_3_292Native2_1_292,
    /// SDK package `0.3.293` with native `2.1.293`.
    V0_3_293Native2_1_293,
    /// SDK package `0.3.294` with native `2.1.294`.
    V0_3_294Native2_1_294,
    /// SDK package `0.3.295` with native `2.1.295`.
    V0_3_295Native2_1_295,
    /// SDK package `0.3.296` with native `2.1.296`.
    V0_3_296Native2_1_296,
    /// SDK package `0.3.299` with native `2.1.299`.
    V0_3_299Native2_1_299,
    /// SDK package `0.3.300` with native `2.1.300`.
    V0_3_300Native2_1_300,
    /// SDK package `0.3.301` with native `2.1.301`.
    V0_3_301Native2_1_301,
    /// SDK package `0.3.302` with native `2.1.302`.
    V0_3_302Native2_1_302,
    /// SDK package `0.3.303` with native `2.1.303`.
    V0_3_303Native2_1_303,
    /// SDK package `0.3.304` with native `2.1.304`.
    V0_3_304Native2_1_304,
    /// SDK package `0.3.305` with native `2.1.305`.
    V0_3_305Native2_1_305,
    /// SDK package `0.3.306` with native `2.1.306`.
    V0_3_306Native2_1_306,
    /// SDK package `0.3.307` with native `2.1.307`.
    V0_3_307Native2_1_307,
    /// SDK package `0.3.308` with native `2.1.308`.
    V0_3_308Native2_1_308,
    /// SDK package `0.3.309` with native `2.1.309`.
    V0_3_309Native2_1_309,
    /// SDK package `0.3.310` with native `2.1.310`.
    V0_3_310Native2_1_310,
    /// SDK package `0.3.311` with native `2.1.311`.
    V0_3_311Native2_1_311,
    /// SDK package `0.3.312` with native `2.1.312`.
    V0_3_312Native2_1_312,
    /// SDK package `0.3.314` with native `2.1.314`.
    V0_3_314Native2_1_314,
    /// SDK package `0.3.315` with native `2.1.315`.
    V0_3_315Native2_1_315,
    /// SDK package `0.3.316` with native `2.1.316`.
    V0_3_316Native2_1_316,
    /// SDK package `0.3.317` with native `2.1.317`.
    V0_3_317Native2_1_317,
}

impl ClaudeAgentSdkPackageNativePair {
    /// The unchanged default selected by [`ClaudeAgentSdkSessionPreparation`](crate::sdk::ClaudeAgentSdkSessionPreparation).
    pub const DEFAULT: Self = Self::V0_3_293Native2_1_293;

    /// Every maintained coupled pair, ordered by publication. The
    /// unpublished `0.3.297`, `0.3.298` and `0.3.313` points are absent.
    pub const MAINTAINED: [Self; 31] = [
        Self::V0_3_284Native2_1_284,
        Self::V0_3_285Native2_1_285,
        Self::V0_3_286Native2_1_286,
        Self::V0_3_287Native2_1_287,
        Self::V0_3_288Native2_1_288,
        Self::V0_3_289Native2_1_289,
        Self::V0_3_290Native2_1_290,
        Self::V0_3_291Native2_1_291,
        Self::V0_3_292Native2_1_292,
        Self::V0_3_293Native2_1_293,
        Self::V0_3_294Native2_1_294,
        Self::V0_3_295Native2_1_295,
        Self::V0_3_296Native2_1_296,
        Self::V0_3_299Native2_1_299,
        Self::V0_3_300Native2_1_300,
        Self::V0_3_301Native2_1_301,
        Self::V0_3_302Native2_1_302,
        Self::V0_3_303Native2_1_303,
        Self::V0_3_304Native2_1_304,
        Self::V0_3_305Native2_1_305,
        Self::V0_3_306Native2_1_306,
        Self::V0_3_307Native2_1_307,
        Self::V0_3_308Native2_1_308,
        Self::V0_3_309Native2_1_309,
        Self::V0_3_310Native2_1_310,
        Self::V0_3_311Native2_1_311,
        Self::V0_3_312Native2_1_312,
        Self::V0_3_314Native2_1_314,
        Self::V0_3_315Native2_1_315,
        Self::V0_3_316Native2_1_316,
        Self::V0_3_317Native2_1_317,
    ];

    /// Returns the exact SDK wrapper package version.
    #[must_use]
    pub const fn package_version(self) -> &'static str {
        match self {
            Self::V0_3_284Native2_1_284 => "0.3.284",
            Self::V0_3_285Native2_1_285 => "0.3.285",
            Self::V0_3_286Native2_1_286 => "0.3.286",
            Self::V0_3_287Native2_1_287 => "0.3.287",
            Self::V0_3_288Native2_1_288 => "0.3.288",
            Self::V0_3_289Native2_1_289 => "0.3.289",
            Self::V0_3_290Native2_1_290 => "0.3.290",
            Self::V0_3_291Native2_1_291 => "0.3.291",
            Self::V0_3_292Native2_1_292 => "0.3.292",
            Self::V0_3_293Native2_1_293 => "0.3.293",
            Self::V0_3_294Native2_1_294 => "0.3.294",
            Self::V0_3_295Native2_1_295 => "0.3.295",
            Self::V0_3_296Native2_1_296 => "0.3.296",
            Self::V0_3_299Native2_1_299 => "0.3.299",
            Self::V0_3_300Native2_1_300 => "0.3.300",
            Self::V0_3_301Native2_1_301 => "0.3.301",
            Self::V0_3_302Native2_1_302 => "0.3.302",
            Self::V0_3_303Native2_1_303 => "0.3.303",
            Self::V0_3_304Native2_1_304 => "0.3.304",
            Self::V0_3_305Native2_1_305 => "0.3.305",
            Self::V0_3_306Native2_1_306 => "0.3.306",
            Self::V0_3_307Native2_1_307 => "0.3.307",
            Self::V0_3_308Native2_1_308 => "0.3.308",
            Self::V0_3_309Native2_1_309 => "0.3.309",
            Self::V0_3_310Native2_1_310 => "0.3.310",
            Self::V0_3_311Native2_1_311 => "0.3.311",
            Self::V0_3_312Native2_1_312 => "0.3.312",
            Self::V0_3_314Native2_1_314 => "0.3.314",
            Self::V0_3_315Native2_1_315 => "0.3.315",
            Self::V0_3_316Native2_1_316 => "0.3.316",
            Self::V0_3_317Native2_1_317 => "0.3.317",
        }
    }

    /// Returns the exact native version embedded by the SDK wrapper.
    #[must_use]
    pub const fn native_version(self) -> &'static str {
        match self {
            Self::V0_3_284Native2_1_284 => "2.1.284",
            Self::V0_3_285Native2_1_285 => "2.1.285",
            Self::V0_3_286Native2_1_286 => "2.1.286",
            Self::V0_3_287Native2_1_287 => "2.1.287",
            Self::V0_3_288Native2_1_288 => "2.1.288",
            Self::V0_3_289Native2_1_289 => "2.1.289",
            Self::V0_3_290Native2_1_290 => "2.1.290",
            Self::V0_3_291Native2_1_291 => "2.1.291",
            Self::V0_3_292Native2_1_292 => "2.1.292",
            Self::V0_3_293Native2_1_293 => "2.1.293",
            Self::V0_3_294Native2_1_294 => "2.1.294",
            Self::V0_3_295Native2_1_295 => "2.1.295",
            Self::V0_3_296Native2_1_296 => "2.1.296",
            Self::V0_3_299Native2_1_299 => "2.1.299",
            Self::V0_3_300Native2_1_300 => "2.1.300",
            Self::V0_3_301Native2_1_301 => "2.1.301",
            Self::V0_3_302Native2_1_302 => "2.1.302",
            Self::V0_3_303Native2_1_303 => "2.1.303",
            Self::V0_3_304Native2_1_304 => "2.1.304",
            Self::V0_3_305Native2_1_305 => "2.1.305",
            Self::V0_3_306Native2_1_306 => "2.1.306",
            Self::V0_3_307Native2_1_307 => "2.1.307",
            Self::V0_3_308Native2_1_308 => "2.1.308",
            Self::V0_3_309Native2_1_309 => "2.1.309",
            Self::V0_3_310Native2_1_310 => "2.1.310",
            Self::V0_3_311Native2_1_311 => "2.1.311",
            Self::V0_3_312Native2_1_312 => "2.1.312",
            Self::V0_3_314Native2_1_314 => "2.1.314",
            Self::V0_3_315Native2_1_315 => "2.1.315",
            Self::V0_3_316Native2_1_316 => "2.1.316",
            Self::V0_3_317Native2_1_317 => "2.1.317",
        }
    }
}

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
        InterfaceVersion::new(CLAUDE_AGENT_SDK_BASELINE_VERSION)
            .expect("static SDK baseline is valid"),
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
        InterfaceVersion::new(CLAUDE_AGENT_SDK_NATIVE_BASELINE_VERSION)
            .expect("static SDK baseline is valid"),
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

pub(crate) fn preparation_pair_is_qualified(pair: ClaudeAgentSdkPackageNativePair) -> bool {
    if !ClaudeAgentSdkPackageNativePair::MAINTAINED.contains(&pair) {
        return false;
    }
    let Some(package) = InterfaceVersion::new(pair.package_version()).ok() else {
        return false;
    };
    let Some(native) = InterfaceVersion::new(pair.native_version()).ok() else {
        return false;
    };
    claude_agent_sdk_package_claim()
        .assess(&package)
        .is_permitted()
        && claude_agent_sdk_native_claim()
            .assess(&native)
            .is_permitted()
        && package_native_pair_matches(pair.package_version(), pair.native_version())
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
    baseline: InterfaceVersion,
    current: InterfaceVersion,
) -> InterfaceCompatibilityClaim {
    window_claim_versions(
        id,
        axis,
        InterfaceVersionScheme::Semantic,
        baseline,
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

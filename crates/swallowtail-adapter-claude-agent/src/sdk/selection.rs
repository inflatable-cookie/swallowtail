//! Interface-version selection for the Claude Agent SDK sidecar route.
//!
//! Five axes bind the SDK wrapper, the native binary declared by its shipped
//! manifest, the approved Node runtime, the private sidecar wire, and the
//! source-tagged sidecar revision. The package/native axes retain their
//! baselines and claim IDs while extending across every published hop through
//! `0.3.322`; the unpublished `0.3.297`, `0.3.298`, `0.3.313`, and `0.3.320` points (and
//! their native mirrors) are explicit claim exclusions, so they assess
//! incompatible on the claims themselves. The independent Node axis is
//! maintained through `22.23.3` (Research 387). Wire and sidecar source stay
//! exact. Package/native versions are checked together; no Claude Code or ACP
//! qualification transfers here.

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
/// while this closed set ensures that only an artifact-backed coupled pair can
/// be selected for preparation. A pair is one row of the static
/// [`MAINTAINED_PAIRS`](ClaudeAgentSdkPackageNativePair::MAINTAINED) table, so
/// extending the window is one table row rather than one variant plus two
/// thirty-five-arm `match` blocks.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ClaudeAgentSdkPackageNativePair(u8);

/// One maintained row: the wrapper package and the native binary it embeds.
struct PairVersions {
    package: &'static str,
    native: &'static str,
}

/// Every artifact-backed coupled pair, ordered by publication.
///
/// The unpublished `0.3.297`, `0.3.298`, `0.3.313`, and `0.3.320` points are
/// absent, so they are not maintained pairs even though their patch numbers
/// would satisfy the coupling predicate.
const MAINTAINED_PAIRS: [PairVersions; 35] = [
    PairVersions {
        package: "0.3.284",
        native: "2.1.284",
    },
    PairVersions {
        package: "0.3.285",
        native: "2.1.285",
    },
    PairVersions {
        package: "0.3.286",
        native: "2.1.286",
    },
    PairVersions {
        package: "0.3.287",
        native: "2.1.287",
    },
    PairVersions {
        package: "0.3.288",
        native: "2.1.288",
    },
    PairVersions {
        package: "0.3.289",
        native: "2.1.289",
    },
    PairVersions {
        package: "0.3.290",
        native: "2.1.290",
    },
    PairVersions {
        package: "0.3.291",
        native: "2.1.291",
    },
    PairVersions {
        package: "0.3.292",
        native: "2.1.292",
    },
    PairVersions {
        package: "0.3.293",
        native: "2.1.293",
    },
    PairVersions {
        package: "0.3.294",
        native: "2.1.294",
    },
    PairVersions {
        package: "0.3.295",
        native: "2.1.295",
    },
    PairVersions {
        package: "0.3.296",
        native: "2.1.296",
    },
    PairVersions {
        package: "0.3.299",
        native: "2.1.299",
    },
    PairVersions {
        package: "0.3.300",
        native: "2.1.300",
    },
    PairVersions {
        package: "0.3.301",
        native: "2.1.301",
    },
    PairVersions {
        package: "0.3.302",
        native: "2.1.302",
    },
    PairVersions {
        package: "0.3.303",
        native: "2.1.303",
    },
    PairVersions {
        package: "0.3.304",
        native: "2.1.304",
    },
    PairVersions {
        package: "0.3.305",
        native: "2.1.305",
    },
    PairVersions {
        package: "0.3.306",
        native: "2.1.306",
    },
    PairVersions {
        package: "0.3.307",
        native: "2.1.307",
    },
    PairVersions {
        package: "0.3.308",
        native: "2.1.308",
    },
    PairVersions {
        package: "0.3.309",
        native: "2.1.309",
    },
    PairVersions {
        package: "0.3.310",
        native: "2.1.310",
    },
    PairVersions {
        package: "0.3.311",
        native: "2.1.311",
    },
    PairVersions {
        package: "0.3.312",
        native: "2.1.312",
    },
    PairVersions {
        package: "0.3.314",
        native: "2.1.314",
    },
    PairVersions {
        package: "0.3.315",
        native: "2.1.315",
    },
    PairVersions {
        package: "0.3.316",
        native: "2.1.316",
    },
    PairVersions {
        package: "0.3.317",
        native: "2.1.317",
    },
    PairVersions {
        package: "0.3.318",
        native: "2.1.318",
    },
    PairVersions {
        package: "0.3.319",
        native: "2.1.319",
    },
    PairVersions {
        package: "0.3.321",
        native: "2.1.321",
    },
    PairVersions {
        package: "0.3.322",
        native: "2.1.322",
    },
];

impl ClaudeAgentSdkPackageNativePair {
    /// The unchanged default selected by [`ClaudeAgentSdkSessionPreparation`](crate::sdk::ClaudeAgentSdkSessionPreparation).
    pub const DEFAULT: Self = Self(9);

    /// Every maintained coupled pair, ordered by publication. The
    /// unpublished `0.3.297`, `0.3.298` and `0.3.313` points are absent.
    pub const MAINTAINED: [Self; MAINTAINED_PAIRS.len()] = {
        let mut pairs = [Self(0); MAINTAINED_PAIRS.len()];
        let mut index = 0;
        while index < MAINTAINED_PAIRS.len() {
            pairs[index] = Self(index as u8);
            index += 1;
        }
        pairs
    };

    /// Returns the maintained pair that carries `package`, when one exists.
    #[must_use]
    pub fn from_package_version(package: &str) -> Option<Self> {
        Self::MAINTAINED
            .into_iter()
            .find(|pair| pair.package_version() == package)
    }

    /// Returns the maintained pair that carries both exact versions, when one
    /// exists. An uncoupled or unknown pair is not maintained.
    #[must_use]
    pub fn from_versions(package: &str, native: &str) -> Option<Self> {
        Self::from_package_version(package).filter(|pair| pair.native_version() == native)
    }

    /// Returns the exact SDK wrapper package version.
    #[must_use]
    pub const fn package_version(self) -> &'static str {
        MAINTAINED_PAIRS[self.0 as usize].package
    }

    /// Returns the exact native version embedded by the SDK wrapper.
    #[must_use]
    pub const fn native_version(self) -> &'static str {
        MAINTAINED_PAIRS[self.0 as usize].native
    }
}

const CLAUDE_AGENT_SDK_NODE_BASELINE: &str = "22.23.2";

/// Unpublished stables inside the coupled package window (Research 472).
const SDK_PACKAGE_UNPUBLISHED_GAPS: &[&str] = &["0.3.297", "0.3.298", "0.3.313", "0.3.320"];
/// Unpublished stables inside the coupled native window (Research 472).
const SDK_NATIVE_UNPUBLISHED_GAPS: &[&str] = &["2.1.297", "2.1.298", "2.1.313", "2.1.320"];

/// Parses one static unpublished-gap version for claim construction. The
/// indirection keeps `InterfaceVersion::new` calls literal at their other
/// use sites.
fn sdk_gap_version(value: &str) -> Option<InterfaceVersion> {
    InterfaceVersion::new(value).ok()
}

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

/// Returns the qualified-only maintained SDK wrapper package segment. The
/// unpublished interior points are explicit claim exclusions, so they assess
/// incompatible even though the segment spans them arithmetically.
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
        InterfaceVersion::new("0.3.318").expect("static v1 ceiling is valid"),
        InterfaceVersion::new("0.3.319").expect("static v2 baseline is valid"),
        SDK_PACKAGE_UNPUBLISHED_GAPS
            .iter()
            .map(|gap| sdk_gap_version(gap).expect("static SDK gap is valid")),
    )
}

/// Returns the qualified-only maintained native binary segment. The
/// unpublished interior points are explicit claim exclusions, so they assess
/// incompatible even though the segment spans them arithmetically.
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
        InterfaceVersion::new("2.1.318").expect("static v1 ceiling is valid"),
        InterfaceVersion::new("2.1.319").expect("static v2 baseline is valid"),
        SDK_NATIVE_UNPUBLISHED_GAPS
            .iter()
            .map(|gap| sdk_gap_version(gap).expect("static SDK gap is valid")),
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
        || assessment.behavior_revision().is_none_or(|revision| {
            ![
                CLAUDE_AGENT_SDK_BEHAVIOR,
                super::CLAUDE_AGENT_SDK_MANAGED_DENIAL_BEHAVIOR,
            ]
            .contains(&revision.as_str())
        })
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
    v1_ceiling: InterfaceVersion,
    v2_baseline: InterfaceVersion,
    exclusions: impl IntoIterator<Item = InterfaceVersion>,
) -> InterfaceCompatibilityClaim {
    InterfaceCompatibilityClaim::new(
        InterfaceCompatibilityClaimId::new(id).expect("static SDK claim id is valid"),
        axis,
        InterfaceVersionScheme::Semantic,
        InterfaceNewerVersionPosture::QualifiedOnly,
        [
            swallowtail_core::InterfaceVersionSegment::new(
                baseline,
                v1_ceiling,
                InterfaceBehaviorRevision::new(CLAUDE_AGENT_SDK_BEHAVIOR)
                    .expect("static v1 revision"),
                InterfaceSupportStatus::Maintained,
            ),
            swallowtail_core::InterfaceVersionSegment::new(
                v2_baseline,
                current,
                InterfaceBehaviorRevision::new(super::CLAUDE_AGENT_SDK_MANAGED_DENIAL_BEHAVIOR)
                    .expect("static v2 revision"),
                InterfaceSupportStatus::Maintained,
            ),
        ],
        exclusions,
    )
    .expect("static coupled SDK claim is valid")
}

fn window_claim_versions(
    id: &str,
    axis: InterfaceVersionAxis,
    scheme: InterfaceVersionScheme,
    baseline: InterfaceVersion,
    current: InterfaceVersion,
) -> InterfaceCompatibilityClaim {
    window_claim_versions_with_exclusions(id, axis, scheme, baseline, current, [])
}

fn window_claim_versions_with_exclusions(
    id: &str,
    axis: InterfaceVersionAxis,
    scheme: InterfaceVersionScheme,
    baseline: InterfaceVersion,
    current: InterfaceVersion,
    exclusions: impl IntoIterator<Item = InterfaceVersion>,
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
        exclusions,
    )
    .expect("static SDK sidecar compatibility claim is valid")
}

#[cfg(test)]
mod selection_tests;

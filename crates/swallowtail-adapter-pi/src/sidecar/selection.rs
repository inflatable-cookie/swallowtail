//! Interface-version selection for the Pi SDK sidecar route.
//!
//! Four separate axes bind the exact SDK package, the exact approved Node
//! runtime, the private sidecar wire, and the source-tagged sidecar revision.
//! Claims are qualified-only. The SDK names each published stable point and
//! leaves unpublished or independently unqualified points rejected.

use super::{
    PI_SDK_SIDECAR_BEHAVIOR, PI_SDK_SIDECAR_NODE_RUNTIME, PI_SDK_SIDECAR_SOURCE_TAG,
    PI_SDK_SIDECAR_WIRE,
};
use crate::failure::failure;
use swallowtail_core::{
    InterfaceBehaviorRevision, InterfaceCompatibilityClaim, InterfaceCompatibilityClaimId,
    InterfaceNewerVersionPosture, InterfaceSupportStatus, InterfaceVersion, InterfaceVersionAxis,
    InterfaceVersionBinding, InterfaceVersionScheme, PreflightPlan,
};
use swallowtail_runtime::RuntimeFailure;

/// Semantic-version axis for the exact SDK package the sidecar loads.
pub const PI_SDK_SIDECAR_PACKAGE_AXIS: &str = "pi.sdk-sidecar.package";
/// Semantic-version axis for the exact approved Node runtime.
pub const PI_SDK_SIDECAR_NODE_AXIS: &str = "pi.sdk-sidecar.node";
/// Opaque axis for the private sidecar wire identity.
pub const PI_SDK_SIDECAR_WIRE_AXIS: &str = "pi.sdk-sidecar.wire";
/// Opaque axis for the source-tagged sidecar revision.
pub const PI_SDK_SIDECAR_SIDECAR_AXIS: &str = "pi.sdk-sidecar.sidecar";

const QUALIFIED_SDK_VERSIONS: [&str; 18] = [
    "0.84.2", "0.84.3", "0.84.4", "0.85.0", "0.85.1", "0.86.0", "0.86.1", "0.87.0", "0.87.1",
    "0.99.0", "0.99.1", "0.99.2", "1.0.0", "1.0.1", "1.0.2", "1.0.3", "1.0.4", "1.1.0",
];

/// Parses one exact sidecar SDK package semantic-version binding.
#[must_use]
pub fn pi_sdk_sidecar_package_binding(value: &str) -> Option<InterfaceVersionBinding> {
    swallowtail_runtime::parse_semantic_version_binding(
        &InterfaceVersionAxis::new(PI_SDK_SIDECAR_PACKAGE_AXIS)
            .expect("static sidecar axis is valid"),
        value,
    )
}

/// Parses one exact Node runtime semantic-version binding.
#[must_use]
pub fn pi_sdk_sidecar_node_binding(value: &str) -> Option<InterfaceVersionBinding> {
    swallowtail_runtime::parse_semantic_version_binding(
        &InterfaceVersionAxis::new(PI_SDK_SIDECAR_NODE_AXIS).expect("static sidecar axis is valid"),
        value,
    )
}

/// Binds the exact opaque sidecar wire identity.
#[must_use]
pub fn pi_sdk_sidecar_wire_binding(value: &str) -> Option<InterfaceVersionBinding> {
    if value != PI_SDK_SIDECAR_WIRE {
        return None;
    }
    Some(InterfaceVersionBinding::new(
        InterfaceVersionAxis::new(PI_SDK_SIDECAR_WIRE_AXIS).expect("static sidecar axis is valid"),
        InterfaceVersion::new(value).ok()?,
    ))
}

/// Binds the exact opaque sidecar source tag.
#[must_use]
pub fn pi_sdk_sidecar_sidecar_binding(value: &str) -> Option<InterfaceVersionBinding> {
    if value != PI_SDK_SIDECAR_SOURCE_TAG {
        return None;
    }
    Some(InterfaceVersionBinding::new(
        InterfaceVersionAxis::new(PI_SDK_SIDECAR_SIDECAR_AXIS)
            .expect("static sidecar axis is valid"),
        InterfaceVersion::new(value).ok()?,
    ))
}

/// Returns the qualified-only SDK package claim for the published exact points.
#[must_use]
pub fn pi_sdk_sidecar_package_claim() -> InterfaceCompatibilityClaim {
    claim_with_segments(
        "pi.sdk-sidecar.package-window-1",
        InterfaceVersionAxis::new(PI_SDK_SIDECAR_PACKAGE_AXIS)
            .expect("static sidecar axis is valid"),
        InterfaceVersionScheme::Semantic,
        QUALIFIED_SDK_VERSIONS
            .into_iter()
            .map(exact_package_segment),
    )
}

/// Returns the qualified-only Node runtime claim for the maintained segment.
#[must_use]
pub fn pi_sdk_sidecar_node_claim() -> InterfaceCompatibilityClaim {
    InterfaceCompatibilityClaim::new(
        InterfaceCompatibilityClaimId::new("pi.sdk-sidecar.node-window-1")
            .expect("static sidecar claim id is valid"),
        InterfaceVersionAxis::new(PI_SDK_SIDECAR_NODE_AXIS).expect("static sidecar axis is valid"),
        InterfaceVersionScheme::Semantic,
        InterfaceNewerVersionPosture::QualifiedOnly,
        [swallowtail_core::InterfaceVersionSegment::new(
            InterfaceVersion::new("22.23.2").expect("static sidecar version is valid"),
            InterfaceVersion::new(PI_SDK_SIDECAR_NODE_RUNTIME)
                .expect("static sidecar version is valid"),
            InterfaceBehaviorRevision::new(PI_SDK_SIDECAR_BEHAVIOR)
                .expect("static sidecar behavior revision is valid"),
            InterfaceSupportStatus::Maintained,
        )],
        [],
    )
    .expect("static sidecar compatibility claim is valid")
}

/// Returns the qualified-only one-point sidecar wire claim.
#[must_use]
pub fn pi_sdk_sidecar_wire_claim() -> InterfaceCompatibilityClaim {
    claim(
        "pi.sdk-sidecar.wire-v1",
        InterfaceVersionAxis::new(PI_SDK_SIDECAR_WIRE_AXIS).expect("static sidecar axis is valid"),
        InterfaceVersionScheme::Opaque,
        InterfaceVersion::new(PI_SDK_SIDECAR_WIRE).expect("static sidecar version is valid"),
    )
}

/// Returns the qualified-only one-point sidecar source-tag claim.
#[must_use]
pub fn pi_sdk_sidecar_sidecar_claim() -> InterfaceCompatibilityClaim {
    claim(
        "pi.sdk-sidecar.sidecar-v1",
        InterfaceVersionAxis::new(PI_SDK_SIDECAR_SIDECAR_AXIS)
            .expect("static sidecar axis is valid"),
        InterfaceVersionScheme::Opaque,
        InterfaceVersion::new(PI_SDK_SIDECAR_SOURCE_TAG)
            .expect("static sidecar source tag is valid"),
    )
}

pub(crate) fn validate_pi_sdk_sidecar_plan_versions(
    plan: &PreflightPlan,
) -> Result<(), RuntimeFailure> {
    for claim in [
        pi_sdk_sidecar_package_claim(),
        pi_sdk_sidecar_node_claim(),
        pi_sdk_sidecar_wire_claim(),
        pi_sdk_sidecar_sidecar_claim(),
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
            "swallowtail.pi.sdk-sidecar.version_missing",
            "Pi SDK sidecar plan is missing an exact bound interface version",
        )
    })?;
    if bindings.next().is_some() {
        return Err(failure(
            "swallowtail.pi.sdk-sidecar.version_ambiguous",
            "Pi SDK sidecar plan contains more than one version on one axis",
        ));
    }
    let assessment = claim.assess(binding.version());
    if assessment != plan.assess_interface_version(binding)
        || !assessment.is_permitted()
        || assessment
            .behavior_revision()
            .is_none_or(|revision| revision.as_str() != PI_SDK_SIDECAR_BEHAVIOR)
    {
        return Err(failure(
            "swallowtail.pi.sdk-sidecar.version_incompatible",
            "Pi SDK sidecar bound version is incompatible with this driver",
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
    claim_with_segments(id, axis, scheme, [exact_segment(version)])
}

fn exact_package_segment(value: &str) -> swallowtail_core::InterfaceVersionSegment {
    // Callers are limited to the compile-time qualified package-point list.
    let version = InterfaceVersion::new(value)
        .unwrap_or_else(|_| unreachable!("qualified SDK package points are valid versions"));
    exact_segment(version)
}

fn exact_segment(version: InterfaceVersion) -> swallowtail_core::InterfaceVersionSegment {
    swallowtail_core::InterfaceVersionSegment::exact(
        version,
        InterfaceBehaviorRevision::new(PI_SDK_SIDECAR_BEHAVIOR)
            .expect("static sidecar behavior revision is valid"),
        InterfaceSupportStatus::Maintained,
    )
}

fn claim_with_segments(
    id: &str,
    axis: InterfaceVersionAxis,
    scheme: InterfaceVersionScheme,
    segments: impl IntoIterator<Item = swallowtail_core::InterfaceVersionSegment>,
) -> InterfaceCompatibilityClaim {
    InterfaceCompatibilityClaim::new(
        InterfaceCompatibilityClaimId::new(id).expect("static sidecar claim id is valid"),
        axis,
        scheme,
        InterfaceNewerVersionPosture::QualifiedOnly,
        segments,
        [],
    )
    .expect("static sidecar compatibility claim is valid")
}

#[cfg(test)]
mod tests {
    use super::{
        PI_SDK_SIDECAR_NODE_AXIS, PI_SDK_SIDECAR_PACKAGE_AXIS, PI_SDK_SIDECAR_SIDECAR_AXIS,
        PI_SDK_SIDECAR_WIRE_AXIS, QUALIFIED_SDK_VERSIONS, pi_sdk_sidecar_node_binding,
        pi_sdk_sidecar_node_claim, pi_sdk_sidecar_package_binding, pi_sdk_sidecar_package_claim,
        pi_sdk_sidecar_sidecar_binding, pi_sdk_sidecar_sidecar_claim, pi_sdk_sidecar_wire_binding,
        pi_sdk_sidecar_wire_claim,
    };
    use crate::sidecar::{
        PI_SDK_SIDECAR_BEHAVIOR, PI_SDK_SIDECAR_NODE_RUNTIME, PI_SDK_SIDECAR_SDK_VERSION,
        PI_SDK_SIDECAR_SOURCE_TAG, PI_SDK_SIDECAR_WIRE,
    };
    use swallowtail_core::InterfaceVersion;

    #[test]
    fn package_claim_qualifies_published_exact_points_and_rejects_gaps() {
        let claim = pi_sdk_sidecar_package_claim();
        assert_eq!(claim.axis().as_str(), PI_SDK_SIDECAR_PACKAGE_AXIS);
        let qualified = version(PI_SDK_SIDECAR_SDK_VERSION);
        let assessment = claim.assess(&qualified);
        assert!(assessment.is_permitted());
        assert_eq!(
            assessment.behavior_revision().unwrap().as_str(),
            PI_SDK_SIDECAR_BEHAVIOR
        );
        for qualified in QUALIFIED_SDK_VERSIONS {
            let assessment = claim.assess(&version(qualified));
            assert!(assessment.is_permitted(), "published point {qualified}");
            assert_eq!(
                assessment.behavior_revision().unwrap().as_str(),
                PI_SDK_SIDECAR_BEHAVIOR
            );
        }
        for rejected in [
            "0.84.1",
            "0.84.5",
            "0.85.2",
            "0.86.2",
            "0.87.2",
            "0.88.0",
            "0.99.3",
            "1.0.5",
            "1.1.1",
            "0.84.2-rc.1",
            "0.80.10",
        ] {
            assert!(
                !claim.permits(&version(rejected)),
                "unqualified point {rejected} must be rejected"
            );
        }
        assert!(pi_sdk_sidecar_package_binding("0.84.2").is_some());
        for value in ["", " 0.84.2", "latest", "0.84.2 "] {
            assert!(pi_sdk_sidecar_package_binding(value).is_none());
        }
    }

    #[test]
    fn node_claim_retains_22_23_2_and_qualifies_22_23_3_only() {
        let claim = pi_sdk_sidecar_node_claim();
        assert_eq!(claim.axis().as_str(), PI_SDK_SIDECAR_NODE_AXIS);
        for qualified in ["22.23.2", PI_SDK_SIDECAR_NODE_RUNTIME] {
            let assessment = claim.assess(&version(qualified));
            assert!(assessment.is_permitted(), "qualified runtime {qualified}");
            assert_eq!(
                assessment.behavior_revision().unwrap().as_str(),
                PI_SDK_SIDECAR_BEHAVIOR
            );
        }
        for rejected in ["22.23.1", "22.23.2-rc.1", "22.23.4", "22.24.0", "23.0.0"] {
            assert!(
                !claim.permits(&version(rejected)),
                "unqualified runtime {rejected} must be rejected"
            );
        }
        assert!(pi_sdk_sidecar_node_binding("22.23.2").is_some());
        assert!(pi_sdk_sidecar_node_binding("22.23.3").is_some());
        assert!(pi_sdk_sidecar_node_binding("22.x").is_none());
    }

    #[test]
    fn opaque_claims_qualify_only_the_exact_wire_and_sidecar_points() {
        let wire = pi_sdk_sidecar_wire_claim();
        assert_eq!(wire.axis().as_str(), PI_SDK_SIDECAR_WIRE_AXIS);
        assert!(wire.permits(&version(PI_SDK_SIDECAR_WIRE)));
        assert!(!wire.permits(&version("swallowtail-pi-sdk-jsonl-v2")));
        assert!(pi_sdk_sidecar_wire_binding(PI_SDK_SIDECAR_WIRE).is_some());
        assert!(pi_sdk_sidecar_wire_binding("strict-lf-jsonl-stdio").is_none());

        let sidecar = pi_sdk_sidecar_sidecar_claim();
        assert_eq!(sidecar.axis().as_str(), PI_SDK_SIDECAR_SIDECAR_AXIS);
        assert!(sidecar.permits(&version(PI_SDK_SIDECAR_SOURCE_TAG)));
        assert!(!sidecar.permits(&version("swallowtail-pi-sdk-sidecar@0.0.0")));
        assert!(pi_sdk_sidecar_sidecar_binding(PI_SDK_SIDECAR_SOURCE_TAG).is_some());
        assert!(pi_sdk_sidecar_sidecar_binding("").is_none());
    }

    #[test]
    fn source_tag_remains_bound_to_the_adapter_release() {
        assert_eq!(
            PI_SDK_SIDECAR_SOURCE_TAG,
            format!("swallowtail-pi-sdk-sidecar@{}", env!("CARGO_PKG_VERSION"))
        );
    }

    fn version(value: &str) -> InterfaceVersion {
        InterfaceVersion::new(value).expect("fixture version is valid")
    }
}

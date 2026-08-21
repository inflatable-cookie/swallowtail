//! Interface-version selection for the Pi SDK sidecar route.
//!
//! Four separate axes bind the exact SDK package, the exact approved Node
//! runtime, the private sidecar wire, and the source-tagged sidecar revision.
//! Every claim is a qualified-only one-point segment; none inherits the RPC
//! package window or its unverified-newer posture.

use super::{
    PI_SDK_SIDECAR_BEHAVIOR, PI_SDK_SIDECAR_NODE_RUNTIME, PI_SDK_SIDECAR_SDK_VERSION,
    PI_SDK_SIDECAR_SOURCE_TAG, PI_SDK_SIDECAR_WIRE,
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

/// Returns the qualified-only one-point SDK package claim.
#[must_use]
pub fn pi_sdk_sidecar_package_claim() -> InterfaceCompatibilityClaim {
    claim(
        "pi.sdk-sidecar.package-window-1",
        InterfaceVersionAxis::new(PI_SDK_SIDECAR_PACKAGE_AXIS)
            .expect("static sidecar axis is valid"),
        InterfaceVersionScheme::Semantic,
        InterfaceVersion::new(PI_SDK_SIDECAR_SDK_VERSION).expect("static sidecar version is valid"),
    )
}

/// Returns the qualified-only one-point Node runtime claim.
#[must_use]
pub fn pi_sdk_sidecar_node_claim() -> InterfaceCompatibilityClaim {
    claim(
        "pi.sdk-sidecar.node-window-1",
        InterfaceVersionAxis::new(PI_SDK_SIDECAR_NODE_AXIS).expect("static sidecar axis is valid"),
        InterfaceVersionScheme::Semantic,
        InterfaceVersion::new(PI_SDK_SIDECAR_NODE_RUNTIME)
            .expect("static sidecar version is valid"),
    )
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
        InterfaceVersion::new(PI_SDK_SIDECAR_SOURCE_TAG).expect("static sidecar version is valid"),
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
    InterfaceCompatibilityClaim::new(
        InterfaceCompatibilityClaimId::new(id).expect("static sidecar claim id is valid"),
        axis,
        scheme,
        InterfaceNewerVersionPosture::QualifiedOnly,
        [swallowtail_core::InterfaceVersionSegment::exact(
            version,
            InterfaceBehaviorRevision::new(PI_SDK_SIDECAR_BEHAVIOR)
                .expect("static sidecar behavior revision is valid"),
            InterfaceSupportStatus::Maintained,
        )],
        [],
    )
    .expect("static sidecar compatibility claim is valid")
}

#[cfg(test)]
mod tests {
    use super::{
        PI_SDK_SIDECAR_NODE_AXIS, PI_SDK_SIDECAR_PACKAGE_AXIS, PI_SDK_SIDECAR_SIDECAR_AXIS,
        PI_SDK_SIDECAR_WIRE_AXIS, pi_sdk_sidecar_node_binding, pi_sdk_sidecar_node_claim,
        pi_sdk_sidecar_package_binding, pi_sdk_sidecar_package_claim,
        pi_sdk_sidecar_sidecar_binding, pi_sdk_sidecar_sidecar_claim, pi_sdk_sidecar_wire_binding,
        pi_sdk_sidecar_wire_claim,
    };
    use crate::sidecar::{
        PI_SDK_SIDECAR_BEHAVIOR, PI_SDK_SIDECAR_NODE_RUNTIME, PI_SDK_SIDECAR_SDK_VERSION,
        PI_SDK_SIDECAR_SOURCE_TAG, PI_SDK_SIDECAR_WIRE,
    };
    use swallowtail_core::InterfaceVersion;

    #[test]
    fn package_claim_qualifies_only_the_exact_sdk_point() {
        let claim = pi_sdk_sidecar_package_claim();
        assert_eq!(claim.axis().as_str(), PI_SDK_SIDECAR_PACKAGE_AXIS);
        let qualified = version(PI_SDK_SIDECAR_SDK_VERSION);
        let assessment = claim.assess(&qualified);
        assert!(assessment.is_permitted());
        assert_eq!(
            assessment.behavior_revision().unwrap().as_str(),
            PI_SDK_SIDECAR_BEHAVIOR
        );
        for rejected in ["0.84.1", "0.84.3", "0.84.2-rc.1", "0.80.10"] {
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
    fn node_claim_qualifies_only_the_exact_runtime_point() {
        let claim = pi_sdk_sidecar_node_claim();
        assert_eq!(claim.axis().as_str(), PI_SDK_SIDECAR_NODE_AXIS);
        assert!(claim.permits(&version(PI_SDK_SIDECAR_NODE_RUNTIME)));
        for rejected in ["22.23.1", "22.23.3", "22.23.2-rc.1", "23.0.0"] {
            assert!(
                !claim.permits(&version(rejected)),
                "unqualified runtime {rejected} must be rejected"
            );
        }
        assert!(pi_sdk_sidecar_node_binding("22.23.2").is_some());
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

    fn version(value: &str) -> InterfaceVersion {
        InterfaceVersion::new(value).expect("fixture version is valid")
    }
}

use swallowtail_core::{
    InterfaceBehaviorRevision, InterfaceCompatibilityClaim, InterfaceCompatibilityClaimId,
    InterfaceNewerVersionPosture, InterfaceSupportStatus, InterfaceVersion, InterfaceVersionAxis,
    InterfaceVersionBinding, InterfaceVersionScheme, InterfaceVersionSegment, PreflightPlan,
};
use swallowtail_runtime::RuntimeFailure;

use crate::failure::failure;

pub const PI_PACKAGE_AXIS: &str = "pi.package";
pub const PI_PACKAGE_BASELINE_VERSION: &str = "0.80.10";
pub const PI_PACKAGE_LATEST_QUALIFIED_VERSION: &str = "0.80.10";

const RPC_BEHAVIOR: &str = "pi.rpc.strict-lf-v1";
const MAX_VERSION_BYTES: usize = 64;

#[must_use]
pub fn pi_package_binding(value: &str) -> Option<InterfaceVersionBinding> {
    if value.is_empty()
        || value.len() > MAX_VERSION_BYTES
        || value.trim() != value
        || value.chars().any(char::is_control)
        || semver::Version::parse(value).is_err()
    {
        return None;
    }
    Some(InterfaceVersionBinding::new(
        axis(),
        InterfaceVersion::new(value).ok()?,
    ))
}

#[must_use]
pub fn pi_rpc_claim() -> InterfaceCompatibilityClaim {
    InterfaceCompatibilityClaim::new(
        InterfaceCompatibilityClaimId::new("pi.rpc.package-window-1")
            .expect("static Pi claim id is valid"),
        axis(),
        InterfaceVersionScheme::Semantic,
        InterfaceNewerVersionPosture::AllowUnverified,
        [InterfaceVersionSegment::exact(
            version(PI_PACKAGE_LATEST_QUALIFIED_VERSION),
            InterfaceBehaviorRevision::new(RPC_BEHAVIOR)
                .expect("static Pi behavior revision is valid"),
            InterfaceSupportStatus::Maintained,
        )],
        [],
    )
    .expect("static Pi compatibility claim is valid")
}

pub(crate) fn validate_pi_plan_version(plan: &PreflightPlan) -> Result<(), RuntimeFailure> {
    let claim = pi_rpc_claim();
    let mut bindings = plan
        .interface_versions()
        .filter(|binding| binding.axis() == claim.axis());
    let binding = bindings.next().ok_or_else(|| {
        failure(
            "swallowtail.pi.rpc.version_missing",
            "Pi RPC plan is missing its exact executable version",
        )
    })?;
    if bindings.next().is_some() {
        return Err(failure(
            "swallowtail.pi.rpc.version_ambiguous",
            "Pi RPC plan contains more than one executable version",
        ));
    }
    let assessment = claim.assess(binding.version());
    if assessment != plan.assess_interface_version(binding)
        || !assessment.is_permitted()
        || assessment
            .behavior_revision()
            .is_none_or(|revision| revision.as_str() != RPC_BEHAVIOR)
    {
        return Err(failure(
            "swallowtail.pi.rpc.version_incompatible",
            "Pi RPC executable version is incompatible with this driver",
        ));
    }
    Ok(())
}

fn axis() -> InterfaceVersionAxis {
    InterfaceVersionAxis::new(PI_PACKAGE_AXIS).expect("static Pi axis is valid")
}

fn version(value: &str) -> InterfaceVersion {
    InterfaceVersion::new(value).expect("static Pi version is valid")
}

#[cfg(test)]
mod tests {
    use super::{pi_package_binding, pi_rpc_claim};
    use swallowtail_core::{InterfaceCompatibilityAssessment, InterfaceVersion};

    #[test]
    fn claim_keeps_newer_stable_versions_executable_but_unverified() {
        let claim = pi_rpc_claim();
        assert!(claim.supports(&version("0.80.10")));
        let InterfaceCompatibilityAssessment::UnverifiedNewer(newer) =
            claim.assess(&version("0.81.1"))
        else {
            panic!("later stable Pi remains unverified");
        };
        assert_eq!(newer.behavior_revision().as_str(), "pi.rpc.strict-lf-v1");
        assert!(!claim.permits(&version("0.81.1-rc.1")));
    }

    #[test]
    fn binding_accepts_only_one_bare_semver() {
        assert!(pi_package_binding("0.80.10").is_some());
        for value in ["", " 0.80.10", "pi 0.80.10", "latest"] {
            assert!(pi_package_binding(value).is_none());
        }
    }

    fn version(value: &str) -> InterfaceVersion {
        InterfaceVersion::new(value).expect("fixture version is valid")
    }
}

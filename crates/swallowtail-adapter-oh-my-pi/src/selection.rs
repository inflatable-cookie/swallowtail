use swallowtail_core::{
    InterfaceBehaviorRevision, InterfaceCompatibilityClaim, InterfaceCompatibilityClaimId,
    InterfaceNewerVersionPosture, InterfaceSupportStatus, InterfaceVersion, InterfaceVersionAxis,
    InterfaceVersionBinding, InterfaceVersionScheme, InterfaceVersionSegment, PreflightPlan,
};
use swallowtail_runtime::RuntimeFailure;

use crate::failure::failure;

/// Semantic-version axis reported by the installed Oh My Pi package.
pub const OH_MY_PI_PACKAGE_AXIS: &str = "oh-my-pi.package";
/// Oldest Oh My Pi package version qualified for RPC v2.
pub const OH_MY_PI_PACKAGE_BASELINE_VERSION: &str = "17.2.9";
/// Newest Oh My Pi package version behaviorally qualified for RPC v2.
pub const OH_MY_PI_PACKAGE_LATEST_QUALIFIED_VERSION: &str = "17.2.9";

const BASELINE_BEHAVIOR: &str = "oh-my-pi.rpc-v2-v17.2.9";
const MAX_VERSION_BYTES: usize = 64;

/// Parses one exact Oh My Pi package semantic-version binding.
#[must_use]
pub fn oh_my_pi_package_binding(value: &str) -> Option<InterfaceVersionBinding> {
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

/// Returns the qualified Oh My Pi RPC package compatibility window.
#[must_use]
pub fn oh_my_pi_rpc_claim() -> InterfaceCompatibilityClaim {
    InterfaceCompatibilityClaim::new(
        InterfaceCompatibilityClaimId::new("oh-my-pi.rpc.package-window-1")
            .expect("static OhMyPi claim id is valid"),
        axis(),
        InterfaceVersionScheme::Semantic,
        InterfaceNewerVersionPosture::AllowUnverified,
        [segment("17.2.9", "17.2.9", BASELINE_BEHAVIOR)],
        [],
    )
    .expect("static OhMyPi compatibility claim is valid")
}

pub(crate) fn validate_oh_my_pi_plan_version(plan: &PreflightPlan) -> Result<(), RuntimeFailure> {
    let claim = oh_my_pi_rpc_claim();
    let mut bindings = plan
        .interface_versions()
        .filter(|binding| binding.axis() == claim.axis());
    let binding = bindings.next().ok_or_else(|| {
        failure(
            "swallowtail.oh_my_pi.rpc.version_missing",
            "OhMyPi RPC plan is missing its exact executable version",
        )
    })?;
    if bindings.next().is_some() {
        return Err(failure(
            "swallowtail.oh_my_pi.rpc.version_ambiguous",
            "OhMyPi RPC plan contains more than one executable version",
        ));
    }
    let assessment = claim.assess(binding.version());
    if assessment != plan.assess_interface_version(binding)
        || !assessment.is_permitted()
        || assessment
            .behavior_revision()
            .is_none_or(|revision| revision.as_str() != BASELINE_BEHAVIOR)
    {
        return Err(failure(
            "swallowtail.oh_my_pi.rpc.version_incompatible",
            "OhMyPi RPC executable version is incompatible with this driver",
        ));
    }
    Ok(())
}

fn axis() -> InterfaceVersionAxis {
    InterfaceVersionAxis::new(OH_MY_PI_PACKAGE_AXIS).expect("static OhMyPi axis is valid")
}

fn segment(start: &str, end: &str, behavior: &str) -> InterfaceVersionSegment {
    InterfaceVersionSegment::new(
        version(start),
        version(end),
        InterfaceBehaviorRevision::new(behavior).expect("static OhMyPi behavior revision is valid"),
        InterfaceSupportStatus::Maintained,
    )
}

fn version(value: &str) -> InterfaceVersion {
    InterfaceVersion::new(value).expect("static OhMyPi version is valid")
}

#[cfg(test)]
mod tests {
    use super::{oh_my_pi_package_binding, oh_my_pi_rpc_claim};
    use swallowtail_core::{InterfaceCompatibilityAssessment, InterfaceVersion};

    #[test]
    fn claim_qualifies_exact_milestones_and_keeps_later_stable_unverified() {
        let claim = oh_my_pi_rpc_claim();
        assert!(claim.supports(&version("17.2.9")));
        assert!(!claim.permits(&version("17.2.8")));
        assert_eq!(
            claim
                .assess(&version("17.2.9"))
                .behavior_revision()
                .unwrap()
                .as_str(),
            "oh-my-pi.rpc-v2-v17.2.9"
        );
        let InterfaceCompatibilityAssessment::UnverifiedNewer(newer) =
            claim.assess(&version("17.3.0"))
        else {
            panic!("later stable OhMyPi remains unverified");
        };
        assert_eq!(
            newer.behavior_revision().as_str(),
            "oh-my-pi.rpc-v2-v17.2.9"
        );
        assert!(!claim.permits(&version("17.3.0-rc.1")));
    }

    #[test]
    fn binding_accepts_only_one_bare_semver() {
        assert!(oh_my_pi_package_binding("17.2.9").is_some());
        for value in ["", " 17.2.9", "omp 17.2.9", "latest"] {
            assert!(oh_my_pi_package_binding(value).is_none());
        }
    }

    fn version(value: &str) -> InterfaceVersion {
        InterfaceVersion::new(value).expect("fixture version is valid")
    }
}

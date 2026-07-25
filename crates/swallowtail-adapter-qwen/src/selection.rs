use swallowtail_core::{
    InterfaceBehaviorRevision, InterfaceCompatibilityClaim, InterfaceCompatibilityClaimId,
    InterfaceNewerVersionPosture, InterfaceSupportStatus, InterfaceVersion, InterfaceVersionAxis,
    InterfaceVersionBinding, InterfaceVersionScheme, InterfaceVersionSegment, PreflightPlan,
};
use swallowtail_runtime::RuntimeFailure;

use crate::validation::failure;

pub const QWEN_CODE_AXIS: &str = "qwen-code.package";
pub const QWEN_CODE_BASELINE_VERSION: &str = "0.19.11";
pub const QWEN_CODE_LATEST_QUALIFIED_VERSION: &str = "0.19.11";

const HEADLESS_BEHAVIOR: &str = "qwen-code.headless.v0.19.11";
const MAX_VERSION_BYTES: usize = 64;

#[must_use]
pub fn qwen_code_binding(value: &str) -> Option<InterfaceVersionBinding> {
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
pub fn qwen_headless_claim() -> InterfaceCompatibilityClaim {
    InterfaceCompatibilityClaim::new(
        InterfaceCompatibilityClaimId::new("qwen-code.headless.package-window-1")
            .expect("static Qwen claim id is valid"),
        axis(),
        InterfaceVersionScheme::Semantic,
        InterfaceNewerVersionPosture::AllowUnverified,
        [InterfaceVersionSegment::exact(
            version(QWEN_CODE_LATEST_QUALIFIED_VERSION),
            InterfaceBehaviorRevision::new(HEADLESS_BEHAVIOR)
                .expect("static Qwen behavior revision is valid"),
            InterfaceSupportStatus::Maintained,
        )],
        [],
    )
    .expect("static Qwen compatibility claim is valid")
}

pub(crate) fn validate_qwen_plan_version(plan: &PreflightPlan) -> Result<(), RuntimeFailure> {
    let claim = qwen_headless_claim();
    let mut bindings = plan
        .interface_versions()
        .filter(|binding| binding.axis() == claim.axis());
    let binding = bindings.next().ok_or_else(|| {
        failure(
            "swallowtail.qwen.headless.version_missing",
            "Qwen headless plan is missing its exact executable version",
        )
    })?;
    if bindings.next().is_some() {
        return Err(failure(
            "swallowtail.qwen.headless.version_ambiguous",
            "Qwen headless plan contains more than one executable version",
        ));
    }
    let assessment = claim.assess(binding.version());
    if assessment != plan.assess_interface_version(binding)
        || !assessment.is_permitted()
        || assessment
            .behavior_revision()
            .is_none_or(|revision| revision.as_str() != HEADLESS_BEHAVIOR)
    {
        return Err(failure(
            "swallowtail.qwen.headless.version_incompatible",
            "Qwen headless executable version is incompatible with this driver",
        ));
    }
    Ok(())
}

fn axis() -> InterfaceVersionAxis {
    InterfaceVersionAxis::new(QWEN_CODE_AXIS).expect("static Qwen axis is valid")
}

fn version(value: &str) -> InterfaceVersion {
    InterfaceVersion::new(value).expect("static Qwen version is valid")
}

#[cfg(test)]
mod tests {
    use super::{qwen_code_binding, qwen_headless_claim};
    use swallowtail_core::{InterfaceCompatibilityAssessment, InterfaceVersion};

    #[test]
    fn claim_keeps_newer_stable_versions_executable_but_unverified() {
        let claim = qwen_headless_claim();
        assert!(claim.supports(&version("0.19.11")));
        let InterfaceCompatibilityAssessment::UnverifiedNewer(newer) =
            claim.assess(&version("0.20.1"))
        else {
            panic!("later stable Qwen remains unverified");
        };
        assert_eq!(
            newer.behavior_revision().as_str(),
            "qwen-code.headless.v0.19.11"
        );
        assert!(!claim.permits(&version("0.20.1-rc.1")));
    }

    #[test]
    fn binding_accepts_only_one_bare_semver() {
        assert!(qwen_code_binding("0.19.11").is_some());
        for value in ["", " 0.19.11", "qwen 0.19.11", "latest"] {
            assert!(qwen_code_binding(value).is_none());
        }
    }

    fn version(value: &str) -> InterfaceVersion {
        InterfaceVersion::new(value).expect("fixture version is valid")
    }
}

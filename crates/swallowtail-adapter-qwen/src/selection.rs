use swallowtail_core::{
    InterfaceBehaviorRevision, InterfaceCompatibilityClaim, InterfaceCompatibilityClaimId,
    InterfaceNewerVersionPosture, InterfaceSupportStatus, InterfaceVersion, InterfaceVersionAxis,
    InterfaceVersionBinding, InterfaceVersionScheme, InterfaceVersionSegment, PreflightPlan,
};
use swallowtail_runtime::RuntimeFailure;

use crate::validation::failure;

/// Package-version interface axis used by Qwen Code.
pub const QWEN_CODE_AXIS: &str = "qwen-code.package";
/// Oldest qualified Qwen Code package version.
pub const QWEN_CODE_BASELINE_VERSION: &str = "0.19.11";
/// Most recent qualified Qwen Code package version.
pub const QWEN_CODE_LATEST_QUALIFIED_VERSION: &str = "0.21.14";

const BASELINE_BEHAVIOR: &str = "qwen-code.headless.v0.19.11";
const CATALOGUE_FILTER_BEHAVIOR: &str = "qwen-code.headless.v0.21.0-catalogue-filter";
const MAX_VERSION_BYTES: usize = 64;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct QwenPlanSelection {
    version: InterfaceVersion,
}

impl QwenPlanSelection {
    pub(crate) const fn version(&self) -> &InterfaceVersion {
        &self.version
    }
}

#[must_use]
/// Parses a Qwen Code semantic version into its interface binding.
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
/// Returns the qualified compatibility claim for Qwen Code operations.
pub fn qwen_headless_claim() -> InterfaceCompatibilityClaim {
    InterfaceCompatibilityClaim::new(
        InterfaceCompatibilityClaimId::new("qwen-code.headless.package-window-2")
            .expect("static Qwen claim id is valid"),
        axis(),
        InterfaceVersionScheme::Semantic,
        InterfaceNewerVersionPosture::AllowUnverified,
        [
            InterfaceVersionSegment::new(
                version(QWEN_CODE_BASELINE_VERSION).expect("static Qwen version is valid"),
                version("0.20.1").expect("static Qwen version is valid"),
                InterfaceBehaviorRevision::new(BASELINE_BEHAVIOR)
                    .expect("static Qwen behavior revision is valid"),
                InterfaceSupportStatus::Deprecated,
            ),
            InterfaceVersionSegment::new(
                version("0.21.0").expect("static Qwen version is valid"),
                version(QWEN_CODE_LATEST_QUALIFIED_VERSION).expect("static Qwen version is valid"),
                InterfaceBehaviorRevision::new(CATALOGUE_FILTER_BEHAVIOR)
                    .expect("static Qwen behavior revision is valid"),
                InterfaceSupportStatus::Maintained,
            ),
        ],
        [],
    )
    .expect("static Qwen compatibility claim is valid")
}

pub(crate) fn validate_qwen_plan_version(
    plan: &PreflightPlan,
) -> Result<QwenPlanSelection, RuntimeFailure> {
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
        || assessment.behavior_revision().is_none_or(|revision| {
            !matches!(
                revision.as_str(),
                BASELINE_BEHAVIOR | CATALOGUE_FILTER_BEHAVIOR
            )
        })
    {
        return Err(failure(
            "swallowtail.qwen.headless.version_incompatible",
            "Qwen headless executable version is incompatible with this driver",
        ));
    }
    Ok(QwenPlanSelection {
        version: binding.version().clone(),
    })
}

fn axis() -> InterfaceVersionAxis {
    InterfaceVersionAxis::new(QWEN_CODE_AXIS).expect("static Qwen axis is valid")
}

fn version(value: &str) -> Option<InterfaceVersion> {
    InterfaceVersion::new(value).ok()
}

#[cfg(test)]
mod tests {
    use super::{qwen_code_binding, qwen_headless_claim};
    use swallowtail_core::{InterfaceCompatibilityAssessment, InterfaceVersion};

    #[test]
    fn claim_qualifies_both_segments_and_keeps_later_stable_unverified() {
        let claim = qwen_headless_claim();
        for candidate in [
            "0.19.11", "0.19.12", "0.20.0", "0.20.1", "0.21.0", "0.21.1", "0.21.2", "0.21.3",
            "0.21.13", "0.21.14",
        ] {
            assert!(claim.supports(&version(candidate)));
        }
        assert_eq!(
            claim
                .assess(&version("0.20.1"))
                .behavior_revision()
                .unwrap()
                .as_str(),
            "qwen-code.headless.v0.19.11"
        );
        assert_eq!(
            claim
                .assess(&version("0.21.0"))
                .behavior_revision()
                .unwrap()
                .as_str(),
            "qwen-code.headless.v0.21.0-catalogue-filter"
        );
        assert!(!claim.permits(&version("0.20.2")));
        let InterfaceCompatibilityAssessment::UnverifiedNewer(newer) =
            claim.assess(&version("0.21.15"))
        else {
            panic!("later stable Qwen remains unverified");
        };
        assert_eq!(
            newer.behavior_revision().as_str(),
            "qwen-code.headless.v0.21.0-catalogue-filter"
        );
        assert!(!claim.permits(&version("0.21.14-rc.1")));
        assert!(!claim.permits(&version("0.21.14-preview.0")));
    }

    #[test]
    fn binding_accepts_only_one_bare_semver() {
        assert!(qwen_code_binding("0.19.11").is_some());
        assert!(qwen_code_binding("0.21.2").is_some());
        assert!(qwen_code_binding("0.21.13").is_some());
        assert!(qwen_code_binding("0.21.14").is_some());
        for value in ["", " 0.19.11", "qwen 0.19.11", "latest"] {
            assert!(qwen_code_binding(value).is_none());
        }
    }

    fn version(value: &str) -> InterfaceVersion {
        InterfaceVersion::new(value).expect("fixture version is valid")
    }
}

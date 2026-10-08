use swallowtail_core::{
    InterfaceBehaviorRevision, InterfaceCompatibilityClaim, InterfaceCompatibilityClaimId,
    InterfaceNewerVersionPosture, InterfaceSupportStatus, InterfaceVersion, InterfaceVersionAxis,
    InterfaceVersionBinding, InterfaceVersionScheme, InterfaceVersionSegment, PreflightPlan,
};
use swallowtail_runtime::RuntimeFailure;

/// Unambiguous executable name used for Command Code discovery.
pub const COMMAND_CODE_EXECUTABLE_NAME: &str = "command-code";
/// Opaque npm version axis for Command Code releases.
pub const COMMAND_CODE_RELEASE_AXIS: &str = "command-code.npm";
/// Latest qualified Command Code npm release.
pub const COMMAND_CODE_RELEASE_VERSION: &str = "1.79.1";

pub(crate) const COMMAND_CODE_HEADLESS_BEHAVIOR: &str = "command-code.agent-event-ndjson-v1";
pub(crate) const COMMAND_CODE_HEADLESS_MODEL_SELECTION_BEHAVIOR: &str =
    "command-code.agent-event-ndjson-v1-model-selection-v2";

pub(crate) const QUALIFIED_COMMAND_CODE_RELEASES: &[&str] = &[
    "1.65.0", "1.65.1", "1.65.2", "1.65.3", "1.65.4", "1.65.5", "1.66.0", "1.67.0", "1.68.0",
    "1.69.0", "1.70.0", "1.71.0", "1.72.0", "1.72.1", "1.72.2", "1.72.3", "1.72.4", "1.73.0",
    "1.73.1", "1.73.2", "1.73.3", "1.73.4", "1.74.0", "1.74.1", "1.74.2", "1.74.3", "1.75.0",
    "1.75.1", "1.76.0", "1.77.0", "1.78.0", "1.79.0", "1.79.1",
];

/// Maximum accepted observed Command Code version text.
const MAX_VERSION_BYTES: usize = 32;

#[must_use]
/// Parses one qualified published Command Code npm release into its interface binding.
///
/// Returns `None` for anything other than the exact qualified release text, so
/// observed CLI output can never panic a caller.
pub fn command_code_release_binding(value: &str) -> Option<InterfaceVersionBinding> {
    if value.is_empty()
        || value.len() > MAX_VERSION_BYTES
        || value.trim() != value
        || value.chars().any(char::is_control)
        || semver::Version::parse(value).is_err()
        || !QUALIFIED_COMMAND_CODE_RELEASES.contains(&value)
    {
        return None;
    }
    Some(InterfaceVersionBinding::new(
        axis(),
        InterfaceVersion::new(value).ok()?,
    ))
}

#[must_use]
/// Returns the exact-published-point headless protocol claim.
pub fn command_code_headless_claim() -> InterfaceCompatibilityClaim {
    let first_model_lane = semver::Version::new(1, 73, 0);
    let segments = QUALIFIED_COMMAND_CODE_RELEASES.iter().map(|release| {
        let version =
            semver::Version::parse(release).expect("static Command Code release version is valid");
        let behavior = if version < first_model_lane {
            COMMAND_CODE_HEADLESS_BEHAVIOR
        } else {
            COMMAND_CODE_HEADLESS_MODEL_SELECTION_BEHAVIOR
        };
        InterfaceVersionSegment::exact(
            InterfaceVersion::new(*release).expect("static Command Code release is valid"),
            InterfaceBehaviorRevision::new(behavior)
                .expect("static Command Code behavior is valid"),
            InterfaceSupportStatus::Maintained,
        )
    });
    InterfaceCompatibilityClaim::new(
        InterfaceCompatibilityClaimId::new("command-code.headless-window-1")
            .expect("static Command Code claim id is valid"),
        axis(),
        InterfaceVersionScheme::Semantic,
        InterfaceNewerVersionPosture::QualifiedOnly,
        segments,
        [],
    )
    .expect("static Command Code claim is valid")
}

pub(crate) fn validate_plan(plan: &PreflightPlan) -> Result<(), RuntimeFailure> {
    let claim = command_code_headless_claim();
    let mut bindings = plan
        .interface_versions()
        .filter(|binding| binding.axis() == claim.axis());
    let binding = bindings.next().ok_or_else(|| {
        crate::failure::failure(
            "swallowtail.command_code.headless.version_missing",
            "Command Code plan is missing its exact qualified release version",
        )
    })?;
    if bindings.next().is_some() {
        return Err(crate::failure::failure(
            "swallowtail.command_code.headless.version_ambiguous",
            "Command Code plan contains more than one release version",
        ));
    }
    let assessment = claim.assess(binding.version());
    if assessment != plan.assess_interface_version(binding)
        || !assessment.is_permitted()
        || assessment.behavior_revision().is_none_or(|revision| {
            !matches!(
                revision.as_str(),
                COMMAND_CODE_HEADLESS_BEHAVIOR | COMMAND_CODE_HEADLESS_MODEL_SELECTION_BEHAVIOR
            )
        })
    {
        return Err(crate::failure::failure(
            "swallowtail.command_code.headless.version_incompatible",
            "Command Code release version is incompatible with the headless driver",
        ));
    }
    Ok(())
}

pub(crate) fn model_selection_observation_enabled(plan: &PreflightPlan) -> bool {
    let claim = command_code_headless_claim();
    plan.interface_versions()
        .filter(|binding| binding.axis() == claim.axis())
        .any(|binding| {
            claim
                .assess(binding.version())
                .behavior_revision()
                .is_some_and(|revision| {
                    revision.as_str() == COMMAND_CODE_HEADLESS_MODEL_SELECTION_BEHAVIOR
                })
        })
}

fn axis() -> InterfaceVersionAxis {
    InterfaceVersionAxis::new(COMMAND_CODE_RELEASE_AXIS)
        .expect("static Command Code release axis is valid")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_published_qualified_release_is_bound() {
        for release in QUALIFIED_COMMAND_CODE_RELEASES {
            assert!(command_code_release_binding(release).is_some(), "{release}");
        }
        for rejected in [
            "",
            "1.65.0.0",
            "1.65.6",
            "1.66.1",
            "1.54.0",
            "v1.65.0",
            "1.65.0-beta",
            "1.65.0\n",
            " 1.65.0",
            "1.65.0 ",
            "command-code 1.65.0",
        ] {
            assert!(
                command_code_release_binding(rejected).is_none(),
                "{rejected}"
            );
        }
    }

    #[test]
    fn claim_preserves_published_points_and_splits_the_model_lane_milestone() {
        let claim = command_code_headless_claim();
        assert_eq!(claim.id().as_str(), "command-code.headless-window-1");
        assert_eq!(claim.baseline().as_str(), "1.65.0");
        assert_eq!(claim.latest_qualified().as_str(), "1.79.1");
        assert_eq!(
            claim.milestones().len(),
            QUALIFIED_COMMAND_CODE_RELEASES.len()
        );
        for release in QUALIFIED_COMMAND_CODE_RELEASES {
            let version = InterfaceVersion::new(*release).unwrap();
            let segment = claim
                .milestones()
                .find(|segment| segment.minimum() == &version && segment.maximum() == &version)
                .expect("each published stable is its own exact segment");
            let expected =
                if semver::Version::parse(release).unwrap() < semver::Version::new(1, 73, 0) {
                    COMMAND_CODE_HEADLESS_BEHAVIOR
                } else {
                    COMMAND_CODE_HEADLESS_MODEL_SELECTION_BEHAVIOR
                };
            assert_eq!(segment.behavior_revision().as_str(), expected, "{release}");
        }
        assert!(!claim.permits(&InterfaceVersion::new("1.65.6").unwrap()));
        assert!(!claim.permits(&InterfaceVersion::new("1.66.1").unwrap()));
        assert!(!claim.permits(&InterfaceVersion::new("1.64.1").unwrap()));
        assert!(!claim.permits(&InterfaceVersion::new("1.65.0-rc.1").unwrap()));
        assert!(matches!(
            claim.assess(&InterfaceVersion::new("1.79.2").unwrap()),
            swallowtail_core::InterfaceCompatibilityAssessment::UnverifiedNewer(_)
        ));
    }
}

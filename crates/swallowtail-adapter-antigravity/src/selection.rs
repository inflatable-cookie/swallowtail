use swallowtail_core::{
    InterfaceBehaviorRevision, InterfaceCompatibilityClaim, InterfaceCompatibilityClaimId,
    InterfaceNewerVersionPosture, InterfaceSupportStatus, InterfaceVersion, InterfaceVersionAxis,
    InterfaceVersionBinding, InterfaceVersionScheme, InterfaceVersionSegment, PreflightPlan,
};
use swallowtail_runtime::RuntimeFailure;

use crate::failure::failure;

/// Default executable name for host-approved Antigravity discovery.
pub const ANTIGRAVITY_AUTOMATIC_EXECUTABLE_NAME: &str = "agy";
/// Semantic-version axis used for installed Antigravity releases.
pub const ANTIGRAVITY_RELEASE_AXIS: &str = "antigravity-cli.release";
/// Oldest release in the current exact qualification window.
pub const ANTIGRAVITY_BASELINE_VERSION: &str = "1.1.9";
/// Latest release in the current exact qualification window.
pub const ANTIGRAVITY_LATEST_QUALIFIED_VERSION: &str = "1.1.9";

pub(crate) const ANTIGRAVITY_CATALOGUE_BEHAVIOR: &str =
    "antigravity.catalogue.cli-1.1.8-artifact-1.1.9-v1";
pub(crate) const ANTIGRAVITY_HEADLESS_BEHAVIOR: &str =
    "antigravity.stream-json.cli-1.1.8-artifact-1.1.9-v1";
const MAX_VERSION_BYTES: usize = 64;

#[must_use]
/// Parses one stable installed release into its interface binding.
pub fn antigravity_release_binding(value: &str) -> Option<InterfaceVersionBinding> {
    if value.is_empty()
        || value.len() > MAX_VERSION_BYTES
        || value.trim() != value
        || value.chars().any(char::is_control)
    {
        return None;
    }
    let parsed = semver::Version::parse(value).ok()?;
    if !parsed.pre.is_empty() || !parsed.build.is_empty() {
        return None;
    }
    Some(InterfaceVersionBinding::new(
        axis(),
        InterfaceVersion::new(value).ok()?,
    ))
}

#[must_use]
/// Returns the catalogue release compatibility claim.
pub fn antigravity_catalogue_claim() -> InterfaceCompatibilityClaim {
    InterfaceCompatibilityClaim::new(
        InterfaceCompatibilityClaimId::new("antigravity.catalogue.release-window-1")
            .expect("static Antigravity claim id is valid"),
        axis(),
        InterfaceVersionScheme::Semantic,
        InterfaceNewerVersionPosture::AllowUnverified,
        [InterfaceVersionSegment::exact(
            version(ANTIGRAVITY_LATEST_QUALIFIED_VERSION),
            InterfaceBehaviorRevision::new(ANTIGRAVITY_CATALOGUE_BEHAVIOR)
                .expect("static Antigravity behavior is valid"),
            InterfaceSupportStatus::Maintained,
        )],
        [],
    )
    .expect("static Antigravity compatibility claim is valid")
}

#[must_use]
/// Returns the headless execution release compatibility claim.
pub fn antigravity_headless_claim() -> InterfaceCompatibilityClaim {
    InterfaceCompatibilityClaim::new(
        InterfaceCompatibilityClaimId::new("antigravity.headless.release-window-1")
            .expect("static Antigravity headless claim id is valid"),
        axis(),
        InterfaceVersionScheme::Semantic,
        InterfaceNewerVersionPosture::AllowUnverified,
        [InterfaceVersionSegment::exact(
            version(ANTIGRAVITY_LATEST_QUALIFIED_VERSION),
            InterfaceBehaviorRevision::new(ANTIGRAVITY_HEADLESS_BEHAVIOR)
                .expect("static Antigravity headless behavior is valid"),
            InterfaceSupportStatus::Maintained,
        )],
        [],
    )
    .expect("static Antigravity headless compatibility claim is valid")
}

pub(crate) fn validate_antigravity_catalogue_plan(
    plan: &PreflightPlan,
) -> Result<(), RuntimeFailure> {
    let claim = antigravity_catalogue_claim();
    let mut bindings = plan
        .interface_versions()
        .filter(|binding| binding.axis() == claim.axis());
    let binding = bindings.next().ok_or_else(|| {
        failure(
            "swallowtail.antigravity.catalogue.version_missing",
            "Antigravity catalogue plan is missing its exact release version",
        )
    })?;
    if bindings.next().is_some() {
        return Err(failure(
            "swallowtail.antigravity.catalogue.version_ambiguous",
            "Antigravity catalogue plan contains more than one release version",
        ));
    }
    let assessment = claim.assess(binding.version());
    if assessment != plan.assess_interface_version(binding)
        || !assessment.is_permitted()
        || assessment
            .behavior_revision()
            .is_none_or(|revision| revision.as_str() != ANTIGRAVITY_CATALOGUE_BEHAVIOR)
    {
        return Err(failure(
            "swallowtail.antigravity.catalogue.version_incompatible",
            "Antigravity release is incompatible with the catalogue driver",
        ));
    }
    Ok(())
}

pub(crate) fn validate_antigravity_headless_plan(
    plan: &PreflightPlan,
) -> Result<(), RuntimeFailure> {
    let claim = antigravity_headless_claim();
    let mut bindings = plan
        .interface_versions()
        .filter(|binding| binding.axis() == claim.axis());
    let binding = bindings.next().ok_or_else(|| {
        failure(
            "swallowtail.antigravity.headless.version_missing",
            "Antigravity headless plan is missing its exact release version",
        )
    })?;
    if bindings.next().is_some() {
        return Err(failure(
            "swallowtail.antigravity.headless.version_ambiguous",
            "Antigravity headless plan contains more than one release version",
        ));
    }
    let assessment = claim.assess(binding.version());
    if assessment != plan.assess_interface_version(binding)
        || !assessment.is_permitted()
        || assessment
            .behavior_revision()
            .is_none_or(|revision| revision.as_str() != ANTIGRAVITY_HEADLESS_BEHAVIOR)
    {
        return Err(failure(
            "swallowtail.antigravity.headless.version_incompatible",
            "Antigravity release is incompatible with the headless driver",
        ));
    }
    Ok(())
}

fn axis() -> InterfaceVersionAxis {
    InterfaceVersionAxis::new(ANTIGRAVITY_RELEASE_AXIS)
        .expect("static Antigravity release axis is valid")
}

fn version(value: &str) -> InterfaceVersion {
    InterfaceVersion::new(value).expect("static Antigravity release is valid")
}

#[cfg(test)]
mod tests {
    use super::{
        ANTIGRAVITY_CATALOGUE_BEHAVIOR, ANTIGRAVITY_RELEASE_AXIS, antigravity_catalogue_claim,
        antigravity_headless_claim, antigravity_release_binding,
    };
    use swallowtail_core::{InterfaceCompatibilityAssessment, InterfaceVersion};

    #[test]
    fn exact_installed_release_is_qualified_and_newer_is_visible() {
        let claim = antigravity_catalogue_claim();
        assert!(claim.supports(&version("1.1.9")));
        assert!(!claim.permits(&version("1.1.8")));
        let InterfaceCompatibilityAssessment::UnverifiedNewer(newer) =
            claim.assess(&version("1.1.10"))
        else {
            panic!("later Antigravity release remains visibly unverified");
        };
        assert_eq!(
            newer.behavior_revision().as_str(),
            ANTIGRAVITY_CATALOGUE_BEHAVIOR
        );
    }

    #[test]
    fn binding_accepts_only_bare_stable_semver() {
        assert_eq!(
            antigravity_release_binding("1.1.9")
                .expect("binding parses")
                .axis()
                .as_str(),
            ANTIGRAVITY_RELEASE_AXIS
        );
        for rejected in [
            "",
            " 1.1.9",
            "agy 1.1.9",
            "1.1.9 extra",
            "1.1.10-alpha.1",
            "1.1.9+build",
        ] {
            assert!(antigravity_release_binding(rejected).is_none());
        }
    }

    #[test]
    fn catalogue_and_headless_keep_distinct_behavior_claims() {
        let catalogue = antigravity_catalogue_claim();
        let headless = antigravity_headless_claim();
        assert_ne!(catalogue.id(), headless.id());
        assert_ne!(
            catalogue.assess(&version("1.1.9")).behavior_revision(),
            headless.assess(&version("1.1.9")).behavior_revision()
        );
    }

    fn version(value: &str) -> InterfaceVersion {
        InterfaceVersion::new(value).expect("fixture version is valid")
    }
}

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
/// Exact qualified Command Code npm release.
pub const COMMAND_CODE_RELEASE_VERSION: &str = "1.15.1";

pub(crate) const COMMAND_CODE_HEADLESS_BEHAVIOR: &str = "command-code.agent-event-ndjson-v1";

/// Maximum accepted observed Command Code version text.
const MAX_VERSION_BYTES: usize = 32;

#[must_use]
/// Parses the one qualified exact Command Code npm release into its interface binding.
///
/// Returns `None` for anything other than the exact qualified release text, so
/// observed CLI output can never panic a caller.
pub fn command_code_release_binding(value: &str) -> Option<InterfaceVersionBinding> {
    if value != COMMAND_CODE_RELEASE_VERSION
        || value.is_empty()
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
/// Returns the qualified-only exact headless protocol claim.
pub fn command_code_headless_claim() -> InterfaceCompatibilityClaim {
    InterfaceCompatibilityClaim::new(
        InterfaceCompatibilityClaimId::new("command-code.headless-window-1")
            .expect("static Command Code claim id is valid"),
        axis(),
        InterfaceVersionScheme::Semantic,
        InterfaceNewerVersionPosture::QualifiedOnly,
        [InterfaceVersionSegment::exact(
            InterfaceVersion::new(COMMAND_CODE_RELEASE_VERSION)
                .expect("static Command Code release version is valid"),
            InterfaceBehaviorRevision::new(COMMAND_CODE_HEADLESS_BEHAVIOR)
                .expect("static Command Code behavior is valid"),
            InterfaceSupportStatus::Maintained,
        )],
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
        || assessment
            .behavior_revision()
            .is_none_or(|revision| revision.as_str() != COMMAND_CODE_HEADLESS_BEHAVIOR)
    {
        return Err(crate::failure::failure(
            "swallowtail.command_code.headless.version_incompatible",
            "Command Code release version is incompatible with the headless driver",
        ));
    }
    Ok(())
}

fn axis() -> InterfaceVersionAxis {
    InterfaceVersionAxis::new(COMMAND_CODE_RELEASE_AXIS)
        .expect("static Command Code release axis is valid")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_the_exact_qualified_release_is_bound() {
        assert!(command_code_release_binding(COMMAND_CODE_RELEASE_VERSION).is_some());
        for rejected in [
            "",
            "1.15.0",
            "1.15.2",
            "1.15",
            "1.15.1.0",
            "v1.15.1",
            "1.15.1-beta",
            "1.15.1\n",
            " 1.15.1",
            "1.15.1 ",
            "command-code 1.15.1",
        ] {
            assert!(
                command_code_release_binding(rejected).is_none(),
                "{rejected}"
            );
        }
    }

    #[test]
    fn claim_qualifies_only_the_exact_release_and_rejects_newer() {
        let claim = command_code_headless_claim();
        assert!(claim.supports(&InterfaceVersion::new(COMMAND_CODE_RELEASE_VERSION).unwrap()));
        assert!(!claim.permits(&InterfaceVersion::new("1.15.2").unwrap()));
        assert!(!claim.permits(&InterfaceVersion::new("1.16.0").unwrap()));
        assert!(!claim.permits(&InterfaceVersion::new("1.15.0").unwrap()));
    }
}

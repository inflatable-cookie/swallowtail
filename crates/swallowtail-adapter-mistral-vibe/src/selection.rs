use swallowtail_core::{
    InterfaceBehaviorRevision, InterfaceCompatibilityClaim, InterfaceCompatibilityClaimId,
    InterfaceNewerVersionPosture, InterfaceSupportStatus, InterfaceVersion, InterfaceVersionAxis,
    InterfaceVersionBinding, InterfaceVersionScheme, InterfaceVersionSegment, PreflightPlan,
};
use swallowtail_runtime::RuntimeFailure;

use crate::failure::failure;

/// Unambiguous executable name used for Vibe discovery.
pub const MISTRAL_VIBE_EXECUTABLE_NAME: &str = "vibe";
/// Opaque GitHub-release axis for Mistral Vibe headless.
pub const MISTRAL_VIBE_RELEASE_AXIS: &str = "mistral-vibe.release";
/// Exact qualified Vibe CLI release used by headless.
pub const MISTRAL_VIBE_RELEASE_VERSION: &str = "2.24.2";

pub(crate) const MISTRAL_VIBE_HEADLESS_BEHAVIOR: &str = "mistral-vibe.headless.stdio-streaming-v1";
const MAX_VERSION_BYTES: usize = 32;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct VibePlanSelection {
    version: InterfaceVersion,
}

impl VibePlanSelection {
    #[allow(dead_code)]
    pub(crate) const fn version(&self) -> &InterfaceVersion {
        &self.version
    }
}

/// Parses installed `--version` stdout into the exact qualified Vibe binding.
#[must_use]
pub(crate) fn parse_vibe_version_output(output: &[u8]) -> Option<InterfaceVersionBinding> {
    let output = std::str::from_utf8(output).ok()?;
    let exact = output.strip_suffix('\n').unwrap_or(output);
    let exact = exact.strip_prefix("vibe ").unwrap_or(exact);
    mistral_vibe_release_binding(exact)
}

/// Parses the one qualified exact Vibe release version into its interface binding.
///
/// Returns `None` for anything other than the exact qualified release text, so
/// observed CLI output can never panic a caller.
#[must_use]
pub fn mistral_vibe_release_binding(value: &str) -> Option<InterfaceVersionBinding> {
    if value != MISTRAL_VIBE_RELEASE_VERSION
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

/// Returns the qualified-only exact Vibe headless protocol claim.
#[must_use]
pub fn mistral_vibe_headless_claim() -> InterfaceCompatibilityClaim {
    InterfaceCompatibilityClaim::new(
        InterfaceCompatibilityClaimId::new("mistral-vibe.headless.release-window-1")
            .expect("static Vibe claim id is valid"),
        axis(),
        InterfaceVersionScheme::Semantic,
        InterfaceNewerVersionPosture::QualifiedOnly,
        [InterfaceVersionSegment::exact(
            InterfaceVersion::new(MISTRAL_VIBE_RELEASE_VERSION)
                .expect("static Vibe version is valid"),
            InterfaceBehaviorRevision::new(MISTRAL_VIBE_HEADLESS_BEHAVIOR)
                .expect("static Vibe behavior is valid"),
            InterfaceSupportStatus::Maintained,
        )],
        [],
    )
    .expect("static Vibe claim is valid")
}

pub(crate) fn select_mistral_vibe_headless_plan(
    plan: &PreflightPlan,
) -> Result<VibePlanSelection, RuntimeFailure> {
    let claim = mistral_vibe_headless_claim();
    let mut bindings = plan
        .interface_versions()
        .filter(|binding| binding.axis() == claim.axis());
    let binding = bindings.next().ok_or_else(|| {
        failure(
            "swallowtail.mistral-vibe.headless.version_missing",
            "Mistral Vibe headless plan is missing its exact release version",
        )
    })?;
    if bindings.next().is_some() {
        return Err(failure(
            "swallowtail.mistral-vibe.headless.version_ambiguous",
            "Mistral Vibe headless plan contains more than one release version",
        ));
    }
    let assessment = claim.assess(binding.version());
    if assessment != plan.assess_interface_version(binding)
        || !assessment.is_permitted()
        || assessment
            .behavior_revision()
            .is_none_or(|revision| revision.as_str() != MISTRAL_VIBE_HEADLESS_BEHAVIOR)
    {
        return Err(failure(
            "swallowtail.mistral-vibe.headless.version_incompatible",
            "Mistral Vibe release version is incompatible with the headless driver",
        ));
    }
    Ok(VibePlanSelection {
        version: binding.version().clone(),
    })
}

fn axis() -> InterfaceVersionAxis {
    InterfaceVersionAxis::new(MISTRAL_VIBE_RELEASE_AXIS).expect("static Vibe release axis is valid")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_the_exact_qualified_release_is_bound() {
        assert!(mistral_vibe_release_binding(MISTRAL_VIBE_RELEASE_VERSION).is_some());
        for rejected in [
            "",
            "2.24.1",
            "2.24.3",
            "2.24",
            "2.24.2.0",
            "v2.24.2",
            "2.24.2-beta",
            "2.24.2\n",
            " 2.24.2",
            "2.24.2 ",
            "vibe 2.24.2",
        ] {
            assert!(
                mistral_vibe_release_binding(rejected).is_none(),
                "{rejected:?} must not bind"
            );
        }
    }

    #[test]
    fn exact_release_is_permitted_and_newer_is_not() {
        let permitted =
            InterfaceVersion::new(MISTRAL_VIBE_RELEASE_VERSION).expect("qualified version");
        let newer = InterfaceVersion::new("2.24.3").expect("newer version");
        let claim = mistral_vibe_headless_claim();
        assert!(claim.assess(&permitted).is_permitted());
        assert!(!claim.assess(&newer).is_permitted());
    }

    #[test]
    fn version_stdout_parser_accepts_bare_or_named_exact_release() {
        assert_eq!(
            parse_vibe_version_output(b"2.24.2\n")
                .expect("exact version parses")
                .version()
                .as_str(),
            "2.24.2"
        );
        assert_eq!(
            parse_vibe_version_output(b"vibe 2.24.2\n")
                .expect("named version parses")
                .version()
                .as_str(),
            "2.24.2"
        );
        assert!(parse_vibe_version_output(b"2.24.3\n").is_none());
        assert!(parse_vibe_version_output(b"v2.24.2\n").is_none());
        assert!(parse_vibe_version_output(b"vibe  2.24.2\n").is_none());
    }
}

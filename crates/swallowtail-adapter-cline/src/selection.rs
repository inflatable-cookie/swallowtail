use swallowtail_core::{
    InterfaceBehaviorRevision, InterfaceCompatibilityClaim, InterfaceCompatibilityClaimId,
    InterfaceNewerVersionPosture, InterfaceSupportStatus, InterfaceVersion, InterfaceVersionAxis,
    InterfaceVersionBinding, InterfaceVersionScheme, InterfaceVersionSegment, PreflightPlan,
};
use swallowtail_runtime::RuntimeFailure;

use crate::failure::failure;

/// Unambiguous executable name used for Cline discovery.
pub const CLINE_EXECUTABLE_NAME: &str = "cline";
/// Opaque npm package-version axis for Cline ACP.
pub const CLINE_PACKAGE_AXIS: &str = "cline.package";
/// Exact qualified Cline npm wrapper used by ACP and headless.
pub const CLINE_PACKAGE_VERSION: &str = "3.0.55";

pub(crate) const CLINE_ACP_BEHAVIOR: &str = "cline.acp.stdio-v1";
pub(crate) const CLINE_HEADLESS_BEHAVIOR: &str = "cline.headless.stdio-json-v1";
const MAX_VERSION_BYTES: usize = 32;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct ClinePlanSelection {
    version: InterfaceVersion,
}

impl ClinePlanSelection {
    pub(crate) const fn version(&self) -> &InterfaceVersion {
        &self.version
    }
}

/// Parses installed `--version` stdout into the exact qualified Cline binding.
#[must_use]
pub(crate) fn parse_cline_version_output(output: &[u8]) -> Option<InterfaceVersionBinding> {
    let output = std::str::from_utf8(output).ok()?;
    let exact = output.strip_suffix('\n').unwrap_or(output);
    cline_package_binding(exact)
}

/// Parses the one qualified exact Cline package version into its interface binding.
///
/// Returns `None` for anything other than the exact qualified release text, so
/// observed CLI output can never panic a caller.
#[must_use]
pub fn cline_package_binding(value: &str) -> Option<InterfaceVersionBinding> {
    if value != CLINE_PACKAGE_VERSION
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

/// Returns the qualified-only exact Cline ACP protocol claim.
#[must_use]
pub fn cline_acp_claim() -> InterfaceCompatibilityClaim {
    InterfaceCompatibilityClaim::new(
        InterfaceCompatibilityClaimId::new("cline.acp.package-window-1")
            .expect("static Cline claim id is valid"),
        axis(),
        InterfaceVersionScheme::Semantic,
        InterfaceNewerVersionPosture::QualifiedOnly,
        [InterfaceVersionSegment::exact(
            InterfaceVersion::new(CLINE_PACKAGE_VERSION).expect("static Cline version is valid"),
            InterfaceBehaviorRevision::new(CLINE_ACP_BEHAVIOR)
                .expect("static Cline behavior is valid"),
            InterfaceSupportStatus::Maintained,
        )],
        [],
    )
    .expect("static Cline claim is valid")
}

/// Returns the qualified-only exact Cline headless JSON protocol claim.
#[must_use]
pub fn cline_headless_claim() -> InterfaceCompatibilityClaim {
    InterfaceCompatibilityClaim::new(
        InterfaceCompatibilityClaimId::new("cline.headless.package-window-1")
            .expect("static Cline headless claim id is valid"),
        axis(),
        InterfaceVersionScheme::Semantic,
        InterfaceNewerVersionPosture::QualifiedOnly,
        [InterfaceVersionSegment::exact(
            InterfaceVersion::new(CLINE_PACKAGE_VERSION).expect("static Cline version is valid"),
            InterfaceBehaviorRevision::new(CLINE_HEADLESS_BEHAVIOR)
                .expect("static Cline headless behavior is valid"),
            InterfaceSupportStatus::Maintained,
        )],
        [],
    )
    .expect("static Cline headless claim is valid")
}

pub(crate) fn select_cline_headless_plan(
    plan: &PreflightPlan,
) -> Result<ClinePlanSelection, RuntimeFailure> {
    select_plan(
        plan,
        &cline_headless_claim(),
        CLINE_HEADLESS_BEHAVIOR,
        PlanSelectionCodes {
            missing: "swallowtail.cline.headless.version_missing",
            missing_message: "Cline headless plan is missing its exact package version",
            ambiguous: "swallowtail.cline.headless.version_ambiguous",
            ambiguous_message: "Cline headless plan contains more than one package version",
            incompatible: "swallowtail.cline.headless.version_incompatible",
            incompatible_message: "Cline package version is incompatible with the headless driver",
        },
    )
}

pub(crate) fn select_cline_acp_plan(
    plan: &PreflightPlan,
) -> Result<ClinePlanSelection, RuntimeFailure> {
    select_plan(
        plan,
        &cline_acp_claim(),
        CLINE_ACP_BEHAVIOR,
        PlanSelectionCodes {
            missing: "swallowtail.cline.acp.version_missing",
            missing_message: "Cline ACP plan is missing its exact package version",
            ambiguous: "swallowtail.cline.acp.version_ambiguous",
            ambiguous_message: "Cline ACP plan contains more than one package version",
            incompatible: "swallowtail.cline.acp.version_incompatible",
            incompatible_message: "Cline package version is incompatible with the ACP driver",
        },
    )
}

struct PlanSelectionCodes {
    missing: &'static str,
    missing_message: &'static str,
    ambiguous: &'static str,
    ambiguous_message: &'static str,
    incompatible: &'static str,
    incompatible_message: &'static str,
}

fn select_plan(
    plan: &PreflightPlan,
    claim: &InterfaceCompatibilityClaim,
    behavior: &str,
    codes: PlanSelectionCodes,
) -> Result<ClinePlanSelection, RuntimeFailure> {
    let mut bindings = plan
        .interface_versions()
        .filter(|binding| binding.axis() == claim.axis());
    let binding = bindings
        .next()
        .ok_or_else(|| failure(codes.missing, codes.missing_message))?;
    if bindings.next().is_some() {
        return Err(failure(codes.ambiguous, codes.ambiguous_message));
    }
    let assessment = claim.assess(binding.version());
    if assessment != plan.assess_interface_version(binding)
        || !assessment.is_permitted()
        || assessment
            .behavior_revision()
            .is_none_or(|revision| revision.as_str() != behavior)
    {
        return Err(failure(codes.incompatible, codes.incompatible_message));
    }
    Ok(ClinePlanSelection {
        version: binding.version().clone(),
    })
}

fn axis() -> InterfaceVersionAxis {
    InterfaceVersionAxis::new(CLINE_PACKAGE_AXIS).expect("static Cline package axis is valid")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_the_exact_qualified_release_is_bound() {
        assert!(cline_package_binding(CLINE_PACKAGE_VERSION).is_some());
        for rejected in [
            "",
            "3.0.54",
            "3.0.56",
            "3.0",
            "3.0.55.0",
            "v3.0.55",
            "3.0.55-beta",
            "3.0.55\n",
            " 3.0.55",
            "3.0.55 ",
            "cline 3.0.55",
        ] {
            assert!(
                cline_package_binding(rejected).is_none(),
                "{rejected:?} must not bind"
            );
        }
    }

    #[test]
    fn exact_package_is_permitted_and_newer_is_not() {
        let permitted = InterfaceVersion::new(CLINE_PACKAGE_VERSION).expect("qualified version");
        let newer = InterfaceVersion::new("3.0.56").expect("newer version");
        for claim in [cline_acp_claim(), cline_headless_claim()] {
            assert!(claim.assess(&permitted).is_permitted());
            assert!(!claim.assess(&newer).is_permitted());
        }
        assert_ne!(
            cline_acp_claim().id().as_str(),
            cline_headless_claim().id().as_str()
        );
    }

    #[test]
    fn version_stdout_parser_requires_the_exact_release_line() {
        assert_eq!(
            parse_cline_version_output(b"3.0.55\n")
                .expect("exact version parses")
                .version()
                .as_str(),
            "3.0.55"
        );
        assert!(parse_cline_version_output(b"3.0.56\n").is_none());
        assert!(parse_cline_version_output(b"cline 3.0.55\n").is_none());
    }
}

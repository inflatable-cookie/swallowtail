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
/// Frozen qualified baseline Cline npm wrapper shared by ACP and headless.
pub const CLINE_PACKAGE_VERSION: &str = "3.0.55";
const CLINE_HEADLESS_LATEST_QUALIFIED_VERSION: &str = "3.0.70";

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

/// Parses installed `--version` stdout into the exact Cline ACP baseline binding.
#[must_use]
pub(crate) fn parse_cline_acp_version_output(output: &[u8]) -> Option<InterfaceVersionBinding> {
    let output = std::str::from_utf8(output).ok()?;
    let exact = output.strip_suffix('\n').unwrap_or(output);
    cline_package_binding(exact)
}

/// Parses a headless installed version candidate before applying its route claim.
#[must_use]
pub(crate) fn parse_cline_headless_version_output(
    output: &[u8],
) -> Option<InterfaceVersionBinding> {
    let output = std::str::from_utf8(output).ok()?;
    let exact = output.strip_suffix('\n').unwrap_or(output);
    package_version_binding(exact)
}

/// Returns the frozen baseline Cline package binding.
///
/// This identity helper deliberately remains pinned to the original `3.0.55`
/// point. Headless discovery parses candidates separately and applies its
/// compatibility claim before promotion; ACP discovery stays on this exact
/// binding.
#[must_use]
pub fn cline_package_binding(value: &str) -> Option<InterfaceVersionBinding> {
    if value != CLINE_PACKAGE_VERSION {
        return None;
    }
    package_version_binding(value)
}

fn package_version_binding(value: &str) -> Option<InterfaceVersionBinding> {
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

/// Returns the qualified-only Cline headless JSON protocol claim.
#[must_use]
pub fn cline_headless_claim() -> InterfaceCompatibilityClaim {
    InterfaceCompatibilityClaim::new(
        InterfaceCompatibilityClaimId::new("cline.headless.package-window-1")
            .expect("static Cline headless claim id is valid"),
        axis(),
        InterfaceVersionScheme::Semantic,
        InterfaceNewerVersionPosture::QualifiedOnly,
        [InterfaceVersionSegment::new(
            InterfaceVersion::new(CLINE_PACKAGE_VERSION).expect("static Cline baseline is valid"),
            InterfaceVersion::new(CLINE_HEADLESS_LATEST_QUALIFIED_VERSION)
                .expect("static Cline ceiling is valid"),
            InterfaceBehaviorRevision::new(CLINE_HEADLESS_BEHAVIOR)
                .expect("static Cline headless behavior is valid"),
            InterfaceSupportStatus::Maintained,
        )],
        [InterfaceVersion::new("3.0.59").expect("static Cline exclusion is valid")],
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
    fn public_identity_binding_stays_on_the_frozen_baseline() {
        assert!(cline_package_binding(CLINE_PACKAGE_VERSION).is_some());
        for rejected in [
            "",
            "3.0.54",
            "3.0.56",
            "3.0.69",
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
    fn headless_window_qualifies_published_hops_and_preserves_the_hole() {
        let headless = cline_headless_claim();
        for qualified in [
            "3.0.55", "3.0.56", "3.0.57", "3.0.58", "3.0.60", "3.0.61", "3.0.62", "3.0.63",
            "3.0.64", "3.0.65", "3.0.66", "3.0.67", "3.0.68", "3.0.69", "3.0.70",
        ] {
            assert!(headless.permits(&InterfaceVersion::new(qualified).expect("version")));
        }
        for rejected in ["3.0.54", "3.0.59", "3.0.71"] {
            assert!(!headless.permits(&InterfaceVersion::new(rejected).expect("version")));
        }
        assert_eq!(headless.baseline().as_str(), CLINE_PACKAGE_VERSION);
        assert_eq!(
            headless.latest_qualified().as_str(),
            CLINE_HEADLESS_LATEST_QUALIFIED_VERSION
        );
        assert_eq!(
            headless
                .exclusions()
                .map(InterfaceVersion::as_str)
                .collect::<Vec<_>>(),
            ["3.0.59"]
        );

        let acp = cline_acp_claim();
        assert!(acp.permits(&InterfaceVersion::new(CLINE_PACKAGE_VERSION).expect("baseline")));
        assert!(!acp.permits(&InterfaceVersion::new("3.0.70").expect("headless latest")));
        assert_ne!(
            cline_acp_claim().id().as_str(),
            cline_headless_claim().id().as_str()
        );
    }

    #[test]
    fn acp_version_stdout_parser_keeps_the_exact_baseline() {
        assert_eq!(
            parse_cline_acp_version_output(b"3.0.55\n")
                .expect("baseline parses")
                .version()
                .as_str(),
            "3.0.55"
        );
        for candidate in [b"3.0.56\n".as_slice(), b"3.0.59\n", b"3.0.70\n"] {
            assert!(parse_cline_acp_version_output(candidate).is_none());
        }
        assert!(parse_cline_acp_version_output(b"cline 3.0.55\n").is_none());
    }

    #[test]
    fn headless_version_stdout_parser_parses_candidates_before_route_classification() {
        for candidate in [
            b"3.0.55\n".as_slice(),
            b"3.0.56\n",
            b"3.0.59\n",
            b"3.0.70\n",
        ] {
            assert!(parse_cline_headless_version_output(candidate).is_some());
        }
        assert!(parse_cline_headless_version_output(b"cline 3.0.55\n").is_none());
    }
}

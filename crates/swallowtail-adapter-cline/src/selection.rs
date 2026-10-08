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
const CLINE_ACP_LATEST_QUALIFIED_VERSION: &str = "3.0.70";

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct ClinePlanSelection {
    version: InterfaceVersion,
}

impl ClinePlanSelection {
    pub(crate) const fn version(&self) -> &InterfaceVersion {
        &self.version
    }
}

/// Parses installed `--version` stdout into an ACP candidate binding.
#[must_use]
pub(crate) fn parse_cline_acp_version_output(output: &[u8]) -> Option<InterfaceVersionBinding> {
    let output = std::str::from_utf8(output).ok()?;
    let exact = output.strip_suffix('\n').unwrap_or(output);
    package_version_binding(exact)
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

/// Returns the maintained Cline ACP compatibility window.
#[must_use]
pub fn cline_acp_claim() -> InterfaceCompatibilityClaim {
    InterfaceCompatibilityClaim::new(
        InterfaceCompatibilityClaimId::new("cline.acp.package-window-1")
            .expect("static Cline claim id is valid"),
        axis(),
        InterfaceVersionScheme::Semantic,
        InterfaceNewerVersionPosture::AllowUnverified,
        [InterfaceVersionSegment::new(
            InterfaceVersion::new(CLINE_PACKAGE_VERSION).expect("static Cline version is valid"),
            InterfaceVersion::new(CLINE_ACP_LATEST_QUALIFIED_VERSION)
                .expect("static Cline version is valid"),
            InterfaceBehaviorRevision::new(CLINE_ACP_BEHAVIOR)
                .expect("static Cline behavior is valid"),
            InterfaceSupportStatus::Maintained,
        )],
        [InterfaceVersion::new("3.0.59").expect("unpublished Cline version is valid")],
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
            missing_message: "Cline ACP plan is missing its package version",
            ambiguous: "swallowtail.cline.acp.version_ambiguous",
            ambiguous_message: "Cline ACP plan contains more than one package version",
            incompatible: "swallowtail.cline.acp.version_incompatible",
            incompatible_message: "Cline package version is outside the ACP compatibility claim",
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
            "3.0.70",
            "3.0",
            "3.0.55.0",
            "v3.0.55",
            "3.0.55-beta",
            "3.0.55+build.1",
            "3.0.55\n",
            " 3.0.55",
            "3.0.55 ",
            "cline 3.0.55",
            "123456789012345678901234567890123",
        ] {
            assert!(
                cline_package_binding(rejected).is_none(),
                "{rejected:?} must not bind",
            );
        }
    }

    #[test]
    fn acp_window_excludes_unpublished_point_and_keeps_headless_window() {
        let acp = cline_acp_claim();
        assert_eq!(acp.id().as_str(), "cline.acp.package-window-1");
        assert_eq!(acp.baseline().as_str(), "3.0.55");
        assert_eq!(acp.latest_qualified().as_str(), "3.0.70");
        assert_eq!(
            acp.newer_version_posture(),
            InterfaceNewerVersionPosture::AllowUnverified
        );
        assert_eq!(
            acp.exclusions()
                .map(InterfaceVersion::as_str)
                .collect::<Vec<_>>(),
            ["3.0.59"]
        );
        for qualified in ["3.0.55", "3.0.56", "3.0.58", "3.0.60", "3.0.70"] {
            let version = InterfaceVersion::new(qualified).expect("qualified version");
            assert!(
                acp.assess(&version).is_permitted(),
                "{qualified} is permitted"
            );
            assert!(acp.classify(&version).is_some(), "{qualified} is qualified");
        }
        let hole = InterfaceVersion::new("3.0.59").expect("excluded version");
        assert!(!acp.assess(&hole).is_permitted());
        let next = InterfaceVersion::new("3.0.71").expect("next stable version");
        assert!(matches!(
            acp.assess(&next),
            swallowtail_core::InterfaceCompatibilityAssessment::UnverifiedNewer(_)
        ));
        let older = InterfaceVersion::new("3.0.54").expect("older version");
        assert!(!acp.assess(&older).is_permitted());

        let headless = cline_headless_claim();
        assert_eq!(headless.baseline().as_str(), "3.0.55");
        assert_eq!(headless.latest_qualified().as_str(), "3.0.70");
        assert!(headless.permits(&InterfaceVersion::new("3.0.70").expect("latest headless")));
        assert!(!headless.permits(&hole));
        assert!(!headless.permits(&next));
        assert_ne!(acp.id().as_str(), headless.id().as_str());
    }

    #[test]
    fn acp_version_stdout_parser_parses_candidates_before_route_classification() {
        assert_eq!(
            parse_cline_acp_version_output(b"3.0.55\n")
                .expect("baseline parses")
                .version()
                .as_str(),
            "3.0.55"
        );
        assert_eq!(
            parse_cline_acp_version_output(b"3.0.70\n")
                .expect("newest qualified version parses")
                .version()
                .as_str(),
            "3.0.70"
        );
        let hole = parse_cline_acp_version_output(b"3.0.59\n").expect("stable version parses");
        assert!(!cline_acp_claim().assess(hole.version()).is_permitted());
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

use swallowtail_core::{
    InterfaceBehaviorRevision, InterfaceCompatibilityClaim, InterfaceCompatibilityClaimId,
    InterfaceNewerVersionPosture, InterfaceSupportStatus, InterfaceVersion, InterfaceVersionAxis,
    InterfaceVersionBinding, InterfaceVersionScheme, InterfaceVersionSegment, PreflightPlan,
};
use swallowtail_runtime::RuntimeFailure;

use crate::failure::failure;

/// Unambiguous executable name used for Goose discovery.
pub const GOOSE_EXECUTABLE_NAME: &str = "goose";
/// Opaque GitHub-release axis for Goose ACP.
pub const GOOSE_RELEASE_AXIS: &str = "goose.release";
/// Exact qualified Goose CLI release used by ACP.
pub const GOOSE_RELEASE_VERSION: &str = "1.46.0";

pub(crate) const GOOSE_ACP_BEHAVIOR: &str = "goose.acp.stdio-v1";
const MAX_VERSION_BYTES: usize = 32;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct GoosePlanSelection {
    version: InterfaceVersion,
}

impl GoosePlanSelection {
    pub(crate) const fn version(&self) -> &InterfaceVersion {
        &self.version
    }
}

/// Parses installed `--version` stdout into the exact qualified Goose binding.
#[must_use]
pub(crate) fn parse_goose_version_output(output: &[u8]) -> Option<InterfaceVersionBinding> {
    let output = std::str::from_utf8(output).ok()?;
    let exact = output.strip_suffix('\n').unwrap_or(output);
    let exact = exact.strip_prefix("goose ").unwrap_or(exact);
    goose_release_binding(exact)
}

/// Parses the one qualified exact Goose release version into its interface binding.
///
/// Returns `None` for anything other than the exact qualified release text, so
/// observed CLI output can never panic a caller.
#[must_use]
pub fn goose_release_binding(value: &str) -> Option<InterfaceVersionBinding> {
    if value != GOOSE_RELEASE_VERSION
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

/// Returns the qualified-only exact Goose ACP protocol claim.
#[must_use]
pub fn goose_acp_claim() -> InterfaceCompatibilityClaim {
    InterfaceCompatibilityClaim::new(
        InterfaceCompatibilityClaimId::new("goose.acp.release-window-1")
            .expect("static Goose claim id is valid"),
        axis(),
        InterfaceVersionScheme::Semantic,
        InterfaceNewerVersionPosture::QualifiedOnly,
        [InterfaceVersionSegment::exact(
            InterfaceVersion::new(GOOSE_RELEASE_VERSION).expect("static Goose version is valid"),
            InterfaceBehaviorRevision::new(GOOSE_ACP_BEHAVIOR)
                .expect("static Goose behavior is valid"),
            InterfaceSupportStatus::Maintained,
        )],
        [],
    )
    .expect("static Goose claim is valid")
}

pub(crate) fn select_goose_acp_plan(
    plan: &PreflightPlan,
) -> Result<GoosePlanSelection, RuntimeFailure> {
    select_plan(
        plan,
        &goose_acp_claim(),
        GOOSE_ACP_BEHAVIOR,
        PlanSelectionCodes {
            missing: "swallowtail.goose.acp.version_missing",
            missing_message: "Goose ACP plan is missing its exact release version",
            ambiguous: "swallowtail.goose.acp.version_ambiguous",
            ambiguous_message: "Goose ACP plan contains more than one release version",
            incompatible: "swallowtail.goose.acp.version_incompatible",
            incompatible_message: "Goose release version is incompatible with the ACP driver",
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
) -> Result<GoosePlanSelection, RuntimeFailure> {
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
    Ok(GoosePlanSelection {
        version: binding.version().clone(),
    })
}

fn axis() -> InterfaceVersionAxis {
    InterfaceVersionAxis::new(GOOSE_RELEASE_AXIS).expect("static Goose release axis is valid")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_the_exact_qualified_release_is_bound() {
        assert!(goose_release_binding(GOOSE_RELEASE_VERSION).is_some());
        for rejected in [
            "",
            "1.45.0",
            "1.46.1",
            "1.46",
            "1.46.0.0",
            "v1.46.0",
            "1.46.0-beta",
            "1.46.0\n",
            " 1.46.0",
            "1.46.0 ",
            "goose 1.46.0",
        ] {
            assert!(
                goose_release_binding(rejected).is_none(),
                "{rejected:?} must not bind"
            );
        }
    }

    #[test]
    fn exact_release_is_permitted_and_newer_is_not() {
        let permitted = InterfaceVersion::new(GOOSE_RELEASE_VERSION).expect("qualified version");
        let newer = InterfaceVersion::new("1.46.1").expect("newer version");
        let claim = goose_acp_claim();
        assert!(claim.assess(&permitted).is_permitted());
        assert!(!claim.assess(&newer).is_permitted());
    }

    #[test]
    fn version_stdout_parser_accepts_bare_or_named_exact_release() {
        assert_eq!(
            parse_goose_version_output(b"1.46.0\n")
                .expect("exact version parses")
                .version()
                .as_str(),
            "1.46.0"
        );
        assert_eq!(
            parse_goose_version_output(b"goose 1.46.0\n")
                .expect("named version parses")
                .version()
                .as_str(),
            "1.46.0"
        );
        assert!(parse_goose_version_output(b"1.46.1\n").is_none());
        assert!(parse_goose_version_output(b"v1.46.0\n").is_none());
        assert!(parse_goose_version_output(b"goose  1.46.0\n").is_none());
    }
}

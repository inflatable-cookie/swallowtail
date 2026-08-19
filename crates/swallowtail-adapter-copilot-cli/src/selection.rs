use swallowtail_core::{
    InterfaceBehaviorRevision, InterfaceCompatibilityClaim, InterfaceCompatibilityClaimId,
    InterfaceNewerVersionPosture, InterfaceSupportStatus, InterfaceVersion, InterfaceVersionAxis,
    InterfaceVersionBinding, InterfaceVersionScheme, InterfaceVersionSegment, PreflightPlan,
};
use swallowtail_runtime::RuntimeFailure;

use crate::failure::failure;

/// Unambiguous executable name used for Copilot CLI discovery.
pub const COPILOT_CLI_EXECUTABLE_NAME: &str = "copilot";
/// Opaque npm package-version axis for Copilot CLI ACP.
pub const COPILOT_CLI_PACKAGE_AXIS: &str = "copilot-cli.package";
/// Exact qualified Copilot CLI npm wrapper used by ACP.
pub const COPILOT_CLI_PACKAGE_VERSION: &str = "1.0.80";
/// Official Copilot CLI ACP server maturity. Must remain visible.
pub const COPILOT_CLI_ACP_MATURITY: &str = "public-preview";

pub(crate) const COPILOT_CLI_ACP_BEHAVIOR: &str = "copilot-cli.acp.stdio-v1";
const MAX_VERSION_BYTES: usize = 32;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct CopilotCliPlanSelection {
    version: InterfaceVersion,
}

impl CopilotCliPlanSelection {
    pub(crate) const fn version(&self) -> &InterfaceVersion {
        &self.version
    }
}

/// Parses installed `--version` stdout into the exact qualified Copilot CLI binding.
#[must_use]
pub(crate) fn parse_copilot_cli_version_output(output: &[u8]) -> Option<InterfaceVersionBinding> {
    let output = std::str::from_utf8(output).ok()?;
    let exact = output.strip_suffix('\n').unwrap_or(output);
    let exact = exact.strip_prefix("copilot ").unwrap_or(exact);
    copilot_cli_package_binding(exact)
}

/// Parses the one qualified exact Copilot CLI release version into its interface binding.
///
/// Returns `None` for anything other than the exact qualified release text, so
/// observed CLI output can never panic a caller.
#[must_use]
pub fn copilot_cli_package_binding(value: &str) -> Option<InterfaceVersionBinding> {
    if value != COPILOT_CLI_PACKAGE_VERSION
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

/// Returns the qualified-only exact Copilot CLI ACP protocol claim.
#[must_use]
pub fn copilot_cli_acp_claim() -> InterfaceCompatibilityClaim {
    InterfaceCompatibilityClaim::new(
        InterfaceCompatibilityClaimId::new("copilot-cli.acp.package-window-1")
            .expect("static Copilot CLI claim id is valid"),
        axis(),
        InterfaceVersionScheme::Semantic,
        InterfaceNewerVersionPosture::QualifiedOnly,
        [InterfaceVersionSegment::exact(
            InterfaceVersion::new(COPILOT_CLI_PACKAGE_VERSION)
                .expect("static Copilot CLI version is valid"),
            InterfaceBehaviorRevision::new(COPILOT_CLI_ACP_BEHAVIOR)
                .expect("static Copilot CLI behavior is valid"),
            InterfaceSupportStatus::Maintained,
        )],
        [],
    )
    .expect("static Copilot CLI claim is valid")
}

pub(crate) fn select_copilot_cli_acp_plan(
    plan: &PreflightPlan,
) -> Result<CopilotCliPlanSelection, RuntimeFailure> {
    select_plan(
        plan,
        &copilot_cli_acp_claim(),
        COPILOT_CLI_ACP_BEHAVIOR,
        PlanSelectionCodes {
            missing: "swallowtail.copilot-cli.acp.version_missing",
            missing_message: "Copilot CLI ACP plan is missing its exact package version",
            ambiguous: "swallowtail.copilot-cli.acp.version_ambiguous",
            ambiguous_message: "Copilot CLI ACP plan contains more than one package version",
            incompatible: "swallowtail.copilot-cli.acp.version_incompatible",
            incompatible_message: "Copilot CLI package version is incompatible with the ACP driver",
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
) -> Result<CopilotCliPlanSelection, RuntimeFailure> {
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
    Ok(CopilotCliPlanSelection {
        version: binding.version().clone(),
    })
}

fn axis() -> InterfaceVersionAxis {
    InterfaceVersionAxis::new(COPILOT_CLI_PACKAGE_AXIS)
        .expect("static Copilot CLI package axis is valid")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_the_exact_qualified_release_is_bound() {
        assert!(copilot_cli_package_binding(COPILOT_CLI_PACKAGE_VERSION).is_some());
        for rejected in [
            "",
            "1.0.79",
            "1.0.81",
            "1.0",
            "1.0.80.0",
            "v1.0.80",
            "1.0.80-beta",
            "1.0.81-0",
            "1.0.81-1",
            "1.0.80\n",
            " 1.0.80",
            "1.0.80 ",
            "copilot 1.0.80",
        ] {
            assert!(
                copilot_cli_package_binding(rejected).is_none(),
                "{rejected:?} must not bind"
            );
        }
    }

    #[test]
    fn exact_release_is_permitted_and_newer_is_not() {
        let permitted =
            InterfaceVersion::new(COPILOT_CLI_PACKAGE_VERSION).expect("qualified version");
        let newer = InterfaceVersion::new("1.0.81").expect("newer version");
        let claim = copilot_cli_acp_claim();
        assert!(claim.assess(&permitted).is_permitted());
        assert!(!claim.assess(&newer).is_permitted());
        assert_eq!(COPILOT_CLI_ACP_MATURITY, "public-preview");
    }

    #[test]
    fn version_stdout_parser_accepts_bare_or_named_exact_release() {
        assert_eq!(
            parse_copilot_cli_version_output(b"1.0.80\n")
                .expect("exact version parses")
                .version()
                .as_str(),
            "1.0.80"
        );
        assert_eq!(
            parse_copilot_cli_version_output(b"copilot 1.0.80\n")
                .expect("named version parses")
                .version()
                .as_str(),
            "1.0.80"
        );
        assert!(parse_copilot_cli_version_output(b"1.0.81\n").is_none());
        assert!(parse_copilot_cli_version_output(b"v1.0.80\n").is_none());
        assert!(parse_copilot_cli_version_output(b"copilot  1.0.80\n").is_none());
        assert!(parse_copilot_cli_version_output(b"1.0.81-1\n").is_none());
    }
}

use swallowtail_core::{
    InterfaceBehaviorRevision, InterfaceCompatibilityClaim, InterfaceCompatibilityClaimId,
    InterfaceNewerVersionPosture, InterfaceSupportStatus, InterfaceVersion, InterfaceVersionAxis,
    InterfaceVersionBinding, InterfaceVersionScheme, InterfaceVersionSegment, PreflightPlan,
};
use swallowtail_runtime::RuntimeFailure;

use crate::failure::failure;

/// Unambiguous executable name used for Deep Agents discovery.
pub const DEEPAGENTS_ACP_EXECUTABLE_NAME: &str = "deepagents-acp";
/// Opaque npm package-version axis for Deep Agents ACP.
pub const DEEPAGENTS_ACP_PACKAGE_AXIS: &str = "deepagents-acp.package";
/// Exact qualified Deep Agents npm package used by ACP.
pub const DEEPAGENTS_ACP_PACKAGE_VERSION: &str = "0.1.25";

pub(crate) const DEEPAGENTS_ACP_BEHAVIOR: &str = "deepagents.acp.stdio-v1";
const MAX_VERSION_BYTES: usize = 32;

/// Parses installed `--version` stdout into the exact qualified Deep Agents binding.
#[must_use]
pub(crate) fn parse_deepagents_acp_version_output(
    output: &[u8],
) -> Option<InterfaceVersionBinding> {
    let output = std::str::from_utf8(output).ok()?;
    let exact = output.strip_suffix('\n').unwrap_or(output);
    let exact = exact.strip_prefix("deepagents-acp ").unwrap_or(exact);
    deepagents_acp_package_binding(exact)
}

/// Parses the one qualified exact Deep Agents package version into its interface binding.
///
/// Returns `None` for anything other than the exact qualified release text, so
/// observed CLI output can never panic a caller.
#[must_use]
pub fn deepagents_acp_package_binding(value: &str) -> Option<InterfaceVersionBinding> {
    if value != DEEPAGENTS_ACP_PACKAGE_VERSION
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

/// Returns the qualified-only exact Deep Agents ACP protocol claim.
#[must_use]
pub fn deepagents_acp_claim() -> InterfaceCompatibilityClaim {
    InterfaceCompatibilityClaim::new(
        InterfaceCompatibilityClaimId::new("deepagents.acp.package-window-1")
            .expect("static Deep Agents claim id is valid"),
        axis(),
        InterfaceVersionScheme::Semantic,
        InterfaceNewerVersionPosture::QualifiedOnly,
        [InterfaceVersionSegment::exact(
            InterfaceVersion::new(DEEPAGENTS_ACP_PACKAGE_VERSION)
                .expect("static Deep Agents version is valid"),
            InterfaceBehaviorRevision::new(DEEPAGENTS_ACP_BEHAVIOR)
                .expect("static Deep Agents behavior is valid"),
            InterfaceSupportStatus::Maintained,
        )],
        [],
    )
    .expect("static Deep Agents claim is valid")
}

pub(crate) fn select_deepagents_acp_plan(plan: &PreflightPlan) -> Result<(), RuntimeFailure> {
    select_plan(
        plan,
        &deepagents_acp_claim(),
        DEEPAGENTS_ACP_BEHAVIOR,
        PlanSelectionCodes {
            missing: "swallowtail.deepagents.acp.version_missing",
            missing_message: "Deep Agents ACP plan is missing its exact release version",
            ambiguous: "swallowtail.deepagents.acp.version_ambiguous",
            ambiguous_message: "Deep Agents ACP plan contains more than one release version",
            incompatible: "swallowtail.deepagents.acp.version_incompatible",
            incompatible_message: "Deep Agents package version is incompatible with the ACP driver",
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
) -> Result<(), RuntimeFailure> {
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
    Ok(())
}

fn axis() -> InterfaceVersionAxis {
    InterfaceVersionAxis::new(DEEPAGENTS_ACP_PACKAGE_AXIS)
        .expect("static Deep Agents package axis is valid")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_the_exact_qualified_release_is_bound() {
        assert!(deepagents_acp_package_binding(DEEPAGENTS_ACP_PACKAGE_VERSION).is_some());
        for rejected in [
            "",
            "0.1.24",
            "0.1.26",
            "0.0.1",
            "0.1.7",
            "0.1",
            "0.1.25.0",
            "v0.1.25",
            "0.1.25-beta",
            "0.1.25\n",
            " 0.1.25",
            "0.1.25 ",
            "deepagents-acp 0.1.25",
        ] {
            assert!(
                deepagents_acp_package_binding(rejected).is_none(),
                "{rejected:?} must not bind"
            );
        }
    }

    #[test]
    fn exact_release_is_permitted_and_newer_is_not() {
        let permitted =
            InterfaceVersion::new(DEEPAGENTS_ACP_PACKAGE_VERSION).expect("qualified version");
        let newer = InterfaceVersion::new("0.1.26").expect("newer version");
        let claim = deepagents_acp_claim();
        assert!(claim.assess(&permitted).is_permitted());
        assert!(!claim.assess(&newer).is_permitted());
    }

    #[test]
    fn version_stdout_parser_accepts_bare_or_named_exact_release() {
        assert_eq!(
            parse_deepagents_acp_version_output(b"0.1.25\n")
                .expect("exact version parses")
                .version()
                .as_str(),
            "0.1.25"
        );
        assert_eq!(
            parse_deepagents_acp_version_output(b"deepagents-acp 0.1.25\n")
                .expect("named version parses")
                .version()
                .as_str(),
            "0.1.25"
        );
        assert!(parse_deepagents_acp_version_output(b"0.1.26\n").is_none());
        assert!(parse_deepagents_acp_version_output(b"0.0.1\n").is_none());
        assert!(parse_deepagents_acp_version_output(b"v0.1.25\n").is_none());
        assert!(parse_deepagents_acp_version_output(b"deepagents-acp  0.1.25\n").is_none());
    }
}

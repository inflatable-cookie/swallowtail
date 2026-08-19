use swallowtail_core::{
    InterfaceBehaviorRevision, InterfaceCompatibilityClaim, InterfaceCompatibilityClaimId,
    InterfaceNewerVersionPosture, InterfaceSupportStatus, InterfaceVersion, InterfaceVersionAxis,
    InterfaceVersionBinding, InterfaceVersionScheme, InterfaceVersionSegment, PreflightPlan,
};
use swallowtail_runtime::RuntimeFailure;

use crate::failure::failure;

/// Unambiguous executable name used for Kiro discovery.
pub const KIRO_CLI_EXECUTABLE_NAME: &str = "kiro-cli";
/// Opaque GitHub-release axis for Kiro ACP.
pub const KIRO_CLI_RELEASE_AXIS: &str = "kiro-cli.release";
/// Exact qualified Kiro CLI release used by ACP.
pub const KIRO_CLI_RELEASE_VERSION: &str = "2.18.1";

pub(crate) const KIRO_ACP_BEHAVIOR: &str = "kiro.acp.stdio-v1";
const MAX_VERSION_BYTES: usize = 32;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct KiroPlanSelection {
    version: InterfaceVersion,
}

impl KiroPlanSelection {
    pub(crate) const fn version(&self) -> &InterfaceVersion {
        &self.version
    }
}

/// Parses installed `--version` stdout into the exact qualified Kiro binding.
#[must_use]
pub(crate) fn parse_kiro_cli_version_output(output: &[u8]) -> Option<InterfaceVersionBinding> {
    let output = std::str::from_utf8(output).ok()?;
    let exact = output.strip_suffix('\n').unwrap_or(output);
    let exact = exact.strip_prefix("kiro-cli ").unwrap_or(exact);
    kiro_cli_release_binding(exact)
}

/// Parses the one qualified exact Kiro release version into its interface binding.
///
/// Returns `None` for anything other than the exact qualified release text, so
/// observed CLI output can never panic a caller.
#[must_use]
pub fn kiro_cli_release_binding(value: &str) -> Option<InterfaceVersionBinding> {
    if value != KIRO_CLI_RELEASE_VERSION
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

/// Returns the qualified-only exact Kiro ACP protocol claim.
#[must_use]
pub fn kiro_acp_claim() -> InterfaceCompatibilityClaim {
    InterfaceCompatibilityClaim::new(
        InterfaceCompatibilityClaimId::new("kiro.acp.release-window-1")
            .expect("static Kiro claim id is valid"),
        axis(),
        InterfaceVersionScheme::Semantic,
        InterfaceNewerVersionPosture::QualifiedOnly,
        [InterfaceVersionSegment::exact(
            InterfaceVersion::new(KIRO_CLI_RELEASE_VERSION).expect("static Kiro version is valid"),
            InterfaceBehaviorRevision::new(KIRO_ACP_BEHAVIOR)
                .expect("static Kiro behavior is valid"),
            InterfaceSupportStatus::Maintained,
        )],
        [],
    )
    .expect("static Kiro claim is valid")
}

pub(crate) fn select_kiro_acp_plan(
    plan: &PreflightPlan,
) -> Result<KiroPlanSelection, RuntimeFailure> {
    select_plan(
        plan,
        &kiro_acp_claim(),
        KIRO_ACP_BEHAVIOR,
        PlanSelectionCodes {
            missing: "swallowtail.kiro.acp.version_missing",
            missing_message: "Kiro ACP plan is missing its exact release version",
            ambiguous: "swallowtail.kiro.acp.version_ambiguous",
            ambiguous_message: "Kiro ACP plan contains more than one release version",
            incompatible: "swallowtail.kiro.acp.version_incompatible",
            incompatible_message: "Kiro release version is incompatible with the ACP driver",
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
) -> Result<KiroPlanSelection, RuntimeFailure> {
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
    Ok(KiroPlanSelection {
        version: binding.version().clone(),
    })
}

fn axis() -> InterfaceVersionAxis {
    InterfaceVersionAxis::new(KIRO_CLI_RELEASE_AXIS).expect("static Kiro release axis is valid")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_the_exact_qualified_release_is_bound() {
        assert!(kiro_cli_release_binding(KIRO_CLI_RELEASE_VERSION).is_some());
        for rejected in [
            "",
            "2.17.0",
            "2.18.2",
            "2.18",
            "2.18.1.0",
            "v2.18.1",
            "2.18.1-beta",
            "2.18.1\n",
            " 2.18.1",
            "2.18.1 ",
            "kiro-cli 2.18.1",
        ] {
            assert!(
                kiro_cli_release_binding(rejected).is_none(),
                "{rejected:?} must not bind"
            );
        }
    }

    #[test]
    fn exact_release_is_permitted_and_newer_is_not() {
        let permitted = InterfaceVersion::new(KIRO_CLI_RELEASE_VERSION).expect("qualified version");
        let newer = InterfaceVersion::new("2.18.2").expect("newer version");
        let claim = kiro_acp_claim();
        assert!(claim.assess(&permitted).is_permitted());
        assert!(!claim.assess(&newer).is_permitted());
    }

    #[test]
    fn version_stdout_parser_accepts_bare_or_named_exact_release() {
        assert_eq!(
            parse_kiro_cli_version_output(b"2.18.1\n")
                .expect("exact version parses")
                .version()
                .as_str(),
            "2.18.1"
        );
        assert_eq!(
            parse_kiro_cli_version_output(b"kiro-cli 2.18.1\n")
                .expect("named version parses")
                .version()
                .as_str(),
            "2.18.1"
        );
        assert!(parse_kiro_cli_version_output(b"2.18.2\n").is_none());
        assert!(parse_kiro_cli_version_output(b"v2.18.1\n").is_none());
        assert!(parse_kiro_cli_version_output(b"kiro-cli  2.18.1\n").is_none());
    }
}

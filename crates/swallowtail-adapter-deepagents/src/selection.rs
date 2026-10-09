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
/// Latest qualified Deep Agents npm package used by ACP.
pub const DEEPAGENTS_ACP_PACKAGE_VERSION: &str = "0.1.34";
const DEEPAGENTS_ACP_PACKAGE_BASELINE_VERSION: &str = "0.1.30";

pub(crate) const DEEPAGENTS_ACP_BEHAVIOR: &str = "deepagents.acp.stdio-v1";
const MAX_VERSION_BYTES: usize = 32;

/// Parses installed `--version` stdout into a stable Deep Agents binding.
#[must_use]
pub(crate) fn parse_deepagents_acp_version_output(
    output: &[u8],
) -> Option<InterfaceVersionBinding> {
    let output = std::str::from_utf8(output).ok()?;
    let exact = output.strip_suffix('\n').unwrap_or(output);
    let exact = exact.strip_prefix("deepagents-acp ").unwrap_or(exact);
    deepagents_acp_package_binding(exact)
}

/// Parses a bare stable Deep Agents package version into its interface binding.
///
/// Compatibility is decided separately by [`deepagents_acp_claim`]. Returning a
/// binding for a stable but unqualified version lets discovery report it as
/// incompatible instead of treating it as malformed output.
#[must_use]
pub fn deepagents_acp_package_binding(value: &str) -> Option<InterfaceVersionBinding> {
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

/// Returns the qualified-only Deep Agents ACP package-window claim.
#[must_use]
pub fn deepagents_acp_claim() -> InterfaceCompatibilityClaim {
    InterfaceCompatibilityClaim::new(
        InterfaceCompatibilityClaimId::new("deepagents.acp.package-window-1")
            .expect("static Deep Agents claim id is valid"),
        axis(),
        InterfaceVersionScheme::Semantic,
        InterfaceNewerVersionPosture::QualifiedOnly,
        [InterfaceVersionSegment::new(
            InterfaceVersion::new(DEEPAGENTS_ACP_PACKAGE_BASELINE_VERSION)
                .expect("static Deep Agents baseline is valid"),
            InterfaceVersion::new(DEEPAGENTS_ACP_PACKAGE_VERSION)
                .expect("static Deep Agents ceiling is valid"),
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
    fn only_bare_stable_semver_is_bound() {
        for accepted in ["0.1.30", "0.1.31", "0.1.32", "0.1.33", "0.1.34", "0.1.35"] {
            assert!(
                deepagents_acp_package_binding(accepted).is_some(),
                "{accepted}"
            );
        }
        for rejected in [
            "",
            "0.1",
            "0.1.30.0",
            "v0.1.30",
            "0.1.30-beta",
            "0.1.30+build.1",
            "0.1.30\n",
            " 0.1.30",
            "0.1.30 ",
            "deepagents-acp 0.1.30",
        ] {
            assert!(
                deepagents_acp_package_binding(rejected).is_none(),
                "{rejected:?} must not bind"
            );
        }
    }

    #[test]
    fn maintained_window_preserves_baseline_and_rejects_outside_points() {
        let claim = deepagents_acp_claim();
        assert_eq!(claim.id().as_str(), "deepagents.acp.package-window-1");
        assert_eq!(claim.axis().as_str(), DEEPAGENTS_ACP_PACKAGE_AXIS);
        assert_eq!(
            claim.newer_version_posture(),
            InterfaceNewerVersionPosture::QualifiedOnly
        );
        assert_eq!(
            claim.baseline().as_str(),
            DEEPAGENTS_ACP_PACKAGE_BASELINE_VERSION
        );
        assert_eq!(
            claim.latest_qualified().as_str(),
            DEEPAGENTS_ACP_PACKAGE_VERSION
        );
        assert_eq!(claim.milestones().len(), 1);
        assert_eq!(claim.exclusions().count(), 0);
        let milestone = claim.milestones().next().expect("one maintained segment");
        assert_eq!(
            milestone.behavior_revision().as_str(),
            DEEPAGENTS_ACP_BEHAVIOR
        );
        assert_eq!(
            milestone.support_status(),
            InterfaceSupportStatus::Maintained
        );
        for qualified in ["0.1.30", "0.1.31", "0.1.32", "0.1.33", "0.1.34"] {
            let version = InterfaceVersion::new(qualified).expect("qualified version");
            assert!(claim.assess(&version).is_permitted(), "{qualified}");
        }
        for incompatible in ["0.1.29", "0.1.35"] {
            let version = InterfaceVersion::new(incompatible).expect("stable version");
            assert!(!claim.assess(&version).is_permitted(), "{incompatible}");
        }
    }

    #[test]
    fn version_stdout_parser_accepts_bare_or_named_stable_release() {
        assert_eq!(
            parse_deepagents_acp_version_output(b"0.1.34\n")
                .expect("current version parses")
                .version()
                .as_str(),
            "0.1.34"
        );
        assert_eq!(
            parse_deepagents_acp_version_output(b"deepagents-acp 0.1.31\n")
                .expect("named stable version parses")
                .version()
                .as_str(),
            "0.1.31"
        );
        assert_eq!(
            parse_deepagents_acp_version_output(b"0.1.35\n")
                .expect("newer stable version parses")
                .version()
                .as_str(),
            "0.1.35"
        );
        assert!(parse_deepagents_acp_version_output(b"0.1.30-beta\n").is_none());
        assert!(parse_deepagents_acp_version_output(b"v0.1.25\n").is_none());
        assert!(parse_deepagents_acp_version_output(b"deepagents-acp  0.1.25\n").is_none());
    }
}

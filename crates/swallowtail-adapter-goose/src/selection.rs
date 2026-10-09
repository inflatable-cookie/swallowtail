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
/// Latest Goose CLI release qualified for ACP.
pub const GOOSE_RELEASE_VERSION: &str = "1.53.0";
/// First Goose CLI release retained by the ACP claim.
#[cfg(test)]
const GOOSE_RELEASE_BASELINE_VERSION: &str = "1.50.1";

/// Adapter-private behavior revision covering typed provider authentication
/// failures on the ACP session/new and session/prompt requests.
pub(crate) const GOOSE_ACP_BEHAVIOR: &str = "goose.acp.stdio-v2.auth-required";
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

/// Parses installed `--version` stdout into a stable Goose release binding.
#[must_use]
pub(crate) fn parse_goose_version_output(output: &[u8]) -> Option<InterfaceVersionBinding> {
    let output = std::str::from_utf8(output).ok()?;
    let exact = output.strip_suffix('\n').unwrap_or(output);
    let exact = exact.strip_prefix("goose ").unwrap_or(exact);
    goose_release_binding(exact)
}

/// Parses a stable Goose release version into its interface binding.
///
/// The compatibility claim classifies the returned version as qualified,
/// unverified newer, or incompatible. Non-semver, prerelease, build-metadata,
/// whitespace-padded, and control-character values are rejected.
#[must_use]
pub fn goose_release_binding(value: &str) -> Option<InterfaceVersionBinding> {
    if value.is_empty() || value.len() > MAX_VERSION_BYTES || value.trim() != value {
        return None;
    }
    let parsed = semver::Version::parse(value).ok()?;
    let baseline = semver::Version::parse("1.50.1").ok()?;
    if parsed < baseline
        || !parsed.pre.is_empty()
        || !parsed.build.is_empty()
        || value.chars().any(char::is_control)
    {
        return None;
    }
    Some(InterfaceVersionBinding::new(
        axis(),
        InterfaceVersion::new(value).ok()?,
    ))
}

/// Returns the Goose ACP compatibility claim through the current stable release.
#[must_use]
pub fn goose_acp_claim() -> InterfaceCompatibilityClaim {
    InterfaceCompatibilityClaim::new(
        InterfaceCompatibilityClaimId::new("goose.acp.release-window-1")
            .expect("static Goose claim id is valid"),
        axis(),
        InterfaceVersionScheme::Semantic,
        InterfaceNewerVersionPosture::AllowUnverified,
        [
            InterfaceVersionSegment::exact(
                InterfaceVersion::new("1.50.1").expect("static Goose version is valid"),
                InterfaceBehaviorRevision::new(GOOSE_ACP_BEHAVIOR)
                    .expect("static Goose behavior is valid"),
                InterfaceSupportStatus::Maintained,
            ),
            InterfaceVersionSegment::exact(
                InterfaceVersion::new("1.51.0").expect("static Goose version is valid"),
                InterfaceBehaviorRevision::new(GOOSE_ACP_BEHAVIOR)
                    .expect("static Goose behavior is valid"),
                InterfaceSupportStatus::Maintained,
            ),
            InterfaceVersionSegment::exact(
                InterfaceVersion::new("1.52.0").expect("static Goose version is valid"),
                InterfaceBehaviorRevision::new(GOOSE_ACP_BEHAVIOR)
                    .expect("static Goose behavior is valid"),
                InterfaceSupportStatus::Maintained,
            ),
            InterfaceVersionSegment::exact(
                InterfaceVersion::new("1.53.0").expect("static Goose version is valid"),
                InterfaceBehaviorRevision::new(GOOSE_ACP_BEHAVIOR)
                    .expect("static Goose behavior is valid"),
                InterfaceSupportStatus::Maintained,
            ),
        ],
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
    fn stable_releases_are_bound_for_claim_classification() {
        for accepted in ["1.50.1", "1.51.0", "1.52.0", "1.53.0", "1.53.1"] {
            assert!(goose_release_binding(accepted).is_some(), "{accepted}");
        }
        for rejected in [
            "",
            "1.46.1",
            "1.50",
            "1.50.1.0",
            "v1.50.1",
            "1.50.1-beta",
            "1.50.1+build.1",
            "1.50.1\n",
            " 1.50.1",
            "1.50.1 ",
            "goose 1.50.1",
            "1.50.1\u{7f}",
        ] {
            assert!(
                goose_release_binding(rejected).is_none(),
                "{rejected:?} must not bind"
            );
        }
    }

    #[test]
    fn published_points_gaps_and_later_stable_keep_their_classification() {
        let claim = goose_acp_claim();
        assert_eq!(claim.baseline().as_str(), GOOSE_RELEASE_BASELINE_VERSION);
        assert_eq!(claim.latest_qualified().as_str(), GOOSE_RELEASE_VERSION);
        assert_eq!(claim.milestones().len(), 4);
        assert_eq!(
            claim.newer_version_posture(),
            InterfaceNewerVersionPosture::AllowUnverified
        );
        for qualified in ["1.50.1", "1.51.0", "1.52.0", "1.53.0"] {
            let assessment = claim.assess(&InterfaceVersion::new(qualified).expect("version"));
            let swallowtail_core::InterfaceCompatibilityAssessment::Qualified(matched) = assessment
            else {
                panic!("{qualified} must remain qualified");
            };
            assert_eq!(matched.behavior_revision().as_str(), GOOSE_ACP_BEHAVIOR);
            assert_eq!(matched.support_status(), InterfaceSupportStatus::Maintained);
        }
        for gap in ["1.50.2", "1.51.1", "1.52.1"] {
            assert!(
                matches!(
                    claim.assess(&InterfaceVersion::new(gap).expect("version")),
                    swallowtail_core::InterfaceCompatibilityAssessment::Incompatible
                ),
                "{gap} must stay outside the exact published points"
            );
        }
        assert!(matches!(
            claim.assess(&InterfaceVersion::new("1.53.1").expect("version")),
            swallowtail_core::InterfaceCompatibilityAssessment::UnverifiedNewer(_)
        ));
        assert!(matches!(
            claim.assess(&InterfaceVersion::new("1.49.9").expect("version")),
            swallowtail_core::InterfaceCompatibilityAssessment::Incompatible
        ));
    }

    #[test]
    fn version_stdout_parser_accepts_bare_or_named_exact_release() {
        assert_eq!(
            parse_goose_version_output(b"1.53.0\n")
                .expect("exact version parses")
                .version()
                .as_str(),
            "1.53.0"
        );
        assert_eq!(
            parse_goose_version_output(b"goose 1.53.1\n")
                .expect("named version parses")
                .version()
                .as_str(),
            "1.53.1"
        );
        assert!(parse_goose_version_output(b"1.53.1-beta\n").is_none());
        assert!(parse_goose_version_output(b"v1.50.1\n").is_none());
        assert!(parse_goose_version_output(b"goose  1.50.1\n").is_none());
    }
}

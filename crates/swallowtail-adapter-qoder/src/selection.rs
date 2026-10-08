use swallowtail_core::{
    InterfaceBehaviorRevision, InterfaceCompatibilityClaim, InterfaceCompatibilityClaimId,
    InterfaceNewerVersionPosture, InterfaceSupportStatus, InterfaceVersion, InterfaceVersionAxis,
    InterfaceVersionBinding, InterfaceVersionScheme, InterfaceVersionSegment, PreflightPlan,
};
use swallowtail_runtime::RuntimeFailure;

use crate::failure::failure;

/// Unambiguous executable name used for Qoder CLI discovery.
pub const QODER_EXECUTABLE_NAME: &str = "qodercli";
/// Opaque npm package-version axis for Qoder headless.
pub const QODER_PACKAGE_AXIS: &str = "qoder.package";
/// Latest qualified Qoder npm package version used by headless.
pub const QODER_PACKAGE_VERSION: &str = "1.1.65";

/// Adapter-private behavior revision for the deliberate AgentLoop bound.
pub(crate) const QODER_HEADLESS_BEHAVIOR: &str = "qoder.headless.stdio-stream-json-v2";
const QODER_PACKAGE_BASELINE_VERSION: &str = "1.1.54";
const MAX_VERSION_BYTES: usize = 32;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct QoderPlanSelection {
    version: InterfaceVersion,
}

impl QoderPlanSelection {
    #[allow(dead_code)]
    pub(crate) const fn version(&self) -> &InterfaceVersion {
        &self.version
    }
}

/// Parses installed `--version` stdout into the exact qualified Qoder binding.
#[must_use]
pub(crate) fn parse_qoder_version_output(output: &[u8]) -> Option<InterfaceVersionBinding> {
    let output = std::str::from_utf8(output).ok()?;
    let exact = output.strip_suffix('\n').unwrap_or(output);
    let exact = exact.strip_prefix("qodercli ").unwrap_or(exact);
    let exact = exact.strip_prefix("qoder ").unwrap_or(exact);
    qoder_package_binding(exact)
}

/// Parses one stable Qoder package version in the qualified headless window.
///
/// Returns `None` for versions outside the qualified window or any decorated
/// semantic version, so observed CLI output can never panic a caller.
#[must_use]
pub fn qoder_package_binding(value: &str) -> Option<InterfaceVersionBinding> {
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
    let version = InterfaceVersion::new(value).ok()?;
    if !qoder_headless_claim().assess(&version).is_permitted() {
        return None;
    }
    Some(InterfaceVersionBinding::new(axis(), version))
}

/// Returns the qualified-only maintained Qoder headless package window.
#[must_use]
pub fn qoder_headless_claim() -> InterfaceCompatibilityClaim {
    InterfaceCompatibilityClaim::new(
        InterfaceCompatibilityClaimId::new("qoder.headless.package-window-2")
            .expect("static Qoder claim id is valid"),
        axis(),
        InterfaceVersionScheme::Semantic,
        InterfaceNewerVersionPosture::QualifiedOnly,
        [InterfaceVersionSegment::new(
            InterfaceVersion::new(QODER_PACKAGE_BASELINE_VERSION)
                .expect("static Qoder baseline is valid"),
            InterfaceVersion::new(QODER_PACKAGE_VERSION).expect("static Qoder version is valid"),
            InterfaceBehaviorRevision::new(QODER_HEADLESS_BEHAVIOR)
                .expect("static Qoder behavior is valid"),
            InterfaceSupportStatus::Maintained,
        )],
        [],
    )
    .expect("static Qoder claim is valid")
}

pub(crate) fn select_qoder_headless_plan(
    plan: &PreflightPlan,
) -> Result<QoderPlanSelection, RuntimeFailure> {
    let claim = qoder_headless_claim();
    let mut bindings = plan
        .interface_versions()
        .filter(|binding| binding.axis() == claim.axis());
    let binding = bindings.next().ok_or_else(|| {
        failure(
            "swallowtail.qoder.headless.version_missing",
            "Qoder headless plan is missing its exact package version",
        )
    })?;
    if bindings.next().is_some() {
        return Err(failure(
            "swallowtail.qoder.headless.version_ambiguous",
            "Qoder headless plan contains more than one package version",
        ));
    }
    let assessment = claim.assess(binding.version());
    if assessment != plan.assess_interface_version(binding)
        || !assessment.is_permitted()
        || assessment
            .behavior_revision()
            .is_none_or(|revision| revision.as_str() != QODER_HEADLESS_BEHAVIOR)
    {
        return Err(failure(
            "swallowtail.qoder.headless.version_incompatible",
            "Qoder package version is incompatible with the headless driver",
        ));
    }
    Ok(QoderPlanSelection {
        version: binding.version().clone(),
    })
}

fn axis() -> InterfaceVersionAxis {
    InterfaceVersionAxis::new(QODER_PACKAGE_AXIS).expect("static Qoder package axis is valid")
}

#[cfg(test)]
mod tests {
    use super::*;
    use swallowtail_core::{InterfaceNewerVersionPosture, InterfaceSupportStatus};

    #[test]
    fn every_published_stable_hop_in_the_qualified_window_is_bound() {
        let versions = [
            "1.1.54", "1.1.55", "1.1.56", "1.1.57", "1.1.58", "1.1.59", "1.1.60", "1.1.61",
            "1.1.62", "1.1.63", "1.1.64", "1.1.65",
        ];
        let claim = qoder_headless_claim();
        assert_eq!(QODER_PACKAGE_VERSION, "1.1.65");
        for version in versions {
            let binding = qoder_package_binding(version).expect("qualified stable version");
            assert_eq!(binding.axis().as_str(), QODER_PACKAGE_AXIS);
            assert_eq!(binding.version().as_str(), version);
            assert!(claim.assess(binding.version()).is_permitted(), "{version}");
        }

        for rejected in [
            "",
            "1.1.24",
            "1.1.25",
            "1.1.53",
            "1.1.66",
            "1.1",
            "1.1.25.0",
            "v1.1.25",
            "1.1.25-beta",
            "1.1.65-beta.1",
            "1.1.65+build.1",
            "1.1.54\n",
            " 1.1.54",
            "1.1.54 ",
            "qodercli 1.1.54",
        ] {
            assert!(
                qoder_package_binding(rejected).is_none(),
                "{rejected:?} must not bind"
            );
        }
    }

    #[test]
    fn qualified_window_is_permitted_and_outside_points_are_not() {
        let claim = qoder_headless_claim();
        assert_eq!(claim.id().as_str(), "qoder.headless.package-window-2");
        assert_eq!(
            claim.newer_version_posture(),
            InterfaceNewerVersionPosture::QualifiedOnly
        );
        assert_eq!(claim.baseline().as_str(), "1.1.54");
        assert_eq!(claim.latest_qualified().as_str(), "1.1.65");
        assert!(claim.exclusions().next().is_none());
        assert_eq!(claim.milestones().len(), 1);
        let segment = claim.milestones().next().expect("maintained window");
        assert_eq!(segment.minimum().as_str(), "1.1.54");
        assert_eq!(segment.maximum().as_str(), "1.1.65");
        assert_eq!(
            segment.behavior_revision().as_str(),
            QODER_HEADLESS_BEHAVIOR
        );
        assert_eq!(segment.support_status(), InterfaceSupportStatus::Maintained);
        assert!(
            claim
                .assess(&InterfaceVersion::new("1.1.54").expect("baseline"))
                .is_permitted()
        );
        assert!(
            claim
                .assess(&InterfaceVersion::new(QODER_PACKAGE_VERSION).expect("latest"))
                .is_permitted()
        );
        assert!(
            !claim
                .assess(&InterfaceVersion::new("1.1.53").expect("below window"))
                .is_permitted()
        );
        assert!(
            !claim
                .assess(&InterfaceVersion::new("1.1.66").expect("above window"))
                .is_permitted()
        );
    }

    #[test]
    fn version_stdout_parser_accepts_qualified_bare_or_named_package() {
        assert_eq!(
            parse_qoder_version_output(b"1.1.65\n")
                .expect("exact version parses")
                .version()
                .as_str(),
            "1.1.65"
        );
        assert_eq!(
            parse_qoder_version_output(b"qodercli 1.1.54\n")
                .expect("named version parses")
                .version()
                .as_str(),
            "1.1.54"
        );
        assert_eq!(
            parse_qoder_version_output(b"qoder 1.1.65\n")
                .expect("dispatcher-named version parses")
                .version()
                .as_str(),
            "1.1.65"
        );
        assert!(parse_qoder_version_output(b"1.1.53\n").is_none());
        assert!(parse_qoder_version_output(b"1.1.66\n").is_none());
        assert!(parse_qoder_version_output(b"v1.1.65\n").is_none());
        assert!(parse_qoder_version_output(b"qodercli  1.1.54\n").is_none());
    }
}

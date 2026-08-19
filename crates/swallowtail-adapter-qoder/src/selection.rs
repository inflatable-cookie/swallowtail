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
/// Exact qualified Qoder npm package used by headless.
pub const QODER_PACKAGE_VERSION: &str = "1.1.25";

pub(crate) const QODER_HEADLESS_BEHAVIOR: &str = "qoder.headless.stdio-stream-json-v1";
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

/// Parses the one qualified exact Qoder package version into its interface binding.
///
/// Returns `None` for anything other than the exact qualified package text, so
/// observed CLI output can never panic a caller.
#[must_use]
pub fn qoder_package_binding(value: &str) -> Option<InterfaceVersionBinding> {
    if value != QODER_PACKAGE_VERSION
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

/// Returns the qualified-only exact Qoder headless protocol claim.
#[must_use]
pub fn qoder_headless_claim() -> InterfaceCompatibilityClaim {
    InterfaceCompatibilityClaim::new(
        InterfaceCompatibilityClaimId::new("qoder.headless.package-window-1")
            .expect("static Qoder claim id is valid"),
        axis(),
        InterfaceVersionScheme::Semantic,
        InterfaceNewerVersionPosture::QualifiedOnly,
        [InterfaceVersionSegment::exact(
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

    #[test]
    fn only_the_exact_qualified_package_is_bound() {
        assert!(qoder_package_binding(QODER_PACKAGE_VERSION).is_some());
        for rejected in [
            "",
            "1.1.24",
            "1.1.26",
            "1.1",
            "1.1.25.0",
            "v1.1.25",
            "1.1.25-beta",
            "1.1.25\n",
            " 1.1.25",
            "1.1.25 ",
            "qodercli 1.1.25",
        ] {
            assert!(
                qoder_package_binding(rejected).is_none(),
                "{rejected:?} must not bind"
            );
        }
    }

    #[test]
    fn exact_package_is_permitted_and_newer_is_not() {
        let permitted = InterfaceVersion::new(QODER_PACKAGE_VERSION).expect("qualified version");
        let newer = InterfaceVersion::new("1.1.26").expect("newer version");
        let claim = qoder_headless_claim();
        assert!(claim.assess(&permitted).is_permitted());
        assert!(!claim.assess(&newer).is_permitted());
    }

    #[test]
    fn version_stdout_parser_accepts_bare_or_named_exact_package() {
        assert_eq!(
            parse_qoder_version_output(b"1.1.25\n")
                .expect("exact version parses")
                .version()
                .as_str(),
            "1.1.25"
        );
        assert_eq!(
            parse_qoder_version_output(b"qodercli 1.1.25\n")
                .expect("named version parses")
                .version()
                .as_str(),
            "1.1.25"
        );
        assert_eq!(
            parse_qoder_version_output(b"qoder 1.1.25\n")
                .expect("dispatcher-named version parses")
                .version()
                .as_str(),
            "1.1.25"
        );
        assert!(parse_qoder_version_output(b"1.1.26\n").is_none());
        assert!(parse_qoder_version_output(b"v1.1.25\n").is_none());
        assert!(parse_qoder_version_output(b"qodercli  1.1.25\n").is_none());
    }
}

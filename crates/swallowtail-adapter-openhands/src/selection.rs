use swallowtail_core::{
    InterfaceBehaviorRevision, InterfaceCompatibilityClaim, InterfaceCompatibilityClaimId,
    InterfaceNewerVersionPosture, InterfaceSupportStatus, InterfaceVersion, InterfaceVersionAxis,
    InterfaceVersionBinding, InterfaceVersionScheme, InterfaceVersionSegment, PreflightPlan,
};
use swallowtail_runtime::RuntimeFailure;

use crate::failure::failure;

/// Opaque PyPI package-version axis for OpenHands Agent Server.
pub const OPENHANDS_PACKAGE_AXIS: &str = "openhands-agent-server.package";
/// Exact qualified OpenHands Agent Server package used by this route.
pub const OPENHANDS_PACKAGE_VERSION: &str = "1.42.1";

pub(crate) const OPENHANDS_AGENT_SERVER_BEHAVIOR: &str =
    "openhands.agent-server.loopback-http-ws-v1";
const MAX_VERSION_BYTES: usize = 32;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct OpenHandsPlanSelection {
    version: InterfaceVersion,
}

impl OpenHandsPlanSelection {
    #[allow(dead_code)]
    pub(crate) const fn version(&self) -> &InterfaceVersion {
        &self.version
    }
}

/// Parses installed package-version stdout into the exact qualified binding.
#[must_use]
pub(crate) fn parse_openhands_version_output(output: &[u8]) -> Option<InterfaceVersionBinding> {
    let output = std::str::from_utf8(output).ok()?;
    let exact = output.strip_suffix('\n').unwrap_or(output);
    let exact = exact
        .strip_prefix("openhands-agent-server ")
        .unwrap_or(exact);
    openhands_package_binding(exact)
}

/// Parses the one qualified exact OpenHands package version into its binding.
///
/// Returns `None` for anything other than the exact qualified package text.
#[must_use]
pub fn openhands_package_binding(value: &str) -> Option<InterfaceVersionBinding> {
    if value != OPENHANDS_PACKAGE_VERSION
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

/// Returns the qualified-only exact OpenHands Agent Server protocol claim.
#[must_use]
pub fn openhands_agent_server_claim() -> InterfaceCompatibilityClaim {
    InterfaceCompatibilityClaim::new(
        InterfaceCompatibilityClaimId::new("openhands.agent-server.package-window-1")
            .expect("static OpenHands claim id is valid"),
        axis(),
        InterfaceVersionScheme::Semantic,
        InterfaceNewerVersionPosture::QualifiedOnly,
        [InterfaceVersionSegment::exact(
            InterfaceVersion::new(OPENHANDS_PACKAGE_VERSION)
                .expect("static OpenHands version is valid"),
            InterfaceBehaviorRevision::new(OPENHANDS_AGENT_SERVER_BEHAVIOR)
                .expect("static OpenHands behavior is valid"),
            InterfaceSupportStatus::Maintained,
        )],
        [],
    )
    .expect("static OpenHands claim is valid")
}

pub(crate) fn select_openhands_agent_server_plan(
    plan: &PreflightPlan,
) -> Result<OpenHandsPlanSelection, RuntimeFailure> {
    let claim = openhands_agent_server_claim();
    let mut bindings = plan
        .interface_versions()
        .filter(|binding| binding.axis() == claim.axis());
    let binding = bindings.next().ok_or_else(|| {
        failure(
            "swallowtail.openhands.agent_server.version_missing",
            "OpenHands Agent Server plan is missing its exact package version",
        )
    })?;
    if bindings.next().is_some() {
        return Err(failure(
            "swallowtail.openhands.agent_server.version_ambiguous",
            "OpenHands Agent Server plan contains more than one package version",
        ));
    }
    let assessment = claim.assess(binding.version());
    if assessment != plan.assess_interface_version(binding)
        || !assessment.is_permitted()
        || assessment
            .behavior_revision()
            .is_none_or(|revision| revision.as_str() != OPENHANDS_AGENT_SERVER_BEHAVIOR)
    {
        return Err(failure(
            "swallowtail.openhands.agent_server.version_incompatible",
            "OpenHands package version is incompatible with the Agent Server driver",
        ));
    }
    Ok(OpenHandsPlanSelection {
        version: binding.version().clone(),
    })
}

fn axis() -> InterfaceVersionAxis {
    InterfaceVersionAxis::new(OPENHANDS_PACKAGE_AXIS)
        .expect("static OpenHands package axis is valid")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_the_exact_qualified_package_is_bound() {
        assert!(openhands_package_binding(OPENHANDS_PACKAGE_VERSION).is_some());
        for rejected in [
            "",
            "1.42.0",
            "1.42.2",
            "1.42",
            "1.42.1.0",
            "v1.42.1",
            "1.42.1-beta",
            "1.42.1\n",
            " 1.42.1",
            "1.42.1 ",
            "openhands-agent-server 1.42.1",
        ] {
            assert!(
                openhands_package_binding(rejected).is_none(),
                "{rejected:?} must not bind"
            );
        }
    }

    #[test]
    fn exact_package_is_permitted_and_newer_is_not() {
        let permitted =
            InterfaceVersion::new(OPENHANDS_PACKAGE_VERSION).expect("qualified version");
        let newer = InterfaceVersion::new("1.42.2").expect("newer version");
        let claim = openhands_agent_server_claim();
        assert!(claim.assess(&permitted).is_permitted());
        assert!(!claim.assess(&newer).is_permitted());
    }

    #[test]
    fn version_stdout_parser_accepts_bare_or_named_exact_package() {
        assert_eq!(
            parse_openhands_version_output(b"1.42.1\n")
                .expect("exact version parses")
                .version()
                .as_str(),
            "1.42.1"
        );
        assert_eq!(
            parse_openhands_version_output(b"openhands-agent-server 1.42.1\n")
                .expect("named version parses")
                .version()
                .as_str(),
            "1.42.1"
        );
        assert!(parse_openhands_version_output(b"1.42.2\n").is_none());
        assert!(parse_openhands_version_output(b"v1.42.1\n").is_none());
    }
}

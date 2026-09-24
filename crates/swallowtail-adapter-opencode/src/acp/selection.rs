use swallowtail_core::{
    InterfaceBehaviorRevision, InterfaceCompatibilityClaim, InterfaceCompatibilityClaimId,
    InterfaceNewerVersionPosture, InterfaceSupportStatus, InterfaceVersion, InterfaceVersionAxis,
    InterfaceVersionBinding, InterfaceVersionScheme, InterfaceVersionSegment, PreflightPlan,
};
use swallowtail_runtime::RuntimeFailure;

use super::failure::failure;

/// Unambiguous executable name used for OpenCode ACP discovery.
pub const OPENCODE_ACP_EXECUTABLE_NAME: &str = "opencode";
/// Executable-release axis for `opencode.acp`, kept unflattened from `opencode.server`.
pub const OPENCODE_ACP_AXIS: &str = "opencode.executable";
/// Oldest accepted OpenCode ACP executable.
pub const OPENCODE_ACP_BASELINE_VERSION: &str = "1.18.18";
/// Newest compiled OpenCode ACP executable.
pub const OPENCODE_ACP_LATEST_QUALIFIED_VERSION: &str = "1.18.32";
/// Last version of the accepted-but-older `v1` behavior.
pub const OPENCODE_ACP_V1_MAXIMUM_VERSION: &str = "1.18.30";
/// First version of the compiled `v2` behavior.
pub const OPENCODE_ACP_V2_MINIMUM_VERSION: &str = "1.18.31";

pub(crate) const OPENCODE_ACP_BEHAVIOR_V1: &str = "opencode.acp-v1.client-mcp-servers-v1";
pub(crate) const OPENCODE_ACP_BEHAVIOR_V2: &str = "opencode.acp-v1.client-mcp-servers-v2";
const MAX_VERSION_BYTES: usize = 64;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct OpenCodeAcpPlanSelection {
    version: InterfaceVersion,
}

impl OpenCodeAcpPlanSelection {
    pub(crate) const fn version(&self) -> &InterfaceVersion {
        &self.version
    }
}

/// Parses installed `--version` stdout into an OpenCode ACP executable binding.
#[must_use]
pub(crate) fn parse_opencode_acp_version_output(output: &[u8]) -> Option<InterfaceVersionBinding> {
    let output = std::str::from_utf8(output).ok()?;
    let exact = output.strip_suffix('\n').unwrap_or(output);
    let exact = exact.strip_prefix("opencode ").unwrap_or(exact);
    opencode_acp_binding(exact)
}

/// Parses a stable OpenCode executable version into its ACP interface binding.
#[must_use]
pub fn opencode_acp_binding(value: &str) -> Option<InterfaceVersionBinding> {
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

/// Returns the OpenCode ACP compatibility claim frozen by Research 337.
#[must_use]
pub fn opencode_acp_claim() -> InterfaceCompatibilityClaim {
    InterfaceCompatibilityClaim::new(
        InterfaceCompatibilityClaimId::new("opencode.acp.executable-window-1")
            .expect("static OpenCode ACP claim id is valid"),
        axis(),
        InterfaceVersionScheme::Semantic,
        InterfaceNewerVersionPosture::AllowUnverified,
        [
            InterfaceVersionSegment::new(
                version(OPENCODE_ACP_BASELINE_VERSION).expect("static OpenCode version is valid"),
                version(OPENCODE_ACP_V1_MAXIMUM_VERSION).expect("static OpenCode version is valid"),
                behavior(OPENCODE_ACP_BEHAVIOR_V1),
                InterfaceSupportStatus::Deprecated,
            ),
            InterfaceVersionSegment::new(
                version(OPENCODE_ACP_V2_MINIMUM_VERSION).expect("static OpenCode version is valid"),
                version(OPENCODE_ACP_LATEST_QUALIFIED_VERSION)
                    .expect("static OpenCode version is valid"),
                behavior(OPENCODE_ACP_BEHAVIOR_V2),
                InterfaceSupportStatus::Maintained,
            ),
        ],
        [],
    )
    .expect("static OpenCode ACP claim is valid")
}

pub(crate) fn select_opencode_acp_plan(
    plan: &PreflightPlan,
) -> Result<OpenCodeAcpPlanSelection, RuntimeFailure> {
    let claim = opencode_acp_claim();
    let mut bindings = plan
        .interface_versions()
        .filter(|binding| binding.axis() == claim.axis());
    let binding = bindings.next().ok_or_else(|| {
        failure(
            "swallowtail.opencode.acp.version_missing",
            "OpenCode ACP plan is missing its exact executable version",
        )
    })?;
    if bindings.next().is_some() {
        return Err(failure(
            "swallowtail.opencode.acp.version_ambiguous",
            "OpenCode ACP plan contains more than one executable version",
        ));
    }
    let assessment = claim.assess(binding.version());
    if assessment != plan.assess_interface_version(binding) || !assessment.is_permitted() {
        return Err(failure(
            "swallowtail.opencode.acp.version_incompatible",
            "OpenCode executable version is incompatible with the ACP driver",
        ));
    }
    let revision = assessment.behavior_revision().ok_or_else(|| {
        failure(
            "swallowtail.opencode.acp.behavior_incompatible",
            "OpenCode ACP behavior is not mapped by this driver",
        )
    })?;
    if revision.as_str() != OPENCODE_ACP_BEHAVIOR_V1
        && revision.as_str() != OPENCODE_ACP_BEHAVIOR_V2
    {
        return Err(failure(
            "swallowtail.opencode.acp.behavior_incompatible",
            "OpenCode ACP behavior is not mapped by this driver",
        ));
    }
    Ok(OpenCodeAcpPlanSelection {
        version: binding.version().clone(),
    })
}

fn axis() -> InterfaceVersionAxis {
    InterfaceVersionAxis::new(OPENCODE_ACP_AXIS).expect("static OpenCode ACP axis is valid")
}

fn version(value: &str) -> Option<InterfaceVersion> {
    InterfaceVersion::new(value).ok()
}

fn behavior(value: &str) -> InterfaceBehaviorRevision {
    InterfaceBehaviorRevision::new(value).expect("static OpenCode ACP behavior is valid")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn well_formed_semver_binds_and_garbage_does_not() {
        assert!(opencode_acp_binding(OPENCODE_ACP_BASELINE_VERSION).is_some());
        assert!(opencode_acp_binding(OPENCODE_ACP_LATEST_QUALIFIED_VERSION).is_some());
        for rejected in [
            "",
            "v1.18.32",
            "1.18.32-beta",
            "1.18.32\n",
            " 1.18.32",
            "1.18.32 ",
            "opencode 1.18.32",
        ] {
            assert!(
                opencode_acp_binding(rejected).is_none(),
                "{rejected:?} must not bind"
            );
        }
    }

    #[test]
    fn claim_accepts_v1_as_older_and_compiles_v2() {
        let claim = opencode_acp_claim();
        let v1 = InterfaceVersion::new("1.18.18").expect("v1");
        let v1_max = InterfaceVersion::new("1.18.30").expect("v1 max");
        let v2 = InterfaceVersion::new("1.18.31").expect("v2");
        let latest = InterfaceVersion::new("1.18.32").expect("latest");
        let newer = InterfaceVersion::new("1.18.33").expect("unverified");
        assert_eq!(
            claim
                .assess(&v1)
                .behavior_revision()
                .map(|value| value.as_str()),
            Some(OPENCODE_ACP_BEHAVIOR_V1)
        );
        assert_eq!(
            claim
                .assess(&v1_max)
                .behavior_revision()
                .map(|value| value.as_str()),
            Some(OPENCODE_ACP_BEHAVIOR_V1)
        );
        assert_eq!(
            claim
                .assess(&v2)
                .behavior_revision()
                .map(|value| value.as_str()),
            Some(OPENCODE_ACP_BEHAVIOR_V2)
        );
        assert_eq!(
            claim
                .assess(&latest)
                .behavior_revision()
                .map(|value| value.as_str()),
            Some(OPENCODE_ACP_BEHAVIOR_V2)
        );
        assert!(claim.assess(&newer).is_permitted());
        assert_eq!(
            claim
                .assess(&newer)
                .behavior_revision()
                .map(|value| value.as_str()),
            Some(OPENCODE_ACP_BEHAVIOR_V2)
        );
        assert_eq!(claim.id().as_str(), "opencode.acp.executable-window-1");
        assert_ne!(claim.axis().as_str(), "opencode.server");
    }

    #[test]
    fn version_stdout_parser_accepts_bare_or_named_release() {
        assert_eq!(
            parse_opencode_acp_version_output(b"1.18.32\n")
                .expect("exact version parses")
                .version()
                .as_str(),
            "1.18.32"
        );
        assert_eq!(
            parse_opencode_acp_version_output(b"opencode 1.18.18\n")
                .expect("named version parses")
                .version()
                .as_str(),
            "1.18.18"
        );
        assert!(parse_opencode_acp_version_output(b"v1.18.32\n").is_none());
        assert!(parse_opencode_acp_version_output(b"opencode  1.18.32\n").is_none());
    }
}

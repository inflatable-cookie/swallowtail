use swallowtail_core::{
    InterfaceBehaviorRevision, InterfaceCompatibilityClaim, InterfaceCompatibilityClaimId,
    InterfaceNewerVersionPosture, InterfaceSupportStatus, InterfaceVersion, InterfaceVersionAxis,
    InterfaceVersionBinding, InterfaceVersionScheme, InterfaceVersionSegment, PreflightPlan,
};
use swallowtail_runtime::RuntimeFailure;

use crate::failure::failure;

pub const GEMINI_CLI_ACP_AXIS: &str = "gemini-cli.acp-agent";
pub const GEMINI_CLI_ACP_BASELINE_VERSION: &str = "0.51.0";
pub const GEMINI_CLI_ACP_LATEST_QUALIFIED_VERSION: &str = "0.51.0";

const BASELINE_BEHAVIOR: &str = "gemini-cli.acp.v0.51.0";
const MAX_VERSION_BYTES: usize = 64;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct GeminiPlanSelection {
    version: InterfaceVersion,
}

impl GeminiPlanSelection {
    pub(crate) const fn version(&self) -> &InterfaceVersion {
        &self.version
    }
}

#[must_use]
pub fn gemini_cli_acp_binding(value: &str) -> Option<InterfaceVersionBinding> {
    if value.is_empty()
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

#[must_use]
pub fn gemini_cli_acp_claim() -> InterfaceCompatibilityClaim {
    InterfaceCompatibilityClaim::new(
        InterfaceCompatibilityClaimId::new("gemini-cli.acp.range-v1")
            .expect("static Gemini CLI claim id is valid"),
        axis(),
        InterfaceVersionScheme::Semantic,
        InterfaceNewerVersionPosture::AllowUnverified,
        [InterfaceVersionSegment::new(
            version(GEMINI_CLI_ACP_BASELINE_VERSION),
            version(GEMINI_CLI_ACP_LATEST_QUALIFIED_VERSION),
            InterfaceBehaviorRevision::new(BASELINE_BEHAVIOR)
                .expect("static Gemini behavior revision is valid"),
            InterfaceSupportStatus::Maintained,
        )],
        [],
    )
    .expect("static Gemini CLI compatibility claim is valid")
}

pub(crate) fn select_gemini_plan(
    plan: &PreflightPlan,
) -> Result<GeminiPlanSelection, RuntimeFailure> {
    let claim = gemini_cli_acp_claim();
    let mut bindings = plan
        .interface_versions()
        .filter(|binding| binding.axis() == claim.axis());
    let binding = bindings.next().ok_or_else(|| {
        failure(
            "swallowtail.gemini.acp.version_missing",
            "Gemini ACP plan is missing its exact CLI version",
        )
    })?;
    if bindings.next().is_some() {
        return Err(failure(
            "swallowtail.gemini.acp.version_ambiguous",
            "Gemini ACP plan contains more than one CLI version",
        ));
    }
    let assessment = claim.assess(binding.version());
    if assessment != plan.assess_interface_version(binding) || !assessment.is_permitted() {
        return Err(failure(
            "swallowtail.gemini.acp.version_incompatible",
            "Gemini ACP CLI version is incompatible with this driver",
        ));
    }
    if assessment
        .behavior_revision()
        .is_none_or(|revision| revision.as_str() != BASELINE_BEHAVIOR)
    {
        return Err(failure(
            "swallowtail.gemini.acp.behavior_incompatible",
            "Gemini ACP behavior is not mapped by this driver",
        ));
    }
    Ok(GeminiPlanSelection {
        version: binding.version().clone(),
    })
}

fn axis() -> InterfaceVersionAxis {
    InterfaceVersionAxis::new(GEMINI_CLI_ACP_AXIS).expect("static Gemini CLI axis is valid")
}

fn version(value: &str) -> InterfaceVersion {
    InterfaceVersion::new(value).expect("static Gemini CLI version is valid")
}

#[cfg(test)]
mod tests {
    use super::{GEMINI_CLI_ACP_AXIS, gemini_cli_acp_binding, gemini_cli_acp_claim};
    use swallowtail_core::{InterfaceCompatibilityAssessment, InterfaceVersion};

    #[test]
    fn exact_release_is_qualified_and_newer_stable_is_visible() {
        let claim = gemini_cli_acp_claim();
        assert!(claim.supports(&version("0.51.0")));
        assert!(!claim.permits(&version("0.50.0")));
        assert!(!claim.permits(&version("0.51.0-rc.1")));
        assert!(matches!(
            claim.assess(&version("0.52.0")),
            InterfaceCompatibilityAssessment::UnverifiedNewer(_)
        ));
    }

    #[test]
    fn binding_accepts_only_one_exact_semantic_version() {
        assert_eq!(
            gemini_cli_acp_binding("0.51.0")
                .expect("version binds")
                .axis()
                .as_str(),
            GEMINI_CLI_ACP_AXIS
        );
        for rejected in ["", " 0.51.0", "gemini 0.51.0", "0.51.0 extra", "latest"] {
            assert!(gemini_cli_acp_binding(rejected).is_none());
        }
    }

    fn version(value: &str) -> InterfaceVersion {
        InterfaceVersion::new(value).expect("fixture version is valid")
    }
}

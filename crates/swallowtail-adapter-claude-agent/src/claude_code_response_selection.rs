use swallowtail_core::{
    InterfaceBehaviorRevision, InterfaceCompatibilityClaim, InterfaceCompatibilityClaimId,
    InterfaceNewerVersionPosture, InterfaceSupportStatus, InterfaceVersion, InterfaceVersionAxis,
    InterfaceVersionBinding, InterfaceVersionScheme, InterfaceVersionSegment, PreflightPlan,
};
use swallowtail_runtime::RuntimeFailure;

use crate::failure::failure;

/// CLI-version axis for tool-free Claude Code response-only runs.
pub const CLAUDE_CODE_RESPONSE_ONLY_AXIS: &str = "claude-code.response-only-stream-json";
/// Exact Claude Code version qualified for response-only runs.
pub const CLAUDE_CODE_RESPONSE_ONLY_VERSION: &str = "2.1.228";

const RESPONSE_ONLY_BEHAVIOR: &str = "claude-code.response-only.stream-json.v1";
const MAX_VERSION_BYTES: usize = 64;

#[must_use]
/// Parses the exact response-only Claude Code version into its interface binding.
pub fn claude_code_response_only_binding(value: &str) -> Option<InterfaceVersionBinding> {
    if value != CLAUDE_CODE_RESPONSE_ONLY_VERSION
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

#[must_use]
/// Returns the qualified-only exact response-only compatibility claim.
pub fn claude_code_response_only_claim() -> InterfaceCompatibilityClaim {
    InterfaceCompatibilityClaim::new(
        InterfaceCompatibilityClaimId::new("claude-code.response-only.window-1")
            .expect("static Claude Code response-only claim id is valid"),
        axis(),
        InterfaceVersionScheme::Semantic,
        InterfaceNewerVersionPosture::QualifiedOnly,
        [InterfaceVersionSegment::exact(
            InterfaceVersion::new(CLAUDE_CODE_RESPONSE_ONLY_VERSION)
                .expect("static Claude Code response-only version is valid"),
            InterfaceBehaviorRevision::new(RESPONSE_ONLY_BEHAVIOR)
                .expect("static Claude Code response-only behavior is valid"),
            InterfaceSupportStatus::Maintained,
        )],
        [],
    )
    .expect("static Claude Code response-only claim is valid")
}

pub(crate) fn select_response_only_plan(plan: &PreflightPlan) -> Result<(), RuntimeFailure> {
    let claim = claude_code_response_only_claim();
    let mut bindings = plan
        .interface_versions()
        .filter(|binding| binding.axis() == claim.axis());
    let binding = bindings.next().ok_or_else(|| {
        failure(
            "swallowtail.claude_code.response_only.version_missing",
            "Claude Code response-only plan is missing its exact CLI version",
        )
    })?;
    if bindings.next().is_some() {
        return Err(failure(
            "swallowtail.claude_code.response_only.version_ambiguous",
            "Claude Code response-only plan contains more than one CLI version",
        ));
    }
    let assessment = claim.assess(binding.version());
    if assessment != plan.assess_interface_version(binding)
        || !assessment.is_permitted()
        || assessment
            .behavior_revision()
            .is_none_or(|revision| revision.as_str() != RESPONSE_ONLY_BEHAVIOR)
    {
        return Err(failure(
            "swallowtail.claude_code.response_only.version_incompatible",
            "Claude Code CLI version is incompatible with the response-only driver",
        ));
    }
    Ok(())
}

fn axis() -> InterfaceVersionAxis {
    InterfaceVersionAxis::new(CLAUDE_CODE_RESPONSE_ONLY_AXIS)
        .expect("static Claude Code response-only axis is valid")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_exact_2_1_228_is_qualified() {
        assert!(claude_code_response_only_binding("2.1.228").is_some());
        for rejected in ["", "2.1.220", "2.1.227", "2.1.229", "v2.1.228", "2.1.228\n"] {
            assert!(claude_code_response_only_binding(rejected).is_none());
        }
        let claim = claude_code_response_only_claim();
        assert!(claim.supports(&InterfaceVersion::new("2.1.228").unwrap()));
        assert!(!claim.permits(&InterfaceVersion::new("2.1.227").unwrap()));
        assert!(!claim.permits(&InterfaceVersion::new("2.1.229").unwrap()));
    }
}

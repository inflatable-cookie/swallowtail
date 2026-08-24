use swallowtail_core::{
    InterfaceBehaviorRevision, InterfaceCompatibilityClaim, InterfaceCompatibilityClaimId,
    InterfaceNewerVersionPosture, InterfaceSupportStatus, InterfaceVersion, InterfaceVersionAxis,
    InterfaceVersionBinding, InterfaceVersionScheme, InterfaceVersionSegment, PreflightPlan,
};
use swallowtail_runtime::RuntimeFailure;

use crate::failure::failure;

/// CLI-version axis for tool-free Claude Code response-only runs.
pub const CLAUDE_CODE_RESPONSE_ONLY_AXIS: &str = "claude-code.response-only-stream-json";
/// Oldest Claude Code version qualified for response-only runs.
pub const CLAUDE_CODE_RESPONSE_ONLY_BASELINE_VERSION: &str = "2.1.227";
/// Most recent Claude Code version with qualified response-only evidence.
pub const CLAUDE_CODE_RESPONSE_ONLY_LATEST_QUALIFIED_VERSION: &str = "2.1.241";
/// Most recent Claude Code version with qualified response-only evidence.
pub const CLAUDE_CODE_RESPONSE_ONLY_VERSION: &str =
    CLAUDE_CODE_RESPONSE_ONLY_LATEST_QUALIFIED_VERSION;
/// Stable Claude Code releases explicitly denied for response-only execution.
pub const CLAUDE_CODE_RESPONSE_ONLY_DENIED_VERSIONS: &[&str] = &[];

const RESPONSE_ONLY_BEHAVIOR: &str = "claude-code.response-only.stream-json.v1";
const MAX_VERSION_BYTES: usize = 64;

#[must_use]
/// Parses one stable Claude Code release into its response-only interface binding.
pub fn claude_code_response_only_binding(value: &str) -> Option<InterfaceVersionBinding> {
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

#[must_use]
/// Returns the response-only protocol compatibility claim.
pub fn claude_code_response_only_claim() -> InterfaceCompatibilityClaim {
    response_only_claim(CLAUDE_CODE_RESPONSE_ONLY_DENIED_VERSIONS)
}

fn response_only_claim(denied_versions: &[&str]) -> InterfaceCompatibilityClaim {
    InterfaceCompatibilityClaim::new(
        InterfaceCompatibilityClaimId::new("claude-code.response-only.window-1")
            .expect("static Claude Code response-only claim id is valid"),
        axis(),
        InterfaceVersionScheme::Semantic,
        InterfaceNewerVersionPosture::AllowUnverified,
        [InterfaceVersionSegment::new(
            InterfaceVersion::new(CLAUDE_CODE_RESPONSE_ONLY_BASELINE_VERSION)
                .expect("static Claude Code response-only baseline is valid"),
            InterfaceVersion::new(CLAUDE_CODE_RESPONSE_ONLY_LATEST_QUALIFIED_VERSION)
                .expect("static Claude Code response-only latest version is valid"),
            InterfaceBehaviorRevision::new(RESPONSE_ONLY_BEHAVIOR)
                .expect("static Claude Code response-only behavior is valid"),
            InterfaceSupportStatus::Maintained,
        )],
        denied_versions.iter().map(|version| {
            InterfaceVersion::new(*version).unwrap_or_else(|_| {
                panic!("static Claude Code response-only denied version is invalid")
            })
        }),
    )
    .expect("static Claude Code response-only claim is valid")
}

pub(crate) fn select_response_only_plan(
    plan: &PreflightPlan,
) -> Result<InterfaceVersionBinding, RuntimeFailure> {
    let claim = claude_code_response_only_claim();
    let mut bindings = plan
        .interface_versions()
        .filter(|binding| binding.axis() == claim.axis());
    let binding = bindings.next().ok_or_else(|| {
        failure(
            "swallowtail.claude_code.response_only.version_missing",
            "Claude Code response-only plan is missing its observed CLI version",
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
    Ok(binding.clone())
}

fn axis() -> InterfaceVersionAxis {
    InterfaceVersionAxis::new(CLAUDE_CODE_RESPONSE_ONLY_AXIS)
        .expect("static Claude Code response-only axis is valid")
}

#[cfg(test)]
mod tests {
    use super::*;
    use swallowtail_core::InterfaceCompatibilityAssessment;

    #[test]
    fn proven_window_and_provisional_newer_are_distinct() {
        assert!(claude_code_response_only_binding("2.1.227").is_some());
        assert!(claude_code_response_only_binding("2.1.228").is_some());
        assert!(claude_code_response_only_binding("2.1.229").is_some());
        for rejected in ["", "v2.1.228", "2.1.228\n", "2.1.229-rc.1", "2.1.229+build"] {
            assert!(claude_code_response_only_binding(rejected).is_none());
        }
        let claim = claude_code_response_only_claim();
        assert!(claim.supports(&InterfaceVersion::new("2.1.227").unwrap()));
        assert!(claim.supports(&InterfaceVersion::new("2.1.228").unwrap()));
        assert!(claim.supports(&InterfaceVersion::new("2.1.234").unwrap()));
        assert!(claim.supports(&InterfaceVersion::new("2.1.235").unwrap()));
        assert!(claim.supports(&InterfaceVersion::new("2.1.236").unwrap()));
        assert!(claim.supports(&InterfaceVersion::new("2.1.237").unwrap()));
        assert!(claim.supports(&InterfaceVersion::new("2.1.238").unwrap()));
        assert!(claim.supports(&InterfaceVersion::new("2.1.239").unwrap()));
        assert!(claim.supports(&InterfaceVersion::new("2.1.240").unwrap()));
        assert!(claim.supports(&InterfaceVersion::new("2.1.241").unwrap()));
        assert!(!claim.permits(&InterfaceVersion::new("2.1.226").unwrap()));
        assert!(matches!(
            claim.assess(&InterfaceVersion::new("2.1.242").unwrap()),
            InterfaceCompatibilityAssessment::UnverifiedNewer(_)
        ));
    }

    #[test]
    fn route_deny_list_excludes_an_otherwise_provisional_release() {
        assert!(CLAUDE_CODE_RESPONSE_ONLY_DENIED_VERSIONS.is_empty());
        let claim = response_only_claim(&["2.1.229"]);
        assert!(!claim.permits(&InterfaceVersion::new("2.1.229").unwrap()));
        assert_eq!(
            claim
                .exclusions()
                .map(InterfaceVersion::as_str)
                .collect::<Vec<_>>(),
            ["2.1.229"]
        );
    }
}

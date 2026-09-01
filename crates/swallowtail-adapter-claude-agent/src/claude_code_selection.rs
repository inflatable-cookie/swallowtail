use swallowtail_core::{
    InterfaceBehaviorRevision, InterfaceCompatibilityClaim, InterfaceCompatibilityClaimId,
    InterfaceNewerVersionPosture, InterfaceSupportStatus, InterfaceVersion, InterfaceVersionAxis,
    InterfaceVersionBinding, InterfaceVersionScheme, InterfaceVersionSegment, PreflightPlan,
};
use swallowtail_runtime::RuntimeFailure;

use crate::failure::failure;

/// CLI-version interface axis used by native Claude Code headless runs.
pub const CLAUDE_CODE_HEADLESS_AXIS: &str = "claude-code.headless-stream-json";
/// Oldest qualified native Claude Code headless version.
pub const CLAUDE_CODE_HEADLESS_BASELINE_VERSION: &str = "2.1.220";
/// Most recent qualified native Claude Code headless version.
pub const CLAUDE_CODE_HEADLESS_LATEST_QUALIFIED_VERSION: &str = "2.1.257";
/// Unpublished stables inside the semantic headless window.
const HEADLESS_UNPUBLISHED_GAPS: &[&str] = &[
    "2.1.244", "2.1.249", "2.1.253", "2.1.254", "2.1.255", "2.1.256",
];

const HEADLESS_BEHAVIOR: &str = "claude-code.headless.stream-json.v1";
const MAX_VERSION_BYTES: usize = 64;

#[must_use]
/// Parses a native Claude Code version into its interface binding.
pub fn claude_code_headless_binding(value: &str) -> Option<InterfaceVersionBinding> {
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
/// Returns the qualified compatibility claim for native headless runs.
pub fn claude_code_headless_claim() -> InterfaceCompatibilityClaim {
    InterfaceCompatibilityClaim::new(
        InterfaceCompatibilityClaimId::new("claude-code.headless.window-1")
            .expect("static Claude Code claim id is valid"),
        axis(),
        InterfaceVersionScheme::Semantic,
        InterfaceNewerVersionPosture::AllowUnverified,
        [InterfaceVersionSegment::new(
            version(CLAUDE_CODE_HEADLESS_BASELINE_VERSION)
                .expect("static Claude Code version is valid"),
            version(CLAUDE_CODE_HEADLESS_LATEST_QUALIFIED_VERSION)
                .expect("static Claude Code version is valid"),
            InterfaceBehaviorRevision::new(HEADLESS_BEHAVIOR)
                .expect("static Claude Code behavior revision is valid"),
            InterfaceSupportStatus::Maintained,
        )],
        HEADLESS_UNPUBLISHED_GAPS
            .iter()
            .map(|gap| version(gap).expect("static Claude Code unpublished gap is valid")),
    )
    .expect("static Claude Code compatibility claim is valid")
}

pub(crate) fn select_claude_code_headless_plan(plan: &PreflightPlan) -> Result<(), RuntimeFailure> {
    let claim = claude_code_headless_claim();
    let mut bindings = plan
        .interface_versions()
        .filter(|binding| binding.axis() == claim.axis());
    let binding = bindings.next().ok_or_else(|| {
        failure(
            "swallowtail.claude_code.headless.version_missing",
            "Claude Code headless plan is missing its exact CLI version",
        )
    })?;
    if bindings.next().is_some() {
        return Err(failure(
            "swallowtail.claude_code.headless.version_ambiguous",
            "Claude Code headless plan contains more than one CLI version",
        ));
    }
    let assessment = claim.assess(binding.version());
    if assessment != plan.assess_interface_version(binding) || !assessment.is_permitted() {
        return Err(failure(
            "swallowtail.claude_code.headless.version_incompatible",
            "Claude Code headless CLI version is incompatible with this driver",
        ));
    }
    if assessment
        .behavior_revision()
        .is_none_or(|revision| revision.as_str() != HEADLESS_BEHAVIOR)
    {
        return Err(failure(
            "swallowtail.claude_code.headless.behavior_incompatible",
            "Claude Code headless behavior is not mapped by this driver",
        ));
    }
    Ok(())
}

/// Reports whether a validated headless plan admits a maximum-turn bound.
///
/// Call this only after [`select_claude_code_headless_plan`] has accepted the
/// plan; it re-reads the same single axis binding and asks whether that exact
/// version is one Research 226 probed. The route's own claim is deliberately
/// weaker: it permits later stable points as `UnverifiedNewer` and spans a
/// semantic range containing an unpublished point.
pub(crate) fn plan_admits_maximum_turns(plan: &PreflightPlan) -> bool {
    headless_binding(plan).is_some_and(crate::claude_code_maximum_turns::admits)
}

pub(crate) fn plan_admits_watchers(plan: &PreflightPlan) -> bool {
    headless_binding(plan).is_some_and(crate::claude_code_watcher::admits)
}

fn headless_binding(plan: &PreflightPlan) -> Option<&swallowtail_core::InterfaceVersionBinding> {
    let axis = axis();
    let mut bindings = plan
        .interface_versions()
        .filter(|binding| binding.axis() == &axis);
    match (bindings.next(), bindings.next()) {
        (Some(binding), None) => Some(binding),
        _ => None,
    }
}

fn axis() -> InterfaceVersionAxis {
    InterfaceVersionAxis::new(CLAUDE_CODE_HEADLESS_AXIS).expect("static Claude Code axis is valid")
}

fn version(value: &str) -> Option<InterfaceVersion> {
    InterfaceVersion::new(value).ok()
}

#[cfg(test)]
mod tests {
    use super::{
        CLAUDE_CODE_HEADLESS_AXIS, claude_code_headless_binding, claude_code_headless_claim,
    };
    use swallowtail_core::{InterfaceCompatibilityAssessment, InterfaceVersion};

    #[test]
    fn qualified_window_covers_2_1_220_through_2_1_257() {
        let claim = claude_code_headless_claim();
        assert!(claim.supports(&version("2.1.220")));
        assert!(claim.supports(&version("2.1.221")));
        assert!(claim.supports(&version("2.1.234")));
        assert!(claim.supports(&version("2.1.235")));
        assert!(claim.supports(&version("2.1.236")));
        assert!(claim.supports(&version("2.1.237")));
        assert!(claim.supports(&version("2.1.238")));
        assert!(claim.supports(&version("2.1.239")));
        assert!(claim.supports(&version("2.1.240")));
        assert!(claim.supports(&version("2.1.241")));
        assert!(claim.supports(&version("2.1.242")));
        assert!(claim.supports(&version("2.1.243")));
        assert!(claim.supports(&version("2.1.245")));
        assert!(claim.supports(&version("2.1.250")));
        assert!(claim.supports(&version("2.1.251")));
        assert!(claim.supports(&version("2.1.252")));
        assert!(claim.supports(&version("2.1.257")));
        assert!(!claim.permits(&version("2.1.219")));
        assert!(!claim.permits(&version("2.1.244")));
        assert!(!claim.permits(&version("2.1.249")));
        assert!(!claim.permits(&version("2.1.253")));
        assert!(!claim.permits(&version("2.1.254")));
        assert!(!claim.permits(&version("2.1.255")));
        assert!(!claim.permits(&version("2.1.256")));
        assert!(matches!(
            claim.assess(&version("2.1.258")),
            InterfaceCompatibilityAssessment::UnverifiedNewer(_)
        ));
        assert_eq!(
            claude_code_headless_binding("2.1.257")
                .expect("version binds")
                .axis()
                .as_str(),
            CLAUDE_CODE_HEADLESS_AXIS
        );
    }

    #[test]
    fn binding_rejects_decorated_or_invalid_versions() {
        for rejected in [
            "",
            " 2.1.220",
            "2.1.220 (Claude Code)",
            "2.1.220 extra",
            "latest",
        ] {
            assert!(claude_code_headless_binding(rejected).is_none());
        }
    }

    fn version(value: &str) -> InterfaceVersion {
        InterfaceVersion::new(value).expect("fixture version is valid")
    }
}

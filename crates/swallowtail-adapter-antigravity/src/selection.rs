use swallowtail_core::{
    InterfaceBehaviorRevision, InterfaceCompatibilityClaim, InterfaceCompatibilityClaimId,
    InterfaceNewerVersionPosture, InterfaceSupportStatus, InterfaceVersion, InterfaceVersionAxis,
    InterfaceVersionBinding, InterfaceVersionScheme, InterfaceVersionSegment, PreflightPlan,
};
use swallowtail_runtime::RuntimeFailure;

use crate::failure::failure;

/// Default executable name for host-approved Antigravity discovery.
pub const ANTIGRAVITY_AUTOMATIC_EXECUTABLE_NAME: &str = "agy";
/// Semantic-version axis used for installed Antigravity releases.
pub const ANTIGRAVITY_RELEASE_AXIS: &str = "antigravity-cli.release";
/// Oldest release in the current maintained qualification window.
pub const ANTIGRAVITY_BASELINE_VERSION: &str = "1.1.9";
/// Latest catalogue release in the current maintained qualification window.
pub const ANTIGRAVITY_CATALOGUE_LATEST_QUALIFIED_VERSION: &str = "1.3.1";
/// Latest headless release in the current maintained qualification window.
/// Exact `1.2.11` qualifies on the retry-disabled behaviour revision under
/// Research 359; `1.1.18..=1.2.10` stay unqualified (no pin evidence).
pub const ANTIGRAVITY_HEADLESS_LATEST_QUALIFIED_VERSION: &str = "1.2.11";
/// Historical common ceiling. Headless remains at exact `1.2.11` under
/// Research 359 with `AGY_CLI_MODEL_API_MAX_RETRIES=0` pinned in its approved
/// environment; catalogue has its family-specific ceiling above.
pub const ANTIGRAVITY_LATEST_QUALIFIED_VERSION: &str = "1.2.11";
/// Required retry-pin control for the `1.2.11` headless segment.
pub const ANTIGRAVITY_HEADLESS_RETRY_PIN_NAME: &str = "AGY_CLI_MODEL_API_MAX_RETRIES";
/// Disabling pin value: provider-managed model-request retry is disabled,
/// so the segment needs no Contract 023 exception.
pub const ANTIGRAVITY_HEADLESS_RETRY_PIN_VALUE: &str = "0";

pub(crate) const ANTIGRAVITY_CATALOGUE_BEHAVIOR: &str =
    "antigravity.catalogue.cli-1.1.8-artifact-1.1.9-v1";
pub(crate) const ANTIGRAVITY_HEADLESS_BEHAVIOR: &str =
    "antigravity.stream-json.cli-1.1.8-artifact-1.1.9-v1";
/// Exact-`1.2.11` headless behaviour with provider-managed model-request
/// retry disabled by the approved-environment pin (Research 359).
pub(crate) const ANTIGRAVITY_HEADLESS_RETRY_DISABLED_BEHAVIOR: &str =
    "antigravity.stream-json.cli-1.1.8-artifact-1.2.11-retry-disabled-v1";
const MAX_VERSION_BYTES: usize = 64;

#[must_use]
/// Parses one stable installed release into its interface binding.
pub fn antigravity_release_binding(value: &str) -> Option<InterfaceVersionBinding> {
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
/// Returns the catalogue release compatibility claim.
pub fn antigravity_catalogue_claim() -> InterfaceCompatibilityClaim {
    InterfaceCompatibilityClaim::new(
        InterfaceCompatibilityClaimId::new("antigravity.catalogue.release-window-1")
            .expect("static Antigravity claim id is valid"),
        axis(),
        InterfaceVersionScheme::Semantic,
        InterfaceNewerVersionPosture::AllowUnverified,
        [InterfaceVersionSegment::new(
            version(ANTIGRAVITY_BASELINE_VERSION).expect("static Antigravity release is valid"),
            version(ANTIGRAVITY_CATALOGUE_LATEST_QUALIFIED_VERSION)
                .expect("static Antigravity release is valid"),
            InterfaceBehaviorRevision::new(ANTIGRAVITY_CATALOGUE_BEHAVIOR)
                .expect("static Antigravity behavior is valid"),
            InterfaceSupportStatus::Maintained,
        )],
        [version("1.2.18").expect("unpublished Antigravity point is valid")],
    )
    .expect("static Antigravity compatibility claim is valid")
}

#[must_use]
/// Returns the headless execution release compatibility claim.
///
/// Two behaviour segments: `1.1.9..=1.1.17` on the original revision
/// (deprecated), and exact `1.2.11` on the retry-disabled revision
/// (maintained) whose approved environment pins
/// `AGY_CLI_MODEL_API_MAX_RETRIES=0` (Research 359). `1.1.18..=1.2.10`
/// fall between the segments and assess incompatible until per-point pin
/// evidence lands. A new milestone segment bumps the claim window.
pub fn antigravity_headless_claim() -> InterfaceCompatibilityClaim {
    InterfaceCompatibilityClaim::new(
        InterfaceCompatibilityClaimId::new("antigravity.headless.release-window-2")
            .expect("static Antigravity headless claim id is valid"),
        axis(),
        InterfaceVersionScheme::Semantic,
        InterfaceNewerVersionPosture::AllowUnverified,
        [
            InterfaceVersionSegment::new(
                version(ANTIGRAVITY_BASELINE_VERSION).expect("static Antigravity release is valid"),
                version("1.1.17").expect("static Antigravity release is valid"),
                InterfaceBehaviorRevision::new(ANTIGRAVITY_HEADLESS_BEHAVIOR)
                    .expect("static Antigravity headless behavior is valid"),
                InterfaceSupportStatus::Deprecated,
            ),
            InterfaceVersionSegment::new(
                version(ANTIGRAVITY_HEADLESS_LATEST_QUALIFIED_VERSION)
                    .expect("static Antigravity release is valid"),
                version(ANTIGRAVITY_HEADLESS_LATEST_QUALIFIED_VERSION)
                    .expect("static Antigravity release is valid"),
                InterfaceBehaviorRevision::new(ANTIGRAVITY_HEADLESS_RETRY_DISABLED_BEHAVIOR)
                    .expect("static Antigravity headless behavior is valid"),
                InterfaceSupportStatus::Maintained,
            ),
        ],
        [],
    )
    .expect("static Antigravity headless compatibility claim is valid")
}

/// Reports whether the plan's single bound headless release sits on the
/// retry-disabled behaviour revision (exact `1.2.11`, Research 359).
/// Dispatch there always passes an explicit `--model`, which `1.2.11`
/// refuses without `--effort`, so callers fail closed on a missing effort
/// instead of spawning a child the CLI rejects.
#[must_use]
pub(crate) fn bound_headless_is_retry_disabled(plan: &PreflightPlan) -> bool {
    let claim = antigravity_headless_claim();
    let mut bindings = plan
        .interface_versions()
        .filter(|binding| binding.axis() == claim.axis());
    let Some(binding) = bindings.next() else {
        return false;
    };
    if bindings.next().is_some() {
        return false;
    }
    claim
        .assess(binding.version())
        .behavior_revision()
        .is_some_and(|revision| revision.as_str() == ANTIGRAVITY_HEADLESS_RETRY_DISABLED_BEHAVIOR)
}

pub(crate) fn validate_antigravity_catalogue_plan(
    plan: &PreflightPlan,
) -> Result<(), RuntimeFailure> {
    let claim = antigravity_catalogue_claim();
    let mut bindings = plan
        .interface_versions()
        .filter(|binding| binding.axis() == claim.axis());
    let binding = bindings.next().ok_or_else(|| {
        failure(
            "swallowtail.antigravity.catalogue.version_missing",
            "Antigravity catalogue plan is missing its exact release version",
        )
    })?;
    if bindings.next().is_some() {
        return Err(failure(
            "swallowtail.antigravity.catalogue.version_ambiguous",
            "Antigravity catalogue plan contains more than one release version",
        ));
    }
    let assessment = claim.assess(binding.version());
    if assessment != plan.assess_interface_version(binding)
        || !assessment.is_permitted()
        || assessment
            .behavior_revision()
            .is_none_or(|revision| revision.as_str() != ANTIGRAVITY_CATALOGUE_BEHAVIOR)
    {
        return Err(failure(
            "swallowtail.antigravity.catalogue.version_incompatible",
            "Antigravity release is incompatible with the catalogue driver",
        ));
    }
    Ok(())
}

pub(crate) fn validate_antigravity_headless_plan(
    plan: &PreflightPlan,
) -> Result<(), RuntimeFailure> {
    let claim = antigravity_headless_claim();
    let mut bindings = plan
        .interface_versions()
        .filter(|binding| binding.axis() == claim.axis());
    let binding = bindings.next().ok_or_else(|| {
        failure(
            "swallowtail.antigravity.headless.version_missing",
            "Antigravity headless plan is missing its exact release version",
        )
    })?;
    if bindings.next().is_some() {
        return Err(failure(
            "swallowtail.antigravity.headless.version_ambiguous",
            "Antigravity headless plan contains more than one release version",
        ));
    }
    let assessment = claim.assess(binding.version());
    if assessment != plan.assess_interface_version(binding)
        || !assessment.is_permitted()
        || assessment.behavior_revision().is_none_or(|revision| {
            revision.as_str() != ANTIGRAVITY_HEADLESS_BEHAVIOR
                && revision.as_str() != ANTIGRAVITY_HEADLESS_RETRY_DISABLED_BEHAVIOR
        })
    {
        return Err(failure(
            "swallowtail.antigravity.headless.version_incompatible",
            "Antigravity release is incompatible with the headless driver",
        ));
    }
    Ok(())
}

fn axis() -> InterfaceVersionAxis {
    InterfaceVersionAxis::new(ANTIGRAVITY_RELEASE_AXIS)
        .expect("static Antigravity release axis is valid")
}

fn version(value: &str) -> Option<InterfaceVersion> {
    InterfaceVersion::new(value).ok()
}

#[cfg(test)]
mod tests {
    use super::{
        ANTIGRAVITY_CATALOGUE_BEHAVIOR, ANTIGRAVITY_HEADLESS_BEHAVIOR,
        ANTIGRAVITY_HEADLESS_RETRY_DISABLED_BEHAVIOR, ANTIGRAVITY_RELEASE_AXIS,
        antigravity_catalogue_claim, antigravity_headless_claim, antigravity_release_binding,
    };
    use swallowtail_core::{
        InterfaceCompatibilityAssessment, InterfaceSupportStatus, InterfaceVersion,
    };

    #[test]
    fn exact_installed_release_is_qualified_and_newer_is_visible() {
        let claim = antigravity_catalogue_claim();
        assert!(claim.supports(&version("1.1.9")));
        assert!(claim.supports(&version("1.1.14")));
        assert!(claim.supports(&version("1.1.15")));
        assert!(claim.supports(&version("1.1.16")));
        assert!(claim.supports(&version("1.1.17")));
        assert!(claim.supports(&version("1.1.27")));
        assert!(claim.supports(&version("1.2.2")));
        assert!(claim.supports(&version("1.2.7")));
        assert!(claim.supports(&version("1.2.8")));
        assert!(claim.supports(&version("1.2.9")));
        assert!(claim.supports(&version("1.2.10")));
        assert!(claim.supports(&version("1.2.11")));
        assert!(claim.supports(&version("1.2.12")));
        assert!(claim.supports(&version("1.2.13")));
        assert!(claim.supports(&version("1.2.14")));
        assert!(claim.supports(&version("1.2.15")));
        assert!(claim.supports(&version("1.2.16")));
        assert!(claim.supports(&version("1.2.17")));
        assert!(claim.supports(&version("1.3.0")));
        assert!(claim.supports(&version("1.3.1")));
        assert!(!claim.permits(&version("1.1.8")));
        assert!(matches!(
            claim.assess(&version("1.2.18")),
            InterfaceCompatibilityAssessment::Incompatible
        ));
        let InterfaceCompatibilityAssessment::UnverifiedNewer(newer) =
            claim.assess(&version("1.3.2"))
        else {
            panic!("later Antigravity release remains visibly unverified");
        };
        assert_eq!(
            newer.behavior_revision().as_str(),
            ANTIGRAVITY_CATALOGUE_BEHAVIOR
        );
    }

    #[test]
    fn headless_claim_keeps_1_1_17_and_pins_exact_1_2_11() {
        let claim = antigravity_headless_claim();
        assert_eq!(claim.id().as_str(), "antigravity.headless.release-window-2");
        for kept in ["1.1.9", "1.1.14", "1.1.17"] {
            assert!(
                matches!(
                    claim.assess(&version(kept)),
                    InterfaceCompatibilityAssessment::Qualified(matched)
                        if matched.behavior_revision().as_str() == ANTIGRAVITY_HEADLESS_BEHAVIOR
                            && matched.support_status() == InterfaceSupportStatus::Deprecated
                ),
                "{kept} stays deprecated on the original headless revision"
            );
        }
        assert!(
            matches!(
                claim.assess(&version("1.2.11")),
                InterfaceCompatibilityAssessment::Qualified(matched)
                    if matched.behavior_revision().as_str()
                        == ANTIGRAVITY_HEADLESS_RETRY_DISABLED_BEHAVIOR
                        && matched.support_status() == InterfaceSupportStatus::Maintained
            ),
            "exact 1.2.11 qualifies maintained on the retry-disabled revision"
        );
        // Pin evidence is 1.2.11-only (Research 359): the interior gap is
        // incompatible, not unverified newer.
        for gap in [
            "1.1.18", "1.1.22", "1.1.27", "1.2.0", "1.2.2", "1.2.7", "1.2.8", "1.2.9", "1.2.10",
        ] {
            assert!(
                matches!(
                    claim.assess(&version(gap)),
                    InterfaceCompatibilityAssessment::Incompatible
                ),
                "{gap} stays unqualified until per-point pin evidence lands"
            );
        }
        let InterfaceCompatibilityAssessment::UnverifiedNewer(newer) =
            claim.assess(&version("1.2.12"))
        else {
            panic!("later Antigravity headless release remains visibly unverified");
        };
        assert_eq!(
            newer.behavior_revision().as_str(),
            ANTIGRAVITY_HEADLESS_RETRY_DISABLED_BEHAVIOR
        );
        assert!(!claim.permits(&version("1.1.8")));
    }

    #[test]
    fn binding_accepts_only_bare_stable_semver() {
        assert_eq!(
            antigravity_release_binding("1.1.9")
                .expect("binding parses")
                .axis()
                .as_str(),
            ANTIGRAVITY_RELEASE_AXIS
        );
        assert!(antigravity_release_binding("1.1.17").is_some());
        for rejected in [
            "",
            " 1.1.9",
            "agy 1.1.9",
            "1.1.9 extra",
            "1.1.10-alpha.1",
            "1.1.9+build",
        ] {
            assert!(antigravity_release_binding(rejected).is_none());
        }
    }

    #[test]
    fn catalogue_and_headless_keep_distinct_behavior_claims() {
        let catalogue = antigravity_catalogue_claim();
        let headless = antigravity_headless_claim();
        assert_ne!(catalogue.id(), headless.id());
        assert_ne!(
            catalogue.assess(&version("1.1.9")).behavior_revision(),
            headless.assess(&version("1.1.9")).behavior_revision()
        );
    }

    fn version(value: &str) -> InterfaceVersion {
        InterfaceVersion::new(value).expect("fixture version is valid")
    }
}

use swallowtail_core::{
    InterfaceBehaviorRevision, InterfaceCompatibilityClaim, InterfaceCompatibilityClaimId,
    InterfaceNewerVersionPosture, InterfaceSupportStatus, InterfaceVersion, InterfaceVersionAxis,
    InterfaceVersionBinding, InterfaceVersionScheme, InterfaceVersionSegment, PreflightPlan,
};
use swallowtail_runtime::RuntimeFailure;

use crate::failure::failure;

pub const KIMI_CODE_AXIS: &str = "kimi-code.executable";
pub const KIMI_CODE_BASELINE_VERSION: &str = "0.28.1";
pub const KIMI_CODE_LATEST_QUALIFIED_VERSION: &str = "0.29.2";
pub const KIMI_HEADLESS_BASELINE_VERSION: &str = "0.29.0";
pub const KIMI_HEADLESS_LATEST_QUALIFIED_VERSION: &str = "0.29.2";

const LEGACY_REASONING_BEHAVIOR: &str = "kimi.acp.reasoning.legacy-select-v1";
const DECLARED_EFFORT_BEHAVIOR: &str = "kimi.acp.reasoning.declared-effort-v2";
pub(crate) const HEADLESS_BEHAVIOR: &str = "kimi.headless.stream-json.v1";
const MAX_VERSION_BYTES: usize = 64;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum KimiAcpBehavior {
    LegacyReasoning,
    DeclaredEffort,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct KimiPlanSelection {
    behavior: KimiAcpBehavior,
    version: InterfaceVersion,
}

impl KimiPlanSelection {
    pub(crate) const fn behavior(&self) -> KimiAcpBehavior {
        self.behavior
    }

    pub(crate) const fn version(&self) -> &InterfaceVersion {
        &self.version
    }
}

#[must_use]
pub fn kimi_code_binding(value: &str) -> Option<InterfaceVersionBinding> {
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
pub fn kimi_acp_claim() -> InterfaceCompatibilityClaim {
    InterfaceCompatibilityClaim::new(
        InterfaceCompatibilityClaimId::new("kimi.acp.executable-window-2")
            .expect("static Kimi claim id is valid"),
        axis(),
        InterfaceVersionScheme::Semantic,
        InterfaceNewerVersionPosture::AllowUnverified,
        [
            InterfaceVersionSegment::exact(
                version(KIMI_CODE_BASELINE_VERSION),
                behavior(LEGACY_REASONING_BEHAVIOR),
                InterfaceSupportStatus::Maintained,
            ),
            InterfaceVersionSegment::new(
                version("0.29.0"),
                version(KIMI_CODE_LATEST_QUALIFIED_VERSION),
                behavior(DECLARED_EFFORT_BEHAVIOR),
                InterfaceSupportStatus::Maintained,
            ),
        ],
        [],
    )
    .expect("static Kimi compatibility claim is valid")
}

#[must_use]
pub fn kimi_headless_claim() -> InterfaceCompatibilityClaim {
    InterfaceCompatibilityClaim::new(
        InterfaceCompatibilityClaimId::new("kimi.headless.executable-window-1")
            .expect("static Kimi headless claim id is valid"),
        axis(),
        InterfaceVersionScheme::Semantic,
        InterfaceNewerVersionPosture::AllowUnverified,
        [InterfaceVersionSegment::new(
            version(KIMI_HEADLESS_BASELINE_VERSION),
            version(KIMI_HEADLESS_LATEST_QUALIFIED_VERSION),
            behavior(HEADLESS_BEHAVIOR),
            InterfaceSupportStatus::Maintained,
        )],
        [],
    )
    .expect("static Kimi headless compatibility claim is valid")
}

pub(crate) fn select_kimi_plan(plan: &PreflightPlan) -> Result<KimiPlanSelection, RuntimeFailure> {
    let claim = kimi_acp_claim();
    let mut bindings = plan
        .interface_versions()
        .filter(|binding| binding.axis() == claim.axis());
    let binding = bindings.next().ok_or_else(|| {
        failure(
            "swallowtail.kimi.acp.version_missing",
            "Kimi ACP plan is missing its exact executable version",
        )
    })?;
    if bindings.next().is_some() {
        return Err(failure(
            "swallowtail.kimi.acp.version_ambiguous",
            "Kimi ACP plan contains more than one executable version",
        ));
    }

    let assessment = claim.assess(binding.version());
    let planned = plan.assess_interface_version(binding);
    if assessment != planned || !assessment.is_permitted() {
        return Err(failure(
            "swallowtail.kimi.acp.version_incompatible",
            "Kimi ACP executable version is incompatible with this driver",
        ));
    }
    let behavior = match assessment
        .behavior_revision()
        .expect("permitted assessment has a behavior revision")
        .as_str()
    {
        LEGACY_REASONING_BEHAVIOR => KimiAcpBehavior::LegacyReasoning,
        DECLARED_EFFORT_BEHAVIOR => KimiAcpBehavior::DeclaredEffort,
        _ => {
            return Err(failure(
                "swallowtail.kimi.acp.behavior_incompatible",
                "Kimi ACP executable behavior is not mapped by this driver",
            ));
        }
    };
    Ok(KimiPlanSelection {
        behavior,
        version: binding.version().clone(),
    })
}

pub(crate) fn select_kimi_headless_plan(
    plan: &PreflightPlan,
) -> Result<InterfaceVersion, RuntimeFailure> {
    let claim = kimi_headless_claim();
    let mut bindings = plan
        .interface_versions()
        .filter(|binding| binding.axis() == claim.axis());
    let binding = bindings.next().ok_or_else(|| {
        failure(
            "swallowtail.kimi.headless.version_missing",
            "Kimi headless plan is missing its exact executable version",
        )
    })?;
    if bindings.next().is_some() {
        return Err(failure(
            "swallowtail.kimi.headless.version_ambiguous",
            "Kimi headless plan contains more than one executable version",
        ));
    }
    let assessment = claim.assess(binding.version());
    if assessment != plan.assess_interface_version(binding) || !assessment.is_permitted() {
        return Err(failure(
            "swallowtail.kimi.headless.version_incompatible",
            "Kimi headless executable version is incompatible with this driver",
        ));
    }
    if assessment
        .behavior_revision()
        .is_none_or(|revision| revision.as_str() != HEADLESS_BEHAVIOR)
    {
        return Err(failure(
            "swallowtail.kimi.headless.behavior_incompatible",
            "Kimi headless behavior is not mapped by this driver",
        ));
    }
    Ok(binding.version().clone())
}

fn axis() -> InterfaceVersionAxis {
    InterfaceVersionAxis::new(KIMI_CODE_AXIS).expect("static Kimi axis is valid")
}

fn version(value: &str) -> InterfaceVersion {
    InterfaceVersion::new(value).expect("static Kimi version is valid")
}

fn behavior(value: &str) -> InterfaceBehaviorRevision {
    InterfaceBehaviorRevision::new(value).expect("static Kimi behavior is valid")
}

#[cfg(test)]
mod tests {
    use super::{
        DECLARED_EFFORT_BEHAVIOR, KIMI_CODE_AXIS, KimiAcpBehavior, kimi_acp_claim,
        kimi_code_binding, kimi_headless_claim,
    };
    use swallowtail_core::{
        InstalledExecutableCompatibility, InstalledExecutableObservation,
        InterfaceCompatibilityAssessment, InterfaceVersion, InterfaceVersionBinding,
    };

    #[test]
    fn claim_preserves_the_baseline_point_and_qualified_declared_effort_range() {
        let claim = kimi_acp_claim();
        let segments = claim.milestones().collect::<Vec<_>>();
        assert_eq!(segments.len(), 2);
        assert_eq!(segments[0].minimum(), segments[0].maximum());
        assert_eq!(segments[1].minimum().as_str(), "0.29.0");
        assert_eq!(segments[1].maximum().as_str(), "0.29.2");
        assert!(claim.supports(&version("0.28.1")));
        for qualified in ["0.29.0", "0.29.1", "0.29.2"] {
            assert!(claim.supports(&version(qualified)));
        }
        for rejected in ["0.28.0", "0.28.2", "0.29.0-rc.1", "invalid"] {
            assert!(!claim.permits(&version(rejected)));
        }

        let InterfaceCompatibilityAssessment::UnverifiedNewer(newer) =
            claim.assess(&version("0.30.0"))
        else {
            panic!("stable newer release remains unverified");
        };
        assert_eq!(newer.behavior_revision().as_str(), DECLARED_EFFORT_BEHAVIOR);
    }

    #[test]
    fn binding_parser_accepts_exact_semver_and_rejects_raw_output() {
        assert_eq!(
            kimi_code_binding("0.29.0")
                .expect("version binds")
                .axis()
                .as_str(),
            KIMI_CODE_AXIS
        );
        for rejected in ["", " 0.29.0", "kimi 0.29.0", "0.29.0 extra", "latest"] {
            assert!(kimi_code_binding(rejected).is_none());
        }
    }

    #[test]
    fn headless_claim_starts_at_the_audited_default_runner() {
        let claim = kimi_headless_claim();
        assert!(!claim.permits(&version("0.28.1")));
        for qualified in ["0.29.0", "0.29.1", "0.29.2"] {
            assert!(claim.supports(&version(qualified)));
        }
        assert!(matches!(
            claim.assess(&version("0.30.0")),
            InterfaceCompatibilityAssessment::UnverifiedNewer(_)
        ));
    }

    #[test]
    fn installed_observation_keeps_qualified_and_unverified_distinct() {
        let claim = kimi_acp_claim();
        for (value, qualified, behavior) in [
            ("0.28.1", true, KimiAcpBehavior::LegacyReasoning),
            ("0.29.0", true, KimiAcpBehavior::DeclaredEffort),
            ("0.29.1", true, KimiAcpBehavior::DeclaredEffort),
            ("0.29.2", true, KimiAcpBehavior::DeclaredEffort),
            ("0.30.0", false, KimiAcpBehavior::DeclaredEffort),
        ] {
            let observation = InstalledExecutableObservation::classify(
                swallowtail_core::ExecutionHostId::new("fixture.host").expect("valid host"),
                InterfaceVersionBinding::new(
                    claim.axis().clone(),
                    InterfaceVersion::new(value).expect("valid version"),
                ),
                &claim,
            )
            .expect("observation classifies");
            assert_eq!(observation.is_qualified(), qualified);
            match observation.compatibility() {
                InstalledExecutableCompatibility::Qualified(matched) => {
                    assert!(qualified);
                    assert_eq!(
                        matched.behavior_revision().as_str(),
                        match behavior {
                            KimiAcpBehavior::LegacyReasoning => {
                                "kimi.acp.reasoning.legacy-select-v1"
                            }
                            KimiAcpBehavior::DeclaredEffort => DECLARED_EFFORT_BEHAVIOR,
                        }
                    );
                }
                InstalledExecutableCompatibility::UnverifiedNewer(newer) => {
                    assert!(!qualified);
                    assert_eq!(newer.behavior_revision().as_str(), DECLARED_EFFORT_BEHAVIOR);
                }
                InstalledExecutableCompatibility::Incompatible => {
                    panic!("selected observations are permitted");
                }
            }
        }
    }

    fn version(value: &str) -> InterfaceVersion {
        InterfaceVersion::new(value).expect("fixture version text is non-empty")
    }
}

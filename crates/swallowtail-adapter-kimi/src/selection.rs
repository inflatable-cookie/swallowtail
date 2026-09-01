use swallowtail_core::{
    InterfaceBehaviorRevision, InterfaceCompatibilityClaim, InterfaceCompatibilityClaimId,
    InterfaceNewerVersionPosture, InterfaceSupportStatus, InterfaceVersion, InterfaceVersionAxis,
    InterfaceVersionBinding, InterfaceVersionScheme, InterfaceVersionSegment, PreflightPlan,
};
use swallowtail_runtime::RuntimeFailure;

use crate::failure::failure;

/// Executable-version interface axis shared by installed Kimi Code routes.
pub const KIMI_CODE_AXIS: &str = "kimi-code.executable";
/// Oldest qualified Kimi Code ACP version.
pub const KIMI_CODE_BASELINE_VERSION: &str = "0.28.1";
/// Most recent qualified Kimi Code ACP version.
///
/// This does not track the family's newest stable. The ACP claim is
/// `QualifiedOnly` at this ceiling: every point above `0.38.0` fails closed.
/// From `0.39.0` the agent-core-v2 ACP terminal runner replaces two
/// fail-closed errors with a local host-process spawn, and Swallowtail always
/// advertises `terminal: false`, so that branch always applies. Nothing in
/// this adapter or the runtime contains that spawn. Exact `0.39.0` and
/// `0.39.1` stay excluded as recorded evidence. See
/// `ACP_EXCLUDED_AUTHORITY_VERSIONS`.
pub const KIMI_CODE_LATEST_QUALIFIED_VERSION: &str = "0.38.0";
/// Oldest qualified Kimi Code headless version.
pub const KIMI_HEADLESS_BASELINE_VERSION: &str = "0.29.0";
/// Most recent qualified Kimi Code headless version.
pub const KIMI_HEADLESS_LATEST_QUALIFIED_VERSION: &str = "0.39.1";

/// Newest Kimi Code release whose default `kimi -p` engine is agent-core v1.
///
/// `experimental-v2.ts` gates the print engine. Through this version
/// `isKimiV2Enabled()` means `KIMI_CODE_EXPERIMENTAL_FLAG` is truthy, so the
/// default is the legacy v1 print body. From the next published point it means
/// `KIMI_CODE_LEGACY_FLAG` is *not* truthy, so the default is agent-core-v2
/// `runV2Print`. This adapter never sets `KIMI_CODE_LEGACY_FLAG`.
const HEADLESS_V1_DEFAULT_CEILING: &str = "0.32.0";
/// Oldest Kimi Code release whose default `kimi -p` engine is agent-core-v2.
const HEADLESS_V2_DEFAULT_BASELINE: &str = "0.33.0";

/// Exact ACP points rejected for an uncontained process-authority change.
///
/// Under `QualifiedOnly` every point above `0.38.0` already fails closed.
/// These exclusions stay as recorded evidence of why the cap exists; they are
/// not a growing deny-list.
const ACP_EXCLUDED_AUTHORITY_VERSIONS: [&str; 2] = ["0.39.0", "0.39.1"];

pub(crate) const LEGACY_REASONING_BEHAVIOR: &str = "kimi.acp.reasoning.legacy-select-v1";
pub(crate) const DECLARED_EFFORT_BEHAVIOR: &str = "kimi.acp.reasoning.declared-effort-v2";
pub(crate) const HEADLESS_BEHAVIOR: &str = "kimi.headless.stream-json.v1";
pub(crate) const HEADLESS_BEHAVIOR_V2: &str = "kimi.headless.stream-json.v2";
const MAX_VERSION_BYTES: usize = 64;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum KimiAcpBehavior {
    LegacyReasoning,
    DeclaredEffort,
}

impl KimiAcpBehavior {
    pub(crate) fn from_revision(value: &str) -> Option<Self> {
        match value {
            LEGACY_REASONING_BEHAVIOR => Some(Self::LegacyReasoning),
            DECLARED_EFFORT_BEHAVIOR => Some(Self::DeclaredEffort),
            _ => None,
        }
    }

    pub(crate) const fn admitted_reasoning_modes(self) -> &'static [&'static str] {
        match self {
            Self::LegacyReasoning => &["off", "on", "low", "medium", "high"],
            Self::DeclaredEffort => &["off", "on", "low", "medium", "high", "xhigh", "max"],
        }
    }
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
/// Parses a Kimi Code semantic version into its interface binding.
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
/// Returns the qualified compatibility claim for Kimi Code ACP.
pub fn kimi_acp_claim() -> InterfaceCompatibilityClaim {
    InterfaceCompatibilityClaim::new(
        InterfaceCompatibilityClaimId::new("kimi.acp.executable-window-5")
            .expect("static Kimi claim id is valid"),
        axis(),
        InterfaceVersionScheme::Semantic,
        InterfaceNewerVersionPosture::QualifiedOnly,
        [
            InterfaceVersionSegment::exact(
                version(KIMI_CODE_BASELINE_VERSION).expect("static Kimi version is valid"),
                behavior(LEGACY_REASONING_BEHAVIOR),
                InterfaceSupportStatus::Deprecated,
            ),
            InterfaceVersionSegment::new(
                version("0.29.0").expect("static Kimi version is valid"),
                version(KIMI_CODE_LATEST_QUALIFIED_VERSION).expect("static Kimi version is valid"),
                behavior(DECLARED_EFFORT_BEHAVIOR),
                InterfaceSupportStatus::Maintained,
            ),
        ],
        ACP_EXCLUDED_AUTHORITY_VERSIONS
            .iter()
            .map(|value| version(value).expect("static Kimi exclusion is valid")),
    )
    .expect("static Kimi compatibility claim is valid")
}

#[must_use]
/// Returns the qualified compatibility claim for Kimi Code headless runs.
pub fn kimi_headless_claim() -> InterfaceCompatibilityClaim {
    InterfaceCompatibilityClaim::new(
        InterfaceCompatibilityClaimId::new("kimi.headless.executable-window-2")
            .expect("static Kimi headless claim id is valid"),
        axis(),
        InterfaceVersionScheme::Semantic,
        InterfaceNewerVersionPosture::AllowUnverified,
        [
            InterfaceVersionSegment::new(
                version(KIMI_HEADLESS_BASELINE_VERSION).expect("static Kimi version is valid"),
                version(HEADLESS_V1_DEFAULT_CEILING).expect("static Kimi version is valid"),
                behavior(HEADLESS_BEHAVIOR),
                InterfaceSupportStatus::Deprecated,
            ),
            InterfaceVersionSegment::new(
                version(HEADLESS_V2_DEFAULT_BASELINE).expect("static Kimi version is valid"),
                version(KIMI_HEADLESS_LATEST_QUALIFIED_VERSION)
                    .expect("static Kimi version is valid"),
                behavior(HEADLESS_BEHAVIOR_V2),
                InterfaceSupportStatus::Maintained,
            ),
        ],
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
    let behavior = KimiAcpBehavior::from_revision(
        assessment
            .behavior_revision()
            .expect("permitted assessment has a behavior revision")
            .as_str(),
    )
    .ok_or_else(|| {
        failure(
            "swallowtail.kimi.acp.behavior_incompatible",
            "Kimi ACP executable behavior is not mapped by this driver",
        )
    })?;
    Ok(KimiPlanSelection {
        behavior,
        version: binding.version().clone(),
    })
}

pub(crate) fn select_kimi_headless_plan(
    plan: &PreflightPlan,
) -> Result<KimiHeadlessPlanSelection, RuntimeFailure> {
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
    let behavior = KimiHeadlessBehavior::from_revision(
        assessment
            .behavior_revision()
            .expect("permitted assessment has a behavior revision")
            .as_str(),
    )
    .ok_or_else(|| {
        failure(
            "swallowtail.kimi.headless.behavior_incompatible",
            "Kimi headless behavior is not mapped by this driver",
        )
    })?;
    Ok(KimiHeadlessPlanSelection {
        behavior,
        version: binding.version().clone(),
    })
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct KimiHeadlessPlanSelection {
    behavior: KimiHeadlessBehavior,
    version: InterfaceVersion,
}

impl KimiHeadlessPlanSelection {
    pub(crate) const fn behavior(&self) -> KimiHeadlessBehavior {
        self.behavior
    }

    pub(crate) const fn version(&self) -> &InterfaceVersion {
        &self.version
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum KimiHeadlessBehavior {
    StreamJsonV1,
    StreamJsonV2,
}

impl KimiHeadlessBehavior {
    pub(crate) fn from_revision(value: &str) -> Option<Self> {
        match value {
            HEADLESS_BEHAVIOR => Some(Self::StreamJsonV1),
            HEADLESS_BEHAVIOR_V2 => Some(Self::StreamJsonV2),
            _ => None,
        }
    }
}

fn axis() -> InterfaceVersionAxis {
    InterfaceVersionAxis::new(KIMI_CODE_AXIS).expect("static Kimi axis is valid")
}

fn version(value: &str) -> Option<InterfaceVersion> {
    InterfaceVersion::new(value).ok()
}

fn behavior(value: &str) -> InterfaceBehaviorRevision {
    InterfaceBehaviorRevision::new(value).expect("static Kimi behavior is valid")
}

#[cfg(test)]
mod tests {
    use super::{
        ACP_EXCLUDED_AUTHORITY_VERSIONS, DECLARED_EFFORT_BEHAVIOR, HEADLESS_BEHAVIOR,
        HEADLESS_BEHAVIOR_V2, HEADLESS_V1_DEFAULT_CEILING, HEADLESS_V2_DEFAULT_BASELINE,
        KIMI_CODE_AXIS, KimiAcpBehavior, kimi_acp_claim, kimi_code_binding, kimi_headless_claim,
    };
    use swallowtail_core::{
        InstalledExecutableCompatibility, InstalledExecutableObservation,
        InterfaceCompatibilityAssessment, InterfaceSupportStatus, InterfaceVersion,
        InterfaceVersionBinding,
    };

    #[test]
    fn claim_preserves_the_baseline_point_and_qualified_declared_effort_range() {
        let claim = kimi_acp_claim();
        let segments = claim.milestones().collect::<Vec<_>>();
        assert_eq!(segments.len(), 2);
        assert_eq!(segments[0].minimum(), segments[0].maximum());
        assert_eq!(segments[1].minimum().as_str(), "0.29.0");
        assert_eq!(segments[1].maximum().as_str(), "0.38.0");
        assert!(claim.supports(&version("0.28.1")));
        for qualified in [
            "0.29.0", "0.29.1", "0.29.2", "0.30.0", "0.31.0", "0.31.1", "0.32.0", "0.33.0",
            "0.34.0", "0.35.0", "0.36.0", "0.36.1", "0.37.0", "0.37.1", "0.37.2", "0.38.0",
        ] {
            assert!(claim.supports(&version(qualified)));
        }
        for rejected in ["0.28.0", "0.28.2", "0.29.0-rc.1", "invalid"] {
            assert!(!claim.permits(&version(rejected)));
        }

        // The 0.39 line carries an uncontained process-authority change under
        // the `terminal: false` capabilities this adapter advertises. The
        // `QualifiedOnly` cap refuses every point above `0.38.0`; the named
        // exclusions stay as recorded evidence. Expected membership is a
        // local exact set, independent of `ACP_EXCLUDED_AUTHORITY_VERSIONS`.
        let excluded: Vec<&str> = claim.exclusions().map(InterfaceVersion::as_str).collect();
        assert_eq!(excluded, ["0.39.0", "0.39.1"]);
        for excluded in ["0.39.0", "0.39.1"] {
            assert_eq!(
                claim.assess(&version(excluded)),
                InterfaceCompatibilityAssessment::Incompatible,
                "{excluded} must not be admitted as unverified newer"
            );
            assert!(!claim.permits(&version(excluded)));
        }

        for newer in ["0.38.1", "0.39.2", "0.40.0"] {
            assert_eq!(
                claim.assess(&version(newer)),
                InterfaceCompatibilityAssessment::Incompatible,
                "{newer} must fail closed under QualifiedOnly"
            );
            assert!(!claim.permits(&version(newer)));
        }
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
    fn headless_claim_splits_v1_and_v2_at_the_default_engine_boundary() {
        let claim = kimi_headless_claim();
        assert!(!claim.permits(&version("0.28.1")));
        // v1 covers only the releases whose default `-p` engine is v1.
        for qualified in [
            "0.29.0", "0.29.1", "0.29.2", "0.30.0", "0.31.0", "0.31.1", "0.32.0",
        ] {
            let assessment = claim.assess(&version(qualified));
            let InterfaceCompatibilityAssessment::Qualified(matched) = assessment else {
                panic!("{qualified} remains qualified under v1");
            };
            assert_eq!(matched.behavior_revision().as_str(), HEADLESS_BEHAVIOR);
            assert_eq!(matched.support_status(), InterfaceSupportStatus::Deprecated);
        }
        // From the boundary the default `-p` path is agent-core-v2
        // `runV2Print`, which prepends a `system.version` preamble the v1
        // decoder rejects. Everything from there is v2, including the points
        // this claim previously mislabelled v1.
        for v2_point in [
            "0.33.0", "0.34.0", "0.35.0", "0.36.0", "0.36.1", "0.37.0", "0.37.1", "0.37.2",
            "0.38.0", "0.39.0", "0.39.1",
        ] {
            let InterfaceCompatibilityAssessment::Qualified(v2) = claim.assess(&version(v2_point))
            else {
                panic!("{v2_point} qualifies under v2");
            };
            assert_eq!(v2.behavior_revision().as_str(), HEADLESS_BEHAVIOR_V2);
            assert_eq!(v2.support_status(), InterfaceSupportStatus::Maintained);
        }
        // The boundary is exact and the two segments are adjacent published
        // points, so no gap opens between them.
        assert_eq!(HEADLESS_V1_DEFAULT_CEILING, "0.32.0");
        assert_eq!(HEADLESS_V2_DEFAULT_BASELINE, "0.33.0");
        let segments = claim.milestones().collect::<Vec<_>>();
        assert_eq!(segments[0].maximum().as_str(), HEADLESS_V1_DEFAULT_CEILING);
        assert_eq!(segments[1].minimum().as_str(), HEADLESS_V2_DEFAULT_BASELINE);

        // The headless axis carries no ACP authority exclusion: the print
        // route never constructs the ACP runtime provider.
        for acp_excluded in ACP_EXCLUDED_AUTHORITY_VERSIONS {
            assert!(claim.supports(&version(acp_excluded)));
        }

        let InterfaceCompatibilityAssessment::UnverifiedNewer(newer) =
            claim.assess(&version("0.39.2"))
        else {
            panic!("stable newer release remains unverified");
        };
        assert_eq!(newer.behavior_revision().as_str(), HEADLESS_BEHAVIOR_V2);
        assert_eq!(newer.latest_qualified().as_str(), "0.39.1");
    }

    #[test]
    fn installed_observation_maps_qualified_acp_points() {
        let claim = kimi_acp_claim();
        for (value, behavior) in [
            ("0.28.1", KimiAcpBehavior::LegacyReasoning),
            ("0.29.0", KimiAcpBehavior::DeclaredEffort),
            ("0.29.1", KimiAcpBehavior::DeclaredEffort),
            ("0.29.2", KimiAcpBehavior::DeclaredEffort),
            ("0.30.0", KimiAcpBehavior::DeclaredEffort),
            ("0.31.0", KimiAcpBehavior::DeclaredEffort),
            ("0.31.1", KimiAcpBehavior::DeclaredEffort),
            ("0.32.0", KimiAcpBehavior::DeclaredEffort),
            ("0.36.1", KimiAcpBehavior::DeclaredEffort),
            ("0.37.2", KimiAcpBehavior::DeclaredEffort),
            ("0.38.0", KimiAcpBehavior::DeclaredEffort),
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
            assert!(observation.is_qualified());
            let InstalledExecutableCompatibility::Qualified(matched) = observation.compatibility()
            else {
                panic!("selected observations stay qualified under QualifiedOnly");
            };
            assert_eq!(
                matched.behavior_revision().as_str(),
                match behavior {
                    KimiAcpBehavior::LegacyReasoning => "kimi.acp.reasoning.legacy-select-v1",
                    KimiAcpBehavior::DeclaredEffort => DECLARED_EFFORT_BEHAVIOR,
                }
            );
        }
    }

    fn version(value: &str) -> InterfaceVersion {
        InterfaceVersion::new(value).expect("fixture version text is non-empty")
    }
}

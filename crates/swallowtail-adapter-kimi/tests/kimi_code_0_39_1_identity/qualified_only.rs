//! QualifiedOnly cap proofs for `kimi-code.acp`.
//!
//! Public `InterfaceCompatibilityAssessment` has one `Incompatible` variant.
//! Exclusion-driven vs posture-driven rejection is distinguished through
//! `exclusions()` and `newer_version_posture()`. These tests reconstruct a
//! mutated claim, show the named assertion fail, then restore the original.

use super::support::version;
use swallowtail_adapter_kimi::{kimi_acp_claim, kimi_headless_claim, kimi_local_server_claim};
use swallowtail_core::{
    InterfaceCompatibilityAssessment, InterfaceCompatibilityClaim, InterfaceNewerVersionPosture,
    InterfaceVersion,
};

const NEWER_POINTS: [&str; 3] = ["0.38.1", "0.39.2", "0.40.0"];
const EXCLUDED: [&str; 2] = ["0.39.0", "0.39.1"];

#[test]
fn every_named_point_above_the_acp_ceiling_fails_closed() {
    let claim = kimi_acp_claim();
    assert_eq!(
        claim.newer_version_posture(),
        InterfaceNewerVersionPosture::QualifiedOnly
    );
    for point in ["0.38.1", "0.39.0", "0.39.1", "0.39.2", "0.40.0"] {
        assert_eq!(
            claim.assess(&version(point)),
            InterfaceCompatibilityAssessment::Incompatible,
            "{point} must fail closed"
        );
        assert!(!claim.permits(&version(point)));
        assert!(!claim.supports(&version(point)));
    }
}

#[test]
fn exclusion_membership_distinguishes_recorded_holes_from_posture_rejection() {
    let claim = kimi_acp_claim();
    let excluded: Vec<&str> = claim.exclusions().map(InterfaceVersion::as_str).collect();
    assert_eq!(excluded, EXCLUDED);

    for point in EXCLUDED {
        assert!(
            claim.exclusions().any(|value| value.as_str() == point),
            "{point} is an exclusion-recorded Incompatible"
        );
        assert_eq!(
            claim.assess(&version(point)),
            InterfaceCompatibilityAssessment::Incompatible
        );
    }

    for point in NEWER_POINTS {
        assert!(
            !claim.exclusions().any(|value| value.as_str() == point),
            "{point} is posture-driven Incompatible, not an exclusion"
        );
        assert_eq!(
            claim.assess(&version(point)),
            InterfaceCompatibilityAssessment::Incompatible
        );
    }
}

#[test]
fn flipping_acp_back_to_allow_unverified_fails_the_newer_point_assertions() {
    let original = kimi_acp_claim();
    for point in NEWER_POINTS {
        assert_eq!(
            original.assess(&version(point)),
            InterfaceCompatibilityAssessment::Incompatible
        );
    }

    let mutated = rebuild(
        &original,
        InterfaceNewerVersionPosture::AllowUnverified,
        original.exclusions().cloned(),
    );
    for point in NEWER_POINTS {
        assert!(
            matches!(
                mutated.assess(&version(point)),
                InterfaceCompatibilityAssessment::UnverifiedNewer(_)
            ),
            "{point} would pass through AllowUnverified; the fail-closed proof would fail"
        );
    }
    for point in EXCLUDED {
        assert_eq!(
            mutated.assess(&version(point)),
            InterfaceCompatibilityAssessment::Incompatible,
            "exclusions still fire before the AllowUnverified path"
        );
    }

    assert_eq!(
        original.newer_version_posture(),
        InterfaceNewerVersionPosture::QualifiedOnly
    );
    for point in NEWER_POINTS {
        assert_eq!(
            original.assess(&version(point)),
            InterfaceCompatibilityAssessment::Incompatible
        );
    }
}

#[test]
fn removing_an_exact_exclusion_fails_the_classification_proof() {
    let original = kimi_acp_claim();
    assert!(
        original
            .exclusions()
            .any(|value| value.as_str() == "0.39.0")
    );

    let remaining: Vec<InterfaceVersion> = original
        .exclusions()
        .filter(|value| value.as_str() != "0.39.0")
        .cloned()
        .collect();
    let mutated = rebuild(
        &original,
        InterfaceNewerVersionPosture::QualifiedOnly,
        remaining,
    );
    assert!(
        !mutated.exclusions().any(|value| value.as_str() == "0.39.0"),
        "dropping 0.39.0 from exclusions fails the exact classification proof"
    );
    assert_eq!(
        mutated.assess(&version("0.39.0")),
        InterfaceCompatibilityAssessment::Incompatible,
        "QualifiedOnly still refuses the point; assessment alone is not the proof"
    );

    assert!(
        original
            .exclusions()
            .any(|value| value.as_str() == "0.39.0")
    );
}

#[test]
fn changing_local_server_posture_fails_the_isolation_proof() {
    let original = kimi_local_server_claim();
    assert_eq!(
        original.newer_version_posture(),
        InterfaceNewerVersionPosture::AllowUnverified
    );
    assert!(matches!(
        original.assess(&version("0.38.1")),
        InterfaceCompatibilityAssessment::UnverifiedNewer(_)
    ));
    for excluded in EXCLUDED {
        assert!(matches!(
            original.assess(&version(excluded)),
            InterfaceCompatibilityAssessment::UnverifiedNewer(_)
        ));
    }

    let mutated = rebuild(
        &original,
        InterfaceNewerVersionPosture::QualifiedOnly,
        original.exclusions().cloned(),
    );
    assert_eq!(
        mutated.assess(&version("0.38.1")),
        InterfaceCompatibilityAssessment::Incompatible,
        "inheriting ACP QualifiedOnly would fail local-server isolation"
    );

    assert_eq!(
        original.newer_version_posture(),
        InterfaceNewerVersionPosture::AllowUnverified
    );
    assert!(matches!(
        original.assess(&version("0.38.1")),
        InterfaceCompatibilityAssessment::UnverifiedNewer(_)
    ));
    assert_eq!(
        kimi_acp_claim().newer_version_posture(),
        InterfaceNewerVersionPosture::QualifiedOnly
    );
}

#[test]
fn headless_does_not_inherit_the_acp_qualified_only_cap() {
    let headless = kimi_headless_claim();
    assert_eq!(
        headless.newer_version_posture(),
        InterfaceNewerVersionPosture::AllowUnverified
    );
    assert!(matches!(
        headless.assess(&version("0.39.2")),
        InterfaceCompatibilityAssessment::UnverifiedNewer(_)
    ));
    for excluded in EXCLUDED {
        assert!(headless.supports(&version(excluded)));
    }
}

fn rebuild(
    original: &InterfaceCompatibilityClaim,
    posture: InterfaceNewerVersionPosture,
    exclusions: impl IntoIterator<Item = InterfaceVersion>,
) -> InterfaceCompatibilityClaim {
    InterfaceCompatibilityClaim::new(
        original.id().clone(),
        original.axis().clone(),
        original.scheme(),
        posture,
        original.milestones().cloned(),
        exclusions,
    )
    .expect("mutated claim stays structurally valid")
}

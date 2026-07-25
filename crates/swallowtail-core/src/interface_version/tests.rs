use super::{
    InterfaceBehaviorRevision, InterfaceCompatibilityAssessment, InterfaceCompatibilityClaim,
    InterfaceCompatibilityClaimId, InterfaceNewerVersionPosture, InterfaceSupportStatus,
    InterfaceVersion, InterfaceVersionAxis, InterfaceVersionScheme, InterfaceVersionSegment,
};

#[test]
fn semantic_window_tracks_baseline_milestones_deprecation_and_exclusion() {
    let claim = InterfaceCompatibilityClaim::new(
        claim_id(),
        axis(),
        InterfaceVersionScheme::Semantic,
        InterfaceNewerVersionPosture::QualifiedOnly,
        [
            segment(
                "0.70.0",
                "0.74.9",
                "rpc-v1",
                InterfaceSupportStatus::Deprecated,
            ),
            segment(
                "0.75.0",
                "0.80.10",
                "rpc-v2",
                InterfaceSupportStatus::Maintained,
            ),
        ],
        [version("0.78.2")],
    )
    .unwrap();

    assert_eq!(claim.baseline().as_str(), "0.70.0");
    assert_eq!(claim.latest_qualified().as_str(), "0.80.10");
    assert_eq!(claim.milestones().len(), 2);
    assert_eq!(
        claim.classify(&version("0.72.0")).unwrap().support_status(),
        InterfaceSupportStatus::Deprecated
    );
    assert_eq!(
        claim
            .classify(&version("0.80.0"))
            .unwrap()
            .behavior_revision()
            .as_str(),
        "rpc-v2"
    );
    assert!(!claim.supports(&version("0.78.2")));
    assert!(!claim.supports(&version("0.80.11")));
    assert!(!claim.supports(&version("0.69.9")));
    assert!(!claim.supports(&version("0.75.0-rc.1")));
}

#[test]
fn semantic_prerelease_requires_a_separate_exact_segment() {
    let claim = InterfaceCompatibilityClaim::new(
        claim_id(),
        axis(),
        InterfaceVersionScheme::Semantic,
        InterfaceNewerVersionPosture::QualifiedOnly,
        [
            segment(
                "0.9.0-rc.1",
                "0.9.0-rc.1",
                "rc-v1",
                InterfaceSupportStatus::Maintained,
            ),
            segment(
                "0.9.0",
                "1.0.0",
                "stable-v1",
                InterfaceSupportStatus::Maintained,
            ),
        ],
        [],
    )
    .unwrap();

    assert!(claim.supports(&version("0.9.0-rc.1")));
    assert!(!claim.supports(&version("0.9.0-rc.2")));
    assert!(claim.supports(&version("0.9.0")));
}

#[test]
fn ordered_newer_versions_are_permitted_without_becoming_qualified() {
    let claim = InterfaceCompatibilityClaim::new(
        claim_id(),
        axis(),
        InterfaceVersionScheme::Semantic,
        InterfaceNewerVersionPosture::AllowUnverified,
        [segment(
            "1.0.0",
            "1.5.0",
            "stable-v1",
            InterfaceSupportStatus::Maintained,
        )],
        [version("1.6.2")],
    )
    .unwrap();

    let InterfaceCompatibilityAssessment::UnverifiedNewer(unverified) =
        claim.assess(&version("1.6.0"))
    else {
        panic!("newer stable release must remain unverified");
    };
    assert_eq!(unverified.version().as_str(), "1.6.0");
    assert_eq!(unverified.latest_qualified().as_str(), "1.5.0");
    assert_eq!(unverified.behavior_revision().as_str(), "stable-v1");
    assert!(!claim.supports(&version("1.6.0")));
    assert!(claim.permits(&version("1.6.0")));

    for incompatible in ["0.9.9", "1.5.1-rc.1", "1.6.2"] {
        assert_eq!(
            claim.assess(&version(incompatible)),
            InterfaceCompatibilityAssessment::Incompatible
        );
        assert!(!claim.permits(&version(incompatible)));
    }
}

#[test]
fn qualified_only_and_opaque_claims_do_not_infer_forward_execution() {
    let qualified_only = InterfaceCompatibilityClaim::new(
        claim_id(),
        axis(),
        InterfaceVersionScheme::Semantic,
        InterfaceNewerVersionPosture::QualifiedOnly,
        [segment(
            "1.0.0",
            "1.5.0",
            "stable-v1",
            InterfaceSupportStatus::Maintained,
        )],
        [],
    )
    .unwrap();
    assert_eq!(
        qualified_only.assess(&version("1.6.0")),
        InterfaceCompatibilityAssessment::Incompatible
    );

    let opaque = InterfaceCompatibilityClaim::new(
        claim_id(),
        axis(),
        InterfaceVersionScheme::Opaque,
        InterfaceNewerVersionPosture::AllowUnverified,
        [InterfaceVersionSegment::exact(
            version("opaque-v1"),
            InterfaceBehaviorRevision::new("opaque-v1").unwrap(),
            InterfaceSupportStatus::Maintained,
        )],
        [],
    )
    .unwrap_err();
    assert_eq!(
        opaque.diagnostic().code(),
        "swallowtail.interface_compatibility_claim_rejected"
    );
}

#[test]
fn opaque_windows_are_exact_only() {
    let error = InterfaceCompatibilityClaim::new(
        claim_id(),
        axis(),
        InterfaceVersionScheme::Opaque,
        InterfaceNewerVersionPosture::QualifiedOnly,
        [segment(
            "alpha",
            "beta",
            "opaque-v1",
            InterfaceSupportStatus::Maintained,
        )],
        [],
    )
    .unwrap_err();

    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.interface_compatibility_claim_rejected"
    );
}

#[test]
fn segments_must_be_ordered_and_non_overlapping() {
    let error = InterfaceCompatibilityClaim::new(
        claim_id(),
        axis(),
        InterfaceVersionScheme::Semantic,
        InterfaceNewerVersionPosture::QualifiedOnly,
        [
            segment("1.0.0", "2.0.0", "v1", InterfaceSupportStatus::Maintained),
            segment("1.5.0", "3.0.0", "v2", InterfaceSupportStatus::Maintained),
        ],
        [],
    )
    .unwrap_err();

    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.interface_compatibility_claim_rejected"
    );
}

#[test]
fn integer_and_calendar_windows_use_their_declared_ordering() {
    let integer = InterfaceCompatibilityClaim::new(
        claim_id(),
        axis(),
        InterfaceVersionScheme::Integer,
        InterfaceNewerVersionPosture::QualifiedOnly,
        [segment(
            "98",
            "102",
            "integer-v1",
            InterfaceSupportStatus::Maintained,
        )],
        [],
    )
    .unwrap();
    assert!(integer.supports(&version("100")));
    assert!(!integer.supports(&version("103")));

    let calendar = InterfaceCompatibilityClaim::new(
        claim_id(),
        axis(),
        InterfaceVersionScheme::CalendarDate,
        InterfaceNewerVersionPosture::QualifiedOnly,
        [segment(
            "2026-01-31",
            "2026-03-01",
            "calendar-v1",
            InterfaceSupportStatus::Maintained,
        )],
        [],
    )
    .unwrap();
    assert!(calendar.supports(&version("2026-02-28")));
    assert!(!calendar.supports(&version("2026-02-29")));
}

fn claim_id() -> InterfaceCompatibilityClaimId {
    InterfaceCompatibilityClaimId::new("fixture-claim-1").unwrap()
}

fn axis() -> InterfaceVersionAxis {
    InterfaceVersionAxis::new("harness.package").unwrap()
}

fn version(value: &str) -> InterfaceVersion {
    InterfaceVersion::new(value).unwrap()
}

fn segment(
    minimum: &str,
    maximum: &str,
    behavior: &str,
    status: InterfaceSupportStatus,
) -> InterfaceVersionSegment {
    InterfaceVersionSegment::new(
        version(minimum),
        version(maximum),
        InterfaceBehaviorRevision::new(behavior).unwrap(),
        status,
    )
}

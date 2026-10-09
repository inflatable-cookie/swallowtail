use super::{
    InterfaceBehaviorRevision, InterfaceCompatibilityAssessment, InterfaceCompatibilityClaim,
    InterfaceCompatibilityClaimId, InterfaceNewerVersionPosture, InterfaceSupportStatus,
    InterfaceVersion, InterfaceVersionAxis, InterfaceVersionScheme, InterfaceVersionSegment,
};
use crate::{
    AdapterId, AdapterIdentity, AdapterVersion, DriverDescriptor, IntegrationFamilyId,
    TransportFamilyId,
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
fn opaque_exact_members_are_canonical_and_membership_is_not_an_interval() {
    let claim = opaque_claim(
        [
            opaque_segment(
                "b9910-f5525f7e7",
                "attached-v1",
                InterfaceSupportStatus::Deprecated,
            ),
            opaque_segment(
                "b11429-d81235049",
                "attached-v2",
                InterfaceSupportStatus::Maintained,
            ),
        ],
        [version("b10566-bb4caa754")],
    )
    .unwrap();

    let members = claim.milestones().collect::<Vec<_>>();
    assert_eq!(members.len(), 2);
    assert_eq!(members[0].minimum().as_str(), "b11429-d81235049");
    assert_eq!(members[1].minimum().as_str(), "b9910-f5525f7e7");
    assert_eq!(claim.baseline().as_str(), "b11429-d81235049");
    assert_eq!(claim.latest_qualified().as_str(), "b9910-f5525f7e7");
    assert!(!claim.has_version_interval());

    let maintained = claim
        .classify(&version("b11429-d81235049"))
        .expect("listed maintained member is qualified");
    assert_eq!(maintained.behavior_revision().as_str(), "attached-v2");
    assert_eq!(
        maintained.support_status(),
        InterfaceSupportStatus::Maintained
    );
    let deprecated = claim
        .classify(&version("b9910-f5525f7e7"))
        .expect("listed deprecated member stays executable");
    assert_eq!(deprecated.behavior_revision().as_str(), "attached-v1");
    assert_eq!(
        deprecated.support_status(),
        InterfaceSupportStatus::Deprecated
    );
    assert!(claim.permits(&version("b9910-f5525f7e7")));

    for non_member in [
        "b10566-bb4caa754",
        "b50000-lexicographic-interior",
        "b11429-d81235049-rc.1",
        "b10069-178a6c449",
        "v0.6.0",
        "b11430-d81235049",
    ] {
        let observed = version(non_member);
        assert_eq!(claim.classify(&observed), None);
        assert_eq!(
            claim.assess(&observed),
            InterfaceCompatibilityAssessment::Incompatible
        );
        assert!(!claim.permits(&observed));
    }

    let reversed = opaque_claim(
        [
            opaque_segment(
                "b11429-d81235049",
                "attached-v2",
                InterfaceSupportStatus::Maintained,
            ),
            opaque_segment(
                "b9910-f5525f7e7",
                "attached-v1",
                InterfaceSupportStatus::Deprecated,
            ),
        ],
        [version("b10566-bb4caa754")],
    )
    .unwrap();
    assert_eq!(claim, reversed);

    let round_trip = InterfaceCompatibilityClaim::new(
        claim.id().clone(),
        claim.axis().clone(),
        claim.scheme(),
        claim.newer_version_posture(),
        claim.milestones().cloned(),
        claim.exclusions().cloned(),
    )
    .unwrap();
    assert_eq!(claim, round_trip);
}

#[test]
fn opaque_singletons_keep_maintained_and_deprecated_results() {
    for (revision, status) in [
        ("opaque-maintained", InterfaceSupportStatus::Maintained),
        ("opaque-deprecated", InterfaceSupportStatus::Deprecated),
    ] {
        let claim = opaque_claim([opaque_segment("opaque-v1", revision, status)], []).unwrap();
        assert_eq!(claim.baseline().as_str(), "opaque-v1");
        assert_eq!(claim.latest_qualified().as_str(), "opaque-v1");
        assert_eq!(claim.milestones().len(), 1);
        assert_eq!(
            claim
                .classify(&version("opaque-v1"))
                .unwrap()
                .support_status(),
            status
        );
        assert!(!claim.supports(&version("opaque-v2")));
    }
}

#[test]
fn opaque_member_bound_accepts_32_and_refuses_the_33rd_yield() {
    let members = (0..32).map(|index| {
        opaque_segment(
            &format!("member-{index:02}"),
            "opaque-current",
            InterfaceSupportStatus::Maintained,
        )
    });
    assert_eq!(opaque_claim(members, []).unwrap().milestones().len(), 32);

    let next_calls = std::rc::Rc::new(std::cell::Cell::new(0));
    let calls = next_calls.clone();
    let segments = std::iter::from_fn(move || {
        let next = calls.get() + 1;
        calls.set(next);
        Some(opaque_segment(
            &format!("member-{next:02}"),
            "opaque-current",
            InterfaceSupportStatus::Maintained,
        ))
    });
    let exclusions_started = std::rc::Rc::new(std::cell::Cell::new(0));
    let exclusion_calls = exclusions_started.clone();
    let exclusions = std::iter::from_fn(move || {
        exclusion_calls.set(exclusion_calls.get() + 1);
        Some(version("unused-exclusion"))
    });
    let error = opaque_claim(segments, exclusions).unwrap_err();
    assert_eq!(next_calls.get(), 33);
    assert_eq!(exclusions_started.get(), 0);
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.interface_compatibility_claim_rejected"
    );
}

#[test]
fn opaque_exclusion_bound_counts_raw_yields_and_collapses_duplicates() {
    let claim = opaque_claim(
        [opaque_segment(
            "opaque-v1",
            "opaque-current",
            InterfaceSupportStatus::Maintained,
        )],
        std::iter::repeat_n(version("bad-point"), 32),
    )
    .unwrap();
    assert_eq!(claim.exclusions().len(), 1);

    let next_calls = std::rc::Rc::new(std::cell::Cell::new(0));
    let calls = next_calls.clone();
    let exclusions = std::iter::from_fn(move || {
        calls.set(calls.get() + 1);
        Some(version("bad-point"))
    });
    let error = opaque_claim(
        [opaque_segment(
            "opaque-v1",
            "opaque-current",
            InterfaceSupportStatus::Maintained,
        )],
        exclusions,
    )
    .unwrap_err();
    assert_eq!(next_calls.get(), 33);
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.interface_compatibility_claim_rejected"
    );
}

#[test]
fn opaque_validation_rejects_duplicates_windows_exclusions_and_overlong_text() {
    let duplicate = opaque_claim(
        [
            opaque_segment("same", "opaque-current", InterfaceSupportStatus::Maintained),
            opaque_segment("same", "opaque-old", InterfaceSupportStatus::Deprecated),
        ],
        [],
    )
    .unwrap_err();
    assert_eq!(
        duplicate.diagnostic().code(),
        "swallowtail.interface_compatibility_claim_rejected"
    );

    let member_excluded = opaque_claim(
        [opaque_segment(
            "opaque-v1",
            "opaque-current",
            InterfaceSupportStatus::Maintained,
        )],
        [version("opaque-v1")],
    )
    .unwrap_err();
    assert_eq!(
        member_excluded.diagnostic().code(),
        "swallowtail.interface_compatibility_claim_rejected"
    );

    let overlong_version = "v".repeat(257);
    assert!(
        opaque_claim(
            [opaque_segment(
                &overlong_version,
                "opaque-current",
                InterfaceSupportStatus::Maintained,
            )],
            [],
        )
        .is_err()
    );

    let at_limit = opaque_claim(
        [opaque_segment(
            &"v".repeat(256),
            &"r".repeat(256),
            InterfaceSupportStatus::Maintained,
        )],
        [version(&"x".repeat(256))],
    )
    .expect("256-byte opaque fields are within the bound");
    assert_eq!(at_limit.milestones().len(), 1);
    let overlong_revision = "r".repeat(257);
    assert!(
        opaque_claim(
            [opaque_segment(
                "opaque-v1",
                &overlong_revision,
                InterfaceSupportStatus::Maintained,
            )],
            [],
        )
        .is_err()
    );
    assert!(
        opaque_claim(
            [opaque_segment(
                "opaque-v1",
                "opaque-current",
                InterfaceSupportStatus::Maintained,
            )],
            [version(&"x".repeat(257))],
        )
        .is_err()
    );
}

#[test]
fn opaque_multi_member_support_status_uses_one_maintained_revision() {
    assert!(
        opaque_claim(
            [
                opaque_segment(
                    "current-a",
                    "shared-revision",
                    InterfaceSupportStatus::Maintained
                ),
                opaque_segment(
                    "current-b",
                    "shared-revision",
                    InterfaceSupportStatus::Maintained
                ),
            ],
            [],
        )
        .is_ok()
    );

    let all_deprecated = opaque_claim(
        [
            opaque_segment("old-a", "old-a", InterfaceSupportStatus::Deprecated),
            opaque_segment("old-b", "old-b", InterfaceSupportStatus::Deprecated),
        ],
        [],
    );
    assert!(all_deprecated.is_err());

    let split_maintained = opaque_claim(
        [
            opaque_segment(
                "current-a",
                "revision-a",
                InterfaceSupportStatus::Maintained,
            ),
            opaque_segment(
                "current-b",
                "revision-b",
                InterfaceSupportStatus::Maintained,
            ),
        ],
        [],
    );
    assert!(split_maintained.is_err());

    let deprecated_shares_current = opaque_claim(
        [
            opaque_segment(
                "current",
                "same-revision",
                InterfaceSupportStatus::Maintained,
            ),
            opaque_segment(
                "deprecated",
                "same-revision",
                InterfaceSupportStatus::Deprecated,
            ),
        ],
        [],
    );
    assert!(deprecated_shares_current.is_err());

    assert!(opaque_claim([], []).is_err());
    assert!(
        InterfaceCompatibilityClaim::new(
            claim_id(),
            axis(),
            InterfaceVersionScheme::Opaque,
            InterfaceNewerVersionPosture::AllowUnverified,
            [opaque_segment(
                "opaque-v1",
                "opaque-current",
                InterfaceSupportStatus::Maintained,
            )],
            [],
        )
        .is_err()
    );
}

#[test]
fn semantic_segments_and_exclusions_remain_unbounded_and_ordered() {
    let segments = (0..40).map(|minor| {
        segment(
            &format!("1.{minor}.0"),
            &format!("1.{minor}.0"),
            if minor == 39 {
                "semantic-v2"
            } else {
                "semantic-v1"
            },
            if minor == 39 {
                InterfaceSupportStatus::Maintained
            } else {
                InterfaceSupportStatus::Deprecated
            },
        )
    });
    let exclusions = (0..40).map(|minor| version(&format!("2.{minor}.0")));
    let claim = InterfaceCompatibilityClaim::new(
        claim_id(),
        axis(),
        InterfaceVersionScheme::Semantic,
        InterfaceNewerVersionPosture::QualifiedOnly,
        segments,
        exclusions,
    )
    .unwrap();
    assert!(claim.has_version_interval());
    assert_eq!(claim.milestones().len(), 40);
    assert_eq!(claim.exclusions().len(), 40);
}

#[test]
fn ordered_schemes_continue_to_report_interval_semantics() {
    for scheme in [
        InterfaceVersionScheme::Semantic,
        InterfaceVersionScheme::Integer,
        InterfaceVersionScheme::CalendarDate,
    ] {
        let claim = InterfaceCompatibilityClaim::new(
            claim_id(),
            axis(),
            scheme,
            InterfaceNewerVersionPosture::QualifiedOnly,
            [segment(
                match scheme {
                    InterfaceVersionScheme::Semantic => "1.0.0",
                    InterfaceVersionScheme::Integer => "1",
                    InterfaceVersionScheme::CalendarDate => "2026-01-01",
                    InterfaceVersionScheme::Opaque => unreachable!(),
                },
                match scheme {
                    InterfaceVersionScheme::Semantic => "2.0.0",
                    InterfaceVersionScheme::Integer => "2",
                    InterfaceVersionScheme::CalendarDate => "2026-02-01",
                    InterfaceVersionScheme::Opaque => unreachable!(),
                },
                "ordered-v1",
                InterfaceSupportStatus::Maintained,
            )],
            [],
        )
        .unwrap();
        assert!(claim.has_version_interval());
    }
}

#[test]
fn opaque_hosted_facade_and_separate_runtime_axes_keep_exact_identity() {
    let current_facade = format!("gemini-live-facade-{}", "x".repeat(128));
    let facade = opaque_claim(
        [opaque_segment(
            &current_facade,
            "gemini-live-current",
            InterfaceSupportStatus::Maintained,
        )],
        [],
    )
    .unwrap();
    assert!(facade.supports(&version(&current_facade)));
    assert!(!facade.supports(&version("gemini-live-facade-superseded")));

    let attached = opaque_claim_on_axis(
        InterfaceVersionAxis::new("llama.cpp.attached-runtime").unwrap(),
        [opaque_segment(
            "b9910-f5525f7e7",
            "attached-b9910",
            InterfaceSupportStatus::Maintained,
        )],
        [],
    )
    .unwrap();
    let owned = opaque_claim_on_axis(
        InterfaceVersionAxis::new("llama.cpp.owned-runtime").unwrap(),
        [opaque_segment(
            "b10069-178a6c449",
            "owned-b10069",
            InterfaceSupportStatus::Maintained,
        )],
        [],
    )
    .unwrap();
    let driver = DriverDescriptor::new(
        AdapterIdentity::new(
            AdapterId::new("fixture.llama-cpp").unwrap(),
            AdapterVersion::new("0.1.0").unwrap(),
        ),
        IntegrationFamilyId::new("fixture").unwrap(),
        TransportFamilyId::new("fixture-runtime").unwrap(),
    )
    .with_interface_compatibility(attached)
    .with_interface_compatibility(owned);

    let attached_binding = crate::InterfaceVersionBinding::new(
        InterfaceVersionAxis::new("llama.cpp.attached-runtime").unwrap(),
        version("b9910-f5525f7e7"),
    );
    let owned_binding = crate::InterfaceVersionBinding::new(
        InterfaceVersionAxis::new("llama.cpp.owned-runtime").unwrap(),
        version("b10069-178a6c449"),
    );
    assert!(driver.permits_interface_version(&attached_binding));
    assert!(driver.permits_interface_version(&owned_binding));
    assert!(
        !driver.permits_interface_version(&crate::InterfaceVersionBinding::new(
            InterfaceVersionAxis::new("llama.cpp.attached-runtime").unwrap(),
            version("b10069-178a6c449"),
        ))
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

fn opaque_claim(
    segments: impl IntoIterator<Item = InterfaceVersionSegment>,
    exclusions: impl IntoIterator<Item = InterfaceVersion>,
) -> Result<InterfaceCompatibilityClaim, super::InvalidInterfaceCompatibilityClaim> {
    opaque_claim_on_axis(axis(), segments, exclusions)
}

fn opaque_claim_on_axis(
    axis: InterfaceVersionAxis,
    segments: impl IntoIterator<Item = InterfaceVersionSegment>,
    exclusions: impl IntoIterator<Item = InterfaceVersion>,
) -> Result<InterfaceCompatibilityClaim, super::InvalidInterfaceCompatibilityClaim> {
    InterfaceCompatibilityClaim::new(
        claim_id(),
        axis,
        InterfaceVersionScheme::Opaque,
        InterfaceNewerVersionPosture::QualifiedOnly,
        segments,
        exclusions,
    )
}

fn opaque_segment(
    version_value: &str,
    behavior: &str,
    status: InterfaceSupportStatus,
) -> InterfaceVersionSegment {
    InterfaceVersionSegment::exact(
        version(version_value),
        InterfaceBehaviorRevision::new(behavior).unwrap(),
        status,
    )
}

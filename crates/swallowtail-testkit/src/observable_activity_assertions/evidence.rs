use crate::{ObservableActivityFixtureCase, ObservableActivityTraceFixture};
use swallowtail_core::{
    ActivityContentStream, ActivityDisclosure, ActivityInterfaceBasis, ActivityKindClass,
    ActivityKindProfile, ActivityLifecycleFidelity, ActivityUnknownEventPosture,
    InterfaceBehaviorRevision, InterfaceCompatibilityAssessment, InterfaceCompatibilityClaim,
    InterfaceCompatibilityClaimId, InterfaceNewerVersionPosture, InterfaceSupportStatus,
    InterfaceVersion, InterfaceVersionAxis, InterfaceVersionScheme, InterfaceVersionSegment,
    ObservableActivityProfile, ProviderActivityRef,
};
use swallowtail_runtime::{
    ActivityContent, ActivityId, ActivityNamespace, OperationContent, RuntimeEventKind,
};

pub(super) fn assert_unverified_newer_profile_is_not_widened() {
    let axis = valid(InterfaceVersionAxis::new, "fixture.activity.interface");
    let revision = valid(
        InterfaceBehaviorRevision::new,
        "fixture.activity.behavior-v1",
    );
    let claim = InterfaceCompatibilityClaim::new(
        valid(InterfaceCompatibilityClaimId::new, "fixture.activity.claim"),
        axis.clone(),
        InterfaceVersionScheme::Semantic,
        InterfaceNewerVersionPosture::AllowUnverified,
        [InterfaceVersionSegment::new(
            version("1.0.0"),
            version("1.4.0"),
            revision.clone(),
            InterfaceSupportStatus::Maintained,
        )],
        [],
    )
    .expect("compatibility claim is valid");
    let InterfaceCompatibilityAssessment::UnverifiedNewer(unverified) =
        claim.assess(&version("1.5.0"))
    else {
        panic!("fixture version must be unverified newer");
    };
    let profile = ObservableActivityProfile::available(
        [ActivityInterfaceBasis::new(
            axis.clone(),
            unverified.behavior_revision().clone(),
        )],
        [ActivityKindProfile::new(
            ActivityKindClass::AssistantMessage,
            ActivityLifecycleFidelity::CompletionOnly,
            [ActivityContentStream::FinalAnswerText],
            ActivityDisclosure::ProviderDisplayContent,
            [],
        )
        .expect("activity kind profile is valid")],
        ActivityUnknownEventPosture::FailClosed,
    )
    .expect("activity profile is valid");

    let basis = profile.interface_basis().collect::<Vec<_>>();
    assert_eq!(basis.len(), 1);
    assert_eq!(basis[0].axis(), &axis);
    assert_eq!(basis[0].behavior_revision(), &revision);
    assert_eq!(profile.kinds().count(), 1);
    assert_eq!(
        profile.unknown_event_posture(),
        ActivityUnknownEventPosture::FailClosed
    );
}

pub(super) fn assert_bounds_and_redaction() {
    assert!(ActivityId::new("x".repeat(257)).is_err());
    assert!(ActivityNamespace::new("x".repeat(129)).is_err());
    assert!(ProviderActivityRef::new("x".repeat(513)).is_err());
    assert!(
        ActivityContent::new(
            OperationContent::new("sensitive activity data").expect("content is valid"),
            8,
        )
        .is_err()
    );

    let fixture =
        ObservableActivityTraceFixture::for_case(ObservableActivityFixtureCase::ReasoningSummary);
    let activity = fixture
        .events()
        .iter()
        .find_map(|event| match event.kind() {
            RuntimeEventKind::Activity(activity) => Some(activity),
            _ => None,
        })
        .expect("fixture contains activity");
    let rendered = format!("{activity:?}");
    assert!(!rendered.contains(activity.activity_id().as_str()));
    assert!(!rendered.contains("provider marked"));
    assert_eq!(activity.to_string(), "<redacted activity observation>");
}

fn version(value: &str) -> InterfaceVersion {
    valid(InterfaceVersion::new, value)
}

fn valid<T, E>(constructor: impl FnOnce(String) -> Result<T, E>, value: &str) -> T
where
    E: std::fmt::Debug,
{
    constructor(value.to_owned()).expect("fixture value is valid")
}

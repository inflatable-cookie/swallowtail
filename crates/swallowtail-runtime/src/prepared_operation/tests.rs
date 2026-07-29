use super::PreparedOperationEvidence;
use crate::PreparationStage;
use swallowtail_core::{
    ActivityContentStream, ActivityDisclosure, ActivityKindClass, ActivityKindProfile,
    ActivityLifecycleFidelity, ActivityUnknownEventPosture, InterfaceCompatibilityAssessment,
    ObservableActivityAvailability, ObservableActivityProfile,
};

#[path = "tests/support.rs"]
mod support;

use support::{
    ACTIVITY_REVISION, Fixture, activity_basis, activity_requirement, full_activity_profile,
};

#[test]
fn prepared_evidence_exposes_exact_profile_and_transport_before_effects() {
    let fixture = Fixture::new(activity_requirement(
        ActivityLifecycleFidelity::CompletionOnly,
    ));
    let evidence = PreparedOperationEvidence::from_plan_with_activity_profile(
        fixture.plan(),
        fixture.access_evidence(),
        full_activity_profile(ACTIVITY_REVISION),
    )
    .expect("qualified activity evidence prepares");

    assert_eq!(
        evidence.binding().transport_family().as_str(),
        "fixture-jsonl"
    );
    assert_eq!(
        evidence.observable_activity().availability(),
        ObservableActivityAvailability::Available
    );
    assert_eq!(
        evidence
            .observable_activity()
            .lifecycle(ActivityKindClass::AssistantMessage),
        ActivityLifecycleFidelity::CompleteLifecycle
    );
    assert!(matches!(
        evidence
            .interface_compatibility()
            .next()
            .expect("interface evidence exists")
            .assessment(),
        InterfaceCompatibilityAssessment::UnverifiedNewer(_)
    ));
    assert_eq!(
        evidence
            .observable_activity()
            .interface_basis()
            .next()
            .expect("activity basis exists")
            .behavior_revision()
            .as_str(),
        ACTIVITY_REVISION
    );
}

#[test]
fn required_activity_needs_an_explicit_prepared_profile() {
    let fixture = Fixture::new(activity_requirement(
        ActivityLifecycleFidelity::CompletionOnly,
    ));
    let failure = PreparedOperationEvidence::from_plan(fixture.plan(), fixture.access_evidence())
        .expect_err("required activity cannot be inferred after preflight");

    assert_eq!(failure.stage(), PreparationStage::Preflight);
    assert_eq!(
        failure.diagnostic().safe().code(),
        "swallowtail.prepared_operation.activity_profile_required"
    );
    assert_eq!(fixture.provider_effect_count, 0);
}

#[test]
fn unverified_newer_cannot_widen_the_qualified_profile_basis() {
    let fixture = Fixture::new(activity_requirement(
        ActivityLifecycleFidelity::CompletionOnly,
    ));
    let failure = PreparedOperationEvidence::from_plan_with_activity_profile(
        fixture.plan(),
        fixture.access_evidence(),
        full_activity_profile("activity-schema-v2"),
    )
    .expect_err("unverified newer version cannot select a wider behavior profile");

    assert_eq!(
        failure.diagnostic().safe().code(),
        "swallowtail.prepared_operation.activity_profile_basis_mismatch"
    );
    assert_eq!(fixture.provider_effect_count, 0);
}

#[test]
fn actual_profile_must_satisfy_preflighted_activity_constraints() {
    let fixture = Fixture::new(activity_requirement(
        ActivityLifecycleFidelity::CompleteLifecycle,
    ));
    let thin = ObservableActivityProfile::available(
        [activity_basis(ACTIVITY_REVISION)],
        [ActivityKindProfile::new(
            ActivityKindClass::AssistantMessage,
            ActivityLifecycleFidelity::CompletionOnly,
            [ActivityContentStream::FinalAnswerText],
            ActivityDisclosure::ProviderDisplayContent,
            [],
        )
        .expect("thin profile is valid")],
        ActivityUnknownEventPosture::FailClosed,
    )
    .expect("thin route profile is valid");
    let failure = PreparedOperationEvidence::from_plan_with_activity_profile(
        fixture.plan(),
        fixture.access_evidence(),
        thin,
    )
    .expect_err("actual profile cannot be thinner than preflight requirements");

    assert_eq!(
        failure.diagnostic().safe().code(),
        "swallowtail.prepared_operation.activity_constraint_mismatch"
    );
    assert_eq!(fixture.provider_effect_count, 0);
}

#[test]
fn prepared_profile_cannot_exceed_qualified_capability_evidence() {
    let fixture = Fixture::new(activity_requirement(
        ActivityLifecycleFidelity::CompletionOnly,
    ));
    let profile = ObservableActivityProfile::available(
        [activity_basis(ACTIVITY_REVISION)],
        [
            ActivityKindProfile::new(
                ActivityKindClass::AssistantMessage,
                ActivityLifecycleFidelity::CompletionOnly,
                [ActivityContentStream::FinalAnswerText],
                ActivityDisclosure::ProviderDisplayContent,
                [],
            )
            .expect("assistant profile is valid"),
            ActivityKindProfile::new(
                ActivityKindClass::Plan,
                ActivityLifecycleFidelity::CompletionOnly,
                [ActivityContentStream::PlanText],
                ActivityDisclosure::ProviderDisplayContent,
                [],
            )
            .expect("plan profile is valid"),
        ],
        ActivityUnknownEventPosture::FailClosed,
    )
    .expect("route profile is valid");
    let failure = PreparedOperationEvidence::from_plan_with_activity_profile(
        fixture.plan(),
        fixture.access_evidence(),
        profile,
    )
    .expect_err("prepared evidence cannot promote an unqualified activity kind");

    assert_eq!(
        failure.diagnostic().safe().code(),
        "swallowtail.prepared_operation.activity_profile_unqualified"
    );
    assert_eq!(fixture.provider_effect_count, 0);
}

#[test]
fn routes_without_activity_requirements_remain_usable_and_unpromoted() {
    let fixture = Fixture::new(None);
    let evidence = PreparedOperationEvidence::from_plan(fixture.plan(), fixture.access_evidence())
        .expect("ordinary route without activity requirements remains usable");

    assert_eq!(
        evidence.observable_activity().availability(),
        ObservableActivityAvailability::Unavailable
    );
    assert_eq!(
        evidence
            .observable_activity()
            .lifecycle(ActivityKindClass::AssistantMessage),
        ActivityLifecycleFidelity::Unavailable
    );
}

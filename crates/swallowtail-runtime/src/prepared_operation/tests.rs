use super::PreparedOperationEvidence;
use crate::PreparationStage;
use swallowtail_core::{
    ActivityContentStream, ActivityDisclosure, ActivityKindClass, ActivityKindProfile,
    ActivityLifecycleFidelity, ActivityUnknownEventPosture, InterfaceBehaviorRevision,
    InterfaceCompatibilityAssessment, InterfaceCompatibilityClaim, InterfaceCompatibilityClaimId,
    InterfaceNewerVersionPosture, InterfaceSupportStatus, InterfaceVersion, InterfaceVersionAxis,
    InterfaceVersionBinding, InterfaceVersionScheme, InterfaceVersionSegment,
    ObservableActivityAvailability, ObservableActivityProfile, PreflightDimension,
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

#[test]
fn opaque_set_preflight_rejects_unknown_points_before_effects() {
    let axis = InterfaceVersionAxis::new("fixture-executable").expect("axis is valid");
    let claim = opaque_claim(
        axis.clone(),
        [opaque_member(
            "runtime-current",
            "runtime-v1",
            InterfaceSupportStatus::Maintained,
        )],
    );
    let observed = InterfaceVersionBinding::new(
        axis,
        InterfaceVersion::new("runtime-unknown").expect("observed point is valid"),
    );
    let fixture = Fixture::with_interface_claim(claim, observed, None);

    let failure = fixture
        .plan_with_driver(fixture.driver())
        .expect_err("unknown opaque point fails preflight");
    assert_eq!(failure.dimension(), PreflightDimension::InterfaceVersion);
    assert_eq!(
        failure.diagnostic().code(),
        "swallowtail.preflight_rejected"
    );
    assert_eq!(fixture.provider_effect_count, 0);
}

#[test]
fn opaque_set_axis_mismatch_stays_driver_incompatible_at_preflight() {
    let claim_axis =
        InterfaceVersionAxis::new("fixture-other-runtime").expect("claim axis is valid");
    let claim = opaque_claim(
        claim_axis,
        [opaque_member(
            "runtime-current",
            "runtime-v1",
            InterfaceSupportStatus::Maintained,
        )],
    );
    let observed = InterfaceVersionBinding::new(
        InterfaceVersionAxis::new("fixture-executable").expect("observed axis is valid"),
        InterfaceVersion::new("runtime-current").expect("observed point is valid"),
    );
    let fixture = Fixture::with_interface_claim(claim, observed, None);

    let failure = fixture
        .plan_with_driver(fixture.driver())
        .expect_err("a claim on another axis cannot permit the binding");
    assert_eq!(failure.dimension(), PreflightDimension::InterfaceVersion);
    assert_eq!(
        failure.diagnostic().code(),
        "swallowtail.preflight_rejected"
    );
}

#[test]
fn opaque_set_membership_edits_stale_a_preflight_plan() {
    let axis = InterfaceVersionAxis::new("fixture-executable").expect("axis is valid");
    let initial_claim = opaque_claim(
        axis.clone(),
        [opaque_member(
            "runtime-old",
            "runtime-v1",
            InterfaceSupportStatus::Maintained,
        )],
    );
    let observed = InterfaceVersionBinding::new(
        axis.clone(),
        InterfaceVersion::new("runtime-old").expect("observed point is valid"),
    );
    let fixture = Fixture::with_interface_claim(initial_claim, observed, None);
    let plan = fixture.plan();

    let changed_claim = opaque_claim(
        axis,
        [
            opaque_member(
                "runtime-old",
                "runtime-v1",
                InterfaceSupportStatus::Deprecated,
            ),
            opaque_member(
                "runtime-current",
                "runtime-v2",
                InterfaceSupportStatus::Maintained,
            ),
        ],
    );
    let changed_driver = fixture.driver_with_claim(changed_claim);
    let stale = fixture
        .validate_plan_with_driver(&plan, &changed_driver)
        .expect_err("claim membership edit invalidates the frozen plan");
    assert_eq!(
        stale.diagnostic().code(),
        "swallowtail.preflight_plan_stale"
    );
}

#[test]
fn prepared_opaque_set_exposes_the_matched_member_assessment() {
    let axis = InterfaceVersionAxis::new("fixture-executable").expect("axis is valid");
    let claim = opaque_claim(
        axis.clone(),
        [
            opaque_member(
                "runtime-old",
                "runtime-v1",
                InterfaceSupportStatus::Deprecated,
            ),
            opaque_member(
                "runtime-current",
                "runtime-v2",
                InterfaceSupportStatus::Maintained,
            ),
        ],
    );
    let observed = InterfaceVersionBinding::new(
        axis,
        InterfaceVersion::new("runtime-current").expect("observed point is valid"),
    );
    let fixture = Fixture::with_interface_claim(claim, observed, None);
    let evidence = PreparedOperationEvidence::from_plan(fixture.plan(), fixture.access_evidence())
        .expect("listed opaque member prepares");

    let compatibility = evidence
        .interface_compatibility()
        .next()
        .expect("prepared binding has compatibility evidence");
    let InterfaceCompatibilityAssessment::Qualified(matched) = compatibility.assessment() else {
        panic!("listed member retains its qualified assessment");
    };
    assert_eq!(matched.behavior_revision().as_str(), "runtime-v2");
    assert_eq!(matched.support_status(), InterfaceSupportStatus::Maintained);
}

fn opaque_claim(
    axis: InterfaceVersionAxis,
    members: impl IntoIterator<Item = InterfaceVersionSegment>,
) -> InterfaceCompatibilityClaim {
    InterfaceCompatibilityClaim::new(
        InterfaceCompatibilityClaimId::new("fixture.opaque-runtime").expect("claim id is valid"),
        axis,
        InterfaceVersionScheme::Opaque,
        InterfaceNewerVersionPosture::QualifiedOnly,
        members,
        [],
    )
    .expect("opaque fixture claim is valid")
}

fn opaque_member(
    version: &str,
    revision: &str,
    support_status: InterfaceSupportStatus,
) -> InterfaceVersionSegment {
    InterfaceVersionSegment::exact(
        InterfaceVersion::new(version).expect("opaque member is valid"),
        InterfaceBehaviorRevision::new(revision).expect("behavior revision is valid"),
        support_status,
    )
}

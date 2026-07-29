use super::{
    ActivityContentStream, ActivityDisclosure, ActivityKindClass, ActivityKindProfile,
    ActivityLifecycleFidelity, ActivityUnknownEventPosture, ObservableActivityAvailability,
    ObservableActivityProfile,
};
use crate::{CapabilityConstraint, CapabilityRequirement};

fn assistant_profile() -> ActivityKindProfile {
    ActivityKindProfile::new(
        ActivityKindClass::AssistantMessage,
        ActivityLifecycleFidelity::CompleteLifecycle,
        [
            ActivityContentStream::IntermediateAssistantText,
            ActivityContentStream::FinalAnswerText,
        ],
        ActivityDisclosure::ProviderDisplayContent,
        [],
    )
    .expect("assistant profile is valid")
}

#[test]
fn richer_profiles_satisfy_thinner_exact_requirements() {
    let profile = ObservableActivityProfile::available(
        [],
        [assistant_profile()],
        ActivityUnknownEventPosture::FailClosed,
    )
    .expect("route profile is valid");
    let requirement = CapabilityRequirement::new(
        crate::Capability::ObservableActivity,
        [
            CapabilityConstraint::ObservableActivityKind(ActivityKindClass::AssistantMessage),
            CapabilityConstraint::ObservableActivityLifecycle(
                ActivityKindClass::AssistantMessage,
                ActivityLifecycleFidelity::CompletionOnly,
            ),
            CapabilityConstraint::ObservableActivityDisclosure(
                ActivityKindClass::AssistantMessage,
                ActivityDisclosure::IdentityAndLifecycleOnly,
            ),
        ],
    );

    assert!(profile.supports(&requirement));
    assert_eq!(
        profile.availability(),
        ObservableActivityAvailability::Available
    );
    assert_eq!(
        profile.lifecycle(ActivityKindClass::AssistantMessage),
        ActivityLifecycleFidelity::CompleteLifecycle
    );
}

#[test]
fn unavailable_and_malformed_profiles_do_not_promote_fidelity() {
    let unavailable =
        ObservableActivityProfile::unavailable([]).expect("unavailable profile is valid");
    assert_eq!(
        unavailable.lifecycle(ActivityKindClass::CommandExecution),
        ActivityLifecycleFidelity::Unavailable
    );
    assert!(unavailable.capability_requirement().is_none());
    assert!(
        ActivityKindProfile::new(
            ActivityKindClass::Plan,
            ActivityLifecycleFidelity::CompletionOnly,
            [ActivityContentStream::CommandOutput],
            ActivityDisclosure::ProviderDisplayContent,
            [],
        )
        .is_err()
    );
}

#[test]
fn unknown_kind_and_preservation_posture_are_one_exact_claim() {
    let unknown = ActivityKindProfile::new(
        ActivityKindClass::Unknown,
        ActivityLifecycleFidelity::CompletionOnly,
        [],
        ActivityDisclosure::IdentityAndLifecycleOnly,
        [],
    )
    .expect("unknown profile is valid");

    assert!(
        ObservableActivityProfile::available(
            [],
            [unknown.clone()],
            ActivityUnknownEventPosture::FailClosed,
        )
        .is_err()
    );
    assert!(
        ObservableActivityProfile::available(
            [],
            [assistant_profile()],
            ActivityUnknownEventPosture::PreserveNamespaced,
        )
        .is_err()
    );
    ObservableActivityProfile::available(
        [],
        [unknown],
        ActivityUnknownEventPosture::PreserveNamespaced,
    )
    .expect("unknown kind and preserve posture agree");
}

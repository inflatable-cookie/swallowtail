use swallowtail_core::{
    ActivityContentStream, ActivityDisclosure, ActivityInterfaceBasis, ActivityKindClass,
    ActivityKindProfile, ActivityLifecycleFidelity, ActivityUnknownEventPosture,
    ObservableActivityProfile,
};

pub(crate) fn activity_profile() -> ObservableActivityProfile {
    let binding = crate::anthropic_managed_facade_binding();
    let assessment = crate::anthropic_managed_facade_claim().assess(binding.version());
    let behavior_revision = assessment
        .behavior_revision()
        .expect("static managed-agent facade is qualified")
        .clone();
    ObservableActivityProfile::available(
        [ActivityInterfaceBasis::new(
            binding.axis().clone(),
            behavior_revision,
        )],
        [
            kind(
                ActivityKindClass::AssistantMessage,
                ActivityLifecycleFidelity::CompletionOnly,
                [],
                ActivityDisclosure::IdentityAndLifecycleOnly,
            ),
            kind(
                ActivityKindClass::ReasoningSummary,
                ActivityLifecycleFidelity::CompletionOnly,
                [],
                ActivityDisclosure::IdentityAndLifecycleOnly,
            ),
            kind(
                ActivityKindClass::ProviderOwnedTool,
                ActivityLifecycleFidelity::CompletionOnly,
                [],
                ActivityDisclosure::ProviderDisplayContent,
            ),
            kind(
                ActivityKindClass::Task,
                ActivityLifecycleFidelity::CompletionOnly,
                [],
                ActivityDisclosure::IdentityAndLifecycleOnly,
            ),
            kind(
                ActivityKindClass::WarningOrError,
                ActivityLifecycleFidelity::CompletionOnly,
                [],
                ActivityDisclosure::IdentityAndLifecycleOnly,
            ),
            kind(
                ActivityKindClass::Unknown,
                ActivityLifecycleFidelity::CompletionOnly,
                [],
                ActivityDisclosure::IdentityAndLifecycleOnly,
            ),
        ],
        ActivityUnknownEventPosture::PreserveNamespaced,
    )
    .expect("static managed-agent activity profile is valid")
}

fn kind(
    class: ActivityKindClass,
    lifecycle: ActivityLifecycleFidelity,
    streams: impl IntoIterator<Item = ActivityContentStream>,
    disclosure: ActivityDisclosure,
) -> ActivityKindProfile {
    ActivityKindProfile::new(class, lifecycle, streams, disclosure, [])
        .expect("static managed-agent activity kind is valid")
}

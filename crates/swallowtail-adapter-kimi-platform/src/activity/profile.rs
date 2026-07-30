use swallowtail_core::{
    ActivityContentStream, ActivityDisclosure, ActivityInterfaceBasis, ActivityKindClass,
    ActivityKindProfile, ActivityLifecycleFidelity, ActivityUnknownEventPosture, CapabilityProfile,
    CapabilityRequirement, ObservableActivityProfile,
};

pub(crate) fn activity_profile() -> ObservableActivityProfile {
    let binding = crate::kimi_platform_facade_binding();
    let assessment = crate::kimi_platform_facade_claim().assess(binding.version());
    let behavior = assessment
        .behavior_revision()
        .expect("static Kimi Platform facade is qualified")
        .clone();
    ObservableActivityProfile::available(
        [ActivityInterfaceBasis::new(
            binding.axis().clone(),
            behavior,
        )],
        [
            kind(
                ActivityKindClass::AssistantMessage,
                ActivityLifecycleFidelity::CompleteLifecycle,
                [ActivityContentStream::FinalAnswerText],
            ),
            kind(
                ActivityKindClass::ReasoningSummary,
                ActivityLifecycleFidelity::UpdateAndCompletion,
                [ActivityContentStream::ReasoningSummaryText],
            ),
        ],
        ActivityUnknownEventPosture::FailClosed,
    )
    .expect("static Kimi Platform activity profile is valid")
}

pub(crate) fn with_activity(
    capabilities: CapabilityProfile,
    profile: &ObservableActivityProfile,
) -> CapabilityProfile {
    let mut requirements = capabilities
        .iter()
        .map(|(capability, constraints)| {
            CapabilityRequirement::new(capability, constraints.iter().cloned())
        })
        .collect::<Vec<_>>();
    requirements.push(
        profile
            .capability_requirement()
            .expect("Kimi Platform activity is available"),
    );
    CapabilityProfile::new(requirements)
}

fn kind(
    class: ActivityKindClass,
    lifecycle: ActivityLifecycleFidelity,
    streams: impl IntoIterator<Item = ActivityContentStream>,
) -> ActivityKindProfile {
    ActivityKindProfile::new(
        class,
        lifecycle,
        streams,
        ActivityDisclosure::ProviderDisplayContent,
        [],
    )
    .expect("static Kimi Platform activity kind is valid")
}

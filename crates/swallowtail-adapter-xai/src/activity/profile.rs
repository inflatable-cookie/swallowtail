use swallowtail_core::{
    ActivityContentStream, ActivityDisclosure, ActivityInterfaceBasis, ActivityKindClass,
    ActivityKindProfile, ActivityLifecycleFidelity, ActivityUnknownEventPosture, CapabilityProfile,
    CapabilityRequirement, ObservableActivityProfile,
};

pub(crate) fn activity_profile() -> ObservableActivityProfile {
    let binding = crate::xai_responses_facade_binding();
    let assessment = crate::xai_responses_facade_claim().assess(binding.version());
    let behavior = assessment
        .behavior_revision()
        .expect("static xAI facade is qualified")
        .clone();
    ObservableActivityProfile::available(
        [ActivityInterfaceBasis::new(
            binding.axis().clone(),
            behavior,
        )],
        [ActivityKindProfile::new(
            ActivityKindClass::AssistantMessage,
            ActivityLifecycleFidelity::CompleteLifecycle,
            [ActivityContentStream::FinalAnswerText],
            ActivityDisclosure::ProviderDisplayContent,
            [],
        )
        .expect("static xAI activity kind is valid")],
        ActivityUnknownEventPosture::FailClosed,
    )
    .expect("static xAI activity profile is valid")
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
            .expect("xAI activity is available"),
    );
    CapabilityProfile::new(requirements)
}

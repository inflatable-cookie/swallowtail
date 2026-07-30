use swallowtail_core::{
    ActivityContentStream, ActivityCorrelationKind, ActivityDisclosure, ActivityInterfaceBasis,
    ActivityKindClass, ActivityKindProfile, ActivityLifecycleFidelity, ActivityUnknownEventPosture,
    CapabilityProfile, CapabilityRequirement, ObservableActivityProfile,
};

pub(crate) fn activity_profile(include_consumer_tool: bool) -> ObservableActivityProfile {
    let binding = crate::deepseek_facade_binding();
    let assessment = crate::deepseek_facade_claim().assess(binding.version());
    let behavior = assessment
        .behavior_revision()
        .expect("static DeepSeek facade is qualified")
        .clone();
    let mut kinds = vec![
        ActivityKindProfile::new(
            ActivityKindClass::AssistantMessage,
            ActivityLifecycleFidelity::UpdateAndCompletion,
            [ActivityContentStream::FinalAnswerText],
            ActivityDisclosure::ProviderDisplayContent,
            [],
        )
        .expect("static DeepSeek assistant activity is valid"),
    ];
    if include_consumer_tool {
        kinds.push(
            ActivityKindProfile::new(
                ActivityKindClass::ConsumerOwnedTool,
                ActivityLifecycleFidelity::CompletionOnly,
                [],
                ActivityDisclosure::IdentityAndLifecycleOnly,
                [ActivityCorrelationKind::DirectToolCall],
            )
            .expect("static DeepSeek tool activity is valid"),
        );
    }
    ObservableActivityProfile::available(
        [ActivityInterfaceBasis::new(
            binding.axis().clone(),
            behavior,
        )],
        kinds,
        ActivityUnknownEventPosture::FailClosed,
    )
    .expect("static DeepSeek activity profile is valid")
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
            .expect("DeepSeek activity is available"),
    );
    CapabilityProfile::new(requirements)
}

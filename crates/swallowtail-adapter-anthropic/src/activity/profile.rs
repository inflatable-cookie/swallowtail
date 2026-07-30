use swallowtail_core::{
    ActivityContentStream, ActivityCorrelationKind, ActivityDisclosure, ActivityInterfaceBasis,
    ActivityKindClass, ActivityKindProfile, ActivityLifecycleFidelity, ActivityUnknownEventPosture,
    CapabilityProfile, CapabilityRequirement, ObservableActivityProfile,
};

pub(crate) fn structured_profile(include_provider_search: bool) -> ObservableActivityProfile {
    profile(include_provider_search, false)
}

pub(crate) fn session_profile() -> ObservableActivityProfile {
    profile(false, true)
}

fn profile(
    include_provider_search: bool,
    include_consumer_tool: bool,
) -> ObservableActivityProfile {
    let binding = crate::anthropic_messages_facade_binding();
    let assessment = crate::anthropic_messages_facade_claim().assess(binding.version());
    let behavior = assessment
        .behavior_revision()
        .expect("static Anthropic facade is qualified")
        .clone();
    let mut kinds = vec![
        ActivityKindProfile::new(
            ActivityKindClass::AssistantMessage,
            ActivityLifecycleFidelity::CompleteLifecycle,
            [ActivityContentStream::FinalAnswerText],
            ActivityDisclosure::ProviderDisplayContent,
            [],
        )
        .expect("static Anthropic assistant activity is valid"),
    ];
    if include_provider_search {
        kinds.push(
            ActivityKindProfile::new(
                ActivityKindClass::ProviderOwnedTool,
                ActivityLifecycleFidelity::CompleteLifecycle,
                [],
                ActivityDisclosure::IdentityAndLifecycleOnly,
                [],
            )
            .expect("static Anthropic provider-tool activity is valid"),
        );
    }
    if include_consumer_tool {
        kinds.push(
            ActivityKindProfile::new(
                ActivityKindClass::ConsumerOwnedTool,
                ActivityLifecycleFidelity::CompleteLifecycle,
                [],
                ActivityDisclosure::IdentityAndLifecycleOnly,
                [ActivityCorrelationKind::DirectToolCall],
            )
            .expect("static Anthropic consumer-tool activity is valid"),
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
    .expect("static Anthropic activity profile is valid")
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
            .expect("Anthropic activity is available"),
    );
    CapabilityProfile::new(requirements)
}

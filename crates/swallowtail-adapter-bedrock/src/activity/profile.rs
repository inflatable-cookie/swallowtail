use swallowtail_core::{
    ActivityContentStream, ActivityDisclosure, ActivityInterfaceBasis, ActivityKindClass,
    ActivityKindProfile, ActivityLifecycleFidelity, ActivityUnknownEventPosture, CapabilityProfile,
    CapabilityRequirement, ObservableActivityProfile,
};

pub(crate) fn activity_profile() -> ObservableActivityProfile {
    let bindings = crate::bedrock_runtime_interface_bindings();
    let claims = crate::bedrock_runtime_interface_claims();
    let basis = bindings
        .iter()
        .map(|binding| {
            let claim = claims
                .iter()
                .find(|claim| claim.axis() == binding.axis())
                .expect("static Bedrock binding has one claim");
            let behavior = claim
                .assess(binding.version())
                .behavior_revision()
                .expect("static Bedrock interface is qualified")
                .clone();
            ActivityInterfaceBasis::new(binding.axis().clone(), behavior)
        })
        .collect::<Vec<_>>();
    ObservableActivityProfile::available(
        basis,
        [ActivityKindProfile::new(
            ActivityKindClass::AssistantMessage,
            ActivityLifecycleFidelity::CompleteLifecycle,
            [ActivityContentStream::FinalAnswerText],
            ActivityDisclosure::ProviderDisplayContent,
            [],
        )
        .expect("static Bedrock activity kind is valid")],
        ActivityUnknownEventPosture::FailClosed,
    )
    .expect("static Bedrock activity profile is valid")
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
            .expect("Bedrock activity is available"),
    );
    CapabilityProfile::new(requirements)
}

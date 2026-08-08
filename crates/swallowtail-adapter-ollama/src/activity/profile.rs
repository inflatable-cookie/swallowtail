use crate::OllamaPreparedIntegration;
use swallowtail_core::{
    ActivityContentStream, ActivityDisclosure, ActivityInterfaceBasis, ActivityKindClass,
    ActivityKindProfile, ActivityLifecycleFidelity, ActivityUnknownEventPosture, CapabilityProfile,
    CapabilityRequirement, ObservableActivityProfile,
};
use swallowtail_runtime::RuntimeFailure;

pub(crate) fn activity_profile(
    prepared: &OllamaPreparedIntegration,
) -> Result<ObservableActivityProfile, RuntimeFailure> {
    let binding = prepared.runtime().runtime_version();
    let assessment = crate::ollama_runtime_claim().assess(binding.version());
    // The prepared version is admitted, so a missing behavior revision is a
    // drift anomaly, not a provider signal; fail closed instead of panicking.
    let behavior = assessment
        .behavior_revision()
        .ok_or_else(|| {
            crate::failure::failure(
                "swallowtail.ollama.activity_profile_unavailable",
                "Ollama prepared runtime version has no qualified activity behavior",
            )
        })?
        .clone();
    Ok(ObservableActivityProfile::available(
        [ActivityInterfaceBasis::new(
            binding.axis().clone(),
            behavior,
        )],
        [ActivityKindProfile::new(
            ActivityKindClass::AssistantMessage,
            ActivityLifecycleFidelity::UpdateAndCompletion,
            [ActivityContentStream::FinalAnswerText],
            ActivityDisclosure::ProviderDisplayContent,
            [],
        )
        .expect("static Ollama activity kind is valid")],
        ActivityUnknownEventPosture::FailClosed,
    )
    .expect("static Ollama activity profile is valid"))
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
            .expect("Ollama activity is available"),
    );
    CapabilityProfile::new(requirements)
}

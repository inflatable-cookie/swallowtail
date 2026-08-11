use crate::ClaudeCodeResponsePreparedIntegration;
use swallowtail_core::{
    ActivityContentStream, ActivityDisclosure, ActivityInterfaceBasis, ActivityKindClass,
    ActivityKindProfile, ActivityLifecycleFidelity, ActivityUnknownEventPosture, CapabilityProfile,
    CapabilityRequirement, Diagnostic, InstalledExecutableCompatibility, ObservableActivityProfile,
    SafeDiagnostic,
};
use swallowtail_runtime::{PreparationFailure, PreparationStage};

pub(crate) fn activity_profile(
    prepared: &ClaudeCodeResponsePreparedIntegration,
) -> Result<ObservableActivityProfile, PreparationFailure> {
    let behavior_revision = match prepared.observation().compatibility() {
        InstalledExecutableCompatibility::Qualified(assessment) => {
            assessment.behavior_revision().clone()
        }
        InstalledExecutableCompatibility::UnverifiedNewer(_)
        | InstalledExecutableCompatibility::Incompatible => {
            return Err(failure(
                "swallowtail.claude_code.response_only.preparation.activity_version_incompatible",
                "Claude Code response-only activity requires the exact qualified version",
            ));
        }
    };
    ObservableActivityProfile::available(
        [ActivityInterfaceBasis::new(
            prepared.observation().version().axis().clone(),
            behavior_revision,
        )],
        [ActivityKindProfile::new(
            ActivityKindClass::AssistantMessage,
            ActivityLifecycleFidelity::CompletionOnly,
            [ActivityContentStream::FinalAnswerText],
            ActivityDisclosure::ProviderDisplayContent,
            [],
        )
        .map_err(|_| invalid_profile())?],
        ActivityUnknownEventPosture::FailClosed,
    )
    .map_err(|_| invalid_profile())
}

pub(crate) fn with_activity(
    capabilities: CapabilityProfile,
    activity: &ObservableActivityProfile,
) -> CapabilityProfile {
    let mut requirements = capabilities
        .iter()
        .map(|(capability, constraints)| {
            CapabilityRequirement::new(capability, constraints.iter().cloned())
        })
        .collect::<Vec<_>>();
    requirements.push(
        activity
            .capability_requirement()
            .expect("response-only activity is available"),
    );
    CapabilityProfile::new(requirements)
}

fn invalid_profile() -> PreparationFailure {
    failure(
        "swallowtail.claude_code.response_only.preparation.activity_profile_invalid",
        "Claude Code response-only activity profile could not be derived",
    )
}

fn failure(code: &'static str, message: &'static str) -> PreparationFailure {
    PreparationFailure::new(
        PreparationStage::Preflight,
        Diagnostic::new(SafeDiagnostic::new(code, message)),
    )
}

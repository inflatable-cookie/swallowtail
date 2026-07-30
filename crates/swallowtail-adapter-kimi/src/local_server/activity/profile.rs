use crate::local_server::KimiLocalServerPreparedIntegration;
use swallowtail_core::{
    ActivityContentStream, ActivityDisclosure, ActivityInterfaceBasis, ActivityKindClass,
    ActivityKindProfile, ActivityLifecycleFidelity, ActivityUnknownEventPosture, CapabilityProfile,
    CapabilityRequirement, ObservableActivityProfile, SubagentObservationFidelity,
};
use swallowtail_runtime::{PreparationFailure, PreparationStage};

pub(in crate::local_server) fn activity_profile(
    prepared: &KimiLocalServerPreparedIntegration,
) -> Result<ObservableActivityProfile, PreparationFailure> {
    let behavior_revision = prepared
        .server()
        .compatibility()
        .behavior_revision()
        .cloned()
        .ok_or_else(version_failure)?;
    ObservableActivityProfile::available(
        [ActivityInterfaceBasis::new(
            prepared.server().binding().axis().clone(),
            behavior_revision,
        )],
        [
            kind(
                ActivityKindClass::AssistantMessage,
                ActivityLifecycleFidelity::UpdateAndCompletion,
                [],
                ActivityDisclosure::IdentityAndLifecycleOnly,
            )?,
            kind(
                ActivityKindClass::ReasoningSummary,
                ActivityLifecycleFidelity::UpdateAndCompletion,
                [ActivityContentStream::ReasoningSummaryText],
                ActivityDisclosure::ProviderDisplayContent,
            )?,
            kind(
                ActivityKindClass::Task,
                ActivityLifecycleFidelity::CompleteLifecycle,
                [],
                ActivityDisclosure::IdentityAndLifecycleOnly,
            )?,
            kind(
                ActivityKindClass::ProviderOwnedTool,
                ActivityLifecycleFidelity::CompleteLifecycle,
                [],
                ActivityDisclosure::ProviderDisplayContent,
            )?,
            kind(
                ActivityKindClass::CommandExecution,
                ActivityLifecycleFidelity::CompleteLifecycle,
                [],
                ActivityDisclosure::IdentityAndLifecycleOnly,
            )?,
            kind(
                ActivityKindClass::SubagentOrCollaboration,
                ActivityLifecycleFidelity::CompleteLifecycle,
                [],
                ActivityDisclosure::IdentityAndLifecycleOnly,
            )?
            .with_subagent_observation(SubagentObservationFidelity::ParentAndMetadata)
            .map_err(|_| profile_failure())?,
            kind(
                ActivityKindClass::ContextCompaction,
                ActivityLifecycleFidelity::CompleteLifecycle,
                [],
                ActivityDisclosure::IdentityAndLifecycleOnly,
            )?,
            kind(
                ActivityKindClass::WarningOrError,
                ActivityLifecycleFidelity::CompletionOnly,
                [],
                ActivityDisclosure::IdentityAndLifecycleOnly,
            )?,
            kind(
                ActivityKindClass::Unknown,
                ActivityLifecycleFidelity::CompletionOnly,
                [],
                ActivityDisclosure::IdentityAndLifecycleOnly,
            )?,
        ],
        ActivityUnknownEventPosture::PreserveNamespaced,
    )
    .map_err(|_| profile_failure())
}

pub(in crate::local_server) fn with_activity(
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
            .expect("prepared Kimi local activity is available"),
    );
    CapabilityProfile::new(requirements)
}

fn kind(
    class: ActivityKindClass,
    lifecycle: ActivityLifecycleFidelity,
    streams: impl IntoIterator<Item = ActivityContentStream>,
    disclosure: ActivityDisclosure,
) -> Result<ActivityKindProfile, PreparationFailure> {
    ActivityKindProfile::new(class, lifecycle, streams, disclosure, [])
        .map_err(|_| profile_failure())
}

fn version_failure() -> PreparationFailure {
    failure(
        "swallowtail.kimi.local_server.preparation.activity_version_incompatible",
        "Kimi local-server activity requires a permitted server version",
    )
}

fn profile_failure() -> PreparationFailure {
    failure(
        "swallowtail.kimi.local_server.preparation.activity_profile_invalid",
        "Kimi local-server activity profile could not be derived",
    )
}

fn failure(code: &'static str, message: &'static str) -> PreparationFailure {
    PreparationFailure::new(
        PreparationStage::Preflight,
        swallowtail_core::Diagnostic::new(swallowtail_core::SafeDiagnostic::new(code, message)),
    )
}

use crate::OpenCodePreparedIntegration;
use swallowtail_core::{
    ActivityContentStream, ActivityDisclosure, ActivityInterfaceBasis, ActivityKindClass,
    ActivityKindProfile, ActivityLifecycleFidelity, ActivityUnknownEventPosture, CapabilityProfile,
    CapabilityRequirement, ObservableActivityProfile,
};
use swallowtail_runtime::{PreparationFailure, PreparationStage};

pub(crate) fn activity_profile(
    prepared: &OpenCodePreparedIntegration,
) -> Result<ObservableActivityProfile, PreparationFailure> {
    let behavior_revision = prepared
        .server()
        .compatibility()
        .behavior_revision()
        .cloned()
        .ok_or_else(version_failure)?;
    let thin = behavior_revision.as_str() == "opencode.http-sse.surface-04";
    let mut kinds = vec![kind(
        ActivityKindClass::AssistantMessage,
        ActivityLifecycleFidelity::UpdateAndCompletion,
        [],
        ActivityDisclosure::IdentityAndLifecycleOnly,
    )?];
    if !thin {
        kinds.extend([
            kind(
                ActivityKindClass::ReasoningSummary,
                ActivityLifecycleFidelity::UpdateAndCompletion,
                [ActivityContentStream::ReasoningSummaryText],
                ActivityDisclosure::ProviderDisplayContent,
            )?,
            kind(
                ActivityKindClass::ProviderOwnedTool,
                ActivityLifecycleFidelity::CompleteLifecycle,
                [],
                ActivityDisclosure::ProviderDisplayContent,
            )?,
            kind(
                ActivityKindClass::Task,
                ActivityLifecycleFidelity::CompletionOnly,
                [],
                ActivityDisclosure::IdentityAndLifecycleOnly,
            )?,
        ]);
    }
    kinds.extend([
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
    ]);
    ObservableActivityProfile::available(
        [ActivityInterfaceBasis::new(
            prepared.server().binding().axis().clone(),
            behavior_revision,
        )],
        kinds,
        ActivityUnknownEventPosture::PreserveNamespaced,
    )
    .map_err(|_| profile_failure())
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
            .expect("prepared OpenCode activity is available"),
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
        "swallowtail.opencode.preparation.activity_version_incompatible",
        "OpenCode activity requires a permitted server version",
    )
}

fn profile_failure() -> PreparationFailure {
    failure(
        "swallowtail.opencode.preparation.activity_profile_invalid",
        "OpenCode activity profile could not be derived",
    )
}

fn failure(code: &'static str, message: &'static str) -> PreparationFailure {
    PreparationFailure::new(
        PreparationStage::Preflight,
        swallowtail_core::Diagnostic::new(swallowtail_core::SafeDiagnostic::new(code, message)),
    )
}

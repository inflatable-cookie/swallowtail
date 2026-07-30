use crate::KimiHeadlessPreparedIntegration;
use swallowtail_core::{
    ActivityContentStream, ActivityDisclosure, ActivityInterfaceBasis, ActivityKindClass,
    ActivityKindProfile, ActivityLifecycleFidelity, ActivityUnknownEventPosture, CapabilityProfile,
    CapabilityRequirement, Diagnostic, InstalledExecutableCompatibility, ObservableActivityProfile,
    SafeDiagnostic,
};
use swallowtail_runtime::{PreparationFailure, PreparationStage};

pub(crate) fn activity_profile(
    prepared: &KimiHeadlessPreparedIntegration,
) -> Result<ObservableActivityProfile, PreparationFailure> {
    let behavior_revision = match prepared.observation().compatibility() {
        InstalledExecutableCompatibility::Qualified(assessment) => {
            assessment.behavior_revision().clone()
        }
        InstalledExecutableCompatibility::UnverifiedNewer(assessment) => {
            assessment.behavior_revision().clone()
        }
        InstalledExecutableCompatibility::Incompatible => {
            return Err(failure(
                "swallowtail.kimi.headless.preparation.activity_version_incompatible",
                "Kimi Code headless activity requires a permitted executable version",
            ));
        }
    };
    ObservableActivityProfile::available(
        [ActivityInterfaceBasis::new(
            prepared.observation().version().axis().clone(),
            behavior_revision,
        )],
        [
            kind(
                ActivityKindClass::AssistantMessage,
                ActivityLifecycleFidelity::CompletionOnly,
                [ActivityContentStream::IntermediateAssistantText],
                ActivityDisclosure::ProviderDisplayContent,
            )?,
            kind(
                ActivityKindClass::ProviderOwnedTool,
                ActivityLifecycleFidelity::CompletionOnly,
                [],
                ActivityDisclosure::ProviderDisplayContent,
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
    .map_err(|_| {
        failure(
            "swallowtail.kimi.headless.preparation.activity_profile_invalid",
            "Kimi Code headless activity profile could not be derived",
        )
    })
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
            .expect("prepared Kimi Code headless activity is available"),
    );
    CapabilityProfile::new(requirements)
}

fn kind(
    class: ActivityKindClass,
    lifecycle: ActivityLifecycleFidelity,
    streams: impl IntoIterator<Item = ActivityContentStream>,
    disclosure: ActivityDisclosure,
) -> Result<ActivityKindProfile, PreparationFailure> {
    ActivityKindProfile::new(class, lifecycle, streams, disclosure, []).map_err(|_| {
        failure(
            "swallowtail.kimi.headless.preparation.activity_profile_invalid",
            "Kimi Code headless activity profile could not be derived",
        )
    })
}

fn failure(code: &'static str, message: &'static str) -> PreparationFailure {
    PreparationFailure::new(
        PreparationStage::Preflight,
        Diagnostic::new(SafeDiagnostic::new(code, message)),
    )
}

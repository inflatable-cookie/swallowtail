use swallowtail_core::{
    ActivityContentStream, ActivityDisclosure, ActivityInterfaceBasis, ActivityKindClass,
    ActivityKindProfile, ActivityLifecycleFidelity, ActivityUnknownEventPosture, Capability,
    CapabilityProfile, CapabilityRequirement, InstalledExecutableCompatibility,
    InstalledExecutableObservation, ObservableActivityProfile,
};
use swallowtail_runtime::{PreparationFailure, PreparationStage};

pub(crate) fn acp(
    observation: &InstalledExecutableObservation,
) -> Result<ObservableActivityProfile, PreparationFailure> {
    profile(
        observation,
        [
            kind(
                ActivityKindClass::AssistantMessage,
                ActivityLifecycleFidelity::UpdateAndCompletion,
                [ActivityContentStream::FinalAnswerText],
                ActivityDisclosure::ProviderDisplayContent,
            )?,
            kind(
                ActivityKindClass::ReasoningSummary,
                ActivityLifecycleFidelity::UpdateAndCompletion,
                [ActivityContentStream::ReasoningSummaryText],
                ActivityDisclosure::ProviderDisplayContent,
            )?,
            kind(
                ActivityKindClass::ProviderOwnedTool,
                ActivityLifecycleFidelity::CompleteLifecycle,
                [ActivityContentStream::ProviderToolDisplay],
                ActivityDisclosure::ProviderDisplayContent,
            )?,
            kind(
                ActivityKindClass::Plan,
                ActivityLifecycleFidelity::UpdateAndCompletion,
                [ActivityContentStream::PlanText],
                ActivityDisclosure::ProviderDisplayContent,
            )?,
            kind(
                ActivityKindClass::Unknown,
                ActivityLifecycleFidelity::CompletionOnly,
                [],
                ActivityDisclosure::IdentityAndLifecycleOnly,
            )?,
        ],
    )
}

pub(crate) fn with_activity(
    capabilities: CapabilityProfile,
    activity: &ObservableActivityProfile,
) -> CapabilityProfile {
    let mut requirements = capabilities
        .iter()
        .filter(|(capability, _)| *capability != Capability::ObservableActivity)
        .map(|(capability, constraints)| {
            CapabilityRequirement::new(capability, constraints.iter().cloned())
        })
        .collect::<Vec<_>>();
    requirements.push(
        activity
            .capability_requirement()
            .expect("prepared Cursor activity is available"),
    );
    CapabilityProfile::new(requirements)
}

pub(crate) fn headless(
    observation: &InstalledExecutableObservation,
) -> Result<ObservableActivityProfile, PreparationFailure> {
    profile(
        observation,
        [
            kind(
                ActivityKindClass::AssistantMessage,
                ActivityLifecycleFidelity::CompleteLifecycle,
                [ActivityContentStream::FinalAnswerText],
                ActivityDisclosure::ProviderDisplayContent,
            )?,
            kind(
                ActivityKindClass::ReasoningSummary,
                ActivityLifecycleFidelity::CompleteLifecycle,
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
                ActivityKindClass::Unknown,
                ActivityLifecycleFidelity::CompletionOnly,
                [],
                ActivityDisclosure::IdentityAndLifecycleOnly,
            )?,
        ],
    )
}

fn profile<const N: usize>(
    observation: &InstalledExecutableObservation,
    kinds: [ActivityKindProfile; N],
) -> Result<ObservableActivityProfile, PreparationFailure> {
    let behavior = match observation.compatibility() {
        InstalledExecutableCompatibility::Qualified(assessment) => {
            assessment.behavior_revision().clone()
        }
        InstalledExecutableCompatibility::UnverifiedNewer(assessment) => {
            assessment.behavior_revision().clone()
        }
        InstalledExecutableCompatibility::Incompatible => {
            return Err(failure(
                "swallowtail.cursor.preparation.activity_version_incompatible",
                "Cursor activity requires a permitted executable version",
            ));
        }
    };
    ObservableActivityProfile::available(
        [ActivityInterfaceBasis::new(
            observation.version().axis().clone(),
            behavior,
        )],
        kinds,
        ActivityUnknownEventPosture::PreserveNamespaced,
    )
    .map_err(|_| {
        failure(
            "swallowtail.cursor.preparation.activity_profile_invalid",
            "Cursor activity profile could not be derived",
        )
    })
}

fn kind(
    class: ActivityKindClass,
    lifecycle: ActivityLifecycleFidelity,
    streams: impl IntoIterator<Item = ActivityContentStream>,
    disclosure: ActivityDisclosure,
) -> Result<ActivityKindProfile, PreparationFailure> {
    ActivityKindProfile::new(class, lifecycle, streams, disclosure, []).map_err(|_| {
        failure(
            "swallowtail.cursor.preparation.activity_profile_invalid",
            "Cursor activity profile could not be derived",
        )
    })
}

fn failure(code: &'static str, message: &'static str) -> PreparationFailure {
    super::failure(PreparationStage::Preflight, code, message)
}

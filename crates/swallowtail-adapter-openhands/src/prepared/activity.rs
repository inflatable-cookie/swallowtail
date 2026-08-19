use swallowtail_core::{
    ActivityContentStream, ActivityDisclosure, ActivityInterfaceBasis, ActivityKindClass,
    ActivityKindProfile, ActivityLifecycleFidelity, ActivityUnknownEventPosture, Capability,
    CapabilityProfile, CapabilityRequirement, InstalledExecutableCompatibility,
    InstalledExecutableObservation, ObservableActivityProfile,
};
use swallowtail_runtime::{PreparationFailure, PreparationStage};

pub(super) fn profile(
    observation: &InstalledExecutableObservation,
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
                "swallowtail.openhands.agent_server.preparation.activity_version_incompatible",
                "OpenHands Agent Server activity requires the exact permitted package",
            ));
        }
    };
    ObservableActivityProfile::available(
        [ActivityInterfaceBasis::new(
            observation.version().axis().clone(),
            behavior,
        )],
        [kind(
            ActivityKindClass::AssistantMessage,
            ActivityLifecycleFidelity::UpdateAndCompletion,
            [ActivityContentStream::FinalAnswerText],
            ActivityDisclosure::ProviderDisplayContent,
        )?],
        ActivityUnknownEventPosture::FailClosed,
    )
    .map_err(|_| {
        failure(
            "swallowtail.openhands.agent_server.preparation.activity_profile_invalid",
            "OpenHands Agent Server activity profile could not be derived",
        )
    })
}

pub(super) fn with_activity(
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
            .expect("prepared OpenHands Agent Server activity is available"),
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
            "swallowtail.openhands.agent_server.preparation.activity_profile_invalid",
            "OpenHands Agent Server activity profile could not be derived",
        )
    })
}

fn failure(code: &'static str, message: &'static str) -> PreparationFailure {
    super::failure(PreparationStage::Preflight, code, message)
}

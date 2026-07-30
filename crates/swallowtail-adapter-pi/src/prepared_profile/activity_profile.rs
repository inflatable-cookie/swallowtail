use super::plan::failure;
use crate::PiPreparedIntegration;
use swallowtail_core::{
    ActivityContentStream, ActivityDisclosure, ActivityInterfaceBasis, ActivityKindClass,
    ActivityKindProfile, ActivityLifecycleFidelity, ActivityUnknownEventPosture, CapabilityProfile,
    CapabilityRequirement, InstalledExecutableCompatibility, ObservableActivityProfile,
};
use swallowtail_runtime::PreparationFailure;

pub(super) fn activity_profile(
    prepared: &PiPreparedIntegration,
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
                "swallowtail.pi.preparation.activity_version_incompatible",
                "Pi RPC activity requires a permitted executable version",
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
                ActivityLifecycleFidelity::CompleteLifecycle,
                [],
                ActivityDisclosure::IdentityAndLifecycleOnly,
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
                ActivityKindClass::ContextCompaction,
                ActivityLifecycleFidelity::CompleteLifecycle,
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
    .map_err(|_| {
        failure(
            "swallowtail.pi.preparation.activity_profile_invalid",
            "Pi RPC activity profile could not be derived",
        )
    })
}

pub(super) fn with_activity(
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
            .expect("prepared Pi activity is available"),
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
            "swallowtail.pi.preparation.activity_profile_invalid",
            "Pi RPC activity profile could not be derived",
        )
    })
}

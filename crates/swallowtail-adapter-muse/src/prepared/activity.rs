use swallowtail_core::{
    ActivityDisclosure, ActivityInterfaceBasis, ActivityKindClass, ActivityKindProfile,
    ActivityLifecycleFidelity, ActivityUnknownEventPosture, Capability, CapabilityProfile,
    CapabilityRequirement, InstalledExecutableCompatibility, InstalledExecutableObservation,
    ObservableActivityProfile,
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
                "swallowtail.muse_code.preparation.activity_version_incompatible",
                "Muse Code activity requires the exact permitted payload revision",
            ));
        }
    };
    ObservableActivityProfile::available(
        [ActivityInterfaceBasis::new(
            observation.version().axis().clone(),
            behavior,
        )],
        [
            kind(
                ActivityKindClass::Task,
                ActivityLifecycleFidelity::CompleteLifecycle,
            )?,
            kind(
                ActivityKindClass::Unknown,
                ActivityLifecycleFidelity::CompletionOnly,
            )?,
        ],
        ActivityUnknownEventPosture::PreserveNamespaced,
    )
    .map_err(|_| {
        failure(
            "swallowtail.muse_code.preparation.activity_profile_invalid",
            "Muse Code activity profile could not be derived",
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
            .expect("prepared Muse Code activity is available"),
    );
    CapabilityProfile::new(requirements)
}

fn kind(
    class: ActivityKindClass,
    lifecycle: ActivityLifecycleFidelity,
) -> Result<ActivityKindProfile, PreparationFailure> {
    ActivityKindProfile::new(
        class,
        lifecycle,
        [],
        ActivityDisclosure::IdentityAndLifecycleOnly,
        [],
    )
    .map_err(|_| {
        failure(
            "swallowtail.muse_code.preparation.activity_profile_invalid",
            "Muse Code activity profile could not be derived",
        )
    })
}

fn failure(code: &'static str, message: &'static str) -> PreparationFailure {
    super::failure(PreparationStage::Preflight, code, message)
}

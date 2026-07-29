use crate::CodexPreparedIntegration;
use crate::prepared_profile::plan::failure;
use semver::Version;
use swallowtail_core::{
    ActivityContentStream, ActivityDisclosure, ActivityInterfaceBasis, ActivityKindClass,
    ActivityKindProfile, ActivityLifecycleFidelity, ActivityUnknownEventPosture,
    InstalledExecutableCompatibility, ObservableActivityProfile,
};
use swallowtail_runtime::PreparationFailure;

pub(super) fn exec_activity_profile(
    prepared: &CodexPreparedIntegration,
) -> Result<ObservableActivityProfile, PreparationFailure> {
    let observed =
        Version::parse(prepared.observation().version().version().as_str()).map_err(|_| {
            failure(
                "swallowtail.codex.preparation.activity_version_invalid",
                "Prepared Codex activity profile requires a semantic executable version",
            )
        })?;
    let qualified = observed.min(Version::new(0, 145, 0));
    let behavior_revision = match prepared.observation().compatibility() {
        InstalledExecutableCompatibility::Qualified(assessment) => {
            assessment.behavior_revision().clone()
        }
        InstalledExecutableCompatibility::UnverifiedNewer(assessment) => {
            assessment.behavior_revision().clone()
        }
        InstalledExecutableCompatibility::Incompatible => {
            return Err(failure(
                "swallowtail.codex.preparation.activity_version_incompatible",
                "Prepared Codex activity profile requires a permitted executable version",
            ));
        }
    };
    let basis = [ActivityInterfaceBasis::new(
        prepared.observation().version().axis().clone(),
        behavior_revision,
    )];
    let mut kinds = baseline_profiles()?;
    if qualified >= Version::new(0, 92, 0) {
        kinds.push(profile(
            ActivityKindClass::SubagentOrCollaboration,
            ActivityLifecycleFidelity::CompleteLifecycle,
            [ActivityContentStream::NormalizedSummary],
            ActivityDisclosure::AdapterNormalizedSummary,
        )?);
    }
    ObservableActivityProfile::available(
        basis,
        kinds,
        ActivityUnknownEventPosture::PreserveNamespaced,
    )
    .map_err(|_| invalid_profile())
}

fn baseline_profiles() -> Result<Vec<ActivityKindProfile>, PreparationFailure> {
    Ok(vec![
        profile(
            ActivityKindClass::AssistantMessage,
            ActivityLifecycleFidelity::CompletionOnly,
            [ActivityContentStream::FinalAnswerText],
            ActivityDisclosure::ProviderDisplayContent,
        )?,
        profile(
            ActivityKindClass::ReasoningSummary,
            ActivityLifecycleFidelity::CompletionOnly,
            [ActivityContentStream::ReasoningSummaryText],
            ActivityDisclosure::ProviderDisplayContent,
        )?,
        profile(
            ActivityKindClass::CommandExecution,
            ActivityLifecycleFidelity::CompleteLifecycle,
            [ActivityContentStream::CommandOutput],
            ActivityDisclosure::ProviderDisplayContent,
        )?,
        profile(
            ActivityKindClass::FileChange,
            ActivityLifecycleFidelity::CompletionOnly,
            [ActivityContentStream::FileChangeOutput],
            ActivityDisclosure::ProviderDisplayContent,
        )?,
        profile(
            ActivityKindClass::ProviderOwnedTool,
            ActivityLifecycleFidelity::CompleteLifecycle,
            [ActivityContentStream::ProviderToolDisplay],
            ActivityDisclosure::ProviderDisplayContent,
        )?,
        profile(
            ActivityKindClass::ExternalSearch,
            ActivityLifecycleFidelity::CompleteLifecycle,
            [ActivityContentStream::ProviderToolDisplay],
            ActivityDisclosure::ProviderDisplayContent,
        )?,
        profile(
            ActivityKindClass::Task,
            ActivityLifecycleFidelity::CompleteLifecycle,
            [ActivityContentStream::NormalizedSummary],
            ActivityDisclosure::AdapterNormalizedSummary,
        )?,
        profile(
            ActivityKindClass::WarningOrError,
            ActivityLifecycleFidelity::CompletionOnly,
            [ActivityContentStream::NormalizedSummary],
            ActivityDisclosure::AdapterNormalizedSummary,
        )?,
        profile(
            ActivityKindClass::Unknown,
            ActivityLifecycleFidelity::CompleteLifecycle,
            [],
            ActivityDisclosure::IdentityAndLifecycleOnly,
        )?,
    ])
}

fn profile(
    kind: ActivityKindClass,
    lifecycle: ActivityLifecycleFidelity,
    streams: impl IntoIterator<Item = ActivityContentStream>,
    disclosure: ActivityDisclosure,
) -> Result<ActivityKindProfile, PreparationFailure> {
    ActivityKindProfile::new(kind, lifecycle, streams, disclosure, [])
        .map_err(|_| invalid_profile())
}

fn invalid_profile() -> PreparationFailure {
    failure(
        "swallowtail.codex.preparation.activity_profile_invalid",
        "Codex exec activity profile could not be derived",
    )
}

use crate::CodexPreparedIntegration;
use crate::prepared_profile::plan::failure;
use semver::Version;
use swallowtail_core::{
    ActivityContentStream, ActivityCorrelationKind, ActivityDisclosure, ActivityInterfaceBasis,
    ActivityKindClass, ActivityKindProfile, ActivityLifecycleFidelity, ActivityUnknownEventPosture,
    InstalledExecutableCompatibility, ObservableActivityProfile,
};
use swallowtail_runtime::PreparationFailure;

pub(super) fn app_server_activity_profile(
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
    let mut kinds = baseline_profiles(&qualified)?;
    if qualified >= Version::new(0, 85, 0) {
        kinds.push(profile(
            ActivityKindClass::SubagentOrCollaboration,
            ActivityLifecycleFidelity::CompleteLifecycle,
            [ActivityContentStream::NormalizedSummary],
            ActivityDisclosure::AdapterNormalizedSummary,
            [],
        )?);
    }
    if qualified >= Version::new(0, 106, 0) {
        kinds.push(profile(
            ActivityKindClass::ConsumerOwnedTool,
            ActivityLifecycleFidelity::CompleteLifecycle,
            [ActivityContentStream::ProviderToolDisplay],
            ActivityDisclosure::ProviderDisplayContent,
            [ActivityCorrelationKind::Callback],
        )?);
    }
    if qualified >= Version::new(0, 114, 0) {
        kinds.push(profile(
            ActivityKindClass::Hook,
            ActivityLifecycleFidelity::CompleteLifecycle,
            [ActivityContentStream::NormalizedSummary],
            ActivityDisclosure::AdapterNormalizedSummary,
            [],
        )?);
    }
    ObservableActivityProfile::available(
        basis,
        kinds,
        ActivityUnknownEventPosture::PreserveNamespaced,
    )
    .map_err(|_| {
        failure(
            "swallowtail.codex.preparation.activity_profile_invalid",
            "Codex app-server activity profile could not be derived",
        )
    })
}

fn baseline_profiles(version: &Version) -> Result<Vec<ActivityKindProfile>, PreparationFailure> {
    let phased_messages = *version >= Version::new(0, 105, 0);
    let assistant_streams = if phased_messages {
        vec![
            ActivityContentStream::IntermediateAssistantText,
            ActivityContentStream::FinalAnswerText,
        ]
    } else {
        Vec::new()
    };
    let assistant_disclosure = if phased_messages {
        ActivityDisclosure::ProviderDisplayContent
    } else {
        ActivityDisclosure::IdentityAndLifecycleOnly
    };
    let compaction_lifecycle = if *version >= Version::new(0, 93, 0) {
        ActivityLifecycleFidelity::CompleteLifecycle
    } else {
        ActivityLifecycleFidelity::CompletionOnly
    };
    let plan_lifecycle = if *version >= Version::new(0, 93, 0) {
        ActivityLifecycleFidelity::CompleteLifecycle
    } else {
        ActivityLifecycleFidelity::UpdateAndCompletion
    };
    Ok(vec![
        profile(
            ActivityKindClass::AssistantMessage,
            ActivityLifecycleFidelity::CompleteLifecycle,
            assistant_streams,
            assistant_disclosure,
            [],
        )?,
        profile(
            ActivityKindClass::ReasoningSummary,
            ActivityLifecycleFidelity::CompleteLifecycle,
            [ActivityContentStream::ReasoningSummaryText],
            ActivityDisclosure::ProviderDisplayContent,
            [],
        )?,
        profile(
            ActivityKindClass::Plan,
            plan_lifecycle,
            [ActivityContentStream::PlanText],
            ActivityDisclosure::ProviderDisplayContent,
            [],
        )?,
        profile(
            ActivityKindClass::CommandExecution,
            ActivityLifecycleFidelity::CompleteLifecycle,
            [ActivityContentStream::CommandOutput],
            ActivityDisclosure::ProviderDisplayContent,
            [],
        )?,
        profile(
            ActivityKindClass::FileChange,
            ActivityLifecycleFidelity::CompleteLifecycle,
            [ActivityContentStream::FileChangeOutput],
            ActivityDisclosure::ProviderDisplayContent,
            [],
        )?,
        profile(
            ActivityKindClass::ProviderOwnedTool,
            ActivityLifecycleFidelity::CompleteLifecycle,
            [ActivityContentStream::ProviderToolDisplay],
            ActivityDisclosure::ProviderDisplayContent,
            [],
        )?,
        profile(
            ActivityKindClass::ExternalSearch,
            ActivityLifecycleFidelity::CompleteLifecycle,
            [ActivityContentStream::ProviderToolDisplay],
            ActivityDisclosure::ProviderDisplayContent,
            [],
        )?,
        profile(
            ActivityKindClass::ImageView,
            ActivityLifecycleFidelity::CompletionOnly,
            [ActivityContentStream::ProviderToolDisplay],
            ActivityDisclosure::ProviderDisplayContent,
            [],
        )?,
        profile(
            ActivityKindClass::ReviewTransition,
            ActivityLifecycleFidelity::CompletionOnly,
            [ActivityContentStream::NormalizedSummary],
            ActivityDisclosure::AdapterNormalizedSummary,
            [],
        )?,
        profile(
            ActivityKindClass::ContextCompaction,
            compaction_lifecycle,
            [],
            ActivityDisclosure::IdentityAndLifecycleOnly,
            [],
        )?,
        profile(
            ActivityKindClass::Unknown,
            ActivityLifecycleFidelity::CompleteLifecycle,
            [],
            ActivityDisclosure::IdentityAndLifecycleOnly,
            [ActivityCorrelationKind::ProviderRequest],
        )?,
    ])
}

fn profile(
    kind: ActivityKindClass,
    lifecycle: ActivityLifecycleFidelity,
    streams: impl IntoIterator<Item = ActivityContentStream>,
    disclosure: ActivityDisclosure,
    correlations: impl IntoIterator<Item = ActivityCorrelationKind>,
) -> Result<ActivityKindProfile, PreparationFailure> {
    ActivityKindProfile::new(kind, lifecycle, streams, disclosure, correlations).map_err(|_| {
        failure(
            "swallowtail.codex.preparation.activity_profile_invalid",
            "Codex app-server activity profile could not be derived",
        )
    })
}

use super::ObservableActivityTraceFixture;
use super::support::{
    activity_id, content, event, kind_profile, observation, provider_ref, trace, with_content,
};
use swallowtail_core::{
    ActivityContentStream, ActivityDisclosure, ActivityKindClass, ActivityLifecycleFidelity,
    ObservableActivityProfile,
};
use swallowtail_runtime::{
    ActivityKind, ActivityLifecyclePhase, ActivityStatus, RuntimeEvent, RuntimeEventKind,
};

pub(super) fn complete() -> ObservableActivityTraceFixture {
    let kind = ActivityKind::Task;
    let id = activity_id("fixture.activity.complete");
    trace(
        kind_profile(
            ActivityKindClass::Task,
            ActivityLifecycleFidelity::CompleteLifecycle,
            [],
            ActivityDisclosure::IdentityAndLifecycleOnly,
            [],
        ),
        [
            event(
                2,
                observation(
                    id.clone(),
                    kind.clone(),
                    ActivityLifecyclePhase::Started,
                    ActivityStatus::Pending,
                    None,
                    ActivityDisclosure::IdentityAndLifecycleOnly,
                )
                .with_provider_activity_ref(provider_ref()),
            ),
            event(
                3,
                observation(
                    id.clone(),
                    kind.clone(),
                    ActivityLifecyclePhase::Updated,
                    ActivityStatus::InProgress,
                    None,
                    ActivityDisclosure::IdentityAndLifecycleOnly,
                )
                .with_provider_activity_ref(provider_ref()),
            ),
            event(
                4,
                observation(
                    id,
                    kind,
                    ActivityLifecyclePhase::Completed,
                    ActivityStatus::Completed,
                    None,
                    ActivityDisclosure::IdentityAndLifecycleOnly,
                )
                .with_provider_activity_ref(provider_ref()),
            ),
        ],
    )
}

pub(super) fn update_and_completion() -> ObservableActivityTraceFixture {
    let id = activity_id("fixture.activity.update-completion");
    trace(
        kind_profile(
            ActivityKindClass::Plan,
            ActivityLifecycleFidelity::UpdateAndCompletion,
            [ActivityContentStream::PlanText],
            ActivityDisclosure::ProviderDisplayContent,
            [],
        ),
        [
            event(
                2,
                with_content(
                    observation(
                        id.clone(),
                        ActivityKind::Plan,
                        ActivityLifecyclePhase::Updated,
                        ActivityStatus::InProgress,
                        None,
                        ActivityDisclosure::ProviderDisplayContent,
                    ),
                    ActivityContentStream::PlanText,
                    "Inspect the portable contract",
                ),
            ),
            event(
                3,
                with_content(
                    observation(
                        id,
                        ActivityKind::Plan,
                        ActivityLifecyclePhase::Completed,
                        ActivityStatus::Completed,
                        None,
                        ActivityDisclosure::ProviderDisplayContent,
                    ),
                    ActivityContentStream::PlanText,
                    "Portable contract inspected",
                ),
            ),
        ],
    )
}

pub(super) fn completion_only() -> ObservableActivityTraceFixture {
    trace(
        kind_profile(
            ActivityKindClass::CommandExecution,
            ActivityLifecycleFidelity::CompletionOnly,
            [ActivityContentStream::CommandOutput],
            ActivityDisclosure::ProviderDisplayContent,
            [],
        ),
        [event(
            2,
            with_content(
                observation(
                    activity_id("fixture.activity.completion-only"),
                    ActivityKind::CommandExecution,
                    ActivityLifecyclePhase::Completed,
                    ActivityStatus::Completed,
                    None,
                    ActivityDisclosure::ProviderDisplayContent,
                ),
                ActivityContentStream::CommandOutput,
                "fixture command completed",
            ),
        )],
    )
}

pub(super) fn unavailable() -> ObservableActivityTraceFixture {
    ObservableActivityTraceFixture {
        profile: ObservableActivityProfile::unavailable([])
            .expect("empty interface basis is valid"),
        events: vec![
            RuntimeEvent::new(1, RuntimeEventKind::Started),
            RuntimeEvent::with_content(
                2,
                RuntimeEventKind::OutputAvailable,
                content("ordinary output"),
            ),
        ],
    }
}

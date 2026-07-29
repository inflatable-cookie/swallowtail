use super::ObservableActivityTraceFixture;
use swallowtail_core::{
    ActivityContentStream, ActivityCorrelationKind, ActivityDisclosure, ActivityKindClass,
    ActivityKindProfile, ActivityLifecycleFidelity, ActivityUnknownEventPosture,
    ObservableActivityProfile, ProviderActivityRef,
};
use swallowtail_runtime::{
    ActivityAssistantPhase, ActivityContent, ActivityContentChangeKind, ActivityContentUpdate,
    ActivityId, ActivityKind, ActivityLifecyclePhase, ActivityObservation, ActivityOperationId,
    ActivityStatus, OperationContent, RuntimeEvent, RuntimeEventKind, RuntimeRunId,
};

const CONTENT_LIMIT: usize = 1_024;

pub(super) fn trace(
    profile: ActivityKindProfile,
    activity_events: impl IntoIterator<Item = RuntimeEvent>,
) -> ObservableActivityTraceFixture {
    trace_with_posture(
        profile,
        activity_events,
        ActivityUnknownEventPosture::FailClosed,
    )
}

pub(super) fn trace_with_posture(
    profile: ActivityKindProfile,
    activity_events: impl IntoIterator<Item = RuntimeEvent>,
    unknown_event_posture: ActivityUnknownEventPosture,
) -> ObservableActivityTraceFixture {
    let mut events = vec![RuntimeEvent::new(1, RuntimeEventKind::Started)];
    events.extend(activity_events);
    ObservableActivityTraceFixture {
        profile: ObservableActivityProfile::available([], [profile], unknown_event_posture)
            .expect("canonical activity profile is valid"),
        events,
    }
}

pub(super) fn available(profile: ActivityKindProfile) -> ObservableActivityProfile {
    ObservableActivityProfile::available([], [profile], ActivityUnknownEventPosture::FailClosed)
        .expect("canonical activity profile is valid")
}

pub(super) fn kind_profile(
    kind: ActivityKindClass,
    lifecycle: ActivityLifecycleFidelity,
    content_streams: impl IntoIterator<Item = ActivityContentStream>,
    disclosure: ActivityDisclosure,
    correlations: impl IntoIterator<Item = ActivityCorrelationKind>,
) -> ActivityKindProfile {
    ActivityKindProfile::new(kind, lifecycle, content_streams, disclosure, correlations)
        .expect("canonical activity kind profile is valid")
}

pub(super) fn observation(
    id: ActivityId,
    kind: ActivityKind,
    phase: ActivityLifecyclePhase,
    status: ActivityStatus,
    assistant_phase: Option<ActivityAssistantPhase>,
    disclosure: ActivityDisclosure,
) -> ActivityObservation {
    ActivityObservation::new(
        id,
        ActivityOperationId::Run(
            RuntimeRunId::new("fixture.activity.run").expect("run id is valid"),
        ),
        kind,
        phase,
        status,
        assistant_phase,
        disclosure,
    )
    .expect("canonical activity observation is valid")
}

pub(super) fn with_content(
    observation: ActivityObservation,
    stream: ActivityContentStream,
    value: &str,
) -> ActivityObservation {
    observation
        .with_content(ActivityContentUpdate::new(
            ActivityContentChangeKind::ReplacementSnapshot,
            stream,
            ActivityContent::new(content(value), CONTENT_LIMIT)
                .expect("canonical activity content is bounded"),
        ))
        .expect("canonical activity content matches its observation")
}

pub(super) fn event(sequence: u64, observation: ActivityObservation) -> RuntimeEvent {
    RuntimeEvent::new(sequence, RuntimeEventKind::Activity(observation))
}

pub(super) fn content(value: &str) -> OperationContent {
    OperationContent::new(value).expect("canonical operation content is valid")
}

pub(super) fn activity_id(value: &str) -> ActivityId {
    ActivityId::new(value).expect("canonical activity id is valid")
}

pub(super) fn provider_ref() -> ProviderActivityRef {
    ProviderActivityRef::new("fixture.provider/private-item").expect("provider ref is valid")
}

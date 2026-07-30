use swallowtail_runtime::{
    ActivityAssistantPhase, ActivityContentChangeKind, ActivityContentStream, ActivityCorrelation,
    ActivityId, ActivityKindClass, ActivityLifecycleFidelity, ActivityLifecyclePhase,
    ActivityOperationId, ActivityStatus, ObservableActivityAvailability, PreparedOperationEvidence,
    RuntimeEvent, RuntimeEventKind,
};

/// The route truth Nucleus inspects before starting provider effects.
pub struct RouteActivitySupport {
    pub availability: ObservableActivityAvailability,
    pub assistant_lifecycle: ActivityLifecycleFidelity,
    pub reasoning_summary_lifecycle: ActivityLifecycleFidelity,
}

pub fn inspect_route(evidence: &PreparedOperationEvidence) -> RouteActivitySupport {
    let profile = evidence.observable_activity();
    RouteActivitySupport {
        availability: profile.availability(),
        assistant_lifecycle: profile.lifecycle(ActivityKindClass::AssistantMessage),
        reasoning_summary_lifecycle: profile.lifecycle(ActivityKindClass::ReasoningSummary),
    }
}

/// One consumer-owned projection decision. Persistence and presentation stay
/// outside Swallowtail.
pub enum ChatProjection<'a> {
    AssistantMessage {
        operation_id: &'a ActivityOperationId,
        activity_id: &'a ActivityId,
        assistant_phase: ActivityAssistantPhase,
        lifecycle: ActivityLifecyclePhase,
        status: ActivityStatus,
        change: Option<ActivityContentChangeKind>,
        stream: Option<ActivityContentStream>,
        content: Option<&'a str>,
    },
    WorkActivity {
        operation_id: &'a ActivityOperationId,
        activity_id: &'a ActivityId,
        kind: ActivityKindClass,
        lifecycle: ActivityLifecyclePhase,
        status: ActivityStatus,
        label: Option<&'a str>,
        correlation: Option<&'a ActivityCorrelation>,
        change: Option<ActivityContentChangeKind>,
        stream: Option<ActivityContentStream>,
        content: Option<&'a str>,
    },
    FinalOutput {
        content: Option<&'a str>,
    },
    Ignore,
}

pub fn project_event(event: &RuntimeEvent) -> ChatProjection<'_> {
    match event.kind() {
        RuntimeEventKind::Activity(activity) => {
            let content = activity.content();
            if activity.kind().class() == ActivityKindClass::AssistantMessage {
                ChatProjection::AssistantMessage {
                    operation_id: activity.operation_id(),
                    activity_id: activity.activity_id(),
                    assistant_phase: activity
                        .assistant_phase()
                        .expect("validated assistant activity has an exact phase"),
                    lifecycle: activity.phase(),
                    status: activity.status(),
                    change: content.map(|value| value.change()),
                    stream: content.map(|value| value.stream()),
                    content: content.map(|value| value.content().as_str()),
                }
            } else {
                ChatProjection::WorkActivity {
                    operation_id: activity.operation_id(),
                    activity_id: activity.activity_id(),
                    kind: activity.kind().class(),
                    lifecycle: activity.phase(),
                    status: activity.status(),
                    label: activity.label().map(|value| value.as_str()),
                    correlation: activity.correlation(),
                    change: content.map(|value| value.change()),
                    stream: content.map(|value| value.stream()),
                    content: content.map(|value| value.content().as_str()),
                }
            }
        }
        RuntimeEventKind::OutputAvailable => ChatProjection::FinalOutput {
            content: event.content().map(|value| value.as_str()),
        },
        _ => ChatProjection::Ignore,
    }
}

fn main() {}

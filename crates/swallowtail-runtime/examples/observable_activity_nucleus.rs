use swallowtail_runtime::{
    ActivityActor, ActivityAssistantPhase, ActivityContentChangeKind, ActivityContentStream,
    ActivityCorrelation, ActivityKey, ActivityKindClass, ActivityLifecycleFidelity,
    ActivityLifecyclePhase, ActivityOperationId, ActivityStatus, ObservableActivityAvailability,
    PreparedOperationEvidence, RuntimeEvent, RuntimeEventKind, SubagentControlActionKind,
    SubagentDirectoryDelta, SubagentDirectoryFailure, SubagentDirectoryProjection,
    SubagentSnapshot,
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
        key: ActivityKey,
        actor: &'a ActivityActor,
        assistant_phase: ActivityAssistantPhase,
        lifecycle: ActivityLifecyclePhase,
        status: ActivityStatus,
        change: Option<ActivityContentChangeKind>,
        stream: Option<ActivityContentStream>,
        content: Option<&'a str>,
        directory: SubagentDirectoryDelta,
    },
    WorkActivity {
        key: ActivityKey,
        actor: &'a ActivityActor,
        kind: ActivityKindClass,
        lifecycle: ActivityLifecyclePhase,
        status: ActivityStatus,
        label: Option<&'a str>,
        correlation: Option<&'a ActivityCorrelation>,
        change: Option<ActivityContentChangeKind>,
        stream: Option<ActivityContentStream>,
        content: Option<&'a str>,
        subagents: Vec<&'a SubagentSnapshot>,
        subagent_control: Option<SubagentControlActionKind>,
        directory: SubagentDirectoryDelta,
    },
    FinalOutput {
        content: Option<&'a str>,
    },
    Ignore,
}

/// Operation-local reducer suitable for a main-thread/child-thread picker.
pub struct AgentChatProjection {
    directory: SubagentDirectoryProjection,
}

impl AgentChatProjection {
    pub fn new(
        operation_id: ActivityOperationId,
        maximum_subagents: usize,
    ) -> Result<Self, SubagentDirectoryFailure> {
        Ok(Self {
            directory: SubagentDirectoryProjection::new(operation_id, maximum_subagents)?,
        })
    }

    pub const fn directory(&self) -> &SubagentDirectoryProjection {
        &self.directory
    }

    pub fn project_event<'a>(
        &mut self,
        event: &'a RuntimeEvent,
    ) -> Result<ChatProjection<'a>, SubagentDirectoryFailure> {
        let directory = self.directory.observe_event(event)?;
        Ok(match event.kind() {
            RuntimeEventKind::Activity(activity) => {
                let content = activity.content();
                let directory = directory.expect("activity event produces a directory delta");
                if activity.kind().class() == ActivityKindClass::AssistantMessage {
                    ChatProjection::AssistantMessage {
                        key: activity.key(),
                        actor: activity.actor(),
                        assistant_phase: activity
                            .assistant_phase()
                            .expect("validated assistant activity has an exact phase"),
                        lifecycle: activity.phase(),
                        status: activity.status(),
                        change: content.map(|value| value.change()),
                        stream: content.map(|value| value.stream()),
                        content: content.map(|value| value.content().as_str()),
                        directory,
                    }
                } else {
                    ChatProjection::WorkActivity {
                        key: activity.key(),
                        actor: activity.actor(),
                        kind: activity.kind().class(),
                        lifecycle: activity.phase(),
                        status: activity.status(),
                        label: activity.label().map(|value| value.as_str()),
                        correlation: activity.correlation(),
                        change: content.map(|value| value.change()),
                        stream: content.map(|value| value.stream()),
                        content: content.map(|value| value.content().as_str()),
                        subagents: activity.subagents().collect(),
                        subagent_control: activity.subagent_control(),
                        directory,
                    }
                }
            }
            RuntimeEventKind::OutputAvailable => ChatProjection::FinalOutput {
                content: event.content().map(|value| value.as_str()),
            },
            _ => ChatProjection::Ignore,
        })
    }
}

fn main() {}

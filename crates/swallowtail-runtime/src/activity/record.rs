use super::subagent::{ActivityActor, SubagentSnapshot, validate_subagents};
use super::validation::{validate_assistant_phase, validate_content, validate_phase_status};
use super::{
    ActivityContentUpdate, ActivityId, ActivityLabel, ActivityNamespace, InvalidActivityRecord,
    TaskListSnapshot,
};
use crate::{CallbackId, DirectToolCallId, RuntimeRunId, RuntimeTurnId};
use std::fmt;
use swallowtail_core::{
    ActivityDisclosure, ActivityKindClass, ProviderActivityRef, ProviderRequestRef,
    SubagentControlActionKind,
};

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum ActivityOperationId {
    Run(RuntimeRunId),
    Turn(RuntimeTurnId),
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum ActivityKind {
    AssistantMessage,
    ReasoningSummary,
    Plan,
    CommandExecution,
    FileChange,
    ProviderOwnedTool,
    ConsumerOwnedTool,
    ExternalSearch,
    ImageView,
    SubagentOrCollaboration,
    ReviewTransition,
    ContextCompaction,
    Task,
    Hook,
    WarningOrError,
    Unknown(ActivityNamespace),
}

impl ActivityKind {
    #[must_use]
    pub const fn class(&self) -> ActivityKindClass {
        match self {
            Self::AssistantMessage => ActivityKindClass::AssistantMessage,
            Self::ReasoningSummary => ActivityKindClass::ReasoningSummary,
            Self::Plan => ActivityKindClass::Plan,
            Self::CommandExecution => ActivityKindClass::CommandExecution,
            Self::FileChange => ActivityKindClass::FileChange,
            Self::ProviderOwnedTool => ActivityKindClass::ProviderOwnedTool,
            Self::ConsumerOwnedTool => ActivityKindClass::ConsumerOwnedTool,
            Self::ExternalSearch => ActivityKindClass::ExternalSearch,
            Self::ImageView => ActivityKindClass::ImageView,
            Self::SubagentOrCollaboration => ActivityKindClass::SubagentOrCollaboration,
            Self::ReviewTransition => ActivityKindClass::ReviewTransition,
            Self::ContextCompaction => ActivityKindClass::ContextCompaction,
            Self::Task => ActivityKindClass::Task,
            Self::Hook => ActivityKindClass::Hook,
            Self::WarningOrError => ActivityKindClass::WarningOrError,
            Self::Unknown(_) => ActivityKindClass::Unknown,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ActivityLifecyclePhase {
    Started,
    Updated,
    Completed,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ActivityStatus {
    Pending,
    InProgress,
    Completed,
    Failed,
    Cancelled,
}

impl ActivityStatus {
    #[must_use]
    pub const fn is_terminal(self) -> bool {
        matches!(self, Self::Completed | Self::Failed | Self::Cancelled)
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ActivityAssistantPhase {
    ProviderUnspecified,
    Intermediate,
    Final,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum ActivityCorrelation {
    Callback(CallbackId),
    DirectToolCall(DirectToolCallId),
    ProviderRequest(ProviderRequestRef),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ActivityObservation {
    activity_id: ActivityId,
    operation_id: ActivityOperationId,
    provider_activity_ref: Option<ProviderActivityRef>,
    kind: ActivityKind,
    phase: ActivityLifecyclePhase,
    status: ActivityStatus,
    assistant_phase: Option<ActivityAssistantPhase>,
    disclosure: ActivityDisclosure,
    correlation: Option<ActivityCorrelation>,
    label: Option<ActivityLabel>,
    content: Option<ActivityContentUpdate>,
    task_list: Option<TaskListSnapshot>,
    actor: ActivityActor,
    subagents: Vec<SubagentSnapshot>,
    subagent_control: Option<SubagentControlActionKind>,
}

impl ActivityObservation {
    pub fn new(
        activity_id: ActivityId,
        operation_id: ActivityOperationId,
        kind: ActivityKind,
        phase: ActivityLifecyclePhase,
        status: ActivityStatus,
        assistant_phase: Option<ActivityAssistantPhase>,
        disclosure: ActivityDisclosure,
    ) -> Result<Self, InvalidActivityRecord> {
        validate_phase_status(phase, status)?;
        validate_assistant_phase(&kind, assistant_phase)?;
        if disclosure == ActivityDisclosure::Unavailable {
            return Err(InvalidActivityRecord::new(
                "Unavailable activity disclosure cannot produce an observation",
            ));
        }
        Ok(Self {
            activity_id,
            operation_id,
            provider_activity_ref: None,
            kind,
            phase,
            status,
            assistant_phase,
            disclosure,
            correlation: None,
            label: None,
            content: None,
            task_list: None,
            actor: ActivityActor::Primary,
            subagents: Vec::new(),
            subagent_control: None,
        })
    }

    #[must_use]
    pub fn with_provider_activity_ref(mut self, reference: ProviderActivityRef) -> Self {
        self.provider_activity_ref = Some(reference);
        self
    }

    #[must_use]
    pub fn with_correlation(mut self, correlation: ActivityCorrelation) -> Self {
        self.correlation = Some(correlation);
        self
    }

    pub fn with_content(
        mut self,
        content: ActivityContentUpdate,
    ) -> Result<Self, InvalidActivityRecord> {
        validate_content(&self, &content)?;
        self.content = Some(content);
        Ok(self)
    }

    pub fn with_label(mut self, label: ActivityLabel) -> Result<Self, InvalidActivityRecord> {
        super::validation::validate_label(&self)?;
        self.label = Some(label);
        Ok(self)
    }

    pub fn with_task_list(
        mut self,
        task_list: TaskListSnapshot,
    ) -> Result<Self, InvalidActivityRecord> {
        super::validation::validate_task_list(&self)?;
        self.task_list = Some(task_list);
        Ok(self)
    }

    #[must_use]
    pub fn with_actor(mut self, actor: ActivityActor) -> Self {
        self.actor = actor;
        self
    }

    pub fn with_subagents(
        mut self,
        subagents: impl IntoIterator<Item = SubagentSnapshot>,
    ) -> Result<Self, InvalidActivityRecord> {
        if !matches!(self.kind, ActivityKind::SubagentOrCollaboration) {
            return Err(InvalidActivityRecord::new(
                "Subagent snapshots require subagent or collaboration activity",
            ));
        }
        let subagents = subagents.into_iter().collect::<Vec<_>>();
        validate_subagents(&subagents)?;
        self.subagents = subagents;
        Ok(self)
    }

    pub fn with_subagent_control(
        mut self,
        action: SubagentControlActionKind,
    ) -> Result<Self, InvalidActivityRecord> {
        if !matches!(self.kind, ActivityKind::SubagentOrCollaboration) {
            return Err(InvalidActivityRecord::new(
                "Subagent control observations require subagent or collaboration activity",
            ));
        }
        self.subagent_control = Some(action);
        Ok(self)
    }

    #[must_use]
    pub const fn activity_id(&self) -> &ActivityId {
        &self.activity_id
    }

    #[must_use]
    pub const fn operation_id(&self) -> &ActivityOperationId {
        &self.operation_id
    }

    #[must_use]
    pub const fn provider_activity_ref(&self) -> Option<&ProviderActivityRef> {
        self.provider_activity_ref.as_ref()
    }

    #[must_use]
    pub const fn kind(&self) -> &ActivityKind {
        &self.kind
    }

    #[must_use]
    pub const fn phase(&self) -> ActivityLifecyclePhase {
        self.phase
    }

    #[must_use]
    pub const fn status(&self) -> ActivityStatus {
        self.status
    }

    #[must_use]
    pub const fn assistant_phase(&self) -> Option<ActivityAssistantPhase> {
        self.assistant_phase
    }

    #[must_use]
    pub const fn disclosure(&self) -> ActivityDisclosure {
        self.disclosure
    }

    #[must_use]
    pub const fn correlation(&self) -> Option<&ActivityCorrelation> {
        self.correlation.as_ref()
    }

    #[must_use]
    pub const fn label(&self) -> Option<&ActivityLabel> {
        self.label.as_ref()
    }

    #[must_use]
    pub const fn content(&self) -> Option<&ActivityContentUpdate> {
        self.content.as_ref()
    }

    #[must_use]
    pub const fn task_list(&self) -> Option<&TaskListSnapshot> {
        self.task_list.as_ref()
    }

    #[must_use]
    pub const fn actor(&self) -> &ActivityActor {
        &self.actor
    }

    pub fn subagents(&self) -> impl ExactSizeIterator<Item = &SubagentSnapshot> {
        self.subagents.iter()
    }

    #[must_use]
    pub const fn subagent_control(&self) -> Option<SubagentControlActionKind> {
        self.subagent_control
    }
}

impl fmt::Display for ActivityObservation {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("<redacted activity observation>")
    }
}

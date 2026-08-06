use super::subagent::{ActivityActor, SubagentSnapshot, validate_subagents};
use super::validation::{validate_assistant_phase, validate_content, validate_phase_status};
use super::{
    ActivityContentUpdate, ActivityId, ActivityLabel, ActivityNamespace, InvalidActivityRecord,
    TaskListSnapshot,
};
use crate::{CallbackId, DirectToolCallId, RuntimeRunId, RuntimeTurnId};
use std::fmt;
use swallowtail_core::{
    ActivityDisclosure, ActivityKindClass, ProviderActivityRef, ProviderRequestRef, SafeDiagnostic,
    SubagentControlActionKind,
};

/// Runtime operation that owns an activity identity.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ActivityOperationId {
    /// Activity observed during a structured run.
    Run(RuntimeRunId),
    /// Activity observed during an interactive turn.
    Turn(RuntimeTurnId),
}

/// Durable portable identity for one activity inside one runtime operation.
///
/// Provider references and operation-local activity ids may repeat in another
/// operation. Consumers should persist and index the complete key.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ActivityKey {
    operation_id: ActivityOperationId,
    activity_id: ActivityId,
}

impl ActivityKey {
    /// Creates a composite operation-local activity key.
    #[must_use]
    pub const fn new(operation_id: ActivityOperationId, activity_id: ActivityId) -> Self {
        Self {
            operation_id,
            activity_id,
        }
    }

    #[must_use]
    /// Returns the owning runtime operation.
    pub const fn operation_id(&self) -> &ActivityOperationId {
        &self.operation_id
    }

    #[must_use]
    /// Returns the activity identity within that operation.
    pub const fn activity_id(&self) -> &ActivityId {
        &self.activity_id
    }
}

/// Portable semantic classification of observable provider work.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum ActivityKind {
    /// Provider-intended assistant message.
    AssistantMessage,
    /// Provider-intended readable reasoning summary.
    ReasoningSummary,
    /// Provider plan activity.
    Plan,
    /// Command or shell execution.
    CommandExecution,
    /// File creation, deletion, or modification.
    FileChange,
    /// Tool executed by the provider or harness.
    ProviderOwnedTool,
    /// Tool delegated to the consumer.
    ConsumerOwnedTool,
    /// Provider-owned external search.
    ExternalSearch,
    /// Image inspection activity.
    ImageView,
    /// Child-agent or collaboration activity.
    SubagentOrCollaboration,
    /// Provider review-mode transition.
    ReviewTransition,
    /// Provider context-compaction activity.
    ContextCompaction,
    /// Provider task or todo activity.
    Task,
    /// Provider or harness hook execution.
    Hook,
    /// Warning or error activity carrying a safe diagnostic.
    WarningOrError,
    /// Safely identified provider activity outside the portable vocabulary.
    Unknown(ActivityNamespace),
}

impl ActivityKind {
    /// Returns the provider-neutral class used by prepared activity profiles.
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

/// Observable lifecycle phase supplied or safely derived by an adapter.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ActivityLifecyclePhase {
    /// The activity became observable.
    Started,
    /// The activity supplied a non-terminal refinement.
    Updated,
    /// The activity supplied its terminal observation.
    Completed,
}

/// Provider-visible status of an activity at one lifecycle phase.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ActivityStatus {
    /// Accepted but not yet executing.
    Pending,
    /// Currently executing.
    InProgress,
    /// Finished successfully.
    Completed,
    /// Finished with failure.
    Failed,
    /// Finished through cancellation.
    Cancelled,
}

impl ActivityStatus {
    /// Returns whether no later status is valid for this activity.
    #[must_use]
    pub const fn is_terminal(self) -> bool {
        matches!(self, Self::Completed | Self::Failed | Self::Cancelled)
    }
}

/// Message position disclosed for assistant-message activity.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ActivityAssistantPhase {
    /// The provider did not distinguish intermediate from final output.
    ProviderUnspecified,
    /// Intermediate assistant content produced before the final answer.
    Intermediate,
    /// Final assistant-answer content.
    Final,
}

/// Exact portable exchange correlated with an activity.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum ActivityCorrelation {
    /// Consumer callback correlation.
    Callback(CallbackId),
    /// Direct-continuation tool-call correlation.
    DirectToolCall(DirectToolCallId),
    /// Representation-aware provider request correlation.
    ProviderRequest(ProviderRequestRef),
}

/// One bounded observation of provider-visible activity.
///
/// The record separates identity, lifecycle, status, disclosure, content,
/// task lists, child attribution, and exchange correlation. Its default
/// formatting redacts operation content.
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
    diagnostic: Option<SafeDiagnostic>,
    content: Option<ActivityContentUpdate>,
    task_list: Option<TaskListSnapshot>,
    actor: ActivityActor,
    subagents: Vec<SubagentSnapshot>,
    subagent_control: Option<SubagentControlActionKind>,
}

impl ActivityObservation {
    /// Creates a validated identity and lifecycle observation.
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
            diagnostic: None,
            content: None,
            task_list: None,
            actor: ActivityActor::Primary,
            subagents: Vec::new(),
            subagent_control: None,
        })
    }

    #[must_use]
    /// Attaches the provider's opaque operation-local activity reference.
    pub fn with_provider_activity_ref(mut self, reference: ProviderActivityRef) -> Self {
        self.provider_activity_ref = Some(reference);
        self
    }

    #[must_use]
    /// Attaches an exact callback, request, or direct-tool correlation.
    pub fn with_correlation(mut self, correlation: ActivityCorrelation) -> Self {
        self.correlation = Some(correlation);
        self
    }

    /// Attaches validated content permitted by the kind and disclosure level.
    pub fn with_content(
        mut self,
        content: ActivityContentUpdate,
    ) -> Result<Self, InvalidActivityRecord> {
        validate_content(&self, &content)?;
        self.content = Some(content);
        Ok(self)
    }

    /// Attaches a bounded provider-intended display label.
    pub fn with_label(mut self, label: ActivityLabel) -> Result<Self, InvalidActivityRecord> {
        super::validation::validate_label(&self)?;
        self.label = Some(label);
        Ok(self)
    }

    /// Attaches a safe diagnostic to warning-or-error activity.
    pub fn with_diagnostic(
        mut self,
        diagnostic: SafeDiagnostic,
    ) -> Result<Self, InvalidActivityRecord> {
        if !matches!(self.kind, ActivityKind::WarningOrError) {
            return Err(InvalidActivityRecord::new(
                "Safe diagnostics require warning-or-error activity",
            ));
        }
        self.diagnostic = Some(diagnostic);
        Ok(self)
    }

    /// Attaches a full task-list replacement to plan or task activity.
    pub fn with_task_list(
        mut self,
        task_list: TaskListSnapshot,
    ) -> Result<Self, InvalidActivityRecord> {
        super::validation::validate_task_list(&self)?;
        self.task_list = Some(task_list);
        Ok(self)
    }

    #[must_use]
    /// Attributes the observation to the primary agent or one admitted child.
    pub fn with_actor(mut self, actor: ActivityActor) -> Self {
        self.actor = actor;
        self
    }

    /// Attaches a bounded replacement snapshot of visible child agents.
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

    /// Attaches an observed provider collaboration-control action.
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
    /// Returns the operation-local activity identity.
    pub const fn activity_id(&self) -> &ActivityId {
        &self.activity_id
    }

    #[must_use]
    /// Returns the runtime operation that owns this observation.
    pub const fn operation_id(&self) -> &ActivityOperationId {
        &self.operation_id
    }

    /// Returns the complete portable persistence key for this activity.
    #[must_use]
    pub fn key(&self) -> ActivityKey {
        ActivityKey::new(self.operation_id.clone(), self.activity_id.clone())
    }

    #[must_use]
    /// Returns the provider's opaque activity reference when supplied.
    pub const fn provider_activity_ref(&self) -> Option<&ProviderActivityRef> {
        self.provider_activity_ref.as_ref()
    }

    #[must_use]
    /// Returns the portable activity kind.
    pub const fn kind(&self) -> &ActivityKind {
        &self.kind
    }

    #[must_use]
    /// Returns the observed lifecycle phase.
    pub const fn phase(&self) -> ActivityLifecyclePhase {
        self.phase
    }

    #[must_use]
    /// Returns the provider-visible activity status.
    pub const fn status(&self) -> ActivityStatus {
        self.status
    }

    #[must_use]
    /// Returns message position for assistant-message activity.
    pub const fn assistant_phase(&self) -> Option<ActivityAssistantPhase> {
        self.assistant_phase
    }

    #[must_use]
    /// Returns the maximum disclosure strength of this observation.
    pub const fn disclosure(&self) -> ActivityDisclosure {
        self.disclosure
    }

    #[must_use]
    /// Returns the exact exchange correlation when present.
    pub const fn correlation(&self) -> Option<&ActivityCorrelation> {
        self.correlation.as_ref()
    }

    #[must_use]
    /// Returns the provider-intended display label when present.
    pub const fn label(&self) -> Option<&ActivityLabel> {
        self.label.as_ref()
    }

    #[must_use]
    /// Returns the safe warning or error diagnostic when present.
    pub const fn diagnostic(&self) -> Option<&SafeDiagnostic> {
        self.diagnostic.as_ref()
    }

    #[must_use]
    /// Returns the bounded content update when present.
    pub const fn content(&self) -> Option<&ActivityContentUpdate> {
        self.content.as_ref()
    }

    #[must_use]
    /// Returns the task-list replacement snapshot when present.
    pub const fn task_list(&self) -> Option<&TaskListSnapshot> {
        self.task_list.as_ref()
    }

    #[must_use]
    /// Returns the agent attributed as the actor.
    pub const fn actor(&self) -> &ActivityActor {
        &self.actor
    }

    /// Iterates over visible child snapshots in provider order.
    pub fn subagents(&self) -> impl ExactSizeIterator<Item = &SubagentSnapshot> {
        self.subagents.iter()
    }

    #[must_use]
    /// Returns the observed collaboration-control action when present.
    pub const fn subagent_control(&self) -> Option<SubagentControlActionKind> {
        self.subagent_control
    }
}

impl fmt::Display for ActivityObservation {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("<redacted activity observation>")
    }
}

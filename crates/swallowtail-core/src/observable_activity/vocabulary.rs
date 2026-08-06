use crate::{InterfaceBehaviorRevision, InterfaceVersionAxis};

/// Provider-neutral activity category used by route capability profiles.
///
/// Runtime observations retain a bounded namespace for `Unknown`; profiles
/// describe only whether that class can be represented safely.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ActivityKindClass {
    /// Assistant-authored conversational content.
    AssistantMessage,
    /// Provider-disclosed reasoning summary.
    ReasoningSummary,
    /// Plan creation or revision.
    Plan,
    /// Shell or process command execution.
    CommandExecution,
    /// File creation, modification, or deletion.
    FileChange,
    /// Tool executed within provider-owned authority.
    ProviderOwnedTool,
    /// Tool requiring consumer-owned execution authority.
    ConsumerOwnedTool,
    /// Provider or harness external search.
    ExternalSearch,
    /// Image inspection activity.
    ImageView,
    /// Child-agent or collaboration activity.
    SubagentOrCollaboration,
    /// Review-state transition.
    ReviewTransition,
    /// Provider context compaction.
    ContextCompaction,
    /// Task-list item or snapshot activity.
    Task,
    /// Provider hook execution.
    Hook,
    /// Provider warning or non-terminal error activity.
    WarningOrError,
    /// Provider-namespaced activity outside common vocabulary.
    Unknown,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
/// Maximum lifecycle detail guaranteed for an activity kind.
pub enum ActivityLifecycleFidelity {
    /// Start, update, and terminal phases are observable.
    CompleteLifecycle,
    /// Updates and terminal completion are observable without a start.
    UpdateAndCompletion,
    /// Only terminal completion is observable.
    CompletionOnly,
    /// Activity kind is not observable.
    Unavailable,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
/// Semantic content channel carried by activity observations.
pub enum ActivityContentStream {
    /// Incremental non-final assistant text.
    IntermediateAssistantText,
    /// Final assistant answer text.
    FinalAnswerText,
    /// Provider-disclosed reasoning-summary text.
    ReasoningSummaryText,
    /// Plan text or task-plan content.
    PlanText,
    /// Command output.
    CommandOutput,
    /// File-change output or patch display.
    FileChangeOutput,
    /// Provider-supplied tool display content.
    ProviderToolDisplay,
    /// Adapter-authored bounded summary.
    NormalizedSummary,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
/// Provenance and detail level of activity content.
pub enum ActivityDisclosure {
    /// Content is provider-supplied display material.
    ProviderDisplayContent,
    /// Content is a bounded adapter-normalized summary.
    AdapterNormalizedSummary,
    /// Only activity identity and lifecycle are disclosed.
    IdentityAndLifecycleOnly,
    /// Activity kind is unavailable.
    Unavailable,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
/// Portable exchange identity correlated with an activity.
pub enum ActivityCorrelationKind {
    /// Consumer callback exchange.
    Callback,
    /// Direct-inference tool call.
    DirectToolCall,
    /// Provider-owned request exchange.
    ProviderRequest,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
/// Handling for provider activity outside the common kind vocabulary.
pub enum ActivityUnknownEventPosture {
    /// Preserve bounded activity under its provider namespace.
    PreserveNamespaced,
    /// Reject unknown event shapes.
    FailClosed,
}

/// Maximum portable child-work detail exposed by one route.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum SubagentObservationFidelity {
    /// Child identity and lifecycle only.
    IdentityAndLifecycle,
    /// Identity, lifecycle, parentage, and bounded metadata.
    ParentAndMetadata,
    /// Child activity can be attributed to the exact child.
    AttributedActivity,
}

/// Provider-owned collaborative action visible on the activity stream.
///
/// These values describe what the harness did. They do not grant the consumer
/// authority to perform the action.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum SubagentControlActionKind {
    /// Spawn a child agent.
    Spawn,
    /// Send input to an existing child.
    SendInput,
    /// Resume a paused or retained child.
    Resume,
    /// Wait for child progress or completion.
    Wait,
    /// Close a child agent.
    Close,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
/// Whether observable activity applies to and is exposed by a route.
pub enum ObservableActivityAvailability {
    /// Route operation shape has no agent activity concept.
    NotApplicable,
    /// Activity applies but the route cannot expose it safely.
    Unavailable,
    /// Route exposes a qualified activity profile.
    Available,
}

/// Qualified interface behavior used to derive an activity guarantee.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ActivityInterfaceBasis {
    axis: InterfaceVersionAxis,
    behavior_revision: InterfaceBehaviorRevision,
}

impl ActivityInterfaceBasis {
    #[must_use]
    /// Couples an interface axis with the behavior revision proving activity.
    pub const fn new(
        axis: InterfaceVersionAxis,
        behavior_revision: InterfaceBehaviorRevision,
    ) -> Self {
        Self {
            axis,
            behavior_revision,
        }
    }

    #[must_use]
    /// Returns the qualified interface axis.
    pub const fn axis(&self) -> &InterfaceVersionAxis {
        &self.axis
    }

    #[must_use]
    /// Returns the behavior revision used as activity evidence.
    pub const fn behavior_revision(&self) -> &InterfaceBehaviorRevision {
        &self.behavior_revision
    }

    pub(super) fn into_parts(self) -> (InterfaceVersionAxis, InterfaceBehaviorRevision) {
        (self.axis, self.behavior_revision)
    }
}

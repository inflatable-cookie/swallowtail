use crate::{InterfaceBehaviorRevision, InterfaceVersionAxis};

/// Provider-neutral activity category used by route capability profiles.
///
/// Runtime observations retain a bounded namespace for `Unknown`; profiles
/// describe only whether that class can be represented safely.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ActivityKindClass {
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
    Unknown,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ActivityLifecycleFidelity {
    CompleteLifecycle,
    UpdateAndCompletion,
    CompletionOnly,
    Unavailable,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ActivityContentStream {
    IntermediateAssistantText,
    FinalAnswerText,
    ReasoningSummaryText,
    PlanText,
    CommandOutput,
    FileChangeOutput,
    ProviderToolDisplay,
    NormalizedSummary,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ActivityDisclosure {
    ProviderDisplayContent,
    AdapterNormalizedSummary,
    IdentityAndLifecycleOnly,
    Unavailable,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ActivityCorrelationKind {
    Callback,
    DirectToolCall,
    ProviderRequest,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ActivityUnknownEventPosture {
    PreserveNamespaced,
    FailClosed,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ObservableActivityAvailability {
    NotApplicable,
    Unavailable,
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
    pub const fn axis(&self) -> &InterfaceVersionAxis {
        &self.axis
    }

    #[must_use]
    pub const fn behavior_revision(&self) -> &InterfaceBehaviorRevision {
        &self.behavior_revision
    }

    pub(super) fn into_parts(self) -> (InterfaceVersionAxis, InterfaceBehaviorRevision) {
        (self.axis, self.behavior_revision)
    }
}

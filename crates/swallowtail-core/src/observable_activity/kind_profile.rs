use super::{
    ActivityContentStream, ActivityCorrelationKind, ActivityDisclosure, ActivityKindClass,
    ActivityLifecycleFidelity, InvalidObservableActivityProfile, SubagentControlActionKind,
    SubagentObservationFidelity,
};
use crate::CapabilityConstraint;
use std::collections::BTreeSet;

/// Exact maximum fidelity for one supported portable activity kind.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ActivityKindProfile {
    kind: ActivityKindClass,
    lifecycle: ActivityLifecycleFidelity,
    content_streams: BTreeSet<ActivityContentStream>,
    disclosure: ActivityDisclosure,
    correlations: BTreeSet<ActivityCorrelationKind>,
    task_list_snapshots: bool,
    subagent_observation: Option<SubagentObservationFidelity>,
    subagent_control_actions: BTreeSet<SubagentControlActionKind>,
}

impl ActivityKindProfile {
    /// Creates and validates maximum fidelity for one activity kind.
    pub fn new(
        kind: ActivityKindClass,
        lifecycle: ActivityLifecycleFidelity,
        content_streams: impl IntoIterator<Item = ActivityContentStream>,
        disclosure: ActivityDisclosure,
        correlations: impl IntoIterator<Item = ActivityCorrelationKind>,
    ) -> Result<Self, InvalidObservableActivityProfile> {
        let profile = Self {
            kind,
            lifecycle,
            content_streams: content_streams.into_iter().collect(),
            disclosure,
            correlations: correlations.into_iter().collect(),
            task_list_snapshots: false,
            subagent_observation: None,
            subagent_control_actions: BTreeSet::new(),
        };
        profile.validate()?;
        Ok(profile)
    }

    #[must_use]
    /// Returns the portable activity kind.
    pub const fn kind(&self) -> ActivityKindClass {
        self.kind
    }

    #[must_use]
    /// Returns maximum lifecycle fidelity.
    pub const fn lifecycle(&self) -> ActivityLifecycleFidelity {
        self.lifecycle
    }

    /// Iterates guaranteed content streams in stable order.
    pub fn content_streams(&self) -> impl ExactSizeIterator<Item = ActivityContentStream> + '_ {
        self.content_streams.iter().copied()
    }

    #[must_use]
    /// Returns the content disclosure posture.
    pub const fn disclosure(&self) -> ActivityDisclosure {
        self.disclosure
    }

    /// Iterates available correlation kinds in stable order.
    pub fn correlations(&self) -> impl ExactSizeIterator<Item = ActivityCorrelationKind> + '_ {
        self.correlations.iter().copied()
    }

    /// Declares complete task-list snapshots for plan or task activity.
    pub fn with_task_list_snapshots(mut self) -> Result<Self, InvalidObservableActivityProfile> {
        self.task_list_snapshots = true;
        self.validate()?;
        Ok(self)
    }

    #[must_use]
    /// Reports whether complete task-list snapshots are guaranteed.
    pub const fn task_list_snapshots(&self) -> bool {
        self.task_list_snapshots
    }

    /// Adds child-agent observation fidelity to collaboration activity.
    pub fn with_subagent_observation(
        mut self,
        fidelity: SubagentObservationFidelity,
    ) -> Result<Self, InvalidObservableActivityProfile> {
        self.subagent_observation = Some(fidelity);
        self.validate()?;
        Ok(self)
    }

    /// Adds provider-observed child-control actions.
    pub fn with_subagent_control_actions(
        mut self,
        actions: impl IntoIterator<Item = SubagentControlActionKind>,
    ) -> Result<Self, InvalidObservableActivityProfile> {
        self.subagent_control_actions = actions.into_iter().collect();
        self.validate()?;
        Ok(self)
    }

    #[must_use]
    /// Returns maximum child-agent observation fidelity.
    pub const fn subagent_observation(&self) -> Option<SubagentObservationFidelity> {
        self.subagent_observation
    }

    /// Iterates observed child-control actions in stable order.
    pub fn subagent_control_actions(
        &self,
    ) -> impl ExactSizeIterator<Item = SubagentControlActionKind> + '_ {
        self.subagent_control_actions.iter().copied()
    }

    pub(super) fn capability_constraints(&self) -> BTreeSet<CapabilityConstraint> {
        let mut constraints =
            BTreeSet::from([CapabilityConstraint::ObservableActivityKind(self.kind)]);
        for lifecycle in satisfied_lifecycle_constraints(self.lifecycle) {
            constraints.insert(CapabilityConstraint::ObservableActivityLifecycle(
                self.kind, *lifecycle,
            ));
        }
        for stream in &self.content_streams {
            constraints.insert(CapabilityConstraint::ObservableActivityContentStream(
                self.kind, *stream,
            ));
        }
        for disclosure in satisfied_disclosure_constraints(self.disclosure) {
            constraints.insert(CapabilityConstraint::ObservableActivityDisclosure(
                self.kind,
                *disclosure,
            ));
        }
        for correlation in &self.correlations {
            constraints.insert(CapabilityConstraint::ObservableActivityCorrelation(
                self.kind,
                *correlation,
            ));
        }
        if self.task_list_snapshots {
            constraints.insert(CapabilityConstraint::ObservableActivityTaskListSnapshots(
                self.kind,
            ));
        }
        if let Some(fidelity) = self.subagent_observation {
            for satisfied in satisfied_subagent_constraints(fidelity) {
                constraints.insert(CapabilityConstraint::ObservableSubagentObservation(
                    *satisfied,
                ));
            }
        }
        for action in &self.subagent_control_actions {
            constraints.insert(CapabilityConstraint::ObservableSubagentControlAction(
                *action,
            ));
        }
        constraints
    }

    fn validate(&self) -> Result<(), InvalidObservableActivityProfile> {
        if self.lifecycle == ActivityLifecycleFidelity::Unavailable
            || self.disclosure == ActivityDisclosure::Unavailable
        {
            return Err(InvalidObservableActivityProfile::new(
                "Supported activity kinds cannot use unavailable fidelity",
            ));
        }
        match self.disclosure {
            ActivityDisclosure::IdentityAndLifecycleOnly if !self.content_streams.is_empty() => {
                return Err(InvalidObservableActivityProfile::new(
                    "Identity-only activity disclosure cannot claim content streams",
                ));
            }
            ActivityDisclosure::AdapterNormalizedSummary
                if self
                    .content_streams
                    .iter()
                    .any(|stream| *stream != ActivityContentStream::NormalizedSummary) =>
            {
                return Err(InvalidObservableActivityProfile::new(
                    "Adapter-summary disclosure can claim only normalized summaries",
                ));
            }
            ActivityDisclosure::ProviderDisplayContent
                if self
                    .content_streams
                    .contains(&ActivityContentStream::NormalizedSummary) =>
            {
                return Err(InvalidObservableActivityProfile::new(
                    "Provider-display disclosure cannot claim adapter-normalized summaries",
                ));
            }
            _ => {}
        }
        if self
            .content_streams
            .iter()
            .any(|stream| !stream_matches_kind(self.kind, *stream))
        {
            return Err(InvalidObservableActivityProfile::new(
                "Activity content stream does not match its activity kind",
            ));
        }
        if self.task_list_snapshots
            && !matches!(self.kind, ActivityKindClass::Plan | ActivityKindClass::Task)
        {
            return Err(InvalidObservableActivityProfile::new(
                "Task-list snapshots require plan or task activity",
            ));
        }
        if (self.subagent_observation.is_some() || !self.subagent_control_actions.is_empty())
            && self.kind != ActivityKindClass::SubagentOrCollaboration
        {
            return Err(InvalidObservableActivityProfile::new(
                "Subagent detail requires subagent or collaboration activity",
            ));
        }
        Ok(())
    }
}

fn satisfied_subagent_constraints(
    fidelity: SubagentObservationFidelity,
) -> &'static [SubagentObservationFidelity] {
    match fidelity {
        SubagentObservationFidelity::AttributedActivity => &[
            SubagentObservationFidelity::AttributedActivity,
            SubagentObservationFidelity::ParentAndMetadata,
            SubagentObservationFidelity::IdentityAndLifecycle,
        ],
        SubagentObservationFidelity::ParentAndMetadata => &[
            SubagentObservationFidelity::ParentAndMetadata,
            SubagentObservationFidelity::IdentityAndLifecycle,
        ],
        SubagentObservationFidelity::IdentityAndLifecycle => {
            &[SubagentObservationFidelity::IdentityAndLifecycle]
        }
    }
}

fn stream_matches_kind(kind: ActivityKindClass, stream: ActivityContentStream) -> bool {
    match stream {
        ActivityContentStream::IntermediateAssistantText
        | ActivityContentStream::FinalAnswerText => kind == ActivityKindClass::AssistantMessage,
        ActivityContentStream::ReasoningSummaryText => kind == ActivityKindClass::ReasoningSummary,
        ActivityContentStream::PlanText => kind == ActivityKindClass::Plan,
        ActivityContentStream::CommandOutput => kind == ActivityKindClass::CommandExecution,
        ActivityContentStream::FileChangeOutput => kind == ActivityKindClass::FileChange,
        ActivityContentStream::ProviderToolDisplay => matches!(
            kind,
            ActivityKindClass::ProviderOwnedTool
                | ActivityKindClass::ConsumerOwnedTool
                | ActivityKindClass::ExternalSearch
                | ActivityKindClass::ImageView
        ),
        ActivityContentStream::NormalizedSummary => true,
    }
}

fn satisfied_lifecycle_constraints(
    fidelity: ActivityLifecycleFidelity,
) -> &'static [ActivityLifecycleFidelity] {
    match fidelity {
        ActivityLifecycleFidelity::CompleteLifecycle => &[
            ActivityLifecycleFidelity::CompleteLifecycle,
            ActivityLifecycleFidelity::UpdateAndCompletion,
            ActivityLifecycleFidelity::CompletionOnly,
        ],
        ActivityLifecycleFidelity::UpdateAndCompletion => &[
            ActivityLifecycleFidelity::UpdateAndCompletion,
            ActivityLifecycleFidelity::CompletionOnly,
        ],
        ActivityLifecycleFidelity::CompletionOnly => &[ActivityLifecycleFidelity::CompletionOnly],
        ActivityLifecycleFidelity::Unavailable => &[],
    }
}

fn satisfied_disclosure_constraints(
    disclosure: ActivityDisclosure,
) -> &'static [ActivityDisclosure] {
    match disclosure {
        ActivityDisclosure::ProviderDisplayContent => &[
            ActivityDisclosure::ProviderDisplayContent,
            ActivityDisclosure::IdentityAndLifecycleOnly,
        ],
        ActivityDisclosure::AdapterNormalizedSummary => &[
            ActivityDisclosure::AdapterNormalizedSummary,
            ActivityDisclosure::IdentityAndLifecycleOnly,
        ],
        ActivityDisclosure::IdentityAndLifecycleOnly => {
            &[ActivityDisclosure::IdentityAndLifecycleOnly]
        }
        ActivityDisclosure::Unavailable => &[],
    }
}

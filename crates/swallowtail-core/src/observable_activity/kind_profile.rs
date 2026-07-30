use super::{
    ActivityContentStream, ActivityCorrelationKind, ActivityDisclosure, ActivityKindClass,
    ActivityLifecycleFidelity, InvalidObservableActivityProfile,
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
}

impl ActivityKindProfile {
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
        };
        profile.validate()?;
        Ok(profile)
    }

    #[must_use]
    pub const fn kind(&self) -> ActivityKindClass {
        self.kind
    }

    #[must_use]
    pub const fn lifecycle(&self) -> ActivityLifecycleFidelity {
        self.lifecycle
    }

    pub fn content_streams(&self) -> impl ExactSizeIterator<Item = ActivityContentStream> + '_ {
        self.content_streams.iter().copied()
    }

    #[must_use]
    pub const fn disclosure(&self) -> ActivityDisclosure {
        self.disclosure
    }

    pub fn correlations(&self) -> impl ExactSizeIterator<Item = ActivityCorrelationKind> + '_ {
        self.correlations.iter().copied()
    }

    pub fn with_task_list_snapshots(mut self) -> Result<Self, InvalidObservableActivityProfile> {
        self.task_list_snapshots = true;
        self.validate()?;
        Ok(self)
    }

    #[must_use]
    pub const fn task_list_snapshots(&self) -> bool {
        self.task_list_snapshots
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
        Ok(())
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

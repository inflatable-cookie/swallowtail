#![deny(missing_docs)]

use crate::{
    ActivityObservation, CallbackId, DirectToolCallId, HarnessUiDisplay, OperationContent,
    ProviderObservation, ProviderOperationCheckpoint, ProviderRecoveredResourceCleanupBinding,
    ProviderRunCheckpoint,
};

/// Buffering strength required for one runtime event.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EventDelivery {
    /// Ordered evidence that cannot be discarded without semantic loss.
    Semantic,
    /// Replaceable progress evidence that may be coalesced under pressure.
    Coalescible,
}

/// Portable kind of an ordered runtime operation event.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RuntimeEventKind {
    /// Operation execution has started.
    Started,
    /// Provider or runtime progress carrying semantic ordering.
    Progress,
    /// Provider-owned external-search progress.
    ExternalSearchProgress,
    /// Readable reasoning-summary progress.
    ReasoningProgress,
    /// Replaceable current progress snapshot.
    ProgressSnapshot,
    /// Liveness signal with no new semantic state.
    Keepalive,
    /// Incremental operation output.
    OutputDelta,
    /// Complete operation output is available.
    OutputAvailable,
    /// A correlated consumer callback is waiting for a response.
    CallbackRequested(CallbackId),
    /// A correlated direct tool call is waiting for a result.
    DirectToolCallAvailable(DirectToolCallId),
    /// Provider-visible agent activity.
    Activity(ActivityObservation),
    /// Typed provider metadata distinct from activity.
    ProviderObservation(ProviderObservation),
    /// Provider-requested harness display state.
    HarnessUiDisplay(HarnessUiDisplay),
}

impl RuntimeEventKind {
    /// Returns whether buffering may coalesce this event.
    #[must_use]
    pub const fn delivery(&self) -> EventDelivery {
        match self {
            Self::ProgressSnapshot | Self::Keepalive => EventDelivery::Coalescible,
            Self::Started
            | Self::Progress
            | Self::ExternalSearchProgress
            | Self::ReasoningProgress
            | Self::OutputDelta
            | Self::OutputAvailable
            | Self::CallbackRequested(_)
            | Self::DirectToolCallAvailable(_)
            | Self::Activity(_)
            | Self::ProviderObservation(_) => EventDelivery::Semantic,
            Self::HarnessUiDisplay(_) => EventDelivery::Semantic,
        }
    }
}

/// One sequenced portable event emitted by a run or turn.
///
/// Optional checkpoints and cleanup bindings are durable semantic evidence and
/// force semantic delivery even when the event kind itself is coalescible.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RuntimeEvent {
    sequence: u64,
    kind: RuntimeEventKind,
    content: Option<OperationContent>,
    reconciliation_checkpoint: Option<ProviderOperationCheckpoint>,
    run_reconciliation_checkpoint: Option<ProviderRunCheckpoint>,
    recovered_resource_cleanup_binding: Option<ProviderRecoveredResourceCleanupBinding>,
}

impl RuntimeEvent {
    /// Creates an event without operation content.
    #[must_use]
    pub const fn new(sequence: u64, kind: RuntimeEventKind) -> Self {
        Self {
            sequence,
            kind,
            content: None,
            reconciliation_checkpoint: None,
            run_reconciliation_checkpoint: None,
            recovered_resource_cleanup_binding: None,
        }
    }

    #[must_use]
    /// Creates an event carrying potentially sensitive operation content.
    pub fn with_content(sequence: u64, kind: RuntimeEventKind, content: OperationContent) -> Self {
        Self {
            sequence,
            kind,
            content: Some(content),
            reconciliation_checkpoint: None,
            run_reconciliation_checkpoint: None,
            recovered_resource_cleanup_binding: None,
        }
    }

    #[must_use]
    /// Adds a durable provider-session reconciliation checkpoint.
    pub fn with_reconciliation_checkpoint(
        mut self,
        checkpoint: ProviderOperationCheckpoint,
    ) -> Self {
        self.reconciliation_checkpoint = Some(checkpoint);
        self
    }

    #[must_use]
    /// Adds a durable provider-run reconciliation checkpoint.
    pub fn with_run_reconciliation_checkpoint(mut self, checkpoint: ProviderRunCheckpoint) -> Self {
        self.run_reconciliation_checkpoint = Some(checkpoint);
        self
    }

    #[must_use]
    /// Adds separately persisted authority for recovered-resource cleanup.
    pub fn with_recovered_resource_cleanup_binding(
        mut self,
        binding: ProviderRecoveredResourceCleanupBinding,
    ) -> Self {
        self.recovered_resource_cleanup_binding = Some(binding);
        self
    }

    #[must_use]
    /// Returns the operation-local monotonic event sequence.
    pub const fn sequence(&self) -> u64 {
        self.sequence
    }

    #[must_use]
    /// Returns the portable event kind.
    pub const fn kind(&self) -> &RuntimeEventKind {
        &self.kind
    }

    #[must_use]
    /// Returns potentially sensitive operation content when present.
    pub const fn content(&self) -> Option<&OperationContent> {
        self.content.as_ref()
    }

    #[must_use]
    /// Returns the provider-session reconciliation checkpoint when present.
    pub const fn reconciliation_checkpoint(&self) -> Option<&ProviderOperationCheckpoint> {
        self.reconciliation_checkpoint.as_ref()
    }

    #[must_use]
    /// Returns the provider-run reconciliation checkpoint when present.
    pub const fn run_reconciliation_checkpoint(&self) -> Option<&ProviderRunCheckpoint> {
        self.run_reconciliation_checkpoint.as_ref()
    }

    #[must_use]
    /// Returns recovered-resource cleanup authority when present.
    pub const fn recovered_resource_cleanup_binding(
        &self,
    ) -> Option<&ProviderRecoveredResourceCleanupBinding> {
        self.recovered_resource_cleanup_binding.as_ref()
    }

    #[must_use]
    /// Returns the buffering strength of this complete event.
    pub const fn delivery(&self) -> EventDelivery {
        if self.reconciliation_checkpoint.is_some()
            || self.run_reconciliation_checkpoint.is_some()
            || self.recovered_resource_cleanup_binding.is_some()
        {
            EventDelivery::Semantic
        } else {
            self.kind.delivery()
        }
    }
}

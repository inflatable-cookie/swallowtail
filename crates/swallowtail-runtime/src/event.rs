use crate::{
    ActivityObservation, CallbackId, DirectToolCallId, HarnessUiDisplay, OperationContent,
    ProviderObservation, ProviderOperationCheckpoint, ProviderRunCheckpoint,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EventDelivery {
    Semantic,
    Coalescible,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RuntimeEventKind {
    Started,
    Progress,
    ExternalSearchProgress,
    ReasoningProgress,
    ProgressSnapshot,
    Keepalive,
    OutputDelta,
    OutputAvailable,
    CallbackRequested(CallbackId),
    DirectToolCallAvailable(DirectToolCallId),
    Activity(ActivityObservation),
    ProviderObservation(ProviderObservation),
    HarnessUiDisplay(HarnessUiDisplay),
}

impl RuntimeEventKind {
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

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RuntimeEvent {
    sequence: u64,
    kind: RuntimeEventKind,
    content: Option<OperationContent>,
    reconciliation_checkpoint: Option<ProviderOperationCheckpoint>,
    run_reconciliation_checkpoint: Option<ProviderRunCheckpoint>,
}

impl RuntimeEvent {
    #[must_use]
    pub const fn new(sequence: u64, kind: RuntimeEventKind) -> Self {
        Self {
            sequence,
            kind,
            content: None,
            reconciliation_checkpoint: None,
            run_reconciliation_checkpoint: None,
        }
    }

    #[must_use]
    pub fn with_content(sequence: u64, kind: RuntimeEventKind, content: OperationContent) -> Self {
        Self {
            sequence,
            kind,
            content: Some(content),
            reconciliation_checkpoint: None,
            run_reconciliation_checkpoint: None,
        }
    }

    #[must_use]
    pub fn with_reconciliation_checkpoint(
        mut self,
        checkpoint: ProviderOperationCheckpoint,
    ) -> Self {
        self.reconciliation_checkpoint = Some(checkpoint);
        self
    }

    #[must_use]
    pub fn with_run_reconciliation_checkpoint(mut self, checkpoint: ProviderRunCheckpoint) -> Self {
        self.run_reconciliation_checkpoint = Some(checkpoint);
        self
    }

    #[must_use]
    pub const fn sequence(&self) -> u64 {
        self.sequence
    }

    #[must_use]
    pub const fn kind(&self) -> &RuntimeEventKind {
        &self.kind
    }

    #[must_use]
    pub const fn content(&self) -> Option<&OperationContent> {
        self.content.as_ref()
    }

    #[must_use]
    pub const fn reconciliation_checkpoint(&self) -> Option<&ProviderOperationCheckpoint> {
        self.reconciliation_checkpoint.as_ref()
    }

    #[must_use]
    pub const fn run_reconciliation_checkpoint(&self) -> Option<&ProviderRunCheckpoint> {
        self.run_reconciliation_checkpoint.as_ref()
    }

    #[must_use]
    pub const fn delivery(&self) -> EventDelivery {
        if self.reconciliation_checkpoint.is_some() || self.run_reconciliation_checkpoint.is_some()
        {
            EventDelivery::Semantic
        } else {
            self.kind.delivery()
        }
    }
}

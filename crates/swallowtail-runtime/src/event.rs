use crate::{CallbackId, HarnessUiDisplay, OperationContent, ProviderObservation};

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
}

impl RuntimeEvent {
    #[must_use]
    pub const fn new(sequence: u64, kind: RuntimeEventKind) -> Self {
        Self {
            sequence,
            kind,
            content: None,
        }
    }

    #[must_use]
    pub fn with_content(sequence: u64, kind: RuntimeEventKind, content: OperationContent) -> Self {
        Self {
            sequence,
            kind,
            content: Some(content),
        }
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
    pub const fn delivery(&self) -> EventDelivery {
        self.kind.delivery()
    }
}

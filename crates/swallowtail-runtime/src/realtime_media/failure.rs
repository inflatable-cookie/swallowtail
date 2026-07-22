use std::error::Error;
use std::fmt;
use swallowtail_core::SafeDiagnostic;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RealtimeMediaFailureKind {
    EmptyChunk,
    ChunkTooLarge,
    FormatMismatch,
    SessionMismatch,
    StreamMismatch,
    SequenceInvalid,
    OrderingInvalid,
    TurnLimitReached,
    SessionClosed,
    EmptyTranscript,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RealtimeMediaFailure {
    kind: RealtimeMediaFailureKind,
    diagnostic: SafeDiagnostic,
}

impl RealtimeMediaFailure {
    pub(crate) fn new(kind: RealtimeMediaFailureKind, message: &'static str) -> Self {
        Self {
            kind,
            diagnostic: SafeDiagnostic::new("swallowtail.realtime_media_rejected", message),
        }
    }

    #[must_use]
    pub const fn kind(&self) -> RealtimeMediaFailureKind {
        self.kind
    }

    #[must_use]
    pub const fn diagnostic(&self) -> &SafeDiagnostic {
        &self.diagnostic
    }
}

impl fmt::Display for RealtimeMediaFailure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.diagnostic.fmt(formatter)
    }
}

impl Error for RealtimeMediaFailure {}

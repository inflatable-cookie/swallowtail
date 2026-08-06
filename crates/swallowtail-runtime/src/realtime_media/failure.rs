use std::error::Error;
use std::fmt;
use swallowtail_core::SafeDiagnostic;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
/// Portable reason a realtime media operation was rejected locally.
pub enum RealtimeMediaFailureKind {
    /// A media chunk contained no bytes.
    EmptyChunk,
    /// A media chunk exceeded the preflight-bound size.
    ChunkTooLarge,
    /// Media direction or format disagreed with the immutable session format.
    FormatMismatch,
    /// Media belonged to a different runtime session.
    SessionMismatch,
    /// Append, commit, or output crossed media stream identities.
    StreamMismatch,
    /// A sequence contained a gap, duplicate, or regression.
    SequenceInvalid,
    /// An event or command violated the session lifecycle order.
    OrderingInvalid,
    /// The session reached its preflight-bound turn count.
    TurnLimitReached,
    /// The session is terminal and cannot accept further work.
    SessionClosed,
    /// A transcript value was empty.
    EmptyTranscript,
}

#[derive(Clone, Debug, Eq, PartialEq)]
/// Safe, provider-neutral rejection from realtime media admission or ordering.
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
    /// Returns the machine-comparable failure reason.
    pub const fn kind(&self) -> RealtimeMediaFailureKind {
        self.kind
    }

    #[must_use]
    /// Returns the safe diagnostic suitable for consumer display or logging.
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

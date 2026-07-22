use super::{MediaChunk, MediaTranscript};
use crate::{ProviderCancellationOutcome, ProviderObservation, RuntimeTurnId};
use std::num::NonZeroU64;
use swallowtail_core::SafeDiagnostic;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RealtimeMediaResponseStatus {
    Completed,
    Cancelled(ProviderCancellationOutcome),
    TimedOut(ProviderCancellationOutcome),
    Failed(SafeDiagnostic),
    Disconnected,
}

impl RealtimeMediaResponseStatus {
    #[must_use]
    pub const fn ends_session(&self) -> bool {
        !matches!(self, Self::Completed)
    }
}

#[derive(Debug)]
pub enum RealtimeMediaEventKind {
    ResponseStarted,
    OutputAudio(MediaChunk),
    TranscriptDelta(MediaTranscript),
    TranscriptCompleted(MediaTranscript),
    ProviderObservation(ProviderObservation),
    ResponseTerminal(RealtimeMediaResponseStatus),
}

#[derive(Debug)]
pub struct RealtimeMediaEvent {
    sequence: NonZeroU64,
    turn_id: RuntimeTurnId,
    kind: RealtimeMediaEventKind,
}

impl RealtimeMediaEvent {
    #[must_use]
    pub const fn new(
        sequence: NonZeroU64,
        turn_id: RuntimeTurnId,
        kind: RealtimeMediaEventKind,
    ) -> Self {
        Self {
            sequence,
            turn_id,
            kind,
        }
    }

    #[must_use]
    pub const fn sequence(&self) -> NonZeroU64 {
        self.sequence
    }

    #[must_use]
    pub const fn turn_id(&self) -> &RuntimeTurnId {
        &self.turn_id
    }

    #[must_use]
    pub const fn kind(&self) -> &RealtimeMediaEventKind {
        &self.kind
    }
}

use super::{MediaChunk, MediaTranscript};
use crate::{ProviderCancellationOutcome, ProviderObservation, RuntimeTurnId};
use std::num::NonZeroU64;
use swallowtail_core::SafeDiagnostic;

#[derive(Clone, Debug, Eq, PartialEq)]
/// Terminal outcome for one realtime media response.
pub enum RealtimeMediaResponseStatus {
    /// The response completed and the session may remain reusable.
    Completed,
    /// Cancellation reached the provider with the recorded outcome.
    Cancelled(ProviderCancellationOutcome),
    /// The deadline elapsed with the recorded provider cancellation outcome.
    TimedOut(ProviderCancellationOutcome),
    /// The provider response failed with a safe diagnostic.
    Failed(SafeDiagnostic),
    /// The live provider connection ended before response completion.
    Disconnected,
}

impl RealtimeMediaResponseStatus {
    #[must_use]
    /// Returns whether this terminal outcome makes the session non-reusable.
    pub const fn ends_session(&self) -> bool {
        !matches!(self, Self::Completed)
    }
}

#[derive(Debug)]
/// Ordered observation emitted during one realtime media response.
pub enum RealtimeMediaEventKind {
    /// The provider accepted the committed input and began the response.
    ResponseStarted,
    /// One opaque, ordered output-audio chunk.
    OutputAudio(MediaChunk),
    /// A nonterminal transcript fragment.
    TranscriptDelta(MediaTranscript),
    /// The terminal transcript value for the response.
    TranscriptCompleted(MediaTranscript),
    /// A provider observation that does not alter media ordering.
    ProviderObservation(ProviderObservation),
    /// The exact terminal response outcome.
    ResponseTerminal(RealtimeMediaResponseStatus),
}

#[derive(Debug)]
/// One monotonically sequenced event attributed to a realtime media turn.
pub struct RealtimeMediaEvent {
    sequence: NonZeroU64,
    turn_id: RuntimeTurnId,
    kind: RealtimeMediaEventKind,
}

impl RealtimeMediaEvent {
    #[must_use]
    /// Creates an event with exact session event sequence and turn identity.
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
    /// Returns the one-based sequence across session response events.
    pub const fn sequence(&self) -> NonZeroU64 {
        self.sequence
    }

    #[must_use]
    /// Returns the turn to which this event is attributed.
    pub const fn turn_id(&self) -> &RuntimeTurnId {
        &self.turn_id
    }

    #[must_use]
    /// Returns the event payload and lifecycle kind.
    pub const fn kind(&self) -> &RealtimeMediaEventKind {
        &self.kind
    }
}

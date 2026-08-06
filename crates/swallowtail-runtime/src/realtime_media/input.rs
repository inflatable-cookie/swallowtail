use super::{RealtimeMediaFailure, RealtimeMediaFailureKind};
use crate::{MediaStreamId, RuntimeSessionId, RuntimeTurnId};
use std::fmt;
use std::num::NonZeroU64;
use swallowtail_core::{MediaDirection, MediaFormat, RealtimeMediaConfig};
use zeroize::Zeroize;

/// One bounded opaque media chunk with exact session, stream, and sequence identity.
pub struct MediaChunk {
    session_id: RuntimeSessionId,
    stream_id: MediaStreamId,
    sequence: NonZeroU64,
    direction: MediaDirection,
    format: MediaFormat,
    bytes: Vec<u8>,
}

impl MediaChunk {
    /// Creates a chunk after enforcing nonempty content, size, and format bounds.
    pub fn new(
        session_id: RuntimeSessionId,
        stream_id: MediaStreamId,
        sequence: NonZeroU64,
        direction: MediaDirection,
        format: MediaFormat,
        bytes: Vec<u8>,
        config: &RealtimeMediaConfig,
    ) -> Result<Self, RealtimeMediaFailure> {
        if bytes.is_empty() {
            return Err(RealtimeMediaFailure::new(
                RealtimeMediaFailureKind::EmptyChunk,
                "Realtime media chunks must not be empty",
            ));
        }
        if u64::try_from(bytes.len()).map_or(true, |len| len > config.maximum_chunk_bytes().get()) {
            return Err(RealtimeMediaFailure::new(
                RealtimeMediaFailureKind::ChunkTooLarge,
                "Realtime media chunk exceeds its preflight-bound maximum",
            ));
        }
        let expected = match direction {
            MediaDirection::Input => config.input_format(),
            MediaDirection::Output => config.output_format(),
        };
        if format != expected {
            return Err(RealtimeMediaFailure::new(
                RealtimeMediaFailureKind::FormatMismatch,
                "Realtime media chunk format does not match the session",
            ));
        }
        Ok(Self {
            session_id,
            stream_id,
            sequence,
            direction,
            format,
            bytes,
        })
    }

    #[must_use]
    /// Returns the runtime session that owns this chunk.
    pub const fn session_id(&self) -> &RuntimeSessionId {
        &self.session_id
    }

    #[must_use]
    /// Returns the logical media stream identity.
    pub const fn stream_id(&self) -> &MediaStreamId {
        &self.stream_id
    }

    #[must_use]
    /// Returns the one-based sequence within the media stream.
    pub const fn sequence(&self) -> NonZeroU64 {
        self.sequence
    }

    #[must_use]
    /// Returns whether the chunk is provider input or output.
    pub const fn direction(&self) -> MediaDirection {
        self.direction
    }

    #[must_use]
    /// Returns the exact preflight-admitted media format.
    pub const fn format(&self) -> MediaFormat {
        self.format
    }

    #[must_use]
    /// Borrows the opaque media bytes.
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }
}

impl fmt::Debug for MediaChunk {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("MediaChunk")
            .field("session_id", &self.session_id)
            .field("stream_id", &self.stream_id)
            .field("sequence", &self.sequence)
            .field("direction", &self.direction)
            .field("format", &self.format)
            .field(
                "bytes",
                &format_args!("<redacted:{} bytes>", self.bytes.len()),
            )
            .finish()
    }
}

impl Drop for MediaChunk {
    fn drop(&mut self) {
        self.bytes.zeroize();
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
/// Consumer authorization to commit one accumulated input stream as a turn.
pub struct MediaInputCommit {
    turn_id: RuntimeTurnId,
    stream_id: MediaStreamId,
}

impl MediaInputCommit {
    #[must_use]
    /// Creates commit evidence for an exact turn and input stream.
    pub const fn new(turn_id: RuntimeTurnId, stream_id: MediaStreamId) -> Self {
        Self { turn_id, stream_id }
    }

    #[must_use]
    /// Returns the turn created by this commit.
    pub const fn turn_id(&self) -> &RuntimeTurnId {
        &self.turn_id
    }

    #[must_use]
    /// Returns the committed input stream identity.
    pub const fn stream_id(&self) -> &MediaStreamId {
        &self.stream_id
    }
}

#[derive(Clone, Eq, PartialEq)]
/// Nonempty, redacted transcript text from a realtime response.
pub struct MediaTranscript(String);

impl MediaTranscript {
    /// Creates transcript content, rejecting an empty value.
    pub fn new(value: impl Into<String>) -> Result<Self, RealtimeMediaFailure> {
        let value = value.into();
        if value.is_empty() {
            Err(RealtimeMediaFailure::new(
                RealtimeMediaFailureKind::EmptyTranscript,
                "Realtime media transcript must not be empty",
            ))
        } else {
            Ok(Self(value))
        }
    }

    #[must_use]
    /// Borrows the transcript text.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Debug for MediaTranscript {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_tuple("MediaTranscript")
            .field(&format_args!("<redacted:{} bytes>", self.0.len()))
            .finish()
    }
}

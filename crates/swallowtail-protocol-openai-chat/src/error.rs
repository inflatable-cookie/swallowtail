use std::error::Error;
use std::fmt;

/// Stable classification of a structural chat codec failure.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProtocolErrorKind {
    /// Buffered SSE input exceeded the configured wire bound.
    BufferLimitExceeded,
    /// A completion chunk contained too many choices.
    ChoiceLimitExceeded,
    /// A JSON object contained too many fields.
    FieldLimitExceeded,
    /// The SSE stream ended with an unfinished record.
    IncompleteRecord,
    /// A provider payload was not valid JSON.
    InvalidJson,
    /// A decoded or requested document had an invalid structural shape.
    InvalidStructure,
    /// An SSE record was not valid UTF-8.
    InvalidUtf8,
    /// A request contained no messages or exceeded the message bound.
    MessageLimitExceeded,
    /// A validated request could not be serialized.
    SerializationFailed,
    /// A structural string exceeded the configured byte bound.
    StringLimitExceeded,
    /// An SSE record used a field other than comments and `data`.
    UnsupportedSseField,
    /// An encoded document or complete SSE record exceeded the wire bound.
    WireLimitExceeded,
}

/// Bounded structural codec error without provider payload content.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ProtocolError {
    kind: ProtocolErrorKind,
}

impl ProtocolError {
    pub(crate) const fn new(kind: ProtocolErrorKind) -> Self {
        Self { kind }
    }

    /// Returns the stable failure classification.
    #[must_use]
    pub const fn kind(self) -> ProtocolErrorKind {
        self.kind
    }
}

impl fmt::Display for ProtocolError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self.kind {
            ProtocolErrorKind::BufferLimitExceeded => "SSE input buffer limit exceeded",
            ProtocolErrorKind::ChoiceLimitExceeded => "chat choice limit exceeded",
            ProtocolErrorKind::FieldLimitExceeded => "chat object field limit exceeded",
            ProtocolErrorKind::IncompleteRecord => "SSE input ended during a record",
            ProtocolErrorKind::InvalidJson => "chat record is not valid JSON",
            ProtocolErrorKind::InvalidStructure => "chat record structure is invalid",
            ProtocolErrorKind::InvalidUtf8 => "SSE record is not valid UTF-8",
            ProtocolErrorKind::MessageLimitExceeded => "chat message limit exceeded",
            ProtocolErrorKind::SerializationFailed => "chat request could not be serialized",
            ProtocolErrorKind::StringLimitExceeded => "chat string limit exceeded",
            ProtocolErrorKind::UnsupportedSseField => "SSE record contains an unsupported field",
            ProtocolErrorKind::WireLimitExceeded => "chat wire document limit exceeded",
        })
    }
}

impl Error for ProtocolError {}

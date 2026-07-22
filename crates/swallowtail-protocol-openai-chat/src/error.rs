use std::error::Error;
use std::fmt;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProtocolErrorKind {
    BufferLimitExceeded,
    ChoiceLimitExceeded,
    FieldLimitExceeded,
    IncompleteRecord,
    InvalidJson,
    InvalidStructure,
    InvalidUtf8,
    MessageLimitExceeded,
    SerializationFailed,
    StringLimitExceeded,
    UnsupportedSseField,
    WireLimitExceeded,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ProtocolError {
    kind: ProtocolErrorKind,
}

impl ProtocolError {
    pub(crate) const fn new(kind: ProtocolErrorKind) -> Self {
        Self { kind }
    }

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

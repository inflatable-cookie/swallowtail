//! Provider-neutral structural codec for compatible Chat Completions wires.

#![forbid(unsafe_code)]
#![deny(missing_docs)]

mod error;
mod json;
mod request;
mod sse;

pub use error::{ProtocolError, ProtocolErrorKind};
pub use json::{
    Choice, Chunk, Delta, ErrorEnvelope, Payload, ProviderError, UnknownField, Usage,
    decode_payload,
};
pub use request::{ChatMessage, ChatRequest, encode_request};
pub use sse::{SseDecoder, SseRecord};

/// Default bound for one buffered SSE record or encoded JSON document.
pub const DEFAULT_MAX_WIRE_BYTES: usize = 1_048_576;
/// Default bound for fields retained from one JSON object.
pub const DEFAULT_MAX_FIELDS: usize = 32;
/// Default bound for choices in one completion chunk.
pub const DEFAULT_MAX_CHOICES: usize = 8;
/// Default bound for messages in one structural request.
pub const DEFAULT_MAX_MESSAGES: usize = 256;
/// Default bound for one structural string.
pub const DEFAULT_MAX_STRING_BYTES: usize = 262_144;

/// Explicit safety limits applied while decoding and encoding chat wires.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CodecLimits {
    maximum_wire_bytes: usize,
    maximum_fields: usize,
    maximum_choices: usize,
    maximum_messages: usize,
    maximum_string_bytes: usize,
}

impl CodecLimits {
    /// Creates an exact set of wire, object, choice, message, and string limits.
    #[must_use]
    pub const fn new(
        maximum_wire_bytes: usize,
        maximum_fields: usize,
        maximum_choices: usize,
        maximum_messages: usize,
        maximum_string_bytes: usize,
    ) -> Self {
        Self {
            maximum_wire_bytes,
            maximum_fields,
            maximum_choices,
            maximum_messages,
            maximum_string_bytes,
        }
    }

    /// Returns the maximum encoded document or buffered SSE record size.
    #[must_use]
    pub const fn maximum_wire_bytes(self) -> usize {
        self.maximum_wire_bytes
    }

    /// Returns the maximum fields retained from any one JSON object.
    #[must_use]
    pub const fn maximum_fields(self) -> usize {
        self.maximum_fields
    }

    /// Returns the maximum choices accepted in one completion chunk.
    #[must_use]
    pub const fn maximum_choices(self) -> usize {
        self.maximum_choices
    }

    /// Returns the maximum messages accepted in one request.
    #[must_use]
    pub const fn maximum_messages(self) -> usize {
        self.maximum_messages
    }

    /// Returns the maximum bytes accepted in one structural string.
    #[must_use]
    pub const fn maximum_string_bytes(self) -> usize {
        self.maximum_string_bytes
    }
}

impl Default for CodecLimits {
    fn default() -> Self {
        Self::new(
            DEFAULT_MAX_WIRE_BYTES,
            DEFAULT_MAX_FIELDS,
            DEFAULT_MAX_CHOICES,
            DEFAULT_MAX_MESSAGES,
            DEFAULT_MAX_STRING_BYTES,
        )
    }
}

#[cfg(test)]
mod tests;

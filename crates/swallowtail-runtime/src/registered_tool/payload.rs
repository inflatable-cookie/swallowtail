//! Bounded call, progress, and result payloads that never reach diagnostics.

use super::failure::{RegisteredToolFailure, RegisteredToolFailureKind, reject};
use super::schema::RegisteredToolSchemaMediaType;
use std::fmt;

/// Bounded opaque payload carried by one call, progress item, or result.
///
/// Contents are available only through an explicit execution accessor. They
/// never appear in `Debug`, `Display`, diagnostics, events, or durable records.
#[derive(Clone, Eq, PartialEq)]
pub struct RegisteredToolPayload {
    media_type: RegisteredToolSchemaMediaType,
    body: Vec<u8>,
}

impl RegisteredToolPayload {
    /// Admits one payload within an explicit positive byte bound.
    pub fn new(
        media_type: RegisteredToolSchemaMediaType,
        body: impl Into<Vec<u8>>,
        max_bytes: usize,
    ) -> Result<Self, RegisteredToolFailure> {
        let body = body.into();
        if max_bytes == 0 {
            return Err(reject(RegisteredToolFailureKind::IdentityRejected));
        }
        if body.len() > max_bytes {
            return Err(reject(RegisteredToolFailureKind::LimitExceeded));
        }
        Ok(Self { media_type, body })
    }

    /// Returns the exact declared media type.
    #[must_use]
    pub const fn media_type(&self) -> &RegisteredToolSchemaMediaType {
        &self.media_type
    }

    /// Returns the payload bytes for validation or dispatch execution only.
    #[must_use]
    pub fn expose_for_execution(&self) -> &[u8] {
        &self.body
    }

    /// Returns the exact payload byte length.
    #[must_use]
    pub fn byte_len(&self) -> usize {
        self.body.len()
    }
}

impl fmt::Debug for RegisteredToolPayload {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("RegisteredToolPayload")
            .field("media_type", &self.media_type)
            .field("body", &"<redacted>")
            .field("bytes", &self.body.len())
            .finish()
    }
}

impl fmt::Display for RegisteredToolPayload {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("<redacted registered tool payload>")
    }
}

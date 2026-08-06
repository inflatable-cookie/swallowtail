use super::{AcpSessionListDecodeError, AcpSessionListDecodeErrorKind, error};
use crate::AcpBoundedText;
use serde_json::{Map, Value};
use std::collections::BTreeSet;
use std::fmt;

/// Bounded unrecognized fields retained from an ACP session-list object.
#[derive(Clone, PartialEq)]
pub struct AcpOpaqueExtensions {
    value: Value,
    encoded_bytes: usize,
}

impl AcpOpaqueExtensions {
    pub(super) fn from_fields(
        object: &Map<String, Value>,
        known: &[&str],
        limit: usize,
    ) -> Result<Self, AcpSessionListDecodeError> {
        let known: BTreeSet<_> = known.iter().copied().collect();
        let value = Value::Object(
            object
                .iter()
                .filter(|(key, _)| !known.contains(key.as_str()))
                .map(|(key, value)| (key.clone(), value.clone()))
                .collect(),
        );
        let encoded_bytes = serde_json::to_vec(&value)
            .map_err(|_| error(AcpSessionListDecodeErrorKind::ExtensionInvalid))?
            .len();
        if encoded_bytes > limit {
            return Err(error(AcpSessionListDecodeErrorKind::LimitExceeded));
        }
        Ok(Self {
            value,
            encoded_bytes,
        })
    }

    /// Returns whether no unrecognized fields were present.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.value.as_object().is_none_or(Map::is_empty)
    }

    /// Returns the encoded byte size used for limit enforcement.
    #[must_use]
    pub const fn byte_len(&self) -> usize {
        self.encoded_bytes
    }
}

impl fmt::Debug for AcpOpaqueExtensions {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("AcpOpaqueExtensions")
            .field("encoded_bytes", &self.encoded_bytes)
            .field("field_count", &self.value.as_object().map_or(0, Map::len))
            .finish()
    }
}

/// One bounded provider-owned session returned by ACP session listing.
#[derive(Clone, Debug, PartialEq)]
pub struct AcpSessionInfo {
    pub(super) session_id: AcpBoundedText,
    pub(super) cwd: AcpBoundedText,
    pub(super) additional_directories: Vec<AcpBoundedText>,
    pub(super) title: Option<AcpBoundedText>,
    pub(super) updated_at: Option<AcpBoundedText>,
    pub(super) updated_at_unix_milliseconds: Option<u64>,
    pub(super) extensions: AcpOpaqueExtensions,
}

impl AcpSessionInfo {
    /// Returns the provider session identity.
    #[must_use]
    pub fn session_id(&self) -> &str {
        self.session_id.as_str()
    }

    /// Returns the absolute primary working directory.
    #[must_use]
    pub fn cwd(&self) -> &str {
        self.cwd.as_str()
    }

    /// Iterates over additional absolute working directories.
    pub fn additional_directories(&self) -> impl ExactSizeIterator<Item = &str> {
        self.additional_directories
            .iter()
            .map(AcpBoundedText::as_str)
    }

    /// Returns the optional provider session title.
    #[must_use]
    pub fn title(&self) -> Option<&str> {
        self.title.as_ref().map(AcpBoundedText::as_str)
    }

    /// Returns the optional provider timestamp text.
    #[must_use]
    pub fn updated_at(&self) -> Option<&str> {
        self.updated_at.as_ref().map(AcpBoundedText::as_str)
    }

    /// Returns the normalized timestamp in Unix milliseconds when available.
    #[must_use]
    pub const fn updated_at_unix_milliseconds(&self) -> Option<u64> {
        self.updated_at_unix_milliseconds
    }

    /// Returns bounded unrecognized session fields.
    #[must_use]
    pub const fn extensions(&self) -> &AcpOpaqueExtensions {
        &self.extensions
    }
}

/// One bounded page of provider-owned ACP sessions.
#[derive(Clone, Debug, PartialEq)]
pub struct AcpSessionListPage {
    pub(super) sessions: Vec<AcpSessionInfo>,
    pub(super) next_cursor: Option<AcpBoundedText>,
    pub(super) extensions: AcpOpaqueExtensions,
}

impl AcpSessionListPage {
    /// Iterates over sessions in provider order.
    pub fn sessions(&self) -> impl ExactSizeIterator<Item = &AcpSessionInfo> {
        self.sessions.iter()
    }

    /// Returns the next-page cursor when more sessions may exist.
    #[must_use]
    pub fn next_cursor(&self) -> Option<&str> {
        self.next_cursor.as_ref().map(AcpBoundedText::as_str)
    }

    /// Returns bounded unrecognized page fields.
    #[must_use]
    pub const fn extensions(&self) -> &AcpOpaqueExtensions {
        &self.extensions
    }
}

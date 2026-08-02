use super::{AcpSessionListDecodeError, AcpSessionListDecodeErrorKind, error};
use crate::AcpBoundedText;
use serde_json::{Map, Value};
use std::collections::BTreeSet;
use std::fmt;

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

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.value.as_object().is_none_or(Map::is_empty)
    }

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
    #[must_use]
    pub fn session_id(&self) -> &str {
        self.session_id.as_str()
    }

    #[must_use]
    pub fn cwd(&self) -> &str {
        self.cwd.as_str()
    }

    pub fn additional_directories(&self) -> impl ExactSizeIterator<Item = &str> {
        self.additional_directories
            .iter()
            .map(AcpBoundedText::as_str)
    }

    #[must_use]
    pub fn title(&self) -> Option<&str> {
        self.title.as_ref().map(AcpBoundedText::as_str)
    }

    #[must_use]
    pub fn updated_at(&self) -> Option<&str> {
        self.updated_at.as_ref().map(AcpBoundedText::as_str)
    }

    #[must_use]
    pub const fn updated_at_unix_milliseconds(&self) -> Option<u64> {
        self.updated_at_unix_milliseconds
    }

    #[must_use]
    pub const fn extensions(&self) -> &AcpOpaqueExtensions {
        &self.extensions
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct AcpSessionListPage {
    pub(super) sessions: Vec<AcpSessionInfo>,
    pub(super) next_cursor: Option<AcpBoundedText>,
    pub(super) extensions: AcpOpaqueExtensions,
}

impl AcpSessionListPage {
    pub fn sessions(&self) -> impl ExactSizeIterator<Item = &AcpSessionInfo> {
        self.sessions.iter()
    }

    #[must_use]
    pub fn next_cursor(&self) -> Option<&str> {
        self.next_cursor.as_ref().map(AcpBoundedText::as_str)
    }

    #[must_use]
    pub const fn extensions(&self) -> &AcpOpaqueExtensions {
        &self.extensions
    }
}

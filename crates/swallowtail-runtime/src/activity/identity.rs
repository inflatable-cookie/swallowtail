use super::InvalidActivityRecord;
use std::fmt;

const MAX_ACTIVITY_ID_BYTES: usize = 256;
const MAX_ACTIVITY_NAMESPACE_BYTES: usize = 128;

#[derive(Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ActivityId(String);

impl ActivityId {
    pub fn new(value: impl Into<String>) -> Result<Self, InvalidActivityRecord> {
        bounded_identity(
            value,
            MAX_ACTIVITY_ID_BYTES,
            "Activity id must use a non-empty bounded value",
        )
        .map(Self)
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Debug for ActivityId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_tuple("ActivityId")
            .field(&"<redacted>")
            .finish()
    }
}

impl fmt::Display for ActivityId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("<redacted activity id>")
    }
}

/// Bounded provider-owned category for an otherwise unknown activity kind.
#[derive(Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ActivityNamespace(String);

impl ActivityNamespace {
    pub fn new(value: impl Into<String>) -> Result<Self, InvalidActivityRecord> {
        bounded_identity(
            value,
            MAX_ACTIVITY_NAMESPACE_BYTES,
            "Unknown activity namespace must use a non-empty bounded value",
        )
        .map(Self)
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Debug for ActivityNamespace {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_tuple("ActivityNamespace")
            .field(&"<redacted>")
            .finish()
    }
}

impl fmt::Display for ActivityNamespace {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("<redacted activity namespace>")
    }
}

fn bounded_identity(
    value: impl Into<String>,
    maximum_bytes: usize,
    message: &'static str,
) -> Result<String, InvalidActivityRecord> {
    let value = value.into();
    if value.trim().is_empty() || value.len() > maximum_bytes || value.chars().any(char::is_control)
    {
        Err(InvalidActivityRecord::new(message))
    } else {
        Ok(value)
    }
}

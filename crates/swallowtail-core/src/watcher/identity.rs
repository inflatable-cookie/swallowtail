use std::error::Error;
use std::fmt;

/// Maximum UTF-8 byte length for one watcher identity.
pub const MAX_WATCHER_ID_BYTES: usize = 256;

/// Maximum UTF-8 byte length for one owning-turn key.
pub const MAX_WATCHER_OWNING_TURN_BYTES: usize = 256;

/// Stable reason a watcher identity record was rejected.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct InvalidWatcherRecord {
    message: &'static str,
}

impl InvalidWatcherRecord {
    pub(crate) const fn new(message: &'static str) -> Self {
        Self { message }
    }

    #[must_use]
    /// Returns the rejection message.
    pub const fn message(self) -> &'static str {
        self.message
    }
}

impl fmt::Display for InvalidWatcherRecord {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.message)
    }
}

impl Error for InvalidWatcherRecord {}

/// Turn-scoped opaque identity for one host-owned watcher.
///
/// The value is never a PID, process group, provider task id, callback id,
/// activity id, or consumer record id.
#[derive(Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct WatcherId(String);

impl WatcherId {
    /// Creates an identity from a non-empty, control-free bounded value.
    pub fn new(value: impl Into<String>) -> Result<Self, InvalidWatcherRecord> {
        bounded_identity(
            value,
            MAX_WATCHER_ID_BYTES,
            "Watcher id must use a non-empty bounded value",
        )
        .map(Self)
    }

    #[must_use]
    /// Returns the unredacted identity for ownership checks and correlation.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Debug for WatcherId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_tuple("WatcherId")
            .field(&"<redacted>")
            .finish()
    }
}

impl fmt::Display for WatcherId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("<redacted watcher id>")
    }
}

/// Opaque key for the runtime turn that owns a watcher set.
#[derive(Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct WatcherOwningTurn(String);

impl WatcherOwningTurn {
    /// Creates an owning-turn key from a non-empty, control-free bounded value.
    pub fn new(value: impl Into<String>) -> Result<Self, InvalidWatcherRecord> {
        bounded_identity(
            value,
            MAX_WATCHER_OWNING_TURN_BYTES,
            "Watcher owning turn must use a non-empty bounded value",
        )
        .map(Self)
    }

    #[must_use]
    /// Returns the unredacted owning-turn key for ownership checks.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Debug for WatcherOwningTurn {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_tuple("WatcherOwningTurn")
            .field(&"<redacted>")
            .finish()
    }
}

impl fmt::Display for WatcherOwningTurn {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("<redacted watcher owning turn>")
    }
}

fn bounded_identity(
    value: impl Into<String>,
    maximum_bytes: usize,
    message: &'static str,
) -> Result<String, InvalidWatcherRecord> {
    let value = value.into();
    if value.trim().is_empty() || value.len() > maximum_bytes || value.chars().any(char::is_control)
    {
        Err(InvalidWatcherRecord::new(message))
    } else {
        Ok(value)
    }
}

#[cfg(test)]
mod tests {
    use super::{WatcherId, WatcherOwningTurn};

    #[test]
    fn watcher_identity_is_redacted_by_default() {
        let id = WatcherId::new("watcher-1").expect("id is valid");
        let turn = WatcherOwningTurn::new("turn-1").expect("turn is valid");

        assert!(!format!("{id:?}").contains("watcher-1"));
        assert!(!format!("{id}").contains("watcher-1"));
        assert!(!format!("{turn:?}").contains("turn-1"));
        assert!(!format!("{turn}").contains("turn-1"));
    }

    #[test]
    fn watcher_identity_rejects_blank_and_control_input() {
        assert!(WatcherId::new(" ").is_err());
        assert!(WatcherId::new("a\nb").is_err());
        assert!(WatcherOwningTurn::new("").is_err());
    }
}

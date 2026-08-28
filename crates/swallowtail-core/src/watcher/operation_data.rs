use super::InvalidWatcherRecord;
use std::fmt;

/// Maximum UTF-8 byte length for one watcher operation-data value.
pub const MAX_WATCHER_OPERATION_DATA_BYTES: usize = 1024;

/// Bounded opaque data interpreted by host watcher policy.
///
/// Operation data is not an executable reference, command, argument vector,
/// process id, permission grant, or host path. Its value is redacted by
/// default and never becomes a watcher summary.
#[derive(Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct WatcherOperationData(String);

impl WatcherOperationData {
    /// Creates non-empty, control-free, bounded operation data.
    pub fn new(value: impl Into<String>) -> Result<Self, InvalidWatcherRecord> {
        let value = value.into();
        if value.trim().is_empty()
            || value.len() > MAX_WATCHER_OPERATION_DATA_BYTES
            || value.chars().any(char::is_control)
        {
            return Err(InvalidWatcherRecord::new(
                "Watcher operation data must use a non-empty bounded value",
            ));
        }
        Ok(Self(value))
    }

    #[must_use]
    /// Returns the value for host-policy interpretation.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Debug for WatcherOperationData {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_tuple("WatcherOperationData")
            .field(&format_args!("<redacted:{} bytes>", self.0.len()))
            .finish()
    }
}

impl fmt::Display for WatcherOperationData {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("<redacted watcher operation data>")
    }
}

#[cfg(test)]
mod tests {
    use super::{MAX_WATCHER_OPERATION_DATA_BYTES, WatcherOperationData};

    #[test]
    fn operation_data_is_bounded_and_redacted_by_default() {
        let data = WatcherOperationData::new("private operation").expect("data is valid");

        assert_eq!(data.as_str(), "private operation");
        assert!(!format!("{data:?}").contains("private operation"));
        assert!(!format!("{data}").contains("private operation"));
    }

    #[test]
    fn operation_data_rejects_blank_control_and_overlength_values() {
        assert!(WatcherOperationData::new(" ").is_err());
        assert!(WatcherOperationData::new("a\nb").is_err());
        assert!(
            WatcherOperationData::new("d".repeat(MAX_WATCHER_OPERATION_DATA_BYTES + 1)).is_err()
        );
    }

    #[test]
    fn operation_data_uses_utf8_byte_bounds() {
        let exact = "é".repeat(MAX_WATCHER_OPERATION_DATA_BYTES / 2);
        assert_eq!(exact.len(), MAX_WATCHER_OPERATION_DATA_BYTES);
        WatcherOperationData::new(exact.clone()).expect("exact UTF-8 bound is valid");
        assert!(WatcherOperationData::new(format!("{exact}é")).is_err());
    }
}

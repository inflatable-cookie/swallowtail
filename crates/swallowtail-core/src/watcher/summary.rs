use super::InvalidWatcherRecord;
use std::fmt;

/// Maximum UTF-8 byte length for one redacted watcher summary.
pub const MAX_WATCHER_SUMMARY_BYTES: usize = 512;

/// Bounded redacted summary of watcher progress or terminal status.
///
/// Summaries never carry raw stdout, stderr, command text, arguments,
/// environment, paths, secrets, or unbounded logs.
#[derive(Clone, Eq, PartialEq)]
pub struct WatcherSummary(String);

impl WatcherSummary {
    /// Creates a non-empty, control-free bounded summary.
    pub fn new(value: impl Into<String>) -> Result<Self, InvalidWatcherRecord> {
        let value = value.into();
        if value.trim().is_empty()
            || value.len() > MAX_WATCHER_SUMMARY_BYTES
            || value.chars().any(char::is_control)
        {
            return Err(InvalidWatcherRecord::new(
                "Watcher summary must use a non-empty bounded value",
            ));
        }
        Ok(Self(value))
    }

    #[must_use]
    /// Returns the unredacted summary text for host-selected projection.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Debug for WatcherSummary {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_tuple("WatcherSummary")
            .field(&"<redacted>")
            .finish()
    }
}

impl fmt::Display for WatcherSummary {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("<redacted watcher summary>")
    }
}

#[cfg(test)]
mod tests {
    use super::{MAX_WATCHER_SUMMARY_BYTES, WatcherSummary};

    #[test]
    fn watcher_summary_is_redacted_by_default() {
        let summary = WatcherSummary::new("progress 12%").expect("summary is valid");
        assert!(!format!("{summary:?}").contains("progress"));
        assert!(!format!("{summary}").contains("progress"));
        assert_eq!(summary.as_str(), "progress 12%");
    }

    #[test]
    fn watcher_summary_rejects_overlength_input() {
        assert!(WatcherSummary::new("s".repeat(MAX_WATCHER_SUMMARY_BYTES + 1)).is_err());
    }
}

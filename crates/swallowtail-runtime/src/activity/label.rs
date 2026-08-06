use super::InvalidActivityRecord;
use std::fmt;

const MAX_ACTIVITY_LABEL_BYTES: usize = 512;

/// Bounded provider-intended display label for an activity.
#[derive(Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ActivityLabel(String);

impl ActivityLabel {
    /// Creates a non-empty, control-free bounded display label.
    pub fn new(value: impl Into<String>) -> Result<Self, InvalidActivityRecord> {
        let value = value.into();
        if value.trim().is_empty()
            || value.len() > MAX_ACTIVITY_LABEL_BYTES
            || value.chars().any(char::is_control)
        {
            return Err(InvalidActivityRecord::new(
                "Activity label must use a non-empty bounded display value",
            ));
        }
        Ok(Self(value))
    }

    #[must_use]
    /// Returns the provider-intended display value.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Debug for ActivityLabel {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_tuple("ActivityLabel")
            .field(&"<redacted>")
            .finish()
    }
}

impl fmt::Display for ActivityLabel {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("<redacted activity label>")
    }
}

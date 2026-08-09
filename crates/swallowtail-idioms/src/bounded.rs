/// Default maximum bytes for one idiom pattern.
pub const DEFAULT_MAX_PATTERN_BYTES: usize = 512;
/// Default maximum bytes for one signal target or correlation value.
pub const DEFAULT_MAX_CORRELATION_BYTES: usize = 64;
/// Default maximum bytes for one identifier, source, or package name.
pub const DEFAULT_MAX_ID_BYTES: usize = 128;
/// Default maximum bytes for one provenance source reference.
pub const DEFAULT_MAX_SOURCE_BYTES: usize = 256;
/// Default maximum bytes for one signal target reference.
pub const DEFAULT_MAX_TARGET_BYTES: usize = 256;

/// Bounded non-blank text value carried by idiom records.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct BoundedText {
    value: String,
}

impl BoundedText {
    /// Creates bounded text after rejecting blank or overlong values.
    pub fn new(value: impl Into<String>, maximum_bytes: usize) -> Result<Self, BoundedTextError> {
        let value = value.into();
        if value.trim().is_empty() {
            return Err(BoundedTextError::Blank);
        }
        if value.len() > maximum_bytes {
            return Err(BoundedTextError::TooLong(maximum_bytes));
        }
        Ok(Self { value })
    }

    /// Returns the retained text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.value
    }

    /// Returns the retained byte length.
    #[must_use]
    pub fn len(&self) -> usize {
        self.value.len()
    }

    /// Returns whether the retained text is empty (never true by construction).
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.value.is_empty()
    }
}

/// Why bounded text was rejected.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BoundedTextError {
    /// The value was blank after trimming.
    Blank,
    /// The value exceeded the byte bound.
    TooLong(usize),
}

#[cfg(test)]
mod tests {
    use super::{BoundedText, BoundedTextError};

    #[test]
    fn accepts_bounded_text() {
        let value = BoundedText::new("named exports", 64).expect("bounded text");
        assert_eq!(value.as_str(), "named exports");
        assert_eq!(value.len(), 13);
    }

    #[test]
    fn rejects_blank_and_overflow() {
        assert_eq!(
            BoundedText::new("   ", 64).expect_err("blank"),
            BoundedTextError::Blank
        );
        assert_eq!(
            BoundedText::new("x".repeat(65), 64).expect_err("long"),
            BoundedTextError::TooLong(64)
        );
    }
}

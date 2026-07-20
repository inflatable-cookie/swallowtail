use crate::InputValueRequired;
use crate::input::required_text;
use std::fmt;

/// Opaque prompt or normalized output content transported by the runtime.
#[derive(Clone, Eq, PartialEq)]
pub struct OperationContent(String);

impl OperationContent {
    pub fn new(value: impl Into<String>) -> Result<Self, InputValueRequired> {
        required_text("operation content", value).map(Self)
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }

    #[must_use]
    pub fn byte_len(&self) -> usize {
        self.0.len()
    }
}

impl fmt::Debug for OperationContent {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_tuple("OperationContent")
            .field(&format_args!("<redacted:{} bytes>", self.byte_len()))
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::OperationContent;

    #[test]
    fn content_is_available_but_redacted_by_default() {
        let content = OperationContent::new("private prompt").expect("content is valid");

        assert_eq!(content.as_str(), "private prompt");
        assert_eq!(content.byte_len(), 14);
        assert!(!format!("{content:?}").contains("private prompt"));
    }

    #[test]
    fn blank_content_is_rejected() {
        let failure = OperationContent::new("  ").expect_err("blank content must fail");

        assert_eq!(failure.field(), "operation content");
    }
}

use std::error::Error;
use std::fmt;

/// A stable code and operator-safe message.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SafeDiagnostic {
    code: &'static str,
    message: String,
}

impl SafeDiagnostic {
    #[must_use]
    pub fn new(code: &'static str, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
        }
    }

    #[must_use]
    pub const fn code(&self) -> &'static str {
        self.code
    }

    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }
}

impl fmt::Display for SafeDiagnostic {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.message)
    }
}

/// A diagnostic with internal detail kept out of default formatting.
#[derive(Clone, Eq, PartialEq)]
pub struct Diagnostic {
    safe: SafeDiagnostic,
    internal_detail: Option<String>,
}

impl Diagnostic {
    #[must_use]
    pub const fn new(safe: SafeDiagnostic) -> Self {
        Self {
            safe,
            internal_detail: None,
        }
    }

    #[must_use]
    pub fn with_internal_detail(mut self, detail: impl Into<String>) -> Self {
        self.internal_detail = Some(detail.into());
        self
    }

    #[must_use]
    pub const fn safe(&self) -> &SafeDiagnostic {
        &self.safe
    }

    /// Internal detail is opt-in and must be handled under host redaction policy.
    #[must_use]
    pub fn internal_detail(&self) -> Option<&str> {
        self.internal_detail.as_deref()
    }
}

impl fmt::Debug for Diagnostic {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("Diagnostic")
            .field("safe", &self.safe)
            .field(
                "internal_detail",
                &self.internal_detail.as_ref().map(|_| "<redacted>"),
            )
            .finish()
    }
}

impl fmt::Display for Diagnostic {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.safe.fmt(formatter)
    }
}

/// Returned when a contract value that must carry text is empty.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ValueRequired {
    field: &'static str,
    diagnostic: SafeDiagnostic,
}

impl ValueRequired {
    pub(crate) fn for_field(field: &'static str) -> Self {
        Self {
            field,
            diagnostic: SafeDiagnostic::new(
                "swallowtail.value_required",
                format!("{field} must not be empty"),
            ),
        }
    }

    #[must_use]
    pub const fn field(&self) -> &'static str {
        self.field
    }

    #[must_use]
    pub const fn diagnostic(&self) -> &SafeDiagnostic {
        &self.diagnostic
    }
}

impl fmt::Display for ValueRequired {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.diagnostic.fmt(formatter)
    }
}

impl Error for ValueRequired {}

pub(crate) fn required_text(
    field: &'static str,
    value: impl Into<String>,
) -> Result<String, ValueRequired> {
    let value = value.into();
    if value.trim().is_empty() {
        Err(ValueRequired::for_field(field))
    } else {
        Ok(value)
    }
}

#[cfg(test)]
mod tests {
    use super::{Diagnostic, SafeDiagnostic};

    #[test]
    fn default_formatting_excludes_internal_detail() {
        let diagnostic = Diagnostic::new(SafeDiagnostic::new(
            "swallowtail.provider_failed",
            "Provider request failed",
        ))
        .with_internal_detail("token=secret-provider-detail");

        assert_eq!(diagnostic.to_string(), "Provider request failed");
        assert!(!format!("{diagnostic:?}").contains("secret-provider-detail"));
        assert_eq!(
            diagnostic.internal_detail(),
            Some("token=secret-provider-detail")
        );
    }
}

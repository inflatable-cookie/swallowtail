#![deny(missing_docs)]

use std::error::Error;
use std::fmt;

use crate::FailureClassification;

/// A stable code and operator-safe message.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SafeDiagnostic {
    code: &'static str,
    message: String,
    classification: FailureClassification,
}

impl SafeDiagnostic {
    /// Creates an operator-safe diagnostic with unknown portable classification.
    #[must_use]
    pub fn new(code: &'static str, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
            classification: FailureClassification::unknown(),
        }
    }

    /// Adds portable evidence without changing the exact route diagnostic.
    #[must_use]
    pub const fn with_failure_classification(
        mut self,
        classification: FailureClassification,
    ) -> Self {
        self.classification = classification;
        self
    }

    #[must_use]
    /// Returns the stable route-specific diagnostic code.
    pub const fn code(&self) -> &'static str {
        self.code
    }

    #[must_use]
    /// Returns the operator-safe diagnostic message.
    pub fn message(&self) -> &str {
        &self.message
    }

    #[must_use]
    /// Returns the portable failure classification carried alongside the code.
    pub const fn failure_classification(&self) -> FailureClassification {
        self.classification
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
    /// Creates a diagnostic without internal detail.
    #[must_use]
    pub const fn new(safe: SafeDiagnostic) -> Self {
        Self {
            safe,
            internal_detail: None,
        }
    }

    #[must_use]
    /// Adds host-private detail excluded from default formatting.
    pub fn with_internal_detail(mut self, detail: impl Into<String>) -> Self {
        self.internal_detail = Some(detail.into());
        self
    }

    #[must_use]
    /// Returns the operator-safe part of the diagnostic.
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
    /// Returns the contract field whose required text was absent.
    pub const fn field(&self) -> &'static str {
        self.field
    }

    #[must_use]
    /// Returns the redacted missing-value diagnostic.
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
mod diagnostic_detail_tests {
    use super::SafeDiagnostic;
    use crate::{FailureClassification, FailureKind, FailureOrigin, FailureRecovery};

    #[test]
    fn diagnostics_default_to_honest_unknown_classification() {
        let diagnostic = SafeDiagnostic::new("fixture.failure", "Fixture failed");

        assert!(diagnostic.failure_classification().is_unknown());
        assert_eq!(diagnostic.code(), "fixture.failure");
        assert_eq!(diagnostic.message(), "Fixture failed");
    }

    #[test]
    fn classification_does_not_replace_exact_diagnostic_identity() {
        let diagnostic = SafeDiagnostic::new("fixture.rate_limited", "Fixture was rate limited")
            .with_failure_classification(FailureClassification::new(
                FailureOrigin::Provider,
                FailureKind::RateLimited,
                FailureRecovery::RetryMaySucceed,
            ));

        assert_eq!(diagnostic.code(), "fixture.rate_limited");
        assert_eq!(diagnostic.message(), "Fixture was rate limited");
        assert_eq!(
            diagnostic.failure_classification(),
            FailureClassification::new(
                FailureOrigin::Provider,
                FailureKind::RateLimited,
                FailureRecovery::RetryMaySucceed,
            )
        );
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

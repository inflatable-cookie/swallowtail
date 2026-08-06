use std::error::Error;
use std::fmt;
use swallowtail_core::{FailureClassification, SafeDiagnostic};

#[derive(Clone, Debug, Eq, PartialEq)]
/// Safe provider-neutral runtime failure retaining an exact diagnostic.
pub struct RuntimeFailure {
    diagnostic: SafeDiagnostic,
}

impl RuntimeFailure {
    #[must_use]
    /// Creates a runtime failure from a redacted safe diagnostic.
    pub const fn new(diagnostic: SafeDiagnostic) -> Self {
        Self { diagnostic }
    }

    #[must_use]
    /// Returns the exact safe diagnostic and optional portable classification.
    pub const fn diagnostic(&self) -> &SafeDiagnostic {
        &self.diagnostic
    }

    /// Adds portable evidence without replacing the exact safe diagnostic.
    #[must_use]
    pub fn with_failure_classification(self, classification: FailureClassification) -> Self {
        Self::new(self.diagnostic.with_failure_classification(classification))
    }
}

impl fmt::Display for RuntimeFailure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.diagnostic.fmt(formatter)
    }
}

impl Error for RuntimeFailure {}

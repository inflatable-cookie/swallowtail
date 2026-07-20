use std::error::Error;
use std::fmt;
use swallowtail_core::SafeDiagnostic;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RuntimeFailure {
    diagnostic: SafeDiagnostic,
}

impl RuntimeFailure {
    #[must_use]
    pub const fn new(diagnostic: SafeDiagnostic) -> Self {
        Self { diagnostic }
    }

    #[must_use]
    pub const fn diagnostic(&self) -> &SafeDiagnostic {
        &self.diagnostic
    }
}

impl fmt::Display for RuntimeFailure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.diagnostic.fmt(formatter)
    }
}

impl Error for RuntimeFailure {}

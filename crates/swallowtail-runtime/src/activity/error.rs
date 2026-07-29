use std::error::Error;
use std::fmt;
use swallowtail_core::SafeDiagnostic;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InvalidActivityRecord {
    diagnostic: SafeDiagnostic,
}

impl InvalidActivityRecord {
    pub(super) fn new(message: &'static str) -> Self {
        Self {
            diagnostic: SafeDiagnostic::new("swallowtail.activity_record_invalid", message),
        }
    }

    #[must_use]
    pub const fn diagnostic(&self) -> &SafeDiagnostic {
        &self.diagnostic
    }
}

impl fmt::Display for InvalidActivityRecord {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.diagnostic.fmt(formatter)
    }
}

impl Error for InvalidActivityRecord {}

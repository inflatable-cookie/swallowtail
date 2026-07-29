use crate::SafeDiagnostic;
use std::error::Error;
use std::fmt;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InvalidObservableActivityProfile {
    diagnostic: SafeDiagnostic,
}

impl InvalidObservableActivityProfile {
    pub(super) fn new(message: &'static str) -> Self {
        Self {
            diagnostic: SafeDiagnostic::new(
                "swallowtail.observable_activity_profile.invalid",
                message,
            ),
        }
    }

    #[must_use]
    pub const fn diagnostic(&self) -> &SafeDiagnostic {
        &self.diagnostic
    }
}

impl fmt::Display for InvalidObservableActivityProfile {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.diagnostic.fmt(formatter)
    }
}

impl Error for InvalidObservableActivityProfile {}

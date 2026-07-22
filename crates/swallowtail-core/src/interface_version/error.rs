use crate::SafeDiagnostic;
use std::error::Error;
use std::fmt;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InvalidInterfaceCompatibilityClaim {
    diagnostic: SafeDiagnostic,
}

impl InvalidInterfaceCompatibilityClaim {
    pub(super) fn new(message: &'static str) -> Self {
        Self {
            diagnostic: SafeDiagnostic::new(
                "swallowtail.interface_compatibility_claim_rejected",
                message,
            ),
        }
    }

    #[must_use]
    pub const fn diagnostic(&self) -> &SafeDiagnostic {
        &self.diagnostic
    }
}

impl fmt::Display for InvalidInterfaceCompatibilityClaim {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.diagnostic.fmt(formatter)
    }
}

impl Error for InvalidInterfaceCompatibilityClaim {}

impl From<semver::Error> for InvalidInterfaceCompatibilityClaim {
    fn from(_: semver::Error) -> Self {
        Self::new("Semantic version is invalid")
    }
}

impl From<std::num::ParseIntError> for InvalidInterfaceCompatibilityClaim {
    fn from(_: std::num::ParseIntError) -> Self {
        Self::new("Ordered integer version is invalid")
    }
}

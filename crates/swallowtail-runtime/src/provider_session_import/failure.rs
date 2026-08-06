use crate::RuntimeFailure;
use std::error::Error;
use std::fmt;
use swallowtail_core::SafeDiagnostic;

/// Stage at which a catalogue or explicit import operation failed.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ProviderSessionOperationFailureStage {
    /// Validation failed before provider work was dispatched.
    BeforeDispatch,
    /// Catalogue provider work failed after dispatch.
    CatalogueDispatch,
    /// Provider catalogue data could not be projected safely.
    CatalogueProjection,
    /// The selected candidate could not be revalidated exactly.
    ImportRevalidation,
    /// A revalidated candidate could not issue exact resume authority.
    ImportBindingIssue,
    /// The operation was cancelled.
    Cancelled,
    /// The operation reached its deadline.
    TimedOut,
    /// Operation-owned work did not clean up completely.
    Cleanup,
}

/// Safe staged failure for provider-session catalogue and import operations.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProviderSessionOperationFailure {
    stage: ProviderSessionOperationFailureStage,
    diagnostic: SafeDiagnostic,
}

impl ProviderSessionOperationFailure {
    #[must_use]
    /// Creates a failure from a stage and bounded diagnostic.
    pub const fn new(
        stage: ProviderSessionOperationFailureStage,
        diagnostic: SafeDiagnostic,
    ) -> Self {
        Self { stage, diagnostic }
    }

    pub(crate) fn from_runtime(
        stage: ProviderSessionOperationFailureStage,
        failure: RuntimeFailure,
    ) -> Self {
        Self::new(stage, failure.diagnostic().clone())
    }

    #[must_use]
    /// Returns the operation stage at which failure was observed.
    pub const fn stage(&self) -> ProviderSessionOperationFailureStage {
        self.stage
    }

    #[must_use]
    /// Returns the bounded, redacted diagnostic.
    pub const fn diagnostic(&self) -> &SafeDiagnostic {
        &self.diagnostic
    }
}

impl fmt::Display for ProviderSessionOperationFailure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.diagnostic.fmt(formatter)
    }
}

impl Error for ProviderSessionOperationFailure {}

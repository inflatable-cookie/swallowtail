use crate::RuntimeFailure;
use std::error::Error;
use std::fmt;
use swallowtail_core::SafeDiagnostic;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ProviderSessionOperationFailureStage {
    BeforeDispatch,
    CatalogueDispatch,
    CatalogueProjection,
    ImportRevalidation,
    ImportBindingIssue,
    Cancelled,
    TimedOut,
    Cleanup,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProviderSessionOperationFailure {
    stage: ProviderSessionOperationFailureStage,
    diagnostic: SafeDiagnostic,
}

impl ProviderSessionOperationFailure {
    #[must_use]
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
    pub const fn stage(&self) -> ProviderSessionOperationFailureStage {
        self.stage
    }

    #[must_use]
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

#![deny(missing_docs)]

use std::error::Error;
use std::fmt;
use swallowtail_core::Diagnostic;

/// Machine-distinct stage at which provider integration preparation failed.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum PreparationStage {
    /// Host-approved executable, endpoint, SDK, or service target selection.
    TargetSelection,
    /// Preparation-owned process creation.
    ProcessSpawn,
    /// Bounded preparation output collection.
    BoundedOutput,
    /// Preparation-owned process exit observation.
    ProcessExit,
    /// Installed or service interface version parsing.
    VersionParse,
    /// Exact interface compatibility classification.
    CompatibilityClassification,
    /// Access status and provenance admission.
    AccessEvidence,
    /// Immutable operation preflight construction or validation.
    Preflight,
    /// Joining or releasing preparation-owned work.
    Cleanup,
}

/// Safe staged failure chain returned by prepared integration construction.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PreparationFailure {
    stage: PreparationStage,
    diagnostic: Diagnostic,
    cause: Option<Box<Self>>,
}

impl PreparationFailure {
    #[must_use]
    /// Creates a preparation failure at one exact stage.
    pub const fn new(stage: PreparationStage, diagnostic: Diagnostic) -> Self {
        Self {
            stage,
            diagnostic,
            cause: None,
        }
    }

    #[must_use]
    /// Attaches an earlier safe preparation failure as the cause.
    pub fn with_cause(mut self, cause: Self) -> Self {
        self.cause = Some(Box::new(cause));
        self
    }

    #[must_use]
    /// Returns the stage at which this failure was observed.
    pub const fn stage(&self) -> PreparationStage {
        self.stage
    }

    #[must_use]
    /// Returns the safe diagnostic, retaining restricted detail only by policy.
    pub const fn diagnostic(&self) -> &Diagnostic {
        &self.diagnostic
    }

    #[must_use]
    /// Returns the earlier safe preparation failure, when present.
    pub fn cause(&self) -> Option<&Self> {
        self.cause.as_deref()
    }
}

impl fmt::Display for PreparationFailure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.diagnostic.fmt(formatter)
    }
}

impl Error for PreparationFailure {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.cause
            .as_deref()
            .map(|cause| cause as &(dyn Error + 'static))
    }
}

#[cfg(test)]
mod tests {
    use super::{PreparationFailure, PreparationStage};
    use swallowtail_core::{Diagnostic, SafeDiagnostic};

    #[test]
    fn formatting_keeps_internal_detail_out_of_the_failure_chain() {
        let cause = PreparationFailure::new(
            PreparationStage::ProcessSpawn,
            Diagnostic::new(SafeDiagnostic::new(
                "swallowtail.preparation.spawn_failed",
                "Target process did not start",
            ))
            .with_internal_detail("path=/secret/bin token=private"),
        );
        let failure = PreparationFailure::new(
            PreparationStage::Cleanup,
            Diagnostic::new(SafeDiagnostic::new(
                "swallowtail.preparation.cleanup_failed",
                "Preparation cleanup failed",
            )),
        )
        .with_cause(cause);

        assert_eq!(failure.stage(), PreparationStage::Cleanup);
        assert_eq!(
            failure.cause().map(PreparationFailure::stage),
            Some(PreparationStage::ProcessSpawn)
        );
        assert_eq!(failure.to_string(), "Preparation cleanup failed");
        assert!(!format!("{failure:?}").contains("/secret/bin"));
        assert!(!format!("{failure:?}").contains("private"));
    }
}

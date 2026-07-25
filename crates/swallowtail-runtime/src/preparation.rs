use std::error::Error;
use std::fmt;
use swallowtail_core::Diagnostic;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum PreparationStage {
    TargetSelection,
    ProcessSpawn,
    BoundedOutput,
    ProcessExit,
    VersionParse,
    CompatibilityClassification,
    AccessEvidence,
    Preflight,
    Cleanup,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PreparationFailure {
    stage: PreparationStage,
    diagnostic: Diagnostic,
    cause: Option<Box<Self>>,
}

impl PreparationFailure {
    #[must_use]
    pub const fn new(stage: PreparationStage, diagnostic: Diagnostic) -> Self {
        Self {
            stage,
            diagnostic,
            cause: None,
        }
    }

    #[must_use]
    pub fn with_cause(mut self, cause: Self) -> Self {
        self.cause = Some(Box::new(cause));
        self
    }

    #[must_use]
    pub const fn stage(&self) -> PreparationStage {
        self.stage
    }

    #[must_use]
    pub const fn diagnostic(&self) -> &Diagnostic {
        &self.diagnostic
    }

    #[must_use]
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

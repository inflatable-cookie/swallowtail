use super::LlamaCppOwnedPreparedIntegration;
use swallowtail_core::{ModelArtifactBinding, PreflightPlan};
use swallowtail_runtime::{PreparationFailure, PreparedOperationEvidence};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LlamaCppOwnedPreparedEvidence {
    operation: PreparedOperationEvidence,
    artifact: ModelArtifactBinding,
}

impl LlamaCppOwnedPreparedEvidence {
    pub(super) fn new(
        prepared: &LlamaCppOwnedPreparedIntegration,
        plan: PreflightPlan,
    ) -> Result<Self, PreparationFailure> {
        Ok(Self {
            operation: PreparedOperationEvidence::from_plan(plan, prepared.evidence.clone())?,
            artifact: prepared.artifact.clone(),
        })
    }

    #[must_use]
    pub const fn operation(&self) -> &PreparedOperationEvidence {
        &self.operation
    }

    #[must_use]
    pub const fn plan(&self) -> &PreflightPlan {
        self.operation.plan()
    }

    #[must_use]
    pub const fn artifact(&self) -> &ModelArtifactBinding {
        &self.artifact
    }

    #[must_use]
    pub const fn expected_build(&self) -> &'static str {
        crate::LLAMA_CPP_OWNED_BUILD
    }

    #[must_use]
    pub const fn expected_commit(&self) -> &'static str {
        crate::LLAMA_CPP_OWNED_COMMIT
    }
}

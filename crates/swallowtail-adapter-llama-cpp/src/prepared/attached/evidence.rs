use super::LlamaCppAttachedPreparedIntegration;
use swallowtail_core::PreflightPlan;
use swallowtail_runtime::{PreparationFailure, PreparedOperationEvidence};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LlamaCppAttachedPreparedEvidence {
    operation: PreparedOperationEvidence,
}

impl LlamaCppAttachedPreparedEvidence {
    pub(super) fn new(
        prepared: &LlamaCppAttachedPreparedIntegration,
        plan: PreflightPlan,
    ) -> Result<Self, PreparationFailure> {
        Ok(Self {
            operation: PreparedOperationEvidence::from_plan(plan, prepared.evidence.clone())?,
        })
    }

    pub(super) fn new_with_activity(
        prepared: &LlamaCppAttachedPreparedIntegration,
        plan: PreflightPlan,
        activity_profile: swallowtail_core::ObservableActivityProfile,
    ) -> Result<Self, PreparationFailure> {
        Ok(Self {
            operation: PreparedOperationEvidence::from_plan_with_activity_profile(
                plan,
                prepared.evidence.clone(),
                activity_profile,
            )?,
        })
    }

    #[must_use]
    pub const fn operation(&self) -> &PreparedOperationEvidence {
        &self.operation
    }

    #[must_use]
    pub const fn observable_activity(&self) -> &swallowtail_core::ObservableActivityProfile {
        self.operation.observable_activity()
    }

    #[must_use]
    pub const fn plan(&self) -> &PreflightPlan {
        self.operation.plan()
    }

    #[must_use]
    pub const fn expected_build(&self) -> &'static str {
        crate::LLAMA_CPP_ATTACHED_BUILD
    }

    #[must_use]
    pub const fn expected_commit(&self) -> &'static str {
        crate::LLAMA_CPP_ATTACHED_COMMIT
    }
}

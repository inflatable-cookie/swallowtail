use super::{ProviderSessionCataloguePlan, ProviderSessionImportPlan};
use crate::{PreparationFailure, PreparedAccessEvidence, PreparedOperationEvidence};

/// Prepared route and plan evidence for provider-session catalogue work.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PreparedProviderSessionCatalogueEvidence {
    operation: PreparedOperationEvidence,
    plan: ProviderSessionCataloguePlan,
}

impl PreparedProviderSessionCatalogueEvidence {
    /// Builds prepared evidence from a validated plan and access evidence.
    pub fn from_plan(
        plan: ProviderSessionCataloguePlan,
        access: PreparedAccessEvidence,
    ) -> Result<Self, PreparationFailure> {
        let operation = PreparedOperationEvidence::from_plan(plan.preflight().clone(), access)?;
        Ok(Self { operation, plan })
    }

    #[must_use]
    /// Returns the shared prepared-operation evidence.
    pub const fn operation(&self) -> &PreparedOperationEvidence {
        &self.operation
    }

    #[must_use]
    /// Returns the validated catalogue plan.
    pub const fn plan(&self) -> &ProviderSessionCataloguePlan {
        &self.plan
    }
}

/// Prepared route and plan evidence for explicit provider-session import.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PreparedProviderSessionImportEvidence {
    operation: PreparedOperationEvidence,
    plan: ProviderSessionImportPlan,
}

impl PreparedProviderSessionImportEvidence {
    /// Builds prepared evidence from a validated plan and access evidence.
    pub fn from_plan(
        plan: ProviderSessionImportPlan,
        access: PreparedAccessEvidence,
    ) -> Result<Self, PreparationFailure> {
        let operation = PreparedOperationEvidence::from_plan(plan.preflight().clone(), access)?;
        Ok(Self { operation, plan })
    }

    #[must_use]
    /// Returns the shared prepared-operation evidence.
    pub const fn operation(&self) -> &PreparedOperationEvidence {
        &self.operation
    }

    #[must_use]
    /// Returns the validated explicit import plan.
    pub const fn plan(&self) -> &ProviderSessionImportPlan {
        &self.plan
    }
}

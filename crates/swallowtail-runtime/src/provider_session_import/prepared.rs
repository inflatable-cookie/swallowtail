use super::{ProviderSessionCataloguePlan, ProviderSessionImportPlan};
use crate::{PreparationFailure, PreparedAccessEvidence, PreparedOperationEvidence};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PreparedProviderSessionCatalogueEvidence {
    operation: PreparedOperationEvidence,
    plan: ProviderSessionCataloguePlan,
}

impl PreparedProviderSessionCatalogueEvidence {
    pub fn from_plan(
        plan: ProviderSessionCataloguePlan,
        access: PreparedAccessEvidence,
    ) -> Result<Self, PreparationFailure> {
        let operation = PreparedOperationEvidence::from_plan(plan.preflight().clone(), access)?;
        Ok(Self { operation, plan })
    }

    #[must_use]
    pub const fn operation(&self) -> &PreparedOperationEvidence {
        &self.operation
    }

    #[must_use]
    pub const fn plan(&self) -> &ProviderSessionCataloguePlan {
        &self.plan
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PreparedProviderSessionImportEvidence {
    operation: PreparedOperationEvidence,
    plan: ProviderSessionImportPlan,
}

impl PreparedProviderSessionImportEvidence {
    pub fn from_plan(
        plan: ProviderSessionImportPlan,
        access: PreparedAccessEvidence,
    ) -> Result<Self, PreparationFailure> {
        let operation = PreparedOperationEvidence::from_plan(plan.preflight().clone(), access)?;
        Ok(Self { operation, plan })
    }

    #[must_use]
    pub const fn operation(&self) -> &PreparedOperationEvidence {
        &self.operation
    }

    #[must_use]
    pub const fn plan(&self) -> &ProviderSessionImportPlan {
        &self.plan
    }
}

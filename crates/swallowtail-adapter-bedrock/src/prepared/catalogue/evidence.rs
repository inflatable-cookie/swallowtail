use super::BedrockCataloguePreparedIntegration;
use crate::BedrockRegion;
use swallowtail_core::PreflightPlan;
use swallowtail_runtime::{PreparationFailure, PreparedOperationEvidence};

#[derive(Clone, Debug, Eq, PartialEq)]
/// Prepared catalogue evidence including region, SDK, and service truth.
pub struct BedrockCataloguePreparedEvidence {
    operation: PreparedOperationEvidence,
    region: BedrockRegion,
}

impl BedrockCataloguePreparedEvidence {
    pub(super) fn new(
        prepared: &BedrockCataloguePreparedIntegration,
        plan: PreflightPlan,
    ) -> Result<Self, PreparationFailure> {
        Ok(Self {
            operation: PreparedOperationEvidence::from_plan(plan, prepared.evidence.clone())?,
            region: prepared.region.clone(),
        })
    }

    #[must_use]
    /// Returns the provider-neutral prepared operation evidence.
    pub const fn operation(&self) -> &PreparedOperationEvidence {
        &self.operation
    }

    #[must_use]
    /// Returns the immutable catalogue preflight plan.
    pub const fn plan(&self) -> &PreflightPlan {
        self.operation.plan()
    }

    #[must_use]
    /// Returns the exact AWS region.
    pub const fn region(&self) -> &BedrockRegion {
        &self.region
    }

    #[must_use]
    /// Returns the qualified control-plane SDK crate name.
    pub const fn sdk_crate(&self) -> &'static str {
        crate::CATALOGUE_SDK_CRATE
    }

    #[must_use]
    /// Returns the qualified control-plane SDK version.
    pub const fn sdk_version(&self) -> &'static str {
        crate::CATALOGUE_SDK_VERSION
    }

    #[must_use]
    /// Returns the qualified catalogue service API.
    pub const fn service_api(&self) -> &'static str {
        crate::CATALOGUE_SERVICE_API
    }
}

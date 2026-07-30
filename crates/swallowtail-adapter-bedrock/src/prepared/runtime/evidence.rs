use super::BedrockRuntimePreparedIntegration;
use crate::BedrockRegion;
use swallowtail_core::PreflightPlan;
use swallowtail_runtime::{PreparationFailure, PreparedOperationEvidence};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BedrockRuntimePreparedEvidence {
    operation: PreparedOperationEvidence,
    region: BedrockRegion,
}

impl BedrockRuntimePreparedEvidence {
    pub(super) fn new_with_activity(
        prepared: &BedrockRuntimePreparedIntegration,
        plan: PreflightPlan,
        activity_profile: swallowtail_core::ObservableActivityProfile,
    ) -> Result<Self, PreparationFailure> {
        Ok(Self {
            operation: PreparedOperationEvidence::from_plan_with_activity_profile(
                plan,
                prepared.evidence.clone(),
                activity_profile,
            )?,
            region: prepared.region.clone(),
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
    pub const fn region(&self) -> &BedrockRegion {
        &self.region
    }

    #[must_use]
    pub const fn sdk_crate(&self) -> &'static str {
        crate::SDK_CRATE
    }

    #[must_use]
    pub const fn sdk_version(&self) -> &'static str {
        crate::SDK_VERSION
    }

    #[must_use]
    pub const fn service_api(&self) -> &'static str {
        crate::SERVICE_API
    }
}

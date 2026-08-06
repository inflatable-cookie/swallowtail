use super::BedrockRuntimePreparedIntegration;
use crate::BedrockRegion;
use swallowtail_core::PreflightPlan;
use swallowtail_runtime::{PreparationFailure, PreparedOperationEvidence};

#[derive(Clone, Debug, Eq, PartialEq)]
/// Prepared Runtime evidence including region, SDK, service, and activity truth.
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
    /// Returns the provider-neutral prepared operation evidence.
    pub const fn operation(&self) -> &PreparedOperationEvidence {
        &self.operation
    }

    #[must_use]
    /// Returns the observable activity profile for Runtime inference.
    pub const fn observable_activity(&self) -> &swallowtail_core::ObservableActivityProfile {
        self.operation.observable_activity()
    }

    #[must_use]
    /// Returns the immutable Runtime preflight plan.
    pub const fn plan(&self) -> &PreflightPlan {
        self.operation.plan()
    }

    #[must_use]
    /// Returns the exact AWS region.
    pub const fn region(&self) -> &BedrockRegion {
        &self.region
    }

    #[must_use]
    /// Returns the qualified Runtime SDK crate name.
    pub const fn sdk_crate(&self) -> &'static str {
        crate::SDK_CRATE
    }

    #[must_use]
    /// Returns the qualified Runtime SDK version.
    pub const fn sdk_version(&self) -> &'static str {
        crate::SDK_VERSION
    }

    #[must_use]
    /// Returns the qualified Runtime service API.
    pub const fn service_api(&self) -> &'static str {
        crate::SERVICE_API
    }
}

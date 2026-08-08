use crate::AlibabaModelStudioPreparedIntegration;
use swallowtail_core::{
    ConfiguredInstance, ModelId, ModelRoute, ModelRouteId, ModelRouteRevision,
    OperationRequirements, PreflightPlan,
};
use swallowtail_runtime::{PreparationFailure, PreparedOperationEvidence};

#[derive(Clone, Debug, Eq, PartialEq)]
/// Prepared operation, access, activity, and preflight evidence.
pub struct AlibabaModelStudioPreparedEvidence {
    operation: PreparedOperationEvidence,
}

impl AlibabaModelStudioPreparedEvidence {
    pub(super) fn from_prepared(
        prepared: &AlibabaModelStudioPreparedIntegration,
        plan: PreflightPlan,
        activity_profile: swallowtail_core::ObservableActivityProfile,
    ) -> Result<Self, PreparationFailure> {
        Ok(Self {
            operation: PreparedOperationEvidence::from_plan_with_activity_profile(
                plan,
                prepared.access_evidence().clone(),
                activity_profile,
            )?,
        })
    }

    #[must_use]
    /// Returns access evidence and its provenance.
    pub const fn access(&self) -> &swallowtail_runtime::PreparedAccessEvidence {
        self.operation.access()
    }

    #[must_use]
    /// Returns the provider-neutral prepared operation evidence.
    pub const fn operation(&self) -> &PreparedOperationEvidence {
        &self.operation
    }

    #[must_use]
    /// Returns the activity profile promised by the prepared operation.
    pub const fn observable_activity(&self) -> &swallowtail_core::ObservableActivityProfile {
        self.operation.observable_activity()
    }

    #[must_use]
    /// Returns the immutable preflight plan.
    pub const fn plan(&self) -> &PreflightPlan {
        self.operation.plan()
    }
}

pub(super) fn model_route(
    prepared: &AlibabaModelStudioPreparedIntegration,
    route_id: ModelRouteId,
    route_revision: ModelRouteRevision,
    model_id: ModelId,
    capabilities: swallowtail_core::CapabilityProfile,
) -> ModelRoute {
    ModelRoute::new(
        route_id,
        route_revision,
        prepared.instance().id().clone(),
        model_id,
        capabilities,
    )
}

pub(super) fn instance_with_capabilities(
    prepared: &AlibabaModelStudioPreparedIntegration,
    capabilities: swallowtail_core::CapabilityProfile,
) -> ConfiguredInstance {
    swallowtail_runtime::instance_with_capabilities(prepared.instance(), capabilities)
}

pub(super) fn build_plan(
    prepared: &AlibabaModelStudioPreparedIntegration,
    instance: &ConfiguredInstance,
    route: &ModelRoute,
    requirements: &OperationRequirements,
) -> Result<PreflightPlan, PreparationFailure> {
    swallowtail_runtime::build_plan(
        &crate::alibaba_model_studio_descriptor(),
        instance,
        Some(route),
        requirements,
        prepared.access_profile(),
        prepared.access_evidence().status(),
        prepared.available_host_services(),
    )
}

pub(super) fn build_plan_without_route(
    prepared: &AlibabaModelStudioPreparedIntegration,
    instance: &ConfiguredInstance,
    requirements: &OperationRequirements,
) -> Result<PreflightPlan, PreparationFailure> {
    swallowtail_runtime::build_plan(
        &crate::alibaba_model_studio_descriptor(),
        instance,
        None,
        requirements,
        prepared.access_profile(),
        prepared.access_evidence().status(),
        prepared.available_host_services(),
    )
}

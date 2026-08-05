use crate::AlibabaModelStudioPreparedIntegration;
use swallowtail_core::{
    ConfiguredInstance, Diagnostic, ModelId, ModelRoute, ModelRouteId, ModelRouteRevision,
    OperationRequirements, PreflightContext, PreflightPlan, preflight,
};
use swallowtail_runtime::{PreparationFailure, PreparationStage, PreparedOperationEvidence};

#[derive(Clone, Debug, Eq, PartialEq)]
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
    pub const fn access(&self) -> &swallowtail_runtime::PreparedAccessEvidence {
        self.operation.access()
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
    let base = prepared.instance();
    ConfiguredInstance::new(
        base.id().clone(),
        base.revision().clone(),
        base.driver_id().clone(),
        base.execution_host_id().clone(),
        base.target_reference().clone(),
        base.ownership(),
        base.access_profile_id().clone(),
        base.support_authority(),
        base.protocol_facade_id().clone(),
        base.policy_id().clone(),
        capabilities,
    )
    .with_interface_versions(base.interface_versions().cloned())
}

pub(super) fn build_plan(
    prepared: &AlibabaModelStudioPreparedIntegration,
    instance: &ConfiguredInstance,
    route: &ModelRoute,
    requirements: &OperationRequirements,
) -> Result<PreflightPlan, PreparationFailure> {
    let descriptor = crate::alibaba_model_studio_descriptor();
    preflight(
        &PreflightContext::new(
            &descriptor,
            instance,
            prepared.access_profile(),
            prepared.access_evidence().status(),
            prepared.available_host_services(),
        )
        .with_model_route(route),
        requirements,
    )
    .map_err(|error| {
        PreparationFailure::new(
            PreparationStage::Preflight,
            Diagnostic::new(error.diagnostic().clone()),
        )
    })
}

pub(super) fn build_plan_without_route(
    prepared: &AlibabaModelStudioPreparedIntegration,
    instance: &ConfiguredInstance,
    requirements: &OperationRequirements,
) -> Result<PreflightPlan, PreparationFailure> {
    let descriptor = crate::alibaba_model_studio_descriptor();
    preflight(
        &PreflightContext::new(
            &descriptor,
            instance,
            prepared.access_profile(),
            prepared.access_evidence().status(),
            prepared.available_host_services(),
        ),
        requirements,
    )
    .map_err(|error| {
        PreparationFailure::new(
            PreparationStage::Preflight,
            Diagnostic::new(error.diagnostic().clone()),
        )
    })
}

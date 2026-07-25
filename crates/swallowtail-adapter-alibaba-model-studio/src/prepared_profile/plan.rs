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
    ) -> Result<Self, PreparationFailure> {
        Ok(Self {
            operation: PreparedOperationEvidence::from_plan(
                plan,
                prepared.access_evidence().clone(),
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
    pub const fn plan(&self) -> &PreflightPlan {
        self.operation.plan()
    }
}

pub(super) fn model_route(
    prepared: &AlibabaModelStudioPreparedIntegration,
    route_id: ModelRouteId,
    route_revision: ModelRouteRevision,
    model_id: ModelId,
) -> ModelRoute {
    ModelRoute::new(
        route_id,
        route_revision,
        prepared.instance().id().clone(),
        model_id,
        prepared.instance().capabilities().clone(),
    )
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

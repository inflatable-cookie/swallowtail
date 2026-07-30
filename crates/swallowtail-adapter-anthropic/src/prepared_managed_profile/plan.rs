use crate::AnthropicManagedPreparedIntegration;
use swallowtail_core::{
    Diagnostic, ModelRoute, PreflightContext, PreflightPlan, ProviderId, preflight,
};
use swallowtail_runtime::{PreparationFailure, PreparationStage, PreparedOperationEvidence};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AnthropicManagedPreparedEvidence {
    operation: PreparedOperationEvidence,
}

impl AnthropicManagedPreparedEvidence {
    pub(super) fn from_prepared(
        prepared: &AnthropicManagedPreparedIntegration,
        plan: PreflightPlan,
    ) -> Result<Self, PreparationFailure> {
        Ok(Self {
            operation: PreparedOperationEvidence::from_plan_with_activity_profile(
                plan,
                prepared.access_evidence().clone(),
                crate::managed_activity::profile::activity_profile(),
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
    prepared: &AnthropicManagedPreparedIntegration,
    selection: super::AnthropicManagedModelSelection,
) -> ModelRoute {
    let (route_id, route_revision, model_id) = selection.into_parts();
    ModelRoute::new(
        route_id,
        route_revision,
        prepared.instance().id().clone(),
        model_id,
        prepared.instance().capabilities().clone(),
    )
    .with_provider_id(
        ProviderId::new("anthropic").expect("static Anthropic provider identity is valid"),
    )
}

pub(super) fn build_plan(
    prepared: &AnthropicManagedPreparedIntegration,
    route: &ModelRoute,
) -> Result<PreflightPlan, PreparationFailure> {
    let descriptor = crate::anthropic_managed_agent_descriptor();
    let requirements =
        crate::anthropic_managed_requirements(prepared.instance().execution_host_id().clone());
    preflight(
        &PreflightContext::new(
            &descriptor,
            prepared.instance(),
            prepared.access_profile(),
            prepared.access_evidence().status(),
            prepared.available_host_services(),
        )
        .with_model_route(route),
        &requirements,
    )
    .map_err(|error| {
        PreparationFailure::new(
            PreparationStage::Preflight,
            Diagnostic::new(error.diagnostic().clone()),
        )
    })
}

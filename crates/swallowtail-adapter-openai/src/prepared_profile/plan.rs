use crate::OpenAiBackgroundPreparedIntegration;
use swallowtail_core::{
    Diagnostic, ModelRoute, PreflightContext, PreflightPlan, ProviderId, preflight,
};
use swallowtail_runtime::{PreparationFailure, PreparationStage, PreparedOperationEvidence};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OpenAiBackgroundPreparedEvidence {
    operation: PreparedOperationEvidence,
}

impl OpenAiBackgroundPreparedEvidence {
    pub(super) fn from_prepared(
        prepared: &OpenAiBackgroundPreparedIntegration,
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
    prepared: &OpenAiBackgroundPreparedIntegration,
    selection: super::OpenAiBackgroundModelSelection,
) -> ModelRoute {
    let (route_id, route_revision, model_id) = selection.into_parts();
    ModelRoute::new(
        route_id,
        route_revision,
        prepared.instance().id().clone(),
        model_id,
        prepared.instance().capabilities().clone(),
    )
    .with_provider_id(ProviderId::new("openai").expect("static OpenAI provider identity is valid"))
}

pub(super) fn build_plan(
    prepared: &OpenAiBackgroundPreparedIntegration,
    route: &ModelRoute,
) -> Result<PreflightPlan, PreparationFailure> {
    let descriptor = crate::openai_background_descriptor();
    let requirements =
        crate::openai_background_requirements(prepared.instance().execution_host_id().clone());
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

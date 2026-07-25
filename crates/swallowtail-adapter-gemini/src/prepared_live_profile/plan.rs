use crate::GeminiLivePreparedIntegration;
use swallowtail_core::{Diagnostic, ModelRoute, PreflightContext, PreflightPlan, preflight};
use swallowtail_runtime::{PreparationFailure, PreparationStage, PreparedOperationEvidence};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GeminiLivePreparedEvidence {
    operation: PreparedOperationEvidence,
}

impl GeminiLivePreparedEvidence {
    pub(super) fn from_prepared(
        prepared: &GeminiLivePreparedIntegration,
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

pub(super) fn model_route(prepared: &GeminiLivePreparedIntegration) -> ModelRoute {
    crate::gemini_live_model_route(
        prepared.instance().id().clone(),
        swallowtail_core::ModelRouteRevision::new("prepared-1")
            .expect("static Gemini Live route revision is valid"),
    )
}

pub(super) fn build_plan(
    prepared: &GeminiLivePreparedIntegration,
    route: &ModelRoute,
) -> Result<PreflightPlan, PreparationFailure> {
    preflight(
        &PreflightContext::new(
            &crate::gemini_live_descriptor(),
            prepared.instance(),
            prepared.access_profile(),
            prepared.access_evidence().status(),
            prepared.available_host_services(),
        )
        .with_model_route(route),
        &crate::gemini_live_requirements(prepared.instance().execution_host_id().clone()),
    )
    .map_err(|error| {
        PreparationFailure::new(
            PreparationStage::Preflight,
            Diagnostic::new(error.diagnostic().clone()),
        )
    })
}

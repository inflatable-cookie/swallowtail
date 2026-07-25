use crate::XaiPreparedIntegration;
use swallowtail_core::{Diagnostic, ModelRoute, PreflightContext, PreflightPlan, preflight};
use swallowtail_runtime::{PreparationFailure, PreparationStage, PreparedOperationEvidence};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct XaiPreparedEvidence {
    operation: PreparedOperationEvidence,
}

impl XaiPreparedEvidence {
    pub(super) fn from_prepared(
        prepared: &XaiPreparedIntegration,
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
    prepared: &XaiPreparedIntegration,
    selection: super::XaiModelSelection,
) -> ModelRoute {
    let (route_id, revision, model_id) = selection.into_parts();
    crate::xai_responses_model_route(
        prepared.instance().id().clone(),
        route_id,
        revision,
        model_id,
    )
}

pub(super) fn build_plan(
    prepared: &XaiPreparedIntegration,
    route: &ModelRoute,
) -> Result<PreflightPlan, PreparationFailure> {
    preflight(
        &PreflightContext::new(
            &crate::xai_websocket_descriptor(),
            prepared.instance(),
            prepared.access_profile(),
            prepared.access_evidence().status(),
            prepared.available_host_services(),
        )
        .with_model_route(route),
        &crate::xai_responses_requirements(prepared.instance().execution_host_id().clone()),
    )
    .map_err(|error| {
        PreparationFailure::new(
            PreparationStage::Preflight,
            Diagnostic::new(error.diagnostic().clone()),
        )
    })
}

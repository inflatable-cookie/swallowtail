use crate::OllamaContextWindow;
use crate::OllamaPreparedEvidence;
use swallowtail_core::{
    ConfiguredInstanceId, InstanceRevision, ModelRouteId, ModelRouteRevision, OperationShape,
    PreflightPlan,
};
use swallowtail_runtime::RuntimeFailure;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct OllamaPreparedDispatchBinding {
    instance_id: ConfiguredInstanceId,
    instance_revision: InstanceRevision,
    model_route_id: ModelRouteId,
    model_route_revision: ModelRouteRevision,
    operation_shape: OperationShape,
    context_window: Option<OllamaContextWindow>,
}

impl OllamaPreparedDispatchBinding {
    pub(crate) fn from_evidence(evidence: &OllamaPreparedEvidence) -> Self {
        let plan = evidence.plan();
        Self {
            instance_id: plan.instance_id().clone(),
            instance_revision: plan.instance_revision().clone(),
            model_route_id: plan
                .model_route_id()
                .expect("prepared Ollama evidence binds a model route")
                .clone(),
            model_route_revision: plan
                .model_route_revision()
                .expect("prepared Ollama evidence binds a model route")
                .clone(),
            operation_shape: plan.requirements().operation_shape(),
            context_window: evidence.context_window(),
        }
    }

    pub(crate) fn context_window(&self) -> Option<OllamaContextWindow> {
        self.context_window
    }

    pub(crate) fn validate_against(&self, plan: &PreflightPlan) -> Result<(), RuntimeFailure> {
        if self.instance_id != *plan.instance_id()
            || self.instance_revision != *plan.instance_revision()
            || self.model_route_id
                != *plan
                    .model_route_id()
                    .expect("prepared Ollama dispatch requires a model route")
            || self.model_route_revision
                != *plan
                    .model_route_revision()
                    .expect("prepared Ollama dispatch requires a model route")
            || self.operation_shape != plan.requirements().operation_shape()
        {
            return Err(crate::failure::failure(
                "swallowtail.ollama.context_window_binding_mismatch",
                "Ollama prepared dispatch binding did not match preflight",
            ));
        }
        Ok(())
    }
}

pub(crate) fn validate_driver_matches_evidence(
    driver: &crate::OllamaNativeAttachedDriver,
    evidence: &OllamaPreparedEvidence,
) -> Result<(), RuntimeFailure> {
    driver.validate_against_prepared_evidence(evidence)
}

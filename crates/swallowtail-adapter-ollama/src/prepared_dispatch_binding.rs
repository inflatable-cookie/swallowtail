use crate::OllamaContextWindow;
use crate::OllamaPreparedEvidence;
use crate::OllamaPreparedInferenceAttempt;
use crate::OllamaPreparedSession;
use swallowtail_core::{
    ConfiguredInstanceId, InstanceRevision, ModelRouteId, ModelRouteRevision, OperationShape,
    PreflightPlan,
};
use swallowtail_runtime::{RequestId, RuntimeFailure};

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct OllamaPreparedDispatchBinding {
    instance_id: ConfiguredInstanceId,
    instance_revision: InstanceRevision,
    model_route_id: ModelRouteId,
    model_route_revision: ModelRouteRevision,
    operation_shape: OperationShape,
    request_id: Option<RequestId>,
    context_window: Option<OllamaContextWindow>,
}

impl OllamaPreparedDispatchBinding {
    fn from_plan_and_operation(
        evidence: &OllamaPreparedEvidence,
        request_id: Option<RequestId>,
    ) -> Self {
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
            request_id,
            context_window: evidence.context_window(),
        }
    }

    pub(crate) fn from_structured_run(attempt: &OllamaPreparedInferenceAttempt) -> Self {
        Self::from_plan_and_operation(
            attempt.evidence(),
            Some(attempt.request().request_id().clone()),
        )
    }

    pub(crate) fn from_open_session(session: &OllamaPreparedSession) -> Self {
        Self::from_plan_and_operation(
            session.evidence(),
            Some(session.request().request_id().clone()),
        )
    }

    pub(crate) fn from_evidence(evidence: &OllamaPreparedEvidence) -> Self {
        Self::from_plan_and_operation(evidence, None)
    }

    pub(crate) fn context_window(&self) -> Option<OllamaContextWindow> {
        self.context_window
    }

    pub(crate) fn validate_prepared_dispatch(
        &self,
        plan: &PreflightPlan,
        request_id: &RequestId,
    ) -> Result<(), RuntimeFailure> {
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
            return Err(binding_mismatch(
                "Ollama prepared dispatch binding did not match preflight",
            ));
        }
        if self.context_window.is_some()
            && self
                .request_id
                .as_ref()
                .is_none_or(|bound| bound != request_id)
        {
            return Err(binding_mismatch(
                "Ollama prepared dispatch binding did not match the bound request",
            ));
        }
        Ok(())
    }
}

fn binding_mismatch(message: impl Into<String>) -> RuntimeFailure {
    crate::failure::failure(
        "swallowtail.ollama.context_window_binding_mismatch",
        message,
    )
}

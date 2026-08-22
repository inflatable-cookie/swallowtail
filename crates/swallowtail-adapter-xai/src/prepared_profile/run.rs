use super::input::XaiRunProfileInput;
use super::plan::{XaiPreparedEvidence, build_plan, instance_with_capabilities, model_route};
use crate::controls::GenerationControls;
use crate::{XaiPreparedIntegration, XaiWebSocketDriver};
use swallowtail_core::PreflightPlan;
use swallowtail_core::{CapabilityProfile, CapabilityRequirement};
use swallowtail_runtime::{
    BoxFuture, HostServices, OperationPolicy, PreparationFailure, RunHandle, RuntimeFailure,
    StructuredRunDriver, StructuredRunRequest,
};

#[derive(Clone, Debug, Eq, PartialEq)]
/// Bound prepared xAI Responses structured run.
pub struct XaiPreparedResponsesRun {
    evidence: XaiPreparedEvidence,
    request: StructuredRunRequest,
}

impl XaiPreparedResponsesRun {
    #[must_use]
    /// Returns the prepared operation evidence.
    pub const fn evidence(&self) -> &XaiPreparedEvidence {
        &self.evidence
    }

    #[must_use]
    /// Returns the immutable structured-run plan.
    pub const fn plan(&self) -> &PreflightPlan {
        self.evidence.plan()
    }

    #[must_use]
    /// Returns the plan-derived structured-run request.
    pub const fn request(&self) -> &StructuredRunRequest {
        &self.request
    }

    #[must_use]
    /// Returns the low-level WebSocket driver.
    pub fn low_level_driver(&self) -> XaiWebSocketDriver {
        XaiWebSocketDriver::new()
    }

    /// Starts the bound one-attempt structured run.
    pub fn start_run(
        &self,
        services: HostServices,
    ) -> BoxFuture<'static, Result<Box<dyn RunHandle>, RuntimeFailure>> {
        let driver = self.low_level_driver();
        let plan = self.plan().clone();
        let request = self.request.clone();
        Box::pin(async move { driver.start_run(plan, request, services).await })
    }

    #[must_use]
    /// Splits the prepared operation into evidence, plan, and request.
    pub fn into_parts(self) -> (XaiPreparedEvidence, PreflightPlan, StructuredRunRequest) {
        let plan = self.evidence.plan().clone();
        (self.evidence, plan, self.request)
    }
}

impl XaiPreparedIntegration {
    /// Prepares one Responses structured run with storage disabled.
    pub fn prepare_responses_run(
        &self,
        input: XaiRunProfileInput,
    ) -> Result<XaiPreparedResponsesRun, PreparationFailure> {
        let (request_id, model, content, deadline, reasoning, maximum_output_tokens) =
            input.into_parts();
        crate::controls::validate_preparation(
            model.model_id(),
            reasoning.as_ref(),
            maximum_output_tokens,
        )?;
        let controls = GenerationControls {
            reasoning,
            maximum_output_tokens,
        };
        let activity = crate::activity::profile::activity_profile();
        let base_requirements =
            crate::xai_responses_run_requirements(self.instance().execution_host_id().clone());
        let capabilities = crate::activity::profile::with_activity(
            CapabilityProfile::new(base_requirements.capabilities().cloned()),
            &activity,
        );
        let capabilities = crate::controls::with_capabilities(
            capabilities,
            controls.reasoning.as_ref(),
            controls.maximum_output_tokens,
        );
        let requirements = base_requirements.with_capabilities(capabilities.iter().map(
            |(capability, constraints)| {
                CapabilityRequirement::new(capability, constraints.iter().cloned())
            },
        ));
        let instance = instance_with_capabilities(self, capabilities.clone());
        let route = model_route(self, model, capabilities);
        let plan = build_plan(self, &instance, &route, &requirements)?;
        let mut policy = OperationPolicy::offline();
        if let Some(reasoning) = controls.reasoning.clone() {
            policy = policy.with_reasoning_mode(reasoning);
        }
        let mut request = StructuredRunRequest::new(request_id, content, policy);
        if let Some(maximum_output_tokens) = controls.maximum_output_tokens {
            request = request.with_maximum_output_tokens(maximum_output_tokens);
        }
        if let Some(deadline) = deadline {
            request = request.with_deadline(deadline);
        }
        Ok(XaiPreparedResponsesRun {
            evidence: XaiPreparedEvidence::from_prepared(self, plan, activity, controls)?,
            request,
        })
    }
}

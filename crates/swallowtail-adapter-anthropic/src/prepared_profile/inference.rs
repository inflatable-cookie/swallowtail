use super::input::AnthropicInferenceAttemptInput;
use super::plan::{
    AnthropicPreparedEvidence, build_plan, instance_with_capabilities, model_route, requirements,
};
use crate::prepared::failure;
use crate::{AnthropicDirectDriver, AnthropicPreparedIntegration};
use swallowtail_core::{
    Capability, CapabilityProfile, CapabilityRequirement, DriverRole, PreflightPlan,
};
use swallowtail_runtime::{
    BoxFuture, HostServices, OperationPolicy, PreparationFailure, PreparationStage, RunHandle,
    RuntimeFailure, StructuredRunDriver, StructuredRunRequest,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AnthropicPreparedInferenceAttempt {
    evidence: AnthropicPreparedEvidence,
    request: StructuredRunRequest,
}

impl AnthropicPreparedInferenceAttempt {
    #[must_use]
    pub const fn evidence(&self) -> &AnthropicPreparedEvidence {
        &self.evidence
    }

    #[must_use]
    pub const fn plan(&self) -> &PreflightPlan {
        self.evidence.plan()
    }

    #[must_use]
    pub const fn request(&self) -> &StructuredRunRequest {
        &self.request
    }

    #[must_use]
    pub fn low_level_driver(&self) -> AnthropicDirectDriver {
        AnthropicDirectDriver::new()
    }

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
    pub fn into_parts(
        self,
    ) -> (
        AnthropicPreparedEvidence,
        PreflightPlan,
        StructuredRunRequest,
    ) {
        let plan = self.evidence.plan().clone();
        (self.evidence, plan, self.request)
    }
}

impl AnthropicPreparedIntegration {
    pub fn prepare_inference_attempt(
        &self,
        input: AnthropicInferenceAttemptInput,
    ) -> Result<AnthropicPreparedInferenceAttempt, PreparationFailure> {
        let (request_id, model, content, maximum, deadline) = input.into_parts();
        if maximum.get() > u64::from(u32::MAX) {
            return Err(failure(
                PreparationStage::Preflight,
                "swallowtail.anthropic.preparation.output_limit_invalid",
                "Anthropic maximum output tokens exceed the supported request range",
            ));
        }
        let capability_requirements = inference_capabilities();
        let capabilities = CapabilityProfile::new(capability_requirements.clone());
        let instance = instance_with_capabilities(self, capabilities.clone());
        let route = model_route(self, model, capabilities);
        let requirements = requirements(self, DriverRole::StructuredRun, capability_requirements)
            .require_model_route();
        let plan = build_plan(self, &instance, Some(&route), &requirements)?;
        let mut request =
            StructuredRunRequest::new(request_id, content, OperationPolicy::offline())
                .with_maximum_output_tokens(maximum);
        if let Some(deadline) = deadline {
            request = request.with_deadline(deadline);
        }
        Ok(AnthropicPreparedInferenceAttempt {
            evidence: AnthropicPreparedEvidence::from_prepared(self, plan)?,
            request,
        })
    }
}

fn inference_capabilities() -> Vec<CapabilityRequirement> {
    [
        Capability::StructuredRun,
        Capability::StreamingEvents,
        Capability::UsageReporting,
        Capability::OutputTokenLimit,
    ]
    .into_iter()
    .map(|capability| CapabilityRequirement::new(capability, []))
    .collect()
}

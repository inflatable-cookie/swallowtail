use super::input::OllamaInferenceAttemptInput;
use super::plan::{
    OllamaPreparedEvidence, build_plan, instance_with_capabilities, model_route, requirements,
};
use crate::prepared::failure;
use crate::{OllamaNativeAttachedDriver, OllamaPreparedIntegration};
use swallowtail_core::{
    AttachedRuntimeResidency, Capability, CapabilityProfile, CapabilityRequirement, DriverRole,
    PreflightPlan,
};
use swallowtail_runtime::{
    BoxFuture, HostServices, OperationPolicy, PreparationFailure, PreparationStage, RunHandle,
    RuntimeFailure, StructuredRunDriver, StructuredRunRequest,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OllamaPreparedInferenceAttempt {
    evidence: OllamaPreparedEvidence,
    request: StructuredRunRequest,
}

impl OllamaPreparedInferenceAttempt {
    #[must_use]
    pub const fn evidence(&self) -> &OllamaPreparedEvidence {
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
    pub fn low_level_driver(&self) -> OllamaNativeAttachedDriver {
        OllamaNativeAttachedDriver::new()
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
    pub fn into_parts(self) -> (OllamaPreparedEvidence, PreflightPlan, StructuredRunRequest) {
        let plan = self.evidence.plan().clone();
        (self.evidence, plan, self.request)
    }
}

impl OllamaPreparedIntegration {
    pub fn prepare_inference_attempt(
        &self,
        input: OllamaInferenceAttemptInput,
    ) -> Result<OllamaPreparedInferenceAttempt, PreparationFailure> {
        let (request_id, content, maximum, deadline) = input.into_parts();
        if maximum.get() > u64::from(u32::MAX) {
            return Err(failure(
                PreparationStage::Preflight,
                "swallowtail.ollama.preparation.output_limit_invalid",
                "Ollama maximum output tokens exceed the supported request range",
            ));
        }
        let capability_requirements = inference_capabilities();
        let capabilities = CapabilityProfile::new(capability_requirements.clone());
        let instance = instance_with_capabilities(self, capabilities.clone());
        let route = model_route(self, self.model_selection().clone(), capabilities);
        let requirements = requirements(
            self,
            &route,
            DriverRole::StructuredRun,
            capability_requirements,
        );
        let plan = build_plan(self, &instance, &route, &requirements)?;
        let policy = OperationPolicy::offline()
            .with_attached_runtime_residency(AttachedRuntimeResidency::RuntimeManaged);
        let mut request = StructuredRunRequest::new(request_id, content, policy)
            .with_maximum_output_tokens(maximum);
        if let Some(deadline) = deadline {
            request = request.with_deadline(deadline);
        }
        Ok(OllamaPreparedInferenceAttempt {
            evidence: OllamaPreparedEvidence::from_prepared(self, plan)?,
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

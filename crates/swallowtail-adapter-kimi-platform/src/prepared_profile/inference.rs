use super::input::KimiPlatformInferenceAttemptInput;
use super::plan::{
    KimiPlatformPreparedEvidence, build_plan, instance_with_capabilities, model_route, requirements,
};
use crate::prepared::failure;
use crate::{KimiPlatformDirectDriver, KimiPlatformPreparedIntegration};
use swallowtail_core::{
    Capability, CapabilityConstraint, CapabilityProfile, CapabilityRequirement, DriverRole,
    PreflightPlan,
};
use swallowtail_runtime::{
    BoxFuture, HostServices, OperationPolicy, PreparationFailure, PreparationStage, RunHandle,
    RuntimeFailure, StructuredRunDriver, StructuredRunRequest,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct KimiPlatformPreparedInferenceAttempt {
    evidence: KimiPlatformPreparedEvidence,
    request: StructuredRunRequest,
}

impl KimiPlatformPreparedInferenceAttempt {
    #[must_use]
    pub const fn evidence(&self) -> &KimiPlatformPreparedEvidence {
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
    pub fn low_level_driver(&self) -> KimiPlatformDirectDriver {
        KimiPlatformDirectDriver::new()
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
        KimiPlatformPreparedEvidence,
        PreflightPlan,
        StructuredRunRequest,
    ) {
        let plan = self.evidence.plan().clone();
        (self.evidence, plan, self.request)
    }
}

impl KimiPlatformPreparedIntegration {
    pub fn prepare_inference_attempt(
        &self,
        input: KimiPlatformInferenceAttemptInput,
    ) -> Result<KimiPlatformPreparedInferenceAttempt, PreparationFailure> {
        let (request_id, model, content, reasoning, maximum, deadline) = input.into_parts();
        if maximum.get() > crate::selection::KIMI_PLATFORM_MAXIMUM_OUTPUT_TOKENS {
            return Err(failure(
                PreparationStage::Preflight,
                "swallowtail.kimi_platform.preparation.output_limit_invalid",
                "Kimi Platform maximum output tokens exceed the K3 route bound",
            ));
        }
        if !matches!(reasoning.as_str(), "low" | "high" | "max") {
            return Err(failure(
                PreparationStage::Preflight,
                "swallowtail.kimi_platform.preparation.reasoning_rejected",
                "Kimi Platform K3 requires low, high, or max reasoning",
            ));
        }
        let capability_requirements = inference_capabilities(&reasoning);
        let capabilities = CapabilityProfile::new(capability_requirements.clone());
        let instance = instance_with_capabilities(self, capabilities.clone());
        let route = model_route(self, model, capabilities);
        if route.model_id().as_str() != crate::KIMI_PLATFORM_MODEL_ID {
            return Err(failure(
                PreparationStage::Preflight,
                "swallowtail.kimi_platform.preparation.model_rejected",
                "Kimi Platform preparation requires the exact kimi-k3 model route",
            ));
        }
        let requirements = requirements(self, DriverRole::StructuredRun, capability_requirements)
            .require_model_route();
        let plan = build_plan(self, &instance, Some(&route), &requirements)?;
        let mut request = StructuredRunRequest::new(
            request_id,
            content,
            OperationPolicy::offline().with_reasoning_mode(reasoning),
        )
        .with_maximum_output_tokens(maximum);
        if let Some(deadline) = deadline {
            request = request.with_deadline(deadline);
        }
        Ok(KimiPlatformPreparedInferenceAttempt {
            evidence: KimiPlatformPreparedEvidence::from_prepared(self, plan)?,
            request,
        })
    }
}

fn inference_capabilities(
    reasoning: &swallowtail_core::ReasoningMode,
) -> Vec<CapabilityRequirement> {
    let mut requirements: Vec<_> = [
        Capability::StructuredRun,
        Capability::StreamingEvents,
        Capability::UsageReporting,
        Capability::OutputTokenLimit,
    ]
    .into_iter()
    .map(|capability| CapabilityRequirement::new(capability, []))
    .collect();
    requirements.push(CapabilityRequirement::new(
        Capability::ReasoningSelection,
        [CapabilityConstraint::ReasoningMode(reasoning.clone())],
    ));
    requirements
}

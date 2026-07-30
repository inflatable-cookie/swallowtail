use super::input::DeepSeekRunProfileInput;
use super::plan::{DeepSeekPreparedEvidence, build_plan, instance_with_capabilities, model_route};
use crate::prepared::failure;
use crate::{DeepSeekDirectDriver, DeepSeekPreparedIntegration};
use swallowtail_core::{
    CapabilityProfile, CapabilityRequirement, PreflightPlan, ProviderInferenceCachePolicy,
};
use swallowtail_runtime::{
    BoxFuture, HostServices, OperationPolicy, PreparationFailure, PreparationStage, RunHandle,
    RuntimeFailure, StructuredRunDriver, StructuredRunRequest,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DeepSeekPreparedRun {
    evidence: DeepSeekPreparedEvidence,
    request: StructuredRunRequest,
}

impl DeepSeekPreparedRun {
    #[must_use]
    pub const fn evidence(&self) -> &DeepSeekPreparedEvidence {
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
    pub fn low_level_driver(&self) -> DeepSeekDirectDriver {
        DeepSeekDirectDriver::new()
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
        DeepSeekPreparedEvidence,
        PreflightPlan,
        StructuredRunRequest,
    ) {
        let plan = self.evidence.plan().clone();
        (self.evidence, plan, self.request)
    }
}

impl DeepSeekPreparedIntegration {
    pub fn prepare_run(
        &self,
        input: DeepSeekRunProfileInput,
    ) -> Result<DeepSeekPreparedRun, PreparationFailure> {
        let (request_id, model, content, reasoning, maximum, cache_policy, deadline) =
            input.into_parts();
        if reasoning.as_str() != "high"
            || maximum.get() > u64::from(u32::MAX)
            || cache_policy != ProviderInferenceCachePolicy::AcceptedWithoutManagementAuthority
        {
            return Err(failure(
                PreparationStage::Preflight,
                "swallowtail.deepseek.preparation.run_options_rejected",
                "DeepSeek structured runs require high reasoning, a supported output-token limit, and explicit unmanaged-cache acceptance",
            ));
        }
        let activity = crate::activity::profile::activity_profile(false);
        let base_requirements = crate::deepseek_v4_run_requirements(
            self.instance().execution_host_id().clone(),
            self.access_profile().id().clone(),
        );
        let capabilities = crate::activity::profile::with_activity(
            CapabilityProfile::new(base_requirements.capabilities().cloned()),
            &activity,
        );
        let requirements = base_requirements.with_capabilities(capabilities.iter().map(
            |(capability, constraints)| {
                CapabilityRequirement::new(capability, constraints.iter().cloned())
            },
        ));
        let instance = instance_with_capabilities(self, capabilities.clone());
        let route = model_route(self, model, capabilities);
        if route.model_id().as_str() != crate::DEEPSEEK_MODEL_ID {
            return Err(failure(
                PreparationStage::Preflight,
                "swallowtail.deepseek.preparation.model_rejected",
                "DeepSeek preparation requires the exact deepseek-v4-pro model route",
            ));
        }
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
        Ok(DeepSeekPreparedRun {
            evidence: DeepSeekPreparedEvidence::from_prepared_with_activity(self, plan, activity)?,
            request,
        })
    }
}

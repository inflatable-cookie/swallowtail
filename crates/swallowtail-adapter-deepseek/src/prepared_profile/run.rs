use super::input::DeepSeekRunProfileInput;
use super::plan::{DeepSeekPreparedEvidence, build_plan, instance_with_capabilities, model_route};
use crate::prepared::failure;
use crate::selection::{
    deepseek_reasoning_mode_is_supported, deepseek_requirements_for_reasoning,
    deepseek_v4_run_requirements_without_reasoning,
};
use crate::{DeepSeekDirectDriver, DeepSeekPreparedIntegration};
use swallowtail_core::{
    CapabilityProfile, CapabilityRequirement, PreflightPlan, ProviderInferenceCachePolicy,
};
use swallowtail_runtime::{
    BoxFuture, HostServices, OperationPolicy, PreparationFailure, PreparationStage, RunHandle,
    RuntimeFailure, StructuredRunDriver, StructuredRunRequest,
};

#[derive(Clone, Debug, Eq, PartialEq)]
/// Bound prepared DeepSeek one-attempt structured run.
pub struct DeepSeekPreparedRun {
    evidence: DeepSeekPreparedEvidence,
    request: StructuredRunRequest,
}

impl DeepSeekPreparedRun {
    #[must_use]
    /// Returns the prepared operation evidence.
    pub const fn evidence(&self) -> &DeepSeekPreparedEvidence {
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
    /// Returns the low-level direct driver.
    pub fn low_level_driver(&self) -> DeepSeekDirectDriver {
        let driver = DeepSeekDirectDriver::new();
        match self.evidence.thinking_mode() {
            Some(thinking_mode) => driver.with_thinking_mode(thinking_mode),
            None => driver,
        }
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
    /// Prepares a tool-free V4 structured run.
    pub fn prepare_run(
        &self,
        input: DeepSeekRunProfileInput,
    ) -> Result<DeepSeekPreparedRun, PreparationFailure> {
        let (request_id, model, content, reasoning, thinking_mode, maximum, cache_policy, deadline) =
            input.into_parts();
        let selection_is_supported = match (&reasoning, thinking_mode) {
            (Some(reasoning), None) => deepseek_reasoning_mode_is_supported(reasoning),
            (None, Some(_)) => true,
            _ => false,
        };
        if !selection_is_supported
            || maximum.get() > u64::from(u32::MAX)
            || cache_policy != ProviderInferenceCachePolicy::AcceptedWithoutManagementAuthority
        {
            return Err(failure(
                PreparationStage::Preflight,
                "swallowtail.deepseek.preparation.run_options_rejected",
                "DeepSeek structured runs require an exact supported reasoning selection or adapter-local thinking mode, a supported output-token limit, and explicit unmanaged-cache acceptance",
            ));
        }
        let activity = crate::activity::profile::activity_profile(false);
        let base_requirements = if reasoning.is_some() {
            crate::deepseek_v4_run_requirements(
                self.instance().execution_host_id().clone(),
                self.access_profile().id().clone(),
            )
        } else {
            deepseek_v4_run_requirements_without_reasoning(
                self.instance().execution_host_id().clone(),
                self.access_profile().id().clone(),
            )
        };
        let capabilities = crate::activity::profile::with_activity(
            CapabilityProfile::new(base_requirements.capabilities().cloned()),
            &activity,
        );
        let requirements = base_requirements.with_capabilities(capabilities.iter().map(
            |(capability, constraints)| {
                CapabilityRequirement::new(capability, constraints.iter().cloned())
            },
        ));
        let requirements = if let Some(reasoning) = reasoning.as_ref() {
            deepseek_requirements_for_reasoning(requirements, reasoning)
        } else {
            requirements
        };
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
        let policy = reasoning
            .as_ref()
            .map_or_else(OperationPolicy::offline, |reasoning| {
                OperationPolicy::offline().with_reasoning_mode(reasoning.clone())
            });
        let mut request = StructuredRunRequest::new(request_id, content, policy)
            .with_maximum_output_tokens(maximum);
        if let Some(deadline) = deadline {
            request = request.with_deadline(deadline);
        }
        Ok(DeepSeekPreparedRun {
            evidence: DeepSeekPreparedEvidence::from_prepared_with_activity(
                self,
                plan,
                activity,
                reasoning,
                thinking_mode,
            )?,
            request,
        })
    }
}

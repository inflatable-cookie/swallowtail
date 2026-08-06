use super::input::DeepSeekSessionProfileInput;
use super::plan::{DeepSeekPreparedEvidence, build_plan, instance_with_capabilities, model_route};
use crate::prepared::failure;
use crate::{DeepSeekDirectDriver, DeepSeekPreparedIntegration};
use swallowtail_core::{
    CapabilityProfile, CapabilityRequirement, PreflightPlan, ProviderInferenceCachePolicy,
};
use swallowtail_runtime::{
    BoxFuture, HostServices, InteractiveSessionDriver, InteractiveSessionHandle,
    OpenDirectContinuationSessionRequest, PreparationFailure, PreparationStage, RuntimeFailure,
};

#[derive(Clone, Debug, Eq, PartialEq)]
/// Bound prepared DeepSeek direct-continuation session.
pub struct DeepSeekPreparedSession {
    evidence: DeepSeekPreparedEvidence,
    request: OpenDirectContinuationSessionRequest,
}

impl DeepSeekPreparedSession {
    #[must_use]
    /// Returns the prepared operation evidence.
    pub const fn evidence(&self) -> &DeepSeekPreparedEvidence {
        &self.evidence
    }

    #[must_use]
    /// Returns the immutable continuation-session plan.
    pub const fn plan(&self) -> &PreflightPlan {
        self.evidence.plan()
    }

    #[must_use]
    /// Returns the plan-derived session-open request.
    pub const fn request(&self) -> &OpenDirectContinuationSessionRequest {
        &self.request
    }

    #[must_use]
    /// Returns the low-level direct driver.
    pub fn low_level_driver(&self) -> DeepSeekDirectDriver {
        DeepSeekDirectDriver::new()
    }

    /// Opens the bound resource-free continuation session.
    pub fn open_session(
        &self,
        services: HostServices,
    ) -> BoxFuture<'static, Result<Box<dyn InteractiveSessionHandle>, RuntimeFailure>> {
        let driver = self.low_level_driver();
        let plan = self.plan().clone();
        let request = self.request.clone();
        Box::pin(async move {
            driver
                .open_direct_continuation_session(plan, request, services)
                .await
        })
    }

    #[must_use]
    /// Splits the prepared operation into evidence, plan, and request.
    pub fn into_parts(
        self,
    ) -> (
        DeepSeekPreparedEvidence,
        PreflightPlan,
        OpenDirectContinuationSessionRequest,
    ) {
        let plan = self.evidence.plan().clone();
        (self.evidence, plan, self.request)
    }
}

impl DeepSeekPreparedIntegration {
    /// Prepares a resource-free V4 continuation session.
    pub fn prepare_session(
        &self,
        input: DeepSeekSessionProfileInput,
    ) -> Result<DeepSeekPreparedSession, PreparationFailure> {
        let (request_id, model, options, cache_policy) = input.into_parts();
        if cache_policy != ProviderInferenceCachePolicy::AcceptedWithoutManagementAuthority {
            return Err(failure(
                PreparationStage::Preflight,
                "swallowtail.deepseek.preparation.cache_posture_rejected",
                "DeepSeek direct continuation requires explicit acceptance of unmanaged provider caching",
            ));
        }
        if options
            .reasoning_mode()
            .is_none_or(|mode| mode.as_str() != "high")
            || options.tools().len() == 0
            || options.tools().len() > 8
        {
            return Err(failure(
                PreparationStage::Preflight,
                "swallowtail.deepseek.preparation.session_options_rejected",
                "DeepSeek direct continuation requires high reasoning and one to eight declared tools",
            ));
        }
        let activity = crate::activity::profile::activity_profile(true);
        let base_requirements = crate::deepseek_v4_requirements(
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
        let request =
            OpenDirectContinuationSessionRequest::new(request_id, crate::deepseek_v4_config())
                .with_options(options);
        crate::validate_deepseek_request_plan(&plan, &request).map_err(|error| {
            PreparationFailure::new(
                PreparationStage::Preflight,
                swallowtail_core::Diagnostic::new(error.diagnostic().clone()),
            )
        })?;
        Ok(DeepSeekPreparedSession {
            evidence: DeepSeekPreparedEvidence::from_prepared_with_activity(self, plan, activity)?,
            request,
        })
    }
}

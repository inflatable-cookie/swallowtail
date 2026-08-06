use super::input::AlibabaRunProfileInput;
use super::plan::{
    AlibabaModelStudioPreparedEvidence, build_plan, instance_with_capabilities, model_route,
};
use crate::prepared::failure;
use crate::{AlibabaModelStudioDriver, AlibabaModelStudioPreparedIntegration};
use swallowtail_core::{CapabilityProfile, CapabilityRequirement, PreflightPlan};
use swallowtail_runtime::{
    BoxFuture, HostServices, OperationPolicy, PreparationFailure, PreparationStage, RunHandle,
    RuntimeFailure, StructuredRunDriver, StructuredRunRequest,
};

#[derive(Clone, Debug, Eq, PartialEq)]
/// Executable unstored structured response with exact plan agreement.
pub struct AlibabaModelStudioPreparedRun {
    evidence: AlibabaModelStudioPreparedEvidence,
    request: StructuredRunRequest,
}

impl AlibabaModelStudioPreparedRun {
    #[must_use]
    /// Returns route-specific prepared evidence.
    pub const fn evidence(&self) -> &AlibabaModelStudioPreparedEvidence {
        &self.evidence
    }

    #[must_use]
    /// Returns the immutable structured-run preflight plan.
    pub const fn plan(&self) -> &PreflightPlan {
        self.evidence.plan()
    }

    #[must_use]
    /// Returns the plan-derived structured-run request.
    pub const fn request(&self) -> &StructuredRunRequest {
        &self.request
    }

    #[must_use]
    /// Returns the public low-level workspace driver.
    pub fn low_level_driver(&self) -> AlibabaModelStudioDriver {
        AlibabaModelStudioDriver::new()
    }

    /// Starts the single unstored provider inference attempt.
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
    /// Splits the prepared value into evidence, plan, and request.
    pub fn into_parts(
        self,
    ) -> (
        AlibabaModelStudioPreparedEvidence,
        PreflightPlan,
        StructuredRunRequest,
    ) {
        let plan = self.evidence.plan().clone();
        (self.evidence, plan, self.request)
    }
}

impl AlibabaModelStudioPreparedIntegration {
    /// Validates and prepares one tool-free unstored structured response.
    pub fn prepare_run(
        &self,
        input: AlibabaRunProfileInput,
    ) -> Result<AlibabaModelStudioPreparedRun, PreparationFailure> {
        let (request_id, route_id, route_revision, model_id, content, deadline) =
            input.into_parts();
        if route_id.as_str() != crate::MODEL_ROUTE_ID || model_id.as_str() != crate::EXACT_MODEL_ID
        {
            return Err(failure(
                PreparationStage::Preflight,
                "swallowtail.alibaba_model_studio.preparation.route_rejected",
                "Alibaba Model Studio preparation requires the exact Singapore Qwen route",
            ));
        }
        let activity = crate::activity::profile::activity_profile();
        let base_requirements = crate::alibaba_model_studio_run_requirements(
            self.instance().execution_host_id().clone(),
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
        let route = model_route(self, route_id, route_revision, model_id, capabilities);
        let plan = build_plan(self, &instance, &route, &requirements)?;
        let mut request =
            StructuredRunRequest::new(request_id, content, OperationPolicy::offline());
        if let Some(deadline) = deadline {
            request = request.with_deadline(deadline);
        }
        Ok(AlibabaModelStudioPreparedRun {
            evidence: AlibabaModelStudioPreparedEvidence::from_prepared(self, plan, activity)?,
            request,
        })
    }
}

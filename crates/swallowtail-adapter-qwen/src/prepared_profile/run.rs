use super::input::QwenRunProfileInput;
use super::plan::{QwenPreparedEvidence, build_plan, instance_with_capabilities, requirements};
use crate::prepared::instance::run_capabilities;
use crate::{QwenHeadlessDriver, QwenPreparedIntegration};
use swallowtail_core::{
    CapabilityRequirement, HarnessConfigurationPosture, HarnessIsolation, ModelRoute,
};
use swallowtail_runtime::{
    BoxFuture, HostServices, OperationPolicy, PreparationFailure, ProviderRetentionPolicy,
    RunHandle, RuntimeFailure, StructuredRunDriver, StructuredRunRequest,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QwenPreparedRun {
    evidence: QwenPreparedEvidence,
    request: StructuredRunRequest,
}

impl QwenPreparedRun {
    #[must_use]
    pub const fn evidence(&self) -> &QwenPreparedEvidence {
        &self.evidence
    }

    #[must_use]
    pub const fn plan(&self) -> &swallowtail_core::PreflightPlan {
        self.evidence.plan()
    }

    #[must_use]
    pub const fn request(&self) -> &StructuredRunRequest {
        &self.request
    }

    #[must_use]
    pub fn low_level_driver(&self) -> QwenHeadlessDriver {
        self.evidence.low_level_driver()
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
        QwenPreparedEvidence,
        swallowtail_core::PreflightPlan,
        StructuredRunRequest,
    ) {
        let plan = self.evidence.plan().clone();
        (self.evidence, plan, self.request)
    }
}

impl QwenPreparedIntegration {
    pub fn prepare_run(
        &self,
        input: QwenRunProfileInput,
    ) -> Result<QwenPreparedRun, PreparationFailure> {
        let (request_id, model, content, working_resource, deadline) = input.into_parts();
        let capabilities = run_capabilities();
        let instance = instance_with_capabilities(self, capabilities.clone());
        let (route_id, route_revision, provider_id, model_id) = model.into_parts();
        let route = ModelRoute::new(
            route_id,
            route_revision,
            self.instance().id().clone(),
            model_id,
            capabilities,
        )
        .with_provider_id(provider_id);
        let requirements = requirements(
            self,
            run_capabilities().iter().map(|(capability, constraints)| {
                CapabilityRequirement::new(capability, constraints.iter().cloned())
            }),
        );
        let plan = build_plan(self, &instance, &route, &requirements)?;
        let policy = OperationPolicy::offline()
            .with_provider_retention(ProviderRetentionPolicy::DurableAllowed)
            .with_harness_isolation(HarnessIsolation::AmbientHost)
            .with_harness_configuration_posture(HarnessConfigurationPosture::Ambient);
        let request = StructuredRunRequest::new(request_id, content, policy)
            .with_working_resource(working_resource)
            .with_deadline(deadline);
        Ok(QwenPreparedRun {
            evidence: QwenPreparedEvidence::from_prepared(self, plan)?,
            request,
        })
    }
}

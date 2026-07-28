use super::PiPreparedRunFuture;
use super::input::PiRunProfileInput;
use super::plan::{PiPreparedEvidence, build_plan, instance_with_capabilities, run_requirements};
use crate::prepared::instance::run_capabilities;
use crate::{PiPreparedIntegration, PiRpcDriver};
use swallowtail_core::{
    CapabilityRequirement, HarnessConfigurationPosture, HarnessIsolation, ModelRoute,
};
use swallowtail_runtime::{
    HostServices, OperationPolicy, PreparationFailure, ProviderRetentionPolicy,
    StructuredRunDriver, StructuredRunRequest,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PiPreparedRun {
    evidence: PiPreparedEvidence,
    request: StructuredRunRequest,
}

impl PiPreparedRun {
    #[must_use]
    pub const fn evidence(&self) -> &PiPreparedEvidence {
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
    pub fn low_level_driver(&self) -> PiRpcDriver {
        self.evidence.low_level_driver()
    }

    pub fn start_run(&self, services: HostServices) -> PiPreparedRunFuture {
        let driver = self.low_level_driver();
        let plan = self.plan().clone();
        let request = self.request.clone();
        Box::pin(async move { driver.start_run(plan, request, services).await })
    }

    #[must_use]
    pub fn into_parts(
        self,
    ) -> (
        PiPreparedEvidence,
        swallowtail_core::PreflightPlan,
        StructuredRunRequest,
    ) {
        let plan = self.evidence.plan().clone();
        (self.evidence, plan, self.request)
    }
}

impl PiPreparedIntegration {
    pub fn prepare_run(
        &self,
        input: PiRunProfileInput,
    ) -> Result<PiPreparedRun, PreparationFailure> {
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
        let requirements = run_requirements(
            self,
            run_capabilities().iter().map(|(capability, constraints)| {
                CapabilityRequirement::new(capability, constraints.iter().cloned())
            }),
        );
        let plan = build_plan(self, &instance, &route, &requirements)?;
        let policy = OperationPolicy::offline()
            .with_provider_retention(ProviderRetentionPolicy::Prohibited)
            .with_harness_isolation(HarnessIsolation::AmbientHost)
            .with_harness_configuration_posture(HarnessConfigurationPosture::ProviderSuppressed);
        let request = StructuredRunRequest::new(request_id, content, policy)
            .with_working_resource(working_resource)
            .with_deadline(deadline);
        Ok(PiPreparedRun {
            evidence: PiPreparedEvidence::from_prepared(self, plan)?,
            request,
        })
    }
}

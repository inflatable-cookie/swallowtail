use super::plan::{
    CodexPreparedEvidence, build_plan, descriptor, instance_with_capabilities, require_driver,
    requirements,
};
use crate::{CodexAppServerDriver, CodexPreparedDriver, CodexPreparedIntegration};
use swallowtail_core::{
    Capability, CapabilityProfile, CapabilityRequirement, DriverRole, HarnessConfigurationPosture,
    HostServiceKind, ModelCatalogEntry, OperationShape, PreflightPlan,
};
use swallowtail_runtime::{
    BoxFuture, Deadline, HostServices, ModelCatalogDriver, ModelCatalogRequest, PreparationFailure,
    RuntimeFailure,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CodexPreparedCatalogue {
    evidence: CodexPreparedEvidence,
    request: ModelCatalogRequest,
}

impl CodexPreparedCatalogue {
    #[must_use]
    pub const fn evidence(&self) -> &CodexPreparedEvidence {
        &self.evidence
    }

    #[must_use]
    pub const fn plan(&self) -> &PreflightPlan {
        self.evidence.plan()
    }

    #[must_use]
    pub const fn request(&self) -> &ModelCatalogRequest {
        &self.request
    }

    #[must_use]
    pub fn low_level_driver(&self) -> CodexAppServerDriver {
        CodexAppServerDriver::new(self.evidence.environment().clone())
    }

    pub fn list_models(
        &self,
        services: HostServices,
    ) -> BoxFuture<'static, Result<Vec<ModelCatalogEntry>, RuntimeFailure>> {
        let driver = self.low_level_driver();
        let plan = self.plan().clone();
        let request = self.request.clone();
        Box::pin(async move { driver.list_models(plan, request, services).await })
    }

    #[must_use]
    pub fn into_parts(self) -> (CodexPreparedEvidence, PreflightPlan, ModelCatalogRequest) {
        let plan = self.evidence.plan().clone();
        (self.evidence, plan, self.request)
    }
}

impl CodexPreparedIntegration {
    pub fn prepare_catalogue(
        &self,
        request_id: swallowtail_runtime::RequestId,
        deadline: Option<Deadline>,
    ) -> Result<CodexPreparedCatalogue, PreparationFailure> {
        require_driver(self, CodexPreparedDriver::AppServer)?;
        let capability = CapabilityRequirement::new(Capability::ModelCatalog, []);
        let capabilities = CapabilityProfile::new([capability.clone()]);
        let instance = instance_with_capabilities(self, capabilities);
        let mut host_services = vec![HostServiceKind::Task, HostServiceKind::Process];
        if deadline.is_some() {
            host_services.push(HostServiceKind::Time);
        }
        let requirements = requirements(
            self,
            OperationShape::InteractiveSession,
            DriverRole::ModelCatalog,
            host_services,
            [capability],
        )
        .with_harness_configuration_posture(HarnessConfigurationPosture::Ambient);
        let plan = build_plan(self, &descriptor(self), &instance, None, &requirements)?;
        let request = match deadline {
            Some(deadline) => ModelCatalogRequest::new(request_id).with_deadline(deadline),
            None => ModelCatalogRequest::new(request_id),
        };
        Ok(CodexPreparedCatalogue {
            evidence: CodexPreparedEvidence::from_prepared(self, plan)?,
            request,
        })
    }
}

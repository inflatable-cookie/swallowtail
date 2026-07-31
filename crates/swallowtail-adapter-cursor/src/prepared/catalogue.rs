use super::CursorPreparedCatalogueIntegration;
use swallowtail_core::{
    Capability, CapabilityProfile, CapabilityRequirement, DriverRole, ExecutionLayer,
    HarnessConfigurationPosture, HarnessIsolation, HostServiceKind, OperationRequirements,
    OperationShape, PreflightPlan,
};
use swallowtail_runtime::{
    BoxFuture, Deadline, HostServices, ModelCatalogDriver, ModelCatalogRequest, PreparationFailure,
    PreparedOperationEvidence, RequestId, RuntimeFailure,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CursorCatalogueProfileInput {
    request_id: RequestId,
    deadline: Option<Deadline>,
}

impl CursorCatalogueProfileInput {
    #[must_use]
    pub const fn new(request_id: RequestId) -> Self {
        Self {
            request_id,
            deadline: None,
        }
    }

    #[must_use]
    pub const fn with_deadline(mut self, deadline: Deadline) -> Self {
        self.deadline = Some(deadline);
        self
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CursorPreparedCatalogue {
    evidence: PreparedOperationEvidence,
    request: ModelCatalogRequest,
    environment: swallowtail_runtime::EnvironmentRef,
}

impl CursorPreparedCatalogueIntegration {
    pub fn prepare_catalogue(
        &self,
        input: CursorCatalogueProfileInput,
    ) -> Result<CursorPreparedCatalogue, PreparationFailure> {
        let capability = CapabilityRequirement::new(Capability::ModelCatalog, []);
        let capabilities = CapabilityProfile::new([capability.clone()]);
        let instance = super::plan::instance_with_capabilities(self.instance(), capabilities);
        let requirements = OperationRequirements::new(
            ExecutionLayer::HarnessInteraction,
            OperationShape::StructuredRun,
            DriverRole::ModelCatalog,
            self.instance().execution_host_id().clone(),
            super::plan::access_requirement(self.access_profile()),
        )
        .with_ownership_modes([self.instance().ownership()])
        .with_host_services([HostServiceKind::Process, HostServiceKind::Time])
        .with_capabilities([capability])
        .with_interface_versions([self.observation().version().clone()])
        .with_harness_isolation(HarnessIsolation::AmbientHost)
        .with_harness_configuration_posture(HarnessConfigurationPosture::Ambient);
        let plan = super::plan::build_plan(
            &crate::cursor_catalogue_descriptor(),
            &instance,
            self.access_profile(),
            self.access_evidence(),
            self.available_host_services(),
            &requirements,
            None,
        )?;
        let request = match input.deadline {
            Some(deadline) => ModelCatalogRequest::new(input.request_id).with_deadline(deadline),
            None => ModelCatalogRequest::new(input.request_id),
        };
        Ok(CursorPreparedCatalogue {
            evidence: PreparedOperationEvidence::from_plan(plan, self.access_evidence().clone())?,
            request,
            environment: self.environment().clone(),
        })
    }
}

impl CursorPreparedCatalogue {
    #[must_use]
    pub const fn evidence(&self) -> &PreparedOperationEvidence {
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
    pub fn low_level_driver(&self) -> crate::CursorCatalogueDriver {
        crate::CursorCatalogueDriver::new(self.environment.clone())
    }

    pub fn list_models(
        &self,
        services: HostServices,
    ) -> BoxFuture<'static, Result<Vec<swallowtail_core::ModelCatalogEntry>, RuntimeFailure>> {
        let driver = self.low_level_driver();
        let plan = self.plan().clone();
        let request = self.request.clone();
        Box::pin(async move { driver.list_models(plan, request, services).await })
    }
}

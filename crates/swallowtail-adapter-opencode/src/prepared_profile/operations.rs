use super::OpenCodePreparedSessionFuture;
use super::input::{OpenCodeCatalogueProfileInput, OpenCodeSessionProfileInput};
use super::plan::{OpenCodePreparedEvidence, build_plan, instance_with_capabilities, requirements};
use crate::{OpenCodeHttpDriver, OpenCodePreparedIntegration};
use swallowtail_core::{
    Capability, CapabilityProfile, CapabilityRequirement, DriverRole, ModelCatalogEntry,
    ModelRoute, PreflightPlan,
};
use swallowtail_runtime::{
    BoxFuture, HostServices, InteractiveSessionDriver, ModelCatalogDriver, ModelCatalogRequest,
    OpenSessionRequest, PreparationFailure, RuntimeFailure,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OpenCodePreparedCatalogue {
    evidence: OpenCodePreparedEvidence,
    request: ModelCatalogRequest,
}

impl OpenCodePreparedCatalogue {
    #[must_use]
    pub const fn evidence(&self) -> &OpenCodePreparedEvidence {
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
    pub fn low_level_driver(&self) -> OpenCodeHttpDriver {
        OpenCodeHttpDriver::new()
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
    pub fn into_parts(self) -> (OpenCodePreparedEvidence, PreflightPlan, ModelCatalogRequest) {
        let plan = self.evidence.plan().clone();
        (self.evidence, plan, self.request)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OpenCodePreparedSession {
    evidence: OpenCodePreparedEvidence,
    request: OpenSessionRequest,
}

impl OpenCodePreparedSession {
    #[must_use]
    pub const fn evidence(&self) -> &OpenCodePreparedEvidence {
        &self.evidence
    }

    #[must_use]
    pub const fn plan(&self) -> &PreflightPlan {
        self.evidence.plan()
    }

    #[must_use]
    pub const fn request(&self) -> &OpenSessionRequest {
        &self.request
    }

    #[must_use]
    pub fn low_level_driver(&self) -> OpenCodeHttpDriver {
        OpenCodeHttpDriver::new()
    }

    pub fn open_session(&self, services: HostServices) -> OpenCodePreparedSessionFuture {
        let driver = self.low_level_driver();
        let plan = self.plan().clone();
        let request = self.request.clone();
        Box::pin(async move { driver.open_session(plan, request, services).await })
    }

    #[must_use]
    pub fn into_parts(self) -> (OpenCodePreparedEvidence, PreflightPlan, OpenSessionRequest) {
        let plan = self.evidence.plan().clone();
        (self.evidence, plan, self.request)
    }
}

impl OpenCodePreparedIntegration {
    pub fn prepare_catalogue(
        &self,
        input: OpenCodeCatalogueProfileInput,
    ) -> Result<OpenCodePreparedCatalogue, PreparationFailure> {
        let capabilities =
            CapabilityProfile::new([CapabilityRequirement::new(Capability::ModelCatalog, [])]);
        let instance = instance_with_capabilities(self, capabilities.clone());
        let requirements = requirements(
            self,
            DriverRole::ModelCatalog,
            capabilities.iter().map(|(capability, constraints)| {
                CapabilityRequirement::new(capability, constraints.iter().cloned())
            }),
        );
        let plan = build_plan(self, &instance, None, &requirements)?;
        let (request_id, deadline) = input.into_parts();
        let request = match deadline {
            Some(deadline) => ModelCatalogRequest::new(request_id).with_deadline(deadline),
            None => ModelCatalogRequest::new(request_id),
        };
        Ok(OpenCodePreparedCatalogue {
            evidence: OpenCodePreparedEvidence::from_prepared(self, plan)?,
            request,
        })
    }

    pub fn prepare_session(
        &self,
        input: OpenCodeSessionProfileInput,
    ) -> Result<OpenCodePreparedSession, PreparationFailure> {
        let (request_id, model, working_resource, deadline) = input.into_parts();
        let capabilities = crate::prepared::all_capabilities();
        let session_capabilities = CapabilityProfile::new(
            capabilities
                .iter()
                .filter(|(capability, _)| *capability != Capability::ModelCatalog)
                .map(|(capability, constraints)| {
                    CapabilityRequirement::new(capability, constraints.iter().cloned())
                }),
        );
        let instance = instance_with_capabilities(self, session_capabilities.clone());
        let (route_id, route_revision, provider_id, model_id) = model.into_parts();
        let route = ModelRoute::new(
            route_id,
            route_revision,
            self.instance().id().clone(),
            model_id,
            session_capabilities.clone(),
        )
        .with_provider_id(provider_id);
        let requirements = requirements(
            self,
            DriverRole::InteractiveSession,
            session_capabilities
                .iter()
                .map(|(capability, constraints)| {
                    CapabilityRequirement::new(capability, constraints.iter().cloned())
                }),
        );
        let plan = build_plan(self, &instance, Some(&route), &requirements)?;
        let request = OpenSessionRequest::from_plan(&plan, request_id, working_resource, deadline)?;
        Ok(OpenCodePreparedSession {
            evidence: OpenCodePreparedEvidence::from_prepared(self, plan)?,
            request,
        })
    }
}

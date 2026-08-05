use super::input::OhMyPiCatalogueProfileInput;
use super::plan::{
    OhMyPiPreparedEvidence, build_catalogue_plan, catalogue_requirements,
    instance_with_capabilities,
};
use crate::{OhMyPiPreparedIntegration, OhMyPiRpcDriver};
use swallowtail_core::{
    Capability, CapabilityProfile, CapabilityRequirement, ModelCatalogEntry, PreflightPlan,
};
use swallowtail_runtime::{
    BoxFuture, HostServices, ModelCatalogDriver, ModelCatalogRequest, PreparationFailure,
    RuntimeFailure,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OhMyPiPreparedCatalogue {
    evidence: OhMyPiPreparedEvidence,
    request: ModelCatalogRequest,
}

impl OhMyPiPreparedCatalogue {
    #[must_use]
    pub const fn evidence(&self) -> &OhMyPiPreparedEvidence {
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
    pub fn low_level_driver(&self) -> OhMyPiRpcDriver {
        self.evidence.low_level_driver()
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
    pub fn into_parts(self) -> (OhMyPiPreparedEvidence, PreflightPlan, ModelCatalogRequest) {
        let plan = self.evidence.plan().clone();
        (self.evidence, plan, self.request)
    }
}

impl OhMyPiPreparedIntegration {
    pub fn prepare_catalogue(
        &self,
        input: OhMyPiCatalogueProfileInput,
    ) -> Result<OhMyPiPreparedCatalogue, PreparationFailure> {
        let capability = CapabilityRequirement::new(Capability::ModelCatalog, []);
        let capabilities = CapabilityProfile::new([capability.clone()]);
        let instance = instance_with_capabilities(self, capabilities);
        let requirements = catalogue_requirements(self, [capability]);
        let plan = build_catalogue_plan(self, &instance, &requirements)?;
        let (request_id, deadline) = input.into_parts();
        let request = match deadline {
            Some(deadline) => ModelCatalogRequest::new(request_id).with_deadline(deadline),
            None => ModelCatalogRequest::new(request_id),
        };
        Ok(OhMyPiPreparedCatalogue {
            evidence: OhMyPiPreparedEvidence::from_prepared(self, plan)?,
            request,
        })
    }
}

use super::input::PiCatalogueProfileInput;
use super::plan::{
    PiPreparedEvidence, build_catalogue_plan, catalogue_requirements, instance_with_capabilities,
};
use crate::{PiPreparedIntegration, PiRpcDriver};
use swallowtail_core::{
    Capability, CapabilityProfile, CapabilityRequirement, ModelCatalogEntry, PreflightPlan,
};
use swallowtail_runtime::{
    BoxFuture, HostServices, ModelCatalogDriver, ModelCatalogRequest, PreparationFailure,
    RuntimeFailure,
};

#[derive(Clone, Debug, Eq, PartialEq)]
/// Prepared Pi model-catalogue request.
pub struct PiPreparedCatalogue {
    evidence: PiPreparedEvidence,
    request: ModelCatalogRequest,
}

impl PiPreparedCatalogue {
    /// Returns the catalogue's preparation evidence.
    #[must_use]
    pub const fn evidence(&self) -> &PiPreparedEvidence {
        &self.evidence
    }

    /// Returns the immutable preflight plan.
    #[must_use]
    pub const fn plan(&self) -> &PreflightPlan {
        self.evidence.plan()
    }

    /// Returns the model-catalogue request.
    #[must_use]
    pub const fn request(&self) -> &ModelCatalogRequest {
        &self.request
    }

    /// Reconstructs the low-level driver from prepared evidence.
    #[must_use]
    pub fn low_level_driver(&self) -> PiRpcDriver {
        self.evidence.low_level_driver()
    }

    /// Dispatches the prepared model-catalogue request.
    pub fn list_models(
        &self,
        services: HostServices,
    ) -> BoxFuture<'static, Result<Vec<ModelCatalogEntry>, RuntimeFailure>> {
        let driver = self.low_level_driver();
        let plan = self.plan().clone();
        let request = self.request.clone();
        Box::pin(async move { driver.list_models(plan, request, services).await })
    }

    /// Consumes the prepared catalogue into evidence, plan, and request.
    #[must_use]
    pub fn into_parts(self) -> (PiPreparedEvidence, PreflightPlan, ModelCatalogRequest) {
        let plan = self.evidence.plan().clone();
        (self.evidence, plan, self.request)
    }
}

impl PiPreparedIntegration {
    /// Validates and prepares a model-catalogue request.
    pub fn prepare_catalogue(
        &self,
        input: PiCatalogueProfileInput,
    ) -> Result<PiPreparedCatalogue, PreparationFailure> {
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
        Ok(PiPreparedCatalogue {
            evidence: PiPreparedEvidence::from_prepared(self, plan)?,
            request,
        })
    }
}

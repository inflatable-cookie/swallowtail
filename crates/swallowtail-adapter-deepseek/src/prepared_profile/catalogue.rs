use super::input::DeepSeekCatalogueProfileInput;
use super::plan::{
    DeepSeekPreparedEvidence, build_plan, catalogue_requirements, instance_with_capabilities,
};
use crate::{DeepSeekDirectDriver, DeepSeekPreparedIntegration};
use swallowtail_core::{
    Capability, CapabilityProfile, CapabilityRequirement, ModelCatalogEntry, PreflightPlan,
};
use swallowtail_runtime::{
    BoxFuture, HostServices, ModelCatalogDriver, ModelCatalogRequest, PreparationFailure,
    RuntimeFailure,
};

#[derive(Clone, Debug, Eq, PartialEq)]
/// Bound prepared DeepSeek catalogue operation.
pub struct DeepSeekPreparedCatalogue {
    evidence: DeepSeekPreparedEvidence,
    request: ModelCatalogRequest,
}

impl DeepSeekPreparedCatalogue {
    #[must_use]
    /// Returns the prepared operation evidence.
    pub const fn evidence(&self) -> &DeepSeekPreparedEvidence {
        &self.evidence
    }

    #[must_use]
    /// Returns the immutable catalogue preflight plan.
    pub const fn plan(&self) -> &PreflightPlan {
        self.evidence.plan()
    }

    #[must_use]
    /// Returns the plan-derived catalogue request.
    pub const fn request(&self) -> &ModelCatalogRequest {
        &self.request
    }

    #[must_use]
    /// Returns the low-level direct driver.
    pub fn low_level_driver(&self) -> DeepSeekDirectDriver {
        DeepSeekDirectDriver::new()
    }

    /// Executes the bound model-catalogue observation.
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
    /// Splits the prepared operation into evidence, plan, and request.
    pub fn into_parts(self) -> (DeepSeekPreparedEvidence, PreflightPlan, ModelCatalogRequest) {
        let plan = self.evidence.plan().clone();
        (self.evidence, plan, self.request)
    }
}

impl DeepSeekPreparedIntegration {
    /// Prepares a bound model-catalogue observation.
    pub fn prepare_catalogue(
        &self,
        input: DeepSeekCatalogueProfileInput,
    ) -> Result<DeepSeekPreparedCatalogue, PreparationFailure> {
        let capabilities =
            CapabilityProfile::new([CapabilityRequirement::new(Capability::ModelCatalog, [])]);
        let instance = instance_with_capabilities(self, capabilities.clone());
        let requirements = catalogue_requirements(
            self,
            capabilities.iter().map(|(capability, constraints)| {
                CapabilityRequirement::new(capability, constraints.iter().cloned())
            }),
        );
        let plan = build_plan(self, &instance, None, &requirements)?;
        let (request_id, deadline) = input.into_parts();
        let mut request = ModelCatalogRequest::new(request_id);
        if let Some(deadline) = deadline {
            request = request.with_deadline(deadline);
        }
        Ok(DeepSeekPreparedCatalogue {
            evidence: DeepSeekPreparedEvidence::from_prepared(self, plan)?,
            request,
        })
    }
}

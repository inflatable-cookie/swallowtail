use super::input::AnthropicCatalogueProfileInput;
use super::plan::{
    AnthropicPreparedEvidence, build_plan, instance_with_capabilities, requirements,
};
use crate::{AnthropicDirectDriver, AnthropicPreparedIntegration};
use swallowtail_core::{
    Capability, CapabilityProfile, CapabilityRequirement, DriverRole, ModelCatalogEntry,
    PreflightPlan,
};
use swallowtail_runtime::{
    BoxFuture, HostServices, ModelCatalogDriver, ModelCatalogRequest, PreparationFailure,
    RuntimeFailure,
};

#[derive(Clone, Debug, Eq, PartialEq)]
/// Bound prepared Anthropic Messages catalogue operation.
pub struct AnthropicPreparedCatalogue {
    evidence: AnthropicPreparedEvidence,
    request: ModelCatalogRequest,
}

impl AnthropicPreparedCatalogue {
    #[must_use]
    /// Returns the prepared operation evidence.
    pub const fn evidence(&self) -> &AnthropicPreparedEvidence {
        &self.evidence
    }

    #[must_use]
    /// Returns the immutable catalogue plan.
    pub const fn plan(&self) -> &PreflightPlan {
        self.evidence.plan()
    }

    #[must_use]
    /// Returns the plan-derived catalogue request.
    pub const fn request(&self) -> &ModelCatalogRequest {
        &self.request
    }

    #[must_use]
    /// Returns the low-level Messages driver.
    pub fn low_level_driver(&self) -> AnthropicDirectDriver {
        AnthropicDirectDriver::new()
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
    pub fn into_parts(
        self,
    ) -> (
        AnthropicPreparedEvidence,
        PreflightPlan,
        ModelCatalogRequest,
    ) {
        let plan = self.evidence.plan().clone();
        (self.evidence, plan, self.request)
    }
}

impl AnthropicPreparedIntegration {
    /// Prepares a bound model-catalogue observation.
    pub fn prepare_catalogue(
        &self,
        input: AnthropicCatalogueProfileInput,
    ) -> Result<AnthropicPreparedCatalogue, PreparationFailure> {
        let capabilities =
            CapabilityProfile::new([CapabilityRequirement::new(Capability::ModelCatalog, [])]);
        let instance = instance_with_capabilities(self, capabilities.clone());
        let requirements = requirements(
            self,
            DriverRole::ModelCatalog,
            capabilities.iter().map(|(capability, constraints)| {
                CapabilityRequirement::new(capability, constraints.iter().cloned())
            }),
            [],
        );
        let plan = build_plan(self, &instance, None, &requirements)?;
        let (request_id, deadline) = input.into_parts();
        let mut request = ModelCatalogRequest::new(request_id);
        if let Some(deadline) = deadline {
            request = request.with_deadline(deadline);
        }
        Ok(AnthropicPreparedCatalogue {
            evidence: AnthropicPreparedEvidence::from_prepared(self, plan)?,
            request,
        })
    }
}

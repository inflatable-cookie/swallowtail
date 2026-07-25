use super::input::KimiPlatformCatalogueProfileInput;
use super::plan::{
    KimiPlatformPreparedEvidence, build_plan, instance_with_capabilities, requirements,
};
use crate::{KimiPlatformDirectDriver, KimiPlatformPreparedIntegration};
use swallowtail_core::{
    Capability, CapabilityProfile, CapabilityRequirement, DriverRole, ModelCatalogEntry,
    PreflightPlan,
};
use swallowtail_runtime::{
    BoxFuture, HostServices, ModelCatalogDriver, ModelCatalogRequest, PreparationFailure,
    RuntimeFailure,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct KimiPlatformPreparedCatalogue {
    evidence: KimiPlatformPreparedEvidence,
    request: ModelCatalogRequest,
}

impl KimiPlatformPreparedCatalogue {
    #[must_use]
    pub const fn evidence(&self) -> &KimiPlatformPreparedEvidence {
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
    pub fn low_level_driver(&self) -> KimiPlatformDirectDriver {
        KimiPlatformDirectDriver::new()
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
    pub fn into_parts(
        self,
    ) -> (
        KimiPlatformPreparedEvidence,
        PreflightPlan,
        ModelCatalogRequest,
    ) {
        let plan = self.evidence.plan().clone();
        (self.evidence, plan, self.request)
    }
}

impl KimiPlatformPreparedIntegration {
    pub fn prepare_catalogue(
        &self,
        input: KimiPlatformCatalogueProfileInput,
    ) -> Result<KimiPlatformPreparedCatalogue, PreparationFailure> {
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
        let mut request = ModelCatalogRequest::new(request_id);
        if let Some(deadline) = deadline {
            request = request.with_deadline(deadline);
        }
        Ok(KimiPlatformPreparedCatalogue {
            evidence: KimiPlatformPreparedEvidence::from_prepared(self, plan)?,
            request,
        })
    }
}

use super::input::OllamaInventoryProfileInput;
use super::plan::{
    OllamaPreparedEvidence, build_plan, instance_with_capabilities, model_route, requirements,
};
use crate::{OllamaNativeAttachedDriver, OllamaPreparedIntegration};
use swallowtail_core::{
    AttachedModelObservation, AttachedModelObservationScope, Capability, CapabilityProfile,
    CapabilityRequirement, DriverRole, ModelCatalogEntry, PreflightPlan,
};
use swallowtail_runtime::{
    BoxFuture, HostServices, ModelCatalogDriver, ModelCatalogRequest, PreparationFailure,
    RuntimeFailure,
};

#[derive(Clone, Debug, Eq, PartialEq)]
/// Fresh model-catalogue entries projected from one runtime inventory probe.
pub struct OllamaInventorySnapshot {
    entries: Vec<ModelCatalogEntry>,
}

impl OllamaInventorySnapshot {
    fn new(entries: Vec<ModelCatalogEntry>) -> Self {
        Self { entries }
    }

    /// Iterates all projected model-catalogue entries.
    pub fn entries(&self) -> impl ExactSizeIterator<Item = &ModelCatalogEntry> {
        self.entries.iter()
    }

    /// Iterates installed-model observations.
    pub fn installed(&self) -> impl Iterator<Item = &AttachedModelObservation> {
        self.observations(AttachedModelObservationScope::InstalledInventory)
    }

    /// Iterates currently running-model observations.
    pub fn running(&self) -> impl Iterator<Item = &AttachedModelObservation> {
        self.observations(AttachedModelObservationScope::RunningInventory)
    }

    /// Returns selected-model detail when present in the snapshot.
    pub fn selected_detail(&self) -> Option<&AttachedModelObservation> {
        self.observations(AttachedModelObservationScope::SelectedModelDetail)
            .next()
    }

    fn observations(
        &self,
        scope: AttachedModelObservationScope,
    ) -> impl Iterator<Item = &AttachedModelObservation> {
        self.entries
            .iter()
            .flat_map(|entry| entry.metadata().attached_model_observations())
            .filter(move |observation| observation.scope() == scope)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
/// Prepared request for a fresh attached-runtime inventory observation.
pub struct OllamaPreparedInventory {
    evidence: OllamaPreparedEvidence,
    request: ModelCatalogRequest,
}

impl OllamaPreparedInventory {
    /// Returns the operation's preparation evidence.
    #[must_use]
    pub const fn evidence(&self) -> &OllamaPreparedEvidence {
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

    /// Creates the stateless low-level native HTTP driver.
    #[must_use]
    pub fn low_level_driver(&self) -> OllamaNativeAttachedDriver {
        OllamaNativeAttachedDriver::new()
    }

    /// Observes current inventory and returns a fresh snapshot.
    pub fn observe_inventory(
        &self,
        services: HostServices,
    ) -> BoxFuture<'static, Result<OllamaInventorySnapshot, RuntimeFailure>> {
        let driver = self.low_level_driver();
        let plan = self.plan().clone();
        let request = self.request.clone();
        Box::pin(async move {
            driver
                .list_models(plan, request, services)
                .await
                .map(OllamaInventorySnapshot::new)
        })
    }

    /// Consumes the inventory operation into evidence, plan, and request.
    #[must_use]
    pub fn into_parts(self) -> (OllamaPreparedEvidence, PreflightPlan, ModelCatalogRequest) {
        let plan = self.evidence.plan().clone();
        (self.evidence, plan, self.request)
    }
}

impl OllamaPreparedIntegration {
    /// Validates and prepares a fresh runtime inventory observation.
    pub fn prepare_inventory(
        &self,
        input: OllamaInventoryProfileInput,
    ) -> Result<OllamaPreparedInventory, PreparationFailure> {
        let capability_requirements =
            vec![CapabilityRequirement::new(Capability::ModelCatalog, [])];
        let capabilities = CapabilityProfile::new(capability_requirements.clone());
        let instance = instance_with_capabilities(self, capabilities.clone());
        let (request_id, deadline) = input.into_parts();
        let route = model_route(self, self.model_selection().clone(), capabilities);
        let requirements = requirements(
            self,
            &route,
            swallowtail_core::OperationShape::StructuredRun,
            DriverRole::ModelCatalog,
            capability_requirements,
        );
        let plan = build_plan(self, &instance, &route, &requirements)?;
        let mut request = ModelCatalogRequest::new(request_id);
        if let Some(deadline) = deadline {
            request = request.with_deadline(deadline);
        }
        Ok(OllamaPreparedInventory {
            evidence: OllamaPreparedEvidence::from_prepared(self, plan, None)?,
            request,
        })
    }
}

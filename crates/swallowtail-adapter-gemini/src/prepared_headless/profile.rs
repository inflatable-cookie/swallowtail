#[path = "profile/plan.rs"]
mod plan;

use super::GeminiHeadlessPreparedIntegration;
use super::instance::run_capabilities;
use plan::{build_plan, instance_with_capabilities, requirements};
use swallowtail_core::{
    CapabilityRequirement, HarnessConfigurationPosture, HarnessIsolation, ModelId, ModelRoute,
    ModelRouteId, ModelRouteRevision, PreflightPlan, ProviderId,
};
use swallowtail_runtime::{
    BoxFuture, Deadline, HostServices, OperationContent, OperationPolicy, PreparationFailure,
    PreparedOperationEvidence, ProviderRetentionPolicy, RequestId, RunHandle, RuntimeFailure,
    StructuredRunDriver, StructuredRunRequest, WorkingResourceRef,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GeminiHeadlessModelSelection {
    route_id: ModelRouteId,
    route_revision: ModelRouteRevision,
    provider_id: ProviderId,
    model_id: ModelId,
}

impl GeminiHeadlessModelSelection {
    #[must_use]
    pub const fn new(
        route_id: ModelRouteId,
        route_revision: ModelRouteRevision,
        provider_id: ProviderId,
        model_id: ModelId,
    ) -> Self {
        Self {
            route_id,
            route_revision,
            provider_id,
            model_id,
        }
    }

    fn into_parts(self) -> (ModelRouteId, ModelRouteRevision, ProviderId, ModelId) {
        (
            self.route_id,
            self.route_revision,
            self.provider_id,
            self.model_id,
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GeminiHeadlessRunProfileInput {
    request_id: RequestId,
    model: GeminiHeadlessModelSelection,
    content: OperationContent,
    working_resource: WorkingResourceRef,
    deadline: Deadline,
}

impl GeminiHeadlessRunProfileInput {
    #[must_use]
    pub const fn new(
        request_id: RequestId,
        model: GeminiHeadlessModelSelection,
        content: OperationContent,
        working_resource: WorkingResourceRef,
        deadline: Deadline,
    ) -> Self {
        Self {
            request_id,
            model,
            content,
            working_resource,
            deadline,
        }
    }

    fn into_parts(
        self,
    ) -> (
        RequestId,
        GeminiHeadlessModelSelection,
        OperationContent,
        WorkingResourceRef,
        Deadline,
    ) {
        (
            self.request_id,
            self.model,
            self.content,
            self.working_resource,
            self.deadline,
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GeminiHeadlessPreparedEvidence {
    observation: swallowtail_core::InstalledExecutableObservation,
    environment: swallowtail_runtime::EnvironmentRef,
    credential: swallowtail_core::CredentialRef,
    operation: PreparedOperationEvidence,
}

impl GeminiHeadlessPreparedEvidence {
    fn from_prepared(
        prepared: &GeminiHeadlessPreparedIntegration,
        plan: PreflightPlan,
    ) -> Result<Self, PreparationFailure> {
        Ok(Self {
            observation: prepared.observation().clone(),
            environment: prepared.environment().clone(),
            credential: prepared
                .access_profile()
                .credential_reference()
                .expect("prepared Gemini headless access has one credential reference")
                .clone(),
            operation: PreparedOperationEvidence::from_plan(
                plan,
                prepared.access_evidence().clone(),
            )?,
        })
    }

    #[must_use]
    pub const fn observation(&self) -> &swallowtail_core::InstalledExecutableObservation {
        &self.observation
    }

    #[must_use]
    pub const fn access(&self) -> &swallowtail_runtime::PreparedAccessEvidence {
        self.operation.access()
    }

    #[must_use]
    pub const fn operation(&self) -> &PreparedOperationEvidence {
        &self.operation
    }

    #[must_use]
    pub const fn plan(&self) -> &PreflightPlan {
        self.operation.plan()
    }

    fn low_level_driver(&self) -> crate::GeminiHeadlessDriver {
        crate::GeminiHeadlessDriver::new(self.environment.clone(), self.credential.clone())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GeminiHeadlessPreparedRun {
    evidence: GeminiHeadlessPreparedEvidence,
    request: StructuredRunRequest,
}

impl GeminiHeadlessPreparedRun {
    #[must_use]
    pub const fn evidence(&self) -> &GeminiHeadlessPreparedEvidence {
        &self.evidence
    }

    #[must_use]
    pub const fn plan(&self) -> &PreflightPlan {
        self.evidence.plan()
    }

    #[must_use]
    pub const fn request(&self) -> &StructuredRunRequest {
        &self.request
    }

    #[must_use]
    pub fn low_level_driver(&self) -> crate::GeminiHeadlessDriver {
        self.evidence.low_level_driver()
    }

    pub fn start_run(
        &self,
        services: HostServices,
    ) -> BoxFuture<'static, Result<Box<dyn RunHandle>, RuntimeFailure>> {
        let driver = self.low_level_driver();
        let plan = self.plan().clone();
        let request = self.request.clone();
        Box::pin(async move { driver.start_run(plan, request, services).await })
    }

    #[must_use]
    pub fn into_parts(
        self,
    ) -> (
        GeminiHeadlessPreparedEvidence,
        PreflightPlan,
        StructuredRunRequest,
    ) {
        let plan = self.evidence.plan().clone();
        (self.evidence, plan, self.request)
    }
}

impl GeminiHeadlessPreparedIntegration {
    pub fn prepare_run(
        &self,
        input: GeminiHeadlessRunProfileInput,
    ) -> Result<GeminiHeadlessPreparedRun, PreparationFailure> {
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
        let requirements = requirements(
            self,
            run_capabilities().iter().map(|(capability, constraints)| {
                CapabilityRequirement::new(capability, constraints.iter().cloned())
            }),
        );
        let plan = build_plan(self, &instance, &route, &requirements)?;
        let policy = OperationPolicy::offline()
            .with_provider_retention(ProviderRetentionPolicy::DurableAllowed)
            .with_harness_isolation(HarnessIsolation::AmbientHost)
            .with_harness_configuration_posture(HarnessConfigurationPosture::Ambient);
        let request = StructuredRunRequest::new(request_id, content, policy)
            .with_working_resource(working_resource)
            .with_deadline(deadline);
        Ok(GeminiHeadlessPreparedRun {
            evidence: GeminiHeadlessPreparedEvidence::from_prepared(self, plan)?,
            request,
        })
    }
}

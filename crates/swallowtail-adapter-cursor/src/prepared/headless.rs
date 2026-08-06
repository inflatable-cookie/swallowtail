use super::CursorPreparedHeadlessIntegration;
use swallowtail_core::{
    CapabilityRequirement, DriverRole, ExecutionLayer, HarnessConfigurationPosture,
    HarnessIsolation, HostServiceKind, ModelId, ModelRoute, ModelRouteId, ModelRouteRevision,
    OperationRequirements, OperationShape, PreflightPlan, ProviderId, ResourceAccess,
};
use swallowtail_runtime::{
    BoxFuture, Deadline, HostServices, OperationContent, OperationPolicy, PreparationFailure,
    PreparedOperationEvidence, ProviderRetentionPolicy, RequestId, RunHandle, RuntimeFailure,
    StructuredRunDriver, StructuredRunRequest, WorkingResourceRef,
};

#[derive(Clone, Debug, Eq, PartialEq)]
/// Exact provider and model route for a Cursor headless run.
pub struct CursorHeadlessModelSelection {
    route_id: ModelRouteId,
    route_revision: ModelRouteRevision,
    provider_id: ProviderId,
    model_id: ModelId,
}

impl CursorHeadlessModelSelection {
    /// Creates an exact Cursor model selection.
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
}

#[derive(Clone, Debug, Eq, PartialEq)]
/// Consumer inputs for one prepared Cursor headless run.
pub struct CursorHeadlessRunProfileInput {
    request_id: RequestId,
    model: CursorHeadlessModelSelection,
    content: OperationContent,
    working_resource: WorkingResourceRef,
    resource_access: ResourceAccess,
    deadline: Deadline,
}

impl CursorHeadlessRunProfileInput {
    /// Creates a bounded Cursor headless-run profile.
    #[must_use]
    pub const fn new(
        request_id: RequestId,
        model: CursorHeadlessModelSelection,
        content: OperationContent,
        working_resource: WorkingResourceRef,
        resource_access: ResourceAccess,
        deadline: Deadline,
    ) -> Self {
        Self {
            request_id,
            model,
            content,
            working_resource,
            resource_access,
            deadline,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
/// Prepared one-shot Cursor stream-JSON run.
pub struct CursorPreparedHeadlessRun {
    evidence: PreparedOperationEvidence,
    request: StructuredRunRequest,
    environment: swallowtail_runtime::EnvironmentRef,
}

impl CursorPreparedHeadlessIntegration {
    /// Prepares a structured run from the admitted headless integration.
    pub fn prepare_run(
        &self,
        input: CursorHeadlessRunProfileInput,
    ) -> Result<CursorPreparedHeadlessRun, PreparationFailure> {
        let activity = super::activity::headless(self.observation())?;
        let capabilities = super::activity::with_activity(
            super::plan::headless_capabilities(input.resource_access),
            &activity,
        );
        let instance =
            super::plan::instance_with_capabilities(self.instance(), capabilities.clone());
        let model = input.model;
        let route = ModelRoute::new(
            model.route_id,
            model.route_revision,
            instance.id().clone(),
            model.model_id,
            capabilities.clone(),
        )
        .with_provider_id(model.provider_id);
        let requirements = OperationRequirements::new(
            ExecutionLayer::HarnessInteraction,
            OperationShape::StructuredRun,
            DriverRole::StructuredRun,
            self.instance().execution_host_id().clone(),
            super::plan::access_requirement(self.access_profile()),
        )
        .with_ownership_modes([self.instance().ownership()])
        .with_host_services([
            HostServiceKind::Task,
            HostServiceKind::Process,
            HostServiceKind::Time,
        ])
        .with_capabilities(capabilities.iter().map(|(capability, constraints)| {
            CapabilityRequirement::new(capability, constraints.iter().cloned())
        }))
        .with_interface_versions([self.observation().version().clone()])
        .with_harness_isolation(HarnessIsolation::AmbientHost)
        .with_harness_configuration_posture(HarnessConfigurationPosture::Ambient)
        .require_model_route();
        let plan = super::plan::build_plan(
            &crate::cursor_headless_descriptor(),
            &instance,
            self.access_profile(),
            self.access_evidence(),
            self.available_host_services(),
            &requirements,
            Some(&route),
        )?;
        let policy = OperationPolicy::offline()
            .with_provider_retention(ProviderRetentionPolicy::DurableAllowed)
            .with_harness_isolation(HarnessIsolation::AmbientHost)
            .with_harness_configuration_posture(HarnessConfigurationPosture::Ambient);
        let request = StructuredRunRequest::new(input.request_id, input.content, policy)
            .with_working_resource(input.working_resource)
            .with_deadline(input.deadline);
        Ok(CursorPreparedHeadlessRun {
            evidence: PreparedOperationEvidence::from_plan_with_activity_profile(
                plan,
                self.access_evidence().clone(),
                activity,
            )?,
            request,
            environment: self.environment().clone(),
        })
    }
}

impl CursorPreparedHeadlessRun {
    /// Returns portable evidence for the prepared run.
    #[must_use]
    pub const fn evidence(&self) -> &PreparedOperationEvidence {
        &self.evidence
    }

    /// Returns the validated preflight plan.
    #[must_use]
    pub const fn plan(&self) -> &PreflightPlan {
        self.evidence.plan()
    }

    /// Returns the bound structured-run request.
    #[must_use]
    pub const fn request(&self) -> &StructuredRunRequest {
        &self.request
    }

    /// Creates the low-level driver bound to this prepared run.
    #[must_use]
    pub fn low_level_driver(&self) -> crate::CursorHeadlessDriver {
        crate::CursorHeadlessDriver::new(self.environment.clone())
    }

    /// Starts the prepared run with caller-supplied host services.
    pub fn start_run(
        &self,
        services: HostServices,
    ) -> BoxFuture<'static, Result<Box<dyn RunHandle>, RuntimeFailure>> {
        let driver = self.low_level_driver();
        let plan = self.plan().clone();
        let request = self.request.clone();
        Box::pin(async move { driver.start_run(plan, request, services).await })
    }
}

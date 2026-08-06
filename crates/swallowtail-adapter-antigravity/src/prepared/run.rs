use super::AntigravityPreparedHeadlessIntegration;
use swallowtail_core::{
    CapabilityRequirement, DriverRole, ExecutionLayer, HarnessConfigurationPosture,
    HarnessIsolation, HostServiceKind, ModelId, ModelRoute, ModelRouteId, ModelRouteRevision,
    OperationRequirements, OperationShape, PreflightPlan, ProviderId, ReasoningMode,
    ResourceAccess,
};
use swallowtail_runtime::{
    BoxFuture, Deadline, HostServices, OperationContent, OperationPolicy, PreparationFailure,
    PreparationStage, PreparedOperationEvidence, ProviderRetentionPolicy, RequestId, RunHandle,
    RuntimeFailure, StructuredOutputDescriptor, StructuredRunDriver, StructuredRunRequest,
    WorkingResourceRef,
};

#[derive(Clone, Debug, Eq, PartialEq)]
/// Exact provider, model, and route selection for headless execution.
pub struct AntigravityHeadlessModelSelection {
    pub(super) route_id: ModelRouteId,
    pub(super) route_revision: ModelRouteRevision,
    pub(super) provider_id: ProviderId,
    pub(super) model_id: ModelId,
}

impl AntigravityHeadlessModelSelection {
    #[must_use]
    /// Creates one exact headless model-route selection.
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
/// Content, resource, isolation, deadline, and optional output controls for one run.
pub struct AntigravityHeadlessRunProfileInput {
    request_id: RequestId,
    model: AntigravityHeadlessModelSelection,
    content: OperationContent,
    working_resource: WorkingResourceRef,
    resource_access: ResourceAccess,
    isolation: HarnessIsolation,
    deadline: Deadline,
    effort: Option<ReasoningMode>,
    structured_output: Option<StructuredOutputDescriptor>,
}

impl AntigravityHeadlessRunProfileInput {
    #[must_use]
    #[allow(clippy::too_many_arguments)]
    /// Creates explicit headless-run input without reasoning or schema defaults.
    pub const fn new(
        request_id: RequestId,
        model: AntigravityHeadlessModelSelection,
        content: OperationContent,
        working_resource: WorkingResourceRef,
        resource_access: ResourceAccess,
        isolation: HarnessIsolation,
        deadline: Deadline,
    ) -> Self {
        Self {
            request_id,
            model,
            content,
            working_resource,
            resource_access,
            isolation,
            deadline,
            effort: None,
            structured_output: None,
        }
    }

    #[must_use]
    /// Selects a supported low, medium, or high reasoning effort.
    pub fn with_effort(mut self, effort: ReasoningMode) -> Self {
        self.effort = Some(effort);
        self
    }

    #[must_use]
    /// Requests provider-enforced structured output.
    pub fn with_structured_output(mut self, output: StructuredOutputDescriptor) -> Self {
        self.structured_output = Some(output);
        self
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
/// Executable headless run with exact plan, request, and environment agreement.
pub struct AntigravityPreparedHeadlessRun {
    evidence: PreparedOperationEvidence,
    request: StructuredRunRequest,
    environment: swallowtail_runtime::EnvironmentRef,
}

impl AntigravityPreparedHeadlessIntegration {
    /// Validates and prepares one stream-JSON headless run.
    pub fn prepare_run(
        &self,
        input: AntigravityHeadlessRunProfileInput,
    ) -> Result<AntigravityPreparedHeadlessRun, PreparationFailure> {
        if !matches!(
            input.isolation,
            HarnessIsolation::AmbientHost | HarnessIsolation::ProviderEnforced
        ) {
            return Err(super::failure(
                PreparationStage::Preflight,
                "swallowtail.antigravity.preparation.isolation_rejected",
                "Antigravity headless isolation must be ambient or provider-enforced",
            ));
        }
        if input
            .effort
            .as_ref()
            .is_some_and(|effort| !matches!(effort.as_str(), "low" | "medium" | "high"))
        {
            return Err(super::failure(
                PreparationStage::Preflight,
                "swallowtail.antigravity.preparation.effort_rejected",
                "Antigravity effort must be low, medium, or high",
            ));
        }
        let activity = super::activity::profile(self.observation())?;
        let capabilities = super::activity::with_activity(
            super::common::run_capabilities(
                input.resource_access,
                input.effort.as_ref(),
                input.structured_output.is_some(),
            ),
            &activity,
        );
        let instance =
            super::common::instance_with_capabilities(self.instance(), capabilities.clone());
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
            super::common::access_requirement(self.access_profile()),
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
        .with_harness_isolation(input.isolation)
        .with_harness_configuration_posture(HarnessConfigurationPosture::Ambient)
        .require_model_route();
        let plan = super::common::build_plan(
            &crate::antigravity_headless_descriptor(),
            &instance,
            self.access_profile(),
            self.access_evidence(),
            self.available_host_services(),
            &requirements,
            Some(&route),
        )?;
        let mut policy = OperationPolicy::offline()
            .with_provider_retention(ProviderRetentionPolicy::DurableAllowed)
            .with_harness_isolation(input.isolation)
            .with_harness_configuration_posture(HarnessConfigurationPosture::Ambient);
        if let Some(effort) = input.effort {
            policy = policy.with_reasoning_mode(effort);
        }
        let mut request = StructuredRunRequest::new(input.request_id, input.content, policy)
            .with_working_resource(input.working_resource)
            .with_deadline(input.deadline);
        if let Some(output) = input.structured_output {
            request = request.with_structured_output(output);
        }
        Ok(AntigravityPreparedHeadlessRun {
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

impl AntigravityPreparedHeadlessRun {
    #[must_use]
    /// Returns prepared operation and activity evidence.
    pub const fn evidence(&self) -> &PreparedOperationEvidence {
        &self.evidence
    }

    #[must_use]
    /// Returns the immutable run preflight plan.
    pub const fn plan(&self) -> &PreflightPlan {
        self.evidence.plan()
    }

    #[must_use]
    /// Returns the plan-derived structured-run request.
    pub const fn request(&self) -> &StructuredRunRequest {
        &self.request
    }

    /// Starts the single prepared headless run.
    pub fn start_run(
        &self,
        services: HostServices,
    ) -> BoxFuture<'static, Result<Box<dyn RunHandle>, RuntimeFailure>> {
        let driver = crate::AntigravityHeadlessDriver::new(self.environment.clone());
        let plan = self.plan().clone();
        let request = self.request.clone();
        Box::pin(async move { driver.start_run(plan, request, services).await })
    }
}

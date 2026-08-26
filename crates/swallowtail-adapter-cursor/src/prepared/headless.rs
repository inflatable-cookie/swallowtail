use super::CursorPreparedHeadlessIntegration;
use crate::headless_command::CursorHeadlessReadMode;
use crate::headless_model_parameters::{
    CursorHeadlessContext, CursorHeadlessFast, CursorHeadlessModelParameters, render_model_id,
    validate_combination, validate_plain_model_id,
};
use swallowtail_core::{
    CapabilityRequirement, DriverRole, ExecutionLayer, HarnessConfigurationPosture,
    HarnessIsolation, HostServiceKind, ModelId, ModelRoute, ModelRouteId, ModelRouteRevision,
    OperationRequirements, OperationShape, PreflightPlan, ProviderId, ReasoningMode,
    ResourceAccess,
};
use swallowtail_runtime::{
    BoxFuture, Deadline, HostServices, OperationContent, OperationPolicy, PreparationFailure,
    PreparationStage, PreparedOperationEvidence, ProviderRetentionPolicy, RequestId, RunHandle,
    RuntimeFailure, StructuredRunDriver, StructuredRunRequest, WorkingResourceRef,
};

#[derive(Clone, Debug, Eq, PartialEq)]
/// Exact provider and model route for a Cursor headless run.
pub struct CursorHeadlessModelSelection {
    route_id: ModelRouteId,
    route_revision: ModelRouteRevision,
    provider_id: ProviderId,
    base_model_id: ModelId,
    parameters: CursorHeadlessModelParameters,
}

impl CursorHeadlessModelSelection {
    /// Creates an exact plain Cursor model selection without bracket parameters.
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
            base_model_id: model_id,
            parameters: CursorHeadlessModelParameters::empty(),
        }
    }

    /// Selects the qualified standard Fast variant for the current base model.
    pub fn with_fast(self, fast: CursorHeadlessFast) -> Result<Self, PreparationFailure> {
        let parameters = self
            .parameters
            .with_fast(self.base_model_id.as_str(), fast)?;
        Ok(Self { parameters, ..self })
    }

    /// Selects a qualified context-window parameter for the current base model.
    pub fn with_context(self, context: CursorHeadlessContext) -> Result<Self, PreparationFailure> {
        let parameters = self
            .parameters
            .with_context(self.base_model_id.as_str(), context)?;
        Ok(Self { parameters, ..self })
    }

    /// Selects qualified high reasoning effort for the current base model.
    pub fn with_effort(self, effort: ReasoningMode) -> Result<Self, PreparationFailure> {
        let parameters = self
            .parameters
            .with_effort(self.base_model_id.as_str(), effort)?;
        Ok(Self { parameters, ..self })
    }

    pub(crate) fn resolved_model_id(&self) -> Result<ModelId, PreparationFailure> {
        validate_plain_model_id(self.base_model_id.as_str())?;
        validate_combination(self.base_model_id.as_str(), &self.parameters)?;
        render_model_id(self.base_model_id.as_str(), &self.parameters)
    }

    pub(crate) const fn parameters(&self) -> &CursorHeadlessModelParameters {
        &self.parameters
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
    read_mode: Option<CursorHeadlessReadMode>,
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
            read_mode: None,
        }
    }

    /// Selects one exact Cursor-local read mode for this run.
    ///
    /// Omitting the selection keeps the exact current mapping: `Read`
    /// dispatches `--mode plan` and `ReadWrite` dispatches no mode. A
    /// selection requires `ResourceAccess::Read`, and
    /// [`CursorHeadlessReadMode::Ask`] additionally requires an exactly
    /// qualified Cursor release at preparation. Both modes are provider
    /// behavior only: neither grants or withholds working-resource,
    /// isolation, permission, tool, approval, or network authority, and the
    /// qualified stream does not report which mode was applied.
    pub fn with_read_mode(
        self,
        read_mode: CursorHeadlessReadMode,
    ) -> Result<Self, PreparationFailure> {
        if self.resource_access != ResourceAccess::Read {
            return Err(super::failure(
                PreparationStage::Preflight,
                "swallowtail.cursor.headless.read_mode_access_rejected",
                "Cursor headless read-mode selection requires read working-resource authority",
            ));
        }
        Ok(Self {
            read_mode: Some(read_mode),
            ..self
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
/// Prepared one-shot Cursor stream-JSON run.
pub struct CursorPreparedHeadlessRun {
    evidence: PreparedOperationEvidence,
    request: StructuredRunRequest,
    environment: swallowtail_runtime::EnvironmentRef,
    read_mode: Option<CursorHeadlessReadMode>,
}

impl CursorPreparedHeadlessIntegration {
    /// Prepares a structured run from the admitted headless integration.
    pub fn prepare_run(
        &self,
        input: CursorHeadlessRunProfileInput,
    ) -> Result<CursorPreparedHeadlessRun, PreparationFailure> {
        let activity = super::activity::headless(self.observation())?;
        if input.read_mode == Some(CursorHeadlessReadMode::Ask)
            && !crate::selection::headless_release_is_exactly_qualified(
                self.observation().version(),
            )
        {
            return Err(super::failure(
                PreparationStage::Preflight,
                "swallowtail.cursor.headless.ask_mode_unqualified",
                "Cursor headless Ask mode requires an exactly qualified Cursor release",
            ));
        }
        let read_mode = crate::headless_command::resolve(input.resource_access, input.read_mode);
        let model = input.model;
        let rendered_model_id = model.resolved_model_id()?;
        let effort = model.parameters().effort().cloned();
        let capabilities = super::activity::with_activity(
            super::plan::headless_capabilities(input.resource_access, model.parameters()),
            &activity,
        );
        let instance =
            super::plan::instance_with_capabilities(self.instance(), capabilities.clone());
        let route = ModelRoute::new(
            model.route_id,
            model.route_revision,
            instance.id().clone(),
            rendered_model_id,
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
        let mut policy = OperationPolicy::offline()
            .with_provider_retention(ProviderRetentionPolicy::DurableAllowed)
            .with_harness_isolation(HarnessIsolation::AmbientHost)
            .with_harness_configuration_posture(HarnessConfigurationPosture::Ambient);
        if let Some(effort) = effort {
            policy = policy.with_reasoning_mode(effort);
        }
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
            read_mode,
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

    /// Returns the exact Cursor read mode this run dispatches, if any.
    ///
    /// `Read` runs report the selected or default mode; `ReadWrite` runs
    /// report `None` and dispatch no `--mode` argument. The value is fixed at
    /// preparation and cannot drift afterwards. It records requested and
    /// dispatched intent only, never applied or effective provider behavior.
    #[must_use]
    pub const fn read_mode(&self) -> Option<CursorHeadlessReadMode> {
        self.read_mode
    }

    /// Creates the low-level driver bound to this prepared run.
    #[must_use]
    pub fn low_level_driver(&self) -> crate::CursorHeadlessDriver {
        let driver = crate::CursorHeadlessDriver::new(self.environment.clone());
        match self.read_mode {
            Some(read_mode) => driver.with_read_mode(read_mode),
            None => driver,
        }
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

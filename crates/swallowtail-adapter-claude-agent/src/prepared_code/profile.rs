#[path = "profile/plan.rs"]
mod plan;

use super::ClaudeCodePreparedIntegration;
use super::instance::{REASONING_MODES, run_capabilities};
use plan::{build_plan, instance_with_capabilities, requirements};
use swallowtail_core::{
    CapabilityRequirement, HarnessConfigurationPosture, HarnessIsolation, ModelId, ModelRoute,
    ModelRouteId, ModelRouteRevision, PreflightPlan, ReasoningMode,
};
use swallowtail_runtime::{
    BoxFuture, Deadline, HostServices, OperationContent, OperationPolicy, PreparationFailure,
    PreparedOperationEvidence, ProviderRetentionPolicy, RequestId, RunHandle, RuntimeFailure,
    StructuredRunDriver, StructuredRunRequest, WorkingResourceRef,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ClaudeCodeModelSelection {
    route_id: ModelRouteId,
    route_revision: ModelRouteRevision,
    model_id: ModelId,
}

impl ClaudeCodeModelSelection {
    #[must_use]
    pub const fn new(
        route_id: ModelRouteId,
        route_revision: ModelRouteRevision,
        model_id: ModelId,
    ) -> Self {
        Self {
            route_id,
            route_revision,
            model_id,
        }
    }

    fn into_parts(self) -> (ModelRouteId, ModelRouteRevision, ModelId) {
        (self.route_id, self.route_revision, self.model_id)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ClaudeCodeRunProfileInput {
    request_id: RequestId,
    model: ClaudeCodeModelSelection,
    content: OperationContent,
    working_resource: WorkingResourceRef,
    deadline: Deadline,
    reasoning_mode: Option<ReasoningMode>,
}

impl ClaudeCodeRunProfileInput {
    #[must_use]
    pub const fn new(
        request_id: RequestId,
        model: ClaudeCodeModelSelection,
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
            reasoning_mode: None,
        }
    }

    #[must_use]
    pub fn with_reasoning_mode(mut self, reasoning_mode: ReasoningMode) -> Self {
        self.reasoning_mode = Some(reasoning_mode);
        self
    }

    fn into_parts(
        self,
    ) -> (
        RequestId,
        ClaudeCodeModelSelection,
        OperationContent,
        WorkingResourceRef,
        Deadline,
        Option<ReasoningMode>,
    ) {
        (
            self.request_id,
            self.model,
            self.content,
            self.working_resource,
            self.deadline,
            self.reasoning_mode,
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ClaudeCodePreparedEvidence {
    observation: swallowtail_core::InstalledExecutableObservation,
    environment: swallowtail_runtime::EnvironmentRef,
    operation: PreparedOperationEvidence,
}

impl ClaudeCodePreparedEvidence {
    fn from_prepared(
        prepared: &ClaudeCodePreparedIntegration,
        plan: PreflightPlan,
    ) -> Result<Self, PreparationFailure> {
        Ok(Self {
            observation: prepared.observation().clone(),
            environment: prepared.environment().clone(),
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

    fn low_level_driver(&self) -> crate::ClaudeCodeHeadlessDriver {
        crate::ClaudeCodeHeadlessDriver::new(self.environment.clone())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ClaudeCodePreparedRun {
    evidence: ClaudeCodePreparedEvidence,
    request: StructuredRunRequest,
}

impl ClaudeCodePreparedRun {
    #[must_use]
    pub const fn evidence(&self) -> &ClaudeCodePreparedEvidence {
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
    pub fn low_level_driver(&self) -> crate::ClaudeCodeHeadlessDriver {
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
        ClaudeCodePreparedEvidence,
        PreflightPlan,
        StructuredRunRequest,
    ) {
        let plan = self.evidence.plan().clone();
        (self.evidence, plan, self.request)
    }
}

impl ClaudeCodePreparedIntegration {
    pub fn prepare_run(
        &self,
        input: ClaudeCodeRunProfileInput,
    ) -> Result<ClaudeCodePreparedRun, PreparationFailure> {
        let (request_id, model, content, working_resource, deadline, reasoning) =
            input.into_parts();
        if reasoning
            .as_ref()
            .is_some_and(|mode| !REASONING_MODES.contains(&mode.as_str()))
        {
            return Err(plan::failure(
                "swallowtail.claude_code.headless.preparation.reasoning_mode_unsupported",
                "Claude Code prepared run reasoning mode is unsupported",
            ));
        }
        let capabilities = run_capabilities();
        let instance = instance_with_capabilities(self, capabilities.clone());
        let operation_capabilities = operation_capabilities(&capabilities, reasoning.as_ref());
        let requirements = requirements(self, operation_capabilities);
        let (route_id, route_revision, model_id) = model.into_parts();
        let route = ModelRoute::new(
            route_id,
            route_revision,
            self.instance().id().clone(),
            model_id,
            capabilities,
        );
        let plan = build_plan(self, &instance, &route, &requirements)?;
        let mut policy = OperationPolicy::offline()
            .with_provider_retention(ProviderRetentionPolicy::Prohibited)
            .with_harness_isolation(HarnessIsolation::AmbientHost)
            .with_harness_configuration_posture(HarnessConfigurationPosture::Ambient);
        if let Some(reasoning) = reasoning {
            policy = policy.with_reasoning_mode(reasoning);
        }
        let request = StructuredRunRequest::new(request_id, content, policy)
            .with_working_resource(working_resource)
            .with_deadline(deadline);
        Ok(ClaudeCodePreparedRun {
            evidence: ClaudeCodePreparedEvidence::from_prepared(self, plan)?,
            request,
        })
    }
}

fn operation_capabilities(
    available: &swallowtail_core::CapabilityProfile,
    reasoning: Option<&ReasoningMode>,
) -> Vec<CapabilityRequirement> {
    let mut capabilities = available
        .iter()
        .filter(|(capability, _)| *capability != swallowtail_core::Capability::ReasoningSelection)
        .map(|(capability, constraints)| {
            CapabilityRequirement::new(capability, constraints.iter().cloned())
        })
        .collect::<Vec<_>>();
    if let Some(mode) = reasoning {
        capabilities.push(CapabilityRequirement::new(
            swallowtail_core::Capability::ReasoningSelection,
            [swallowtail_core::CapabilityConstraint::ReasoningMode(
                mode.clone(),
            )],
        ));
    }
    capabilities
}

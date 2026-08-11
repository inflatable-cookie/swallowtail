#[path = "profile/plan.rs"]
mod plan;

use super::ClaudeCodeResponsePreparedIntegration;
use super::instance::{REASONING_MODES, run_capabilities};
use crate::claude_code_response_activity::{activity_profile, with_activity};
use plan::{build_plan, instance_with_capabilities, requirements};
use swallowtail_core::{
    CapabilityRequirement, HarnessConfigurationPosture, HarnessIsolation, ModelId, ModelRoute,
    ModelRouteId, ModelRouteRevision, PreflightPlan, ReasoningMode,
};
use swallowtail_runtime::{
    BoxFuture, Deadline, HostServices, OperationContent, OperationPolicy, PreparationFailure,
    PreparedOperationEvidence, ProviderRetentionPolicy, RequestId, RunHandle, RuntimeFailure,
    StructuredRunDriver, StructuredRunRequest,
};

#[derive(Clone, Debug, Eq, PartialEq)]
/// Exact model route for a Claude Code response-only run.
pub struct ClaudeCodeResponseModelSelection {
    route_id: ModelRouteId,
    route_revision: ModelRouteRevision,
    model_id: ModelId,
}

impl ClaudeCodeResponseModelSelection {
    /// Creates an exact response-only model selection.
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
/// Consumer inputs for one bounded tool-free Claude Code text response.
pub struct ClaudeCodeResponseProfileInput {
    request_id: RequestId,
    model: ClaudeCodeResponseModelSelection,
    content: OperationContent,
    deadline: Deadline,
    reasoning_mode: Option<ReasoningMode>,
}

impl ClaudeCodeResponseProfileInput {
    /// Creates a bounded response-only run profile.
    #[must_use]
    pub const fn new(
        request_id: RequestId,
        model: ClaudeCodeResponseModelSelection,
        content: OperationContent,
        deadline: Deadline,
    ) -> Self {
        Self {
            request_id,
            model,
            content,
            deadline,
            reasoning_mode: None,
        }
    }

    /// Selects a qualified reasoning mode.
    #[must_use]
    pub fn with_reasoning_mode(mut self, reasoning_mode: ReasoningMode) -> Self {
        self.reasoning_mode = Some(reasoning_mode);
        self
    }

    fn into_parts(
        self,
    ) -> (
        RequestId,
        ClaudeCodeResponseModelSelection,
        OperationContent,
        Deadline,
        Option<ReasoningMode>,
    ) {
        (
            self.request_id,
            self.model,
            self.content,
            self.deadline,
            self.reasoning_mode,
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
/// Portable evidence for a prepared response-only run.
pub struct ClaudeCodeResponsePreparedEvidence {
    observation: swallowtail_core::InstalledExecutableObservation,
    environment: swallowtail_runtime::EnvironmentRef,
    operation: PreparedOperationEvidence,
}

impl ClaudeCodeResponsePreparedEvidence {
    fn from_prepared(
        prepared: &ClaudeCodeResponsePreparedIntegration,
        plan: PreflightPlan,
        activity: swallowtail_core::ObservableActivityProfile,
    ) -> Result<Self, PreparationFailure> {
        Ok(Self {
            observation: prepared.observation().clone(),
            environment: prepared.environment().clone(),
            operation: PreparedOperationEvidence::from_plan_with_activity_profile(
                plan,
                prepared.access_evidence().clone(),
                activity,
            )?,
        })
    }

    /// Returns the qualified installed-executable observation.
    #[must_use]
    pub const fn observation(&self) -> &swallowtail_core::InstalledExecutableObservation {
        &self.observation
    }

    /// Returns the prepared access evidence.
    #[must_use]
    pub const fn access(&self) -> &swallowtail_runtime::PreparedAccessEvidence {
        self.operation.access()
    }

    /// Returns the complete prepared-operation evidence.
    #[must_use]
    pub const fn operation(&self) -> &PreparedOperationEvidence {
        &self.operation
    }

    /// Returns the validated preflight plan.
    #[must_use]
    pub const fn plan(&self) -> &PreflightPlan {
        self.operation.plan()
    }

    fn low_level_driver(&self) -> crate::ClaudeCodeResponseOnlyDriver {
        crate::ClaudeCodeResponseOnlyDriver::new(self.environment.clone())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
/// Prepared one-shot tool-free Claude Code text response.
pub struct ClaudeCodeResponsePreparedRun {
    evidence: ClaudeCodeResponsePreparedEvidence,
    request: StructuredRunRequest,
}

impl ClaudeCodeResponsePreparedRun {
    /// Returns portable evidence for the prepared run.
    #[must_use]
    pub const fn evidence(&self) -> &ClaudeCodeResponsePreparedEvidence {
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

    /// Creates the low-level response-only driver bound to this run.
    #[must_use]
    pub fn low_level_driver(&self) -> crate::ClaudeCodeResponseOnlyDriver {
        self.evidence.low_level_driver()
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

    /// Splits the prepared run into evidence, plan, and request.
    #[must_use]
    pub fn into_parts(
        self,
    ) -> (
        ClaudeCodeResponsePreparedEvidence,
        PreflightPlan,
        StructuredRunRequest,
    ) {
        let plan = self.evidence.plan().clone();
        (self.evidence, plan, self.request)
    }
}

impl ClaudeCodeResponsePreparedIntegration {
    /// Prepares one bounded tool-free Claude Code text response.
    pub fn prepare_run(
        &self,
        input: ClaudeCodeResponseProfileInput,
    ) -> Result<ClaudeCodeResponsePreparedRun, PreparationFailure> {
        let (request_id, model, content, deadline, reasoning) = input.into_parts();
        if reasoning
            .as_ref()
            .is_some_and(|mode| !REASONING_MODES.contains(&mode.as_str()))
        {
            return Err(plan::failure(
                "swallowtail.claude_code.response_only.preparation.reasoning_mode_unsupported",
                "Claude Code response-only reasoning mode is unsupported",
            ));
        }
        let activity = activity_profile(self)?;
        let capabilities = with_activity(run_capabilities(), &activity);
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
            .with_harness_configuration_posture(HarnessConfigurationPosture::ProviderSuppressed);
        if let Some(reasoning) = reasoning {
            policy = policy.with_reasoning_mode(reasoning);
        }
        let request =
            StructuredRunRequest::new(request_id, content, policy).with_deadline(deadline);
        Ok(ClaudeCodeResponsePreparedRun {
            evidence: ClaudeCodeResponsePreparedEvidence::from_prepared(self, plan, activity)?,
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

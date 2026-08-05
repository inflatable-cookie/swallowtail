use super::{AntigravityHeadlessModelSelection, AntigravityPreparedContinuationIntegration};
use swallowtail_core::{
    CapabilityRequirement, DriverRole, ExecutionLayer, HarnessConfigurationPosture,
    HarnessIsolation, HostServiceKind, ModelRoute, OperationRequirements, OperationShape,
    PreflightPlan, ResourceAccess, SessionAccessPolicy, SessionProviderStatePolicy,
};
use swallowtail_runtime::{
    BoxFuture, Deadline, HostServices, InteractiveSessionDriver, InteractiveSessionHandle,
    OpenSessionRequest, PreparationFailure, PreparedOperationEvidence,
    PreparedWorkingStateRestoration, RequestId, RuntimeFailure, RuntimeTurnId, WorkingResourceRef,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AntigravityContinuationProfileInput {
    request_id: RequestId,
    model: AntigravityHeadlessModelSelection,
    working_resource: WorkingResourceRef,
    deadline: Option<Deadline>,
}

impl AntigravityContinuationProfileInput {
    #[must_use]
    pub const fn new(
        request_id: RequestId,
        model: AntigravityHeadlessModelSelection,
        working_resource: WorkingResourceRef,
    ) -> Self {
        Self {
            request_id,
            model,
            working_resource,
            deadline: None,
        }
    }

    #[must_use]
    pub const fn with_deadline(mut self, deadline: Deadline) -> Self {
        self.deadline = Some(deadline);
        self
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AntigravityPreparedContinuation {
    evidence: PreparedOperationEvidence,
    request: OpenSessionRequest,
    environment: swallowtail_runtime::EnvironmentRef,
}

impl AntigravityPreparedContinuationIntegration {
    pub fn prepare_session(
        &self,
        input: AntigravityContinuationProfileInput,
    ) -> Result<AntigravityPreparedContinuation, PreparationFailure> {
        let activity = super::activity::profile(self.observation())?;
        let capabilities =
            super::activity::with_activity(super::common::continuation_capabilities(), &activity);
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
            OperationShape::InteractiveSession,
            DriverRole::InteractiveSession,
            self.instance().execution_host_id().clone(),
            super::common::access_requirement(self.access_profile()),
        )
        .with_ownership_modes([self.instance().ownership()])
        .with_host_services([
            HostServiceKind::Task,
            HostServiceKind::Process,
            HostServiceKind::Time,
            HostServiceKind::WorkingResource,
        ])
        .with_capabilities(capabilities.iter().map(|(capability, constraints)| {
            CapabilityRequirement::new(capability, constraints.iter().cloned())
        }))
        .with_interface_versions([self.observation().version().clone()])
        .with_harness_isolation(HarnessIsolation::AmbientHost)
        .with_harness_configuration_posture(HarnessConfigurationPosture::Ambient)
        .with_session_access_policy(SessionAccessPolicy::ambient_harness(ResourceAccess::Read))
        .with_session_provider_state_policy(
            SessionProviderStatePolicy::DurableProviderSessionPreserved,
        )
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
        let request = OpenSessionRequest::from_plan(
            &plan,
            input.request_id,
            input.working_resource,
            input.deadline,
        )?;
        Ok(AntigravityPreparedContinuation {
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

impl AntigravityPreparedContinuation {
    #[must_use]
    pub const fn evidence(&self) -> &PreparedOperationEvidence {
        &self.evidence
    }

    #[must_use]
    pub const fn plan(&self) -> &PreflightPlan {
        self.evidence.plan()
    }

    #[must_use]
    pub const fn request(&self) -> &OpenSessionRequest {
        &self.request
    }

    pub fn open_session(
        &self,
        services: HostServices,
    ) -> BoxFuture<'static, Result<Box<dyn InteractiveSessionHandle>, RuntimeFailure>> {
        let driver = crate::AntigravityHeadlessDriver::new(self.environment.clone());
        let plan = self.plan().clone();
        let request = self.request.clone();
        Box::pin(async move { driver.open_session(plan, request, services).await })
    }

    #[must_use]
    pub fn prepare_working_state_restoration(
        &self,
        interrupted_turn_id: RuntimeTurnId,
    ) -> PreparedWorkingStateRestoration {
        PreparedWorkingStateRestoration::fresh_session_replacement(
            interrupted_turn_id,
            crate::AntigravityHeadlessDriver::new(self.environment.clone()),
            self.plan().clone(),
            self.request.clone(),
        )
    }
}

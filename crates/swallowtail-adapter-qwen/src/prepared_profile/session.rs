use super::input::QwenSessionProfileInput;
use super::plan::{QwenPreparedEvidence, build_plan, instance_with_capabilities, requirements};
use crate::activity::profile::{activity_profile, with_activity};
use crate::{QwenHeadlessDriver, QwenPreparedIntegration};
use swallowtail_core::{
    CancellationScope, Capability, CapabilityConstraint, CapabilityProfile, CapabilityRequirement,
    DriverRole, ModelRoute, OperationShape, ResourceAccess, SessionProviderStatePolicy,
};
use swallowtail_runtime::{
    BoxFuture, HostServices, InteractiveSessionDriver, InteractiveSessionHandle,
    OpenSessionRequest, PreparationFailure, PreparedWorkingStateRestoration, RuntimeFailure,
    RuntimeTurnId, SessionAccessPolicy,
};

#[derive(Clone, Debug, Eq, PartialEq)]
/// Prepared turn-scoped Qwen interactive session.
pub struct QwenPreparedSession {
    evidence: QwenPreparedEvidence,
    request: OpenSessionRequest,
}

impl QwenPreparedSession {
    /// Returns portable evidence for the prepared session.
    #[must_use]
    pub const fn evidence(&self) -> &QwenPreparedEvidence {
        &self.evidence
    }

    /// Returns the validated preflight plan.
    #[must_use]
    pub const fn plan(&self) -> &swallowtail_core::PreflightPlan {
        self.evidence.plan()
    }

    /// Returns the bound session-open request.
    #[must_use]
    pub const fn request(&self) -> &OpenSessionRequest {
        &self.request
    }

    /// Creates the low-level driver bound to this session.
    #[must_use]
    pub fn low_level_driver(&self) -> QwenHeadlessDriver {
        self.evidence.low_level_driver()
    }

    /// Opens the prepared turn-scoped session with caller-supplied host services.
    pub fn open_session(
        &self,
        services: HostServices,
    ) -> BoxFuture<'static, Result<Box<dyn InteractiveSessionHandle>, RuntimeFailure>> {
        let driver = self.low_level_driver();
        let plan = self.plan().clone();
        let request = self.request.clone();
        Box::pin(async move { driver.open_session(plan, request, services).await })
    }

    /// Prepares context-losing recovery through a fresh session replacement.
    #[must_use]
    pub fn prepare_working_state_restoration(
        &self,
        interrupted_turn_id: RuntimeTurnId,
    ) -> PreparedWorkingStateRestoration {
        PreparedWorkingStateRestoration::fresh_session_replacement(
            interrupted_turn_id,
            self.low_level_driver(),
            self.plan().clone(),
            self.request.clone(),
        )
    }

    /// Splits the prepared session into evidence, plan, and request.
    #[must_use]
    pub fn into_parts(
        self,
    ) -> (
        QwenPreparedEvidence,
        swallowtail_core::PreflightPlan,
        OpenSessionRequest,
    ) {
        let plan = self.evidence.plan().clone();
        (self.evidence, plan, self.request)
    }
}

impl QwenPreparedIntegration {
    /// Prepares a turn-scoped interactive session from the admitted integration.
    pub fn prepare_session(
        &self,
        input: QwenSessionProfileInput,
    ) -> Result<QwenPreparedSession, PreparationFailure> {
        let (request_id, model, working_resource, deadline, reasoning, budgets) =
            input.into_parts();
        let activity = activity_profile(self)?;
        let (route_id, route_revision, provider_id, model_id) = model.into_parts();
        if let Some(reasoning) = reasoning.as_ref() {
            crate::reasoning::validate_preparation(
                self.observation().version().version(),
                &provider_id,
                &model_id,
                reasoning,
            )?;
        }
        crate::budgets::validate_preparation(self.observation().version().version(), budgets)?;
        let mut session_capabilities = session_capabilities();
        if let Some(reasoning) = reasoning.as_ref() {
            session_capabilities.push(CapabilityRequirement::new(
                Capability::ReasoningSelection,
                [CapabilityConstraint::ReasoningMode(reasoning.clone())],
            ));
        }
        let capabilities = with_activity(CapabilityProfile::new(session_capabilities), &activity);
        let capability_requirements = capabilities
            .iter()
            .map(|(capability, constraints)| {
                CapabilityRequirement::new(capability, constraints.iter().cloned())
            })
            .collect::<Vec<_>>();
        let instance = instance_with_capabilities(self, capabilities.clone());
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
            OperationShape::InteractiveSession,
            DriverRole::InteractiveSession,
            capability_requirements,
        )
        .with_session_access_policy(SessionAccessPolicy::ambient_harness(ResourceAccess::Read))
        .with_session_provider_state_policy(
            SessionProviderStatePolicy::DurableProviderSessionPreserved,
        );
        let plan = build_plan(self, &instance, &route, &requirements)?;
        let mut options = swallowtail_runtime::SessionOptions::default();
        if let Some(reasoning) = reasoning.clone() {
            options = options.with_reasoning_mode(reasoning);
        }
        let request = OpenSessionRequest::from_plan(&plan, request_id, working_resource, deadline)?
            .with_options(options);
        Ok(QwenPreparedSession {
            evidence: QwenPreparedEvidence::from_prepared(
                self, plan, activity, reasoning, budgets,
            )?,
            request,
        })
    }
}

fn session_capabilities() -> Vec<CapabilityRequirement> {
    vec![
        CapabilityRequirement::new(
            Capability::InteractiveSession,
            [CapabilityConstraint::MaximumTurns(24)],
        ),
        CapabilityRequirement::new(
            Capability::StreamingEvents,
            [CapabilityConstraint::StreamRecordMaximumCount(4096)],
        ),
        CapabilityRequirement::new(Capability::UsageReporting, []),
        CapabilityRequirement::new(
            Capability::Interruption,
            [CapabilityConstraint::CancellationScope(
                CancellationScope::ActiveTurn,
            )],
        ),
        CapabilityRequirement::new(Capability::ProviderDurableRetention, []),
    ]
}

use super::input::OllamaSessionProfileInput;
use super::plan::{
    OllamaPreparedEvidence, bind_low_level_driver, build_plan, instance_with_capabilities,
    model_route, requirements,
};
use crate::{OllamaNativeAttachedDriver, OllamaPreparedIntegration};
use swallowtail_core::{
    CancellationScope, Capability, CapabilityConstraint, CapabilityProfile, CapabilityRequirement,
    DriverRole, OperationShape, SessionProviderStatePolicy,
};
use swallowtail_runtime::{
    BoxFuture, HostServices, InteractiveSessionDriver, InteractiveSessionHandle,
    OpenSessionRequest, PreparationFailure, PreparationStage, PreparedWorkingStateRestoration,
    RuntimeFailure, RuntimeTurnId, SessionAccessPolicy,
};

#[derive(Clone, Debug, Eq, PartialEq)]
/// Prepared resource-free Ollama session ready for explicit opening.
pub struct OllamaPreparedSession {
    evidence: OllamaPreparedEvidence,
    request: OpenSessionRequest,
}

impl OllamaPreparedSession {
    /// Returns the session's preparation evidence.
    #[must_use]
    pub const fn evidence(&self) -> &OllamaPreparedEvidence {
        &self.evidence
    }

    /// Returns the immutable preflight plan.
    #[must_use]
    pub const fn plan(&self) -> &swallowtail_core::PreflightPlan {
        self.evidence.plan()
    }

    /// Returns the open-session request.
    #[must_use]
    pub const fn request(&self) -> &OpenSessionRequest {
        &self.request
    }

    /// Creates the stateless low-level native HTTP driver.
    #[must_use]
    pub fn low_level_driver(&self) -> OllamaNativeAttachedDriver {
        bind_low_level_driver(&self.evidence)
    }

    /// Opens the prepared resource-free session.
    pub fn open_session(
        &self,
        services: HostServices,
    ) -> BoxFuture<'static, Result<Box<dyn InteractiveSessionHandle>, RuntimeFailure>> {
        let driver = self.low_level_driver();
        if let Err(error) =
            super::plan::validate_prepared_context_window_binding(&driver, &self.evidence)
        {
            return Box::pin(async move { Err(error) });
        }
        let plan = self.plan().clone();
        let request = self.request.clone();
        Box::pin(async move { driver.open_session(plan, request, services).await })
    }

    /// Prepares a fresh session after interruption, with private history lost.
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

    /// Consumes the prepared session into evidence, plan, and request.
    #[must_use]
    pub fn into_parts(
        self,
    ) -> (
        OllamaPreparedEvidence,
        swallowtail_core::PreflightPlan,
        OpenSessionRequest,
    ) {
        let plan = self.evidence.plan().clone();
        (self.evidence, plan, self.request)
    }
}

impl OllamaPreparedIntegration {
    /// Validates and prepares a resource-free interactive session.
    pub fn prepare_session(
        &self,
        input: OllamaSessionProfileInput,
    ) -> Result<OllamaPreparedSession, PreparationFailure> {
        let (request_id, context_window, deadline) = input.into_parts();
        let capability_requirements = session_capabilities();
        let activity = crate::activity::profile::activity_profile(self).map_err(|error| {
            PreparationFailure::new(
                PreparationStage::Preflight,
                swallowtail_core::Diagnostic::new(error.diagnostic().clone()),
            )
        })?;
        let capabilities = crate::activity::profile::with_activity(
            CapabilityProfile::new(capability_requirements),
            &activity,
        );
        let capability_requirements = capabilities
            .iter()
            .map(|(capability, constraints)| {
                CapabilityRequirement::new(capability, constraints.iter().cloned())
            })
            .collect::<Vec<_>>();
        let instance = instance_with_capabilities(self, capabilities.clone());
        let route = model_route(self, self.model_selection().clone(), capabilities);
        let requirements = requirements(
            self,
            &route,
            OperationShape::InteractiveSession,
            DriverRole::InteractiveSession,
            capability_requirements,
        )
        .with_session_access_policy(SessionAccessPolicy::resource_free())
        .with_session_provider_state_policy(SessionProviderStatePolicy::Prohibited);
        let plan = build_plan(self, &instance, &route, &requirements)?;
        let request = OpenSessionRequest::resource_free_from_plan(&plan, request_id, deadline)?;
        Ok(OllamaPreparedSession {
            evidence: OllamaPreparedEvidence::from_prepared_with_activity(
                self,
                plan,
                activity,
                context_window,
            )?,
            request,
        })
    }
}

fn session_capabilities() -> Vec<CapabilityRequirement> {
    vec![
        CapabilityRequirement::new(
            Capability::InteractiveSession,
            [
                CapabilityConstraint::MaximumTurns(24),
                CapabilityConstraint::PrivateHistoryMaximumBytes(1_048_576),
            ],
        ),
        CapabilityRequirement::new(
            Capability::StreamingEvents,
            [CapabilityConstraint::StreamRecordMaximumCount(4096)],
        ),
        CapabilityRequirement::new(Capability::UsageReporting, []),
        CapabilityRequirement::new(
            Capability::OutputTokenLimit,
            [CapabilityConstraint::OutputTokenMaximum(8)],
        ),
        CapabilityRequirement::new(
            Capability::Interruption,
            [CapabilityConstraint::CancellationScope(
                CancellationScope::ActiveTurn,
            )],
        ),
    ]
}

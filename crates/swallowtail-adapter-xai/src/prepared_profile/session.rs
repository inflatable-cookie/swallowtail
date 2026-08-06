use super::input::XaiSessionProfileInput;
use super::plan::{XaiPreparedEvidence, build_plan, instance_with_capabilities, model_route};
use crate::{XaiPreparedIntegration, XaiWebSocketDriver};
use swallowtail_core::PreflightPlan;
use swallowtail_core::{CapabilityProfile, CapabilityRequirement};
use swallowtail_runtime::{
    BoxFuture, HostServices, InteractiveSessionDriver, InteractiveSessionHandle,
    OpenSessionRequest, PreparationFailure, PreparedWorkingStateRestoration, RuntimeFailure,
    RuntimeTurnId,
};

#[derive(Clone, Debug, Eq, PartialEq)]
/// Bound prepared serial xAI Responses WebSocket session.
pub struct XaiPreparedResponsesSession {
    evidence: XaiPreparedEvidence,
    request: OpenSessionRequest,
}

impl XaiPreparedResponsesSession {
    #[must_use]
    /// Returns the prepared operation evidence.
    pub const fn evidence(&self) -> &XaiPreparedEvidence {
        &self.evidence
    }

    #[must_use]
    /// Returns the immutable session plan.
    pub const fn plan(&self) -> &PreflightPlan {
        self.evidence.plan()
    }

    #[must_use]
    /// Returns the plan-derived session-open request.
    pub const fn request(&self) -> &OpenSessionRequest {
        &self.request
    }

    #[must_use]
    /// Returns the low-level WebSocket driver.
    pub fn low_level_driver(&self) -> XaiWebSocketDriver {
        XaiWebSocketDriver::new()
    }

    /// Opens the bound serial Responses session.
    pub fn open_session(
        &self,
        services: HostServices,
    ) -> BoxFuture<'static, Result<Box<dyn InteractiveSessionHandle>, RuntimeFailure>> {
        let driver = self.low_level_driver();
        let plan = self.plan().clone();
        let request = self.request.clone();
        Box::pin(async move { driver.open_session(plan, request, services).await })
    }

    #[must_use]
    /// Prepares fresh-session replacement after connection loss.
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

    #[must_use]
    /// Splits the prepared operation into evidence, plan, and request.
    pub fn into_parts(self) -> (XaiPreparedEvidence, PreflightPlan, OpenSessionRequest) {
        let plan = self.evidence.plan().clone();
        (self.evidence, plan, self.request)
    }
}

impl XaiPreparedIntegration {
    /// Prepares a serial resource-free Responses WebSocket session.
    pub fn prepare_responses_session(
        &self,
        input: XaiSessionProfileInput,
    ) -> Result<XaiPreparedResponsesSession, PreparationFailure> {
        let (request_id, model, deadline) = input.into_parts();
        let activity = crate::activity::profile::activity_profile();
        let base_requirements =
            crate::xai_responses_requirements(self.instance().execution_host_id().clone());
        let capabilities = crate::activity::profile::with_activity(
            CapabilityProfile::new(base_requirements.capabilities().cloned()),
            &activity,
        );
        let requirements = base_requirements.with_capabilities(capabilities.iter().map(
            |(capability, constraints)| {
                CapabilityRequirement::new(capability, constraints.iter().cloned())
            },
        ));
        let instance = instance_with_capabilities(self, capabilities.clone());
        let route = model_route(self, model, capabilities);
        let plan = build_plan(self, &instance, &route, &requirements)?;
        let request = OpenSessionRequest::resource_free_from_plan(&plan, request_id, deadline)?;
        Ok(XaiPreparedResponsesSession {
            evidence: XaiPreparedEvidence::from_prepared(self, plan, activity)?,
            request,
        })
    }
}

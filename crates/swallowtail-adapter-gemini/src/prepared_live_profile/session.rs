use super::input::GeminiLiveSessionProfileInput;
use super::plan::{GeminiLivePreparedEvidence, build_plan, model_route};
use crate::prepared_live::failure;
use crate::{GeminiLiveDriver, GeminiLivePreparedIntegration};
use swallowtail_core::PreflightPlan;
use swallowtail_runtime::{
    BoxFuture, HostServices, OpenRealtimeMediaSessionRequest, PreparationFailure, PreparationStage,
    PreparedWorkingStateRestoration, RealtimeMediaSessionDriver, RealtimeMediaSessionHandle,
    RuntimeFailure, RuntimeTurnId,
};

#[derive(Clone, Debug, Eq, PartialEq)]
/// Prepared Gemini Live media session ready for explicit opening.
pub struct GeminiPreparedLiveSession {
    evidence: GeminiLivePreparedEvidence,
    request: OpenRealtimeMediaSessionRequest,
}

impl GeminiPreparedLiveSession {
    /// Returns the session's preparation evidence.
    #[must_use]
    pub const fn evidence(&self) -> &GeminiLivePreparedEvidence {
        &self.evidence
    }

    /// Returns the immutable preflight plan.
    #[must_use]
    pub const fn plan(&self) -> &PreflightPlan {
        self.evidence.plan()
    }

    /// Returns the realtime open-session request.
    #[must_use]
    pub const fn request(&self) -> &OpenRealtimeMediaSessionRequest {
        &self.request
    }

    /// Creates the stateless low-level Live driver.
    #[must_use]
    pub fn low_level_driver(&self) -> GeminiLiveDriver {
        GeminiLiveDriver::new()
    }

    /// Opens the prepared media session using the supplied host services.
    pub fn open_session(
        &self,
        services: HostServices,
    ) -> BoxFuture<'static, Result<Box<dyn RealtimeMediaSessionHandle>, RuntimeFailure>> {
        let driver = self.low_level_driver();
        let plan = self.plan().clone();
        let request = self.request.clone();
        Box::pin(async move {
            driver
                .open_realtime_media_session(plan, request, services)
                .await
        })
    }

    /// Prepares a fresh media session after interruption, with prior connection state lost.
    #[must_use]
    pub fn prepare_working_state_restoration(
        &self,
        interrupted_turn_id: RuntimeTurnId,
    ) -> PreparedWorkingStateRestoration {
        PreparedWorkingStateRestoration::fresh_realtime_session_replacement(
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
        GeminiLivePreparedEvidence,
        PreflightPlan,
        OpenRealtimeMediaSessionRequest,
    ) {
        let plan = self.evidence.plan().clone();
        (self.evidence, plan, self.request)
    }
}

impl GeminiLivePreparedIntegration {
    /// Validates and prepares a Live session without opening the socket.
    pub fn prepare_live_session(
        &self,
        input: GeminiLiveSessionProfileInput,
    ) -> Result<GeminiPreparedLiveSession, PreparationFailure> {
        let (request_id, config, deadline, rollover) = input.into_parts();
        if config != crate::gemini_live_media_config() {
            return Err(failure(
                PreparationStage::Preflight,
                "swallowtail.gemini.live_preparation.config_rejected",
                "Gemini Live preparation requires the exact asymmetric manual PCM format",
            ));
        }
        if rollover != crate::gemini_live_rollover_policy() {
            return Err(failure(
                PreparationStage::Preflight,
                "swallowtail.gemini.live_preparation.rollover_rejected",
                "Gemini Live preparation requires exactly one planned connection rollover",
            ));
        }
        let route = model_route(self);
        let plan = build_plan(self, &route)?;
        let request = OpenRealtimeMediaSessionRequest::new(request_id, config, deadline)
            .with_planned_connection_rollover(rollover);
        Ok(GeminiPreparedLiveSession {
            evidence: GeminiLivePreparedEvidence::from_prepared(self, plan)?,
            request,
        })
    }
}

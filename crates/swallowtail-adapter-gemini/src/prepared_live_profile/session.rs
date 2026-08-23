use super::input::GeminiLiveSessionProfileInput;
use super::plan::{
    GeminiLivePreparedEvidence, build_plan, instance_with_capabilities, model_route,
};
use crate::prepared_live::failure;
use crate::{GEMINI_LIVE_MAX_OUTPUT_TOKENS, GeminiLiveDriver, GeminiLivePreparedIntegration};
use std::num::NonZeroU64;
use swallowtail_core::{
    Capability, CapabilityConstraint, CapabilityProfile, CapabilityRequirement, PreflightPlan,
    ReasoningMode,
};
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
        match self.evidence.context_window_compression() {
            Some(compression) => {
                GeminiLiveDriver::new().with_context_window_compression(compression)
            }
            None => GeminiLiveDriver::new(),
        }
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
        let (
            request_id,
            config,
            deadline,
            rollover,
            reasoning_mode,
            maximum_output_tokens,
            context_window_compression,
        ) = input.into_parts();
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
        if reasoning_mode
            .as_ref()
            .is_some_and(|mode| crate::live_reasoning::thinking_level(mode).is_none())
        {
            return Err(failure(
                PreparationStage::Preflight,
                "swallowtail.gemini.live_preparation.reasoning_value_unsupported",
                "Gemini Live preparation admits only minimal, low, medium, or high thinking",
            ));
        }
        if maximum_output_tokens
            .is_some_and(|maximum| maximum.get() > GEMINI_LIVE_MAX_OUTPUT_TOKENS)
        {
            return Err(failure(
                PreparationStage::Preflight,
                "swallowtail.gemini.live_preparation.output_limit_invalid",
                "Gemini Live output-token maximum must be between 1 and 65536",
            ));
        }
        let capability_requirements = capabilities(reasoning_mode.as_ref(), maximum_output_tokens);
        let capability_profile = CapabilityProfile::new(capability_requirements.clone());
        let instance = instance_with_capabilities(self, capability_profile.clone());
        let route = model_route(self, capability_profile);
        let plan = build_plan(self, &instance, &route, capability_requirements)?;
        let mut request = OpenRealtimeMediaSessionRequest::new(request_id, config, deadline)
            .with_planned_connection_rollover(rollover);
        if let Some(mode) = reasoning_mode {
            request = request.with_reasoning_mode(mode);
        }
        if let Some(maximum) = maximum_output_tokens {
            request = request.with_maximum_output_tokens(maximum);
        }
        Ok(GeminiPreparedLiveSession {
            evidence: GeminiLivePreparedEvidence::from_prepared(
                self,
                plan,
                context_window_compression,
            )?,
            request,
        })
    }
}

fn capabilities(
    mode: Option<&ReasoningMode>,
    maximum: Option<NonZeroU64>,
) -> Vec<CapabilityRequirement> {
    let mut capabilities = crate::gemini_live_base_capabilities();
    if let Some(mode) = mode {
        capabilities.push(CapabilityRequirement::new(
            Capability::ReasoningSelection,
            [CapabilityConstraint::ReasoningMode(mode.clone())],
        ));
    }
    if let Some(maximum) = maximum {
        capabilities.push(CapabilityRequirement::new(
            Capability::OutputTokenLimit,
            [CapabilityConstraint::OutputTokenMaximum(maximum.get())],
        ));
    }
    capabilities
}

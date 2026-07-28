use super::input::OpenAiRealtimeSessionProfileInput;
use super::plan::{
    OpenAiRealtimePreparedEvidence, build_plan, instance_with_capabilities, model_route,
};
use crate::prepared_realtime::failure;
use crate::{OpenAiRealtimeDriver, OpenAiRealtimePreparedIntegration};
use swallowtail_core::{
    CancellationScope, Capability, CapabilityConstraint, CapabilityProfile, CapabilityRequirement,
    PlannedConnectionRolloverPolicy, PreflightPlan,
};
use swallowtail_runtime::{
    BoxFuture, HostServices, OpenRealtimeMediaSessionRequest, PreparationFailure, PreparationStage,
    RealtimeMediaSessionDriver, RealtimeMediaSessionHandle, RuntimeFailure,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OpenAiPreparedRealtimeSession {
    evidence: OpenAiRealtimePreparedEvidence,
    request: OpenRealtimeMediaSessionRequest,
}

impl OpenAiPreparedRealtimeSession {
    #[must_use]
    pub const fn evidence(&self) -> &OpenAiRealtimePreparedEvidence {
        &self.evidence
    }

    #[must_use]
    pub const fn plan(&self) -> &PreflightPlan {
        self.evidence.plan()
    }

    #[must_use]
    pub const fn request(&self) -> &OpenRealtimeMediaSessionRequest {
        &self.request
    }

    #[must_use]
    pub fn low_level_driver(&self) -> OpenAiRealtimeDriver {
        OpenAiRealtimeDriver::new()
    }

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

    #[must_use]
    pub fn into_parts(
        self,
    ) -> (
        OpenAiRealtimePreparedEvidence,
        PreflightPlan,
        OpenRealtimeMediaSessionRequest,
    ) {
        let plan = self.evidence.plan().clone();
        (self.evidence, plan, self.request)
    }
}

impl OpenAiRealtimePreparedIntegration {
    pub fn prepare_realtime_session(
        &self,
        input: OpenAiRealtimeSessionProfileInput,
    ) -> Result<OpenAiPreparedRealtimeSession, PreparationFailure> {
        let (request_id, config, deadline, rollover, maximum_output_tokens) = input.into_parts();
        if config != crate::openai_realtime_media_config() {
            return Err(failure(
                PreparationStage::Preflight,
                "swallowtail.openai.realtime_preparation.config_rejected",
                "OpenAI Realtime preparation requires the exact manual PCM format and bounds",
            ));
        }
        if rollover != PlannedConnectionRolloverPolicy::Disabled {
            return Err(failure(
                PreparationStage::Preflight,
                "swallowtail.openai.realtime_preparation.rollover_rejected",
                "OpenAI Realtime preparation does not permit planned connection rollover",
            ));
        }
        if maximum_output_tokens.is_some_and(|maximum| maximum.get() > 4096) {
            return Err(failure(
                PreparationStage::Preflight,
                "swallowtail.openai.realtime_preparation.output_limit_invalid",
                "OpenAI Realtime output-token maximum must be between 1 and 4096",
            ));
        }
        let capability_requirements = capabilities(maximum_output_tokens);
        let capability_profile = CapabilityProfile::new(capability_requirements.clone());
        let instance = instance_with_capabilities(self, capability_profile.clone());
        let route = model_route(self, capability_profile);
        let plan = build_plan(self, &instance, &route, capability_requirements)?;
        let mut request = OpenRealtimeMediaSessionRequest::new(request_id, config, deadline)
            .with_planned_connection_rollover(rollover);
        if let Some(maximum) = maximum_output_tokens {
            request = request.with_maximum_output_tokens(maximum);
        }
        Ok(OpenAiPreparedRealtimeSession {
            evidence: OpenAiRealtimePreparedEvidence::from_prepared(self, plan)?,
            request,
        })
    }
}

fn capabilities(maximum: Option<std::num::NonZeroU64>) -> Vec<CapabilityRequirement> {
    let mut capabilities = vec![
        CapabilityRequirement::new(Capability::StreamingEvents, []),
        CapabilityRequirement::new(Capability::UsageReporting, []),
        CapabilityRequirement::new(
            Capability::Interruption,
            [CapabilityConstraint::CancellationScope(
                CancellationScope::ActiveResponse,
            )],
        ),
        crate::openai_realtime_media_config().capability_requirement(),
    ];
    if let Some(maximum) = maximum {
        capabilities.push(CapabilityRequirement::new(
            Capability::OutputTokenLimit,
            [CapabilityConstraint::OutputTokenMaximum(maximum.get())],
        ));
    }
    capabilities
}

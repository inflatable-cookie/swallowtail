use super::input::OpenAiRealtimeSessionProfileInput;
use super::plan::{OpenAiRealtimePreparedEvidence, build_plan, model_route};
use crate::prepared_realtime::failure;
use crate::{OpenAiRealtimeDriver, OpenAiRealtimePreparedIntegration};
use swallowtail_core::{PlannedConnectionRolloverPolicy, PreflightPlan};
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
        let (request_id, config, deadline, rollover) = input.into_parts();
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
        let route = model_route(self);
        let plan = build_plan(self, &route)?;
        let request = OpenRealtimeMediaSessionRequest::new(request_id, config, deadline)
            .with_planned_connection_rollover(rollover);
        Ok(OpenAiPreparedRealtimeSession {
            evidence: OpenAiRealtimePreparedEvidence::from_prepared(self, plan)?,
            request,
        })
    }
}

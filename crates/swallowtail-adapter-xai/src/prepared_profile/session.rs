use super::input::XaiSessionProfileInput;
use super::plan::{XaiPreparedEvidence, build_plan, model_route};
use crate::{XaiPreparedIntegration, XaiWebSocketDriver};
use swallowtail_core::PreflightPlan;
use swallowtail_runtime::{
    BoxFuture, HostServices, InteractiveSessionDriver, InteractiveSessionHandle,
    OpenSessionRequest, PreparationFailure, RuntimeFailure,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct XaiPreparedResponsesSession {
    evidence: XaiPreparedEvidence,
    request: OpenSessionRequest,
}

impl XaiPreparedResponsesSession {
    #[must_use]
    pub const fn evidence(&self) -> &XaiPreparedEvidence {
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

    #[must_use]
    pub fn low_level_driver(&self) -> XaiWebSocketDriver {
        XaiWebSocketDriver::new()
    }

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
    pub fn into_parts(self) -> (XaiPreparedEvidence, PreflightPlan, OpenSessionRequest) {
        let plan = self.evidence.plan().clone();
        (self.evidence, plan, self.request)
    }
}

impl XaiPreparedIntegration {
    pub fn prepare_responses_session(
        &self,
        input: XaiSessionProfileInput,
    ) -> Result<XaiPreparedResponsesSession, PreparationFailure> {
        let (request_id, model, deadline) = input.into_parts();
        let route = model_route(self, model);
        let requirements =
            crate::xai_responses_requirements(self.instance().execution_host_id().clone());
        let plan = build_plan(self, &route, &requirements)?;
        let request = OpenSessionRequest::resource_free_from_plan(&plan, request_id, deadline)?;
        Ok(XaiPreparedResponsesSession {
            evidence: XaiPreparedEvidence::from_prepared(self, plan)?,
            request,
        })
    }
}

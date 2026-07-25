use super::input::AlibabaConversationProfileInput;
use super::plan::{AlibabaModelStudioPreparedEvidence, build_plan, model_route};
use crate::prepared::failure;
use crate::{AlibabaModelStudioDriver, AlibabaModelStudioPreparedIntegration};
use swallowtail_core::{PreflightPlan, SessionAccessPolicy, SessionProviderStatePolicy};
use swallowtail_runtime::{
    BoxFuture, HostServices, InteractiveSessionDriver, InteractiveSessionHandle,
    OpenSessionRequest, PreparationFailure, PreparationStage, RuntimeFailure, SessionPlanAgreement,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AlibabaModelStudioPreparedConversation {
    evidence: AlibabaModelStudioPreparedEvidence,
    request: OpenSessionRequest,
}

impl AlibabaModelStudioPreparedConversation {
    #[must_use]
    pub const fn evidence(&self) -> &AlibabaModelStudioPreparedEvidence {
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
    pub fn low_level_driver(&self) -> AlibabaModelStudioDriver {
        AlibabaModelStudioDriver::new()
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
    pub fn into_parts(
        self,
    ) -> (
        AlibabaModelStudioPreparedEvidence,
        PreflightPlan,
        OpenSessionRequest,
    ) {
        let plan = self.evidence.plan().clone();
        (self.evidence, plan, self.request)
    }
}

impl AlibabaModelStudioPreparedIntegration {
    pub fn prepare_conversation(
        &self,
        input: AlibabaConversationProfileInput,
    ) -> Result<AlibabaModelStudioPreparedConversation, PreparationFailure> {
        let (request_id, route_id, route_revision, model_id, provider_state, deadline) =
            input.into_parts();
        if route_id.as_str() != crate::MODEL_ROUTE_ID || model_id.as_str() != crate::EXACT_MODEL_ID
        {
            return Err(failure(
                PreparationStage::Preflight,
                "swallowtail.alibaba_model_studio.preparation.route_rejected",
                "Alibaba Model Studio preparation requires the exact Singapore Qwen route",
            ));
        }
        if provider_state != SessionProviderStatePolicy::DurableConversationDeleteOnClose {
            return Err(failure(
                PreparationStage::Preflight,
                "swallowtail.alibaba_model_studio.preparation.retention_rejected",
                "Alibaba Model Studio conversation retention and delete-on-close must be explicit",
            ));
        }
        let requirements =
            crate::alibaba_model_studio_requirements(self.instance().execution_host_id().clone());
        let route = model_route(self, route_id, route_revision, model_id);
        let plan = build_plan(self, self.instance(), &route, &requirements)?;
        let agreement = SessionPlanAgreement::explicit(
            SessionAccessPolicy::resource_free(),
            Some(provider_state),
            None,
        );
        let request = OpenSessionRequest::resource_free(request_id, deadline, agreement);
        Ok(AlibabaModelStudioPreparedConversation {
            evidence: AlibabaModelStudioPreparedEvidence::from_prepared(self, plan)?,
            request,
        })
    }
}

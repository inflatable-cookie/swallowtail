use super::input::OpenAiBackgroundRunProfileInput;
use super::plan::{OpenAiBackgroundPreparedEvidence, build_plan, model_route};
use crate::prepared::failure;
use crate::{OpenAiBackgroundDriver, OpenAiBackgroundPreparedIntegration};
use std::num::NonZeroU32;
use swallowtail_core::PreflightPlan;
use swallowtail_runtime::{
    BoxFuture, HostServices, OperationPolicy, PreparationFailure, PreparationStage,
    ProviderExecutionPolicy, ProviderRetentionPolicy, RunHandle, RuntimeFailure,
    StreamReattachmentPolicy, StructuredRunDriver, StructuredRunRequest,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OpenAiPreparedBackgroundRun {
    evidence: OpenAiBackgroundPreparedEvidence,
    request: StructuredRunRequest,
}

impl OpenAiPreparedBackgroundRun {
    #[must_use]
    pub const fn evidence(&self) -> &OpenAiBackgroundPreparedEvidence {
        &self.evidence
    }

    #[must_use]
    pub const fn plan(&self) -> &PreflightPlan {
        self.evidence.plan()
    }

    #[must_use]
    pub const fn request(&self) -> &StructuredRunRequest {
        &self.request
    }

    #[must_use]
    pub fn low_level_driver(&self) -> OpenAiBackgroundDriver {
        OpenAiBackgroundDriver::new()
    }

    pub fn start_run(
        &self,
        services: HostServices,
    ) -> BoxFuture<'static, Result<Box<dyn RunHandle>, RuntimeFailure>> {
        let driver = self.low_level_driver();
        let plan = self.plan().clone();
        let request = self.request.clone();
        Box::pin(async move { driver.start_run(plan, request, services).await })
    }

    #[must_use]
    pub fn into_parts(
        self,
    ) -> (
        OpenAiBackgroundPreparedEvidence,
        PreflightPlan,
        StructuredRunRequest,
    ) {
        let plan = self.evidence.plan().clone();
        (self.evidence, plan, self.request)
    }
}

impl OpenAiBackgroundPreparedIntegration {
    pub fn prepare_background_run(
        &self,
        input: OpenAiBackgroundRunProfileInput,
    ) -> Result<OpenAiPreparedBackgroundRun, PreparationFailure> {
        let (
            request_id,
            model,
            content,
            maximum,
            deadline,
            provider_execution,
            provider_retention,
            stream_reattachment,
        ) = input.into_parts();
        if provider_execution != ProviderExecutionPolicy::Background {
            return Err(failure(
                PreparationStage::Preflight,
                "swallowtail.openai.preparation.background_not_selected",
                "OpenAI provider-owned background execution must be selected explicitly",
            ));
        }
        if provider_retention != ProviderRetentionPolicy::TemporaryAllowed {
            return Err(failure(
                PreparationStage::Preflight,
                "swallowtail.openai.preparation.retention_not_accepted",
                "OpenAI background temporary provider retention must be accepted explicitly",
            ));
        }
        if stream_reattachment
            != StreamReattachmentPolicy::Bounded(NonZeroU32::new(1).expect("one is non-zero"))
        {
            return Err(failure(
                PreparationStage::Preflight,
                "swallowtail.openai.preparation.reattachment_rejected",
                "OpenAI background preparation permits exactly one stream reattachment",
            ));
        }
        let route = model_route(self, model);
        if route.id().as_str() != crate::OPENAI_BACKGROUND_MODEL_ROUTE_ID
            || route.model_id().as_str() != crate::OPENAI_BACKGROUND_MODEL_ID
        {
            return Err(failure(
                PreparationStage::Preflight,
                "swallowtail.openai.preparation.route_rejected",
                "OpenAI background preparation requires the exact GPT-5.6 route",
            ));
        }
        let plan = build_plan(self, &route)?;
        let policy = OperationPolicy::offline()
            .with_provider_execution(provider_execution)
            .with_provider_retention(provider_retention)
            .with_stream_reattachment(stream_reattachment);
        let request = StructuredRunRequest::new(request_id, content, policy)
            .with_maximum_output_tokens(maximum)
            .with_deadline(deadline);
        Ok(OpenAiPreparedBackgroundRun {
            evidence: OpenAiBackgroundPreparedEvidence::from_prepared(self, plan)?,
            request,
        })
    }
}

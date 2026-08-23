use std::num::{NonZeroU32, NonZeroU64};
use swallowtail_core::{ModelId, ModelRouteId, ModelRouteRevision, ReasoningMode};
use swallowtail_runtime::{
    Deadline, OperationContent, ProviderExecutionPolicy, ProviderRetentionPolicy, RequestId,
    StreamReattachmentPolicy, StructuredOutputDescriptor,
};

type OpenAiBackgroundRunParts = (
    RequestId,
    OpenAiBackgroundModelSelection,
    OperationContent,
    NonZeroU64,
    Option<ReasoningMode>,
    Option<StructuredOutputDescriptor>,
    Deadline,
    ProviderExecutionPolicy,
    ProviderRetentionPolicy,
    StreamReattachmentPolicy,
    bool,
    Option<crate::OpenAiBackgroundServiceTier>,
);

#[derive(Clone, Debug, Eq, PartialEq)]
/// Exact model-route selection for an OpenAI background response.
pub struct OpenAiBackgroundModelSelection {
    route_id: ModelRouteId,
    route_revision: ModelRouteRevision,
    model_id: ModelId,
}

impl OpenAiBackgroundModelSelection {
    #[must_use]
    /// Creates an exact route, route-revision, and model selection.
    pub const fn new(
        route_id: ModelRouteId,
        route_revision: ModelRouteRevision,
        model_id: ModelId,
    ) -> Self {
        Self {
            route_id,
            route_revision,
            model_id,
        }
    }

    pub(crate) fn into_parts(self) -> (ModelRouteId, ModelRouteRevision, ModelId) {
        (self.route_id, self.route_revision, self.model_id)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
/// Explicit policy and content for one provider-managed background run.
pub struct OpenAiBackgroundRunProfileInput {
    request_id: RequestId,
    model: OpenAiBackgroundModelSelection,
    content: OperationContent,
    maximum_output_tokens: NonZeroU64,
    reasoning: Option<ReasoningMode>,
    structured_output: Option<StructuredOutputDescriptor>,
    deadline: Deadline,
    provider_execution: ProviderExecutionPolicy,
    provider_retention: ProviderRetentionPolicy,
    stream_reattachment: StreamReattachmentPolicy,
    active_run_detachment: bool,
    service_tier: Option<crate::OpenAiBackgroundServiceTier>,
}

impl OpenAiBackgroundRunProfileInput {
    #[must_use]
    #[allow(clippy::too_many_arguments)]
    /// Creates background-run input without selecting policy defaults.
    pub const fn new(
        request_id: RequestId,
        model: OpenAiBackgroundModelSelection,
        content: OperationContent,
        maximum_output_tokens: NonZeroU64,
        deadline: Deadline,
        provider_execution: ProviderExecutionPolicy,
        provider_retention: ProviderRetentionPolicy,
        stream_reattachment: StreamReattachmentPolicy,
    ) -> Self {
        Self {
            request_id,
            model,
            content,
            maximum_output_tokens,
            reasoning: None,
            structured_output: None,
            deadline,
            provider_execution,
            provider_retention,
            stream_reattachment,
            active_run_detachment: false,
            service_tier: None,
        }
    }

    #[must_use]
    /// Creates the supported temporary-retention profile with one reattachment.
    pub fn background_with_temporary_retention_and_one_reattachment(
        request_id: RequestId,
        model: OpenAiBackgroundModelSelection,
        content: OperationContent,
        maximum_output_tokens: NonZeroU64,
        deadline: Deadline,
    ) -> Self {
        Self::new(
            request_id,
            model,
            content,
            maximum_output_tokens,
            deadline,
            ProviderExecutionPolicy::Background,
            ProviderRetentionPolicy::TemporaryAllowed,
            StreamReattachmentPolicy::Bounded(NonZeroU32::new(1).expect("one is non-zero")),
        )
    }

    #[must_use]
    /// Selects an optional provider reasoning mode.
    pub fn with_reasoning_mode(mut self, reasoning: ReasoningMode) -> Self {
        self.reasoning = Some(reasoning);
        self
    }

    #[must_use]
    /// Requests provider-native structured output from one inline schema.
    pub fn with_structured_output(mut self, output: StructuredOutputDescriptor) -> Self {
        self.structured_output = Some(output);
        self
    }

    #[must_use]
    /// Allows the active provider run to outlive the local attachment.
    pub const fn with_active_run_detachment(mut self) -> Self {
        self.active_run_detachment = true;
        self
    }

    #[must_use]
    /// Selects exact standard-processing service tier for ordinary attached runs.
    ///
    /// This is dispatch-only. It is rejected with active-run detachment.
    /// Selected-tier checkpoints are marked in the adapter-owned cursor and
    /// restart reconciliation rejects them before network work.
    pub const fn with_service_tier(
        mut self,
        service_tier: crate::OpenAiBackgroundServiceTier,
    ) -> Self {
        self.service_tier = Some(service_tier);
        self
    }

    pub(super) fn into_parts(self) -> OpenAiBackgroundRunParts {
        (
            self.request_id,
            self.model,
            self.content,
            self.maximum_output_tokens,
            self.reasoning,
            self.structured_output,
            self.deadline,
            self.provider_execution,
            self.provider_retention,
            self.stream_reattachment,
            self.active_run_detachment,
            self.service_tier,
        )
    }
}

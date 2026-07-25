use std::num::{NonZeroU32, NonZeroU64};
use swallowtail_core::{ModelId, ModelRouteId, ModelRouteRevision};
use swallowtail_runtime::{
    Deadline, OperationContent, ProviderExecutionPolicy, ProviderRetentionPolicy, RequestId,
    StreamReattachmentPolicy,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OpenAiBackgroundModelSelection {
    route_id: ModelRouteId,
    route_revision: ModelRouteRevision,
    model_id: ModelId,
}

impl OpenAiBackgroundModelSelection {
    #[must_use]
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

    pub(super) fn into_parts(self) -> (ModelRouteId, ModelRouteRevision, ModelId) {
        (self.route_id, self.route_revision, self.model_id)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OpenAiBackgroundRunProfileInput {
    request_id: RequestId,
    model: OpenAiBackgroundModelSelection,
    content: OperationContent,
    maximum_output_tokens: NonZeroU64,
    deadline: Deadline,
    provider_execution: ProviderExecutionPolicy,
    provider_retention: ProviderRetentionPolicy,
    stream_reattachment: StreamReattachmentPolicy,
}

impl OpenAiBackgroundRunProfileInput {
    #[must_use]
    #[allow(clippy::too_many_arguments)]
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
            deadline,
            provider_execution,
            provider_retention,
            stream_reattachment,
        }
    }

    #[must_use]
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

    pub(super) fn into_parts(
        self,
    ) -> (
        RequestId,
        OpenAiBackgroundModelSelection,
        OperationContent,
        NonZeroU64,
        Deadline,
        ProviderExecutionPolicy,
        ProviderRetentionPolicy,
        StreamReattachmentPolicy,
    ) {
        (
            self.request_id,
            self.model,
            self.content,
            self.maximum_output_tokens,
            self.deadline,
            self.provider_execution,
            self.provider_retention,
            self.stream_reattachment,
        )
    }
}

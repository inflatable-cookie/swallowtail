use std::num::NonZeroU64;
use swallowtail_core::{
    ModelId, ModelRouteId, ModelRouteRevision, ProviderInferenceCachePolicy, ReasoningMode,
};
use swallowtail_runtime::{Deadline, OperationContent, RequestId, SessionOptions, ToolDeclaration};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DeepSeekCatalogueProfileInput {
    request_id: RequestId,
    deadline: Option<Deadline>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DeepSeekRunProfileInput {
    request_id: RequestId,
    model: DeepSeekModelSelection,
    content: OperationContent,
    reasoning: ReasoningMode,
    maximum_output_tokens: NonZeroU64,
    cache_policy: ProviderInferenceCachePolicy,
    deadline: Option<Deadline>,
}

impl DeepSeekRunProfileInput {
    #[must_use]
    pub const fn new(
        request_id: RequestId,
        model: DeepSeekModelSelection,
        content: OperationContent,
        reasoning: ReasoningMode,
        maximum_output_tokens: NonZeroU64,
        cache_policy: ProviderInferenceCachePolicy,
    ) -> Self {
        Self {
            request_id,
            model,
            content,
            reasoning,
            maximum_output_tokens,
            cache_policy,
            deadline: None,
        }
    }

    #[must_use]
    pub const fn with_deadline(mut self, deadline: Deadline) -> Self {
        self.deadline = Some(deadline);
        self
    }

    pub(super) fn into_parts(
        self,
    ) -> (
        RequestId,
        DeepSeekModelSelection,
        OperationContent,
        ReasoningMode,
        NonZeroU64,
        ProviderInferenceCachePolicy,
        Option<Deadline>,
    ) {
        (
            self.request_id,
            self.model,
            self.content,
            self.reasoning,
            self.maximum_output_tokens,
            self.cache_policy,
            self.deadline,
        )
    }
}

impl DeepSeekCatalogueProfileInput {
    #[must_use]
    pub const fn new(request_id: RequestId) -> Self {
        Self {
            request_id,
            deadline: None,
        }
    }

    #[must_use]
    pub const fn with_deadline(mut self, deadline: Deadline) -> Self {
        self.deadline = Some(deadline);
        self
    }

    pub(super) fn into_parts(self) -> (RequestId, Option<Deadline>) {
        (self.request_id, self.deadline)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DeepSeekModelSelection {
    route_id: ModelRouteId,
    route_revision: ModelRouteRevision,
    model_id: ModelId,
}

impl DeepSeekModelSelection {
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
pub struct DeepSeekSessionProfileInput {
    request_id: RequestId,
    model: DeepSeekModelSelection,
    reasoning: ReasoningMode,
    tools: Vec<ToolDeclaration>,
    cache_policy: ProviderInferenceCachePolicy,
}

impl DeepSeekSessionProfileInput {
    #[must_use]
    pub fn new(
        request_id: RequestId,
        model: DeepSeekModelSelection,
        reasoning: ReasoningMode,
        tools: impl IntoIterator<Item = ToolDeclaration>,
        cache_policy: ProviderInferenceCachePolicy,
    ) -> Self {
        Self {
            request_id,
            model,
            reasoning,
            tools: tools.into_iter().collect(),
            cache_policy,
        }
    }

    pub(super) fn into_parts(
        self,
    ) -> (
        RequestId,
        DeepSeekModelSelection,
        SessionOptions,
        ProviderInferenceCachePolicy,
    ) {
        (
            self.request_id,
            self.model,
            SessionOptions::default()
                .with_reasoning_mode(self.reasoning)
                .with_tools(self.tools),
            self.cache_policy,
        )
    }
}

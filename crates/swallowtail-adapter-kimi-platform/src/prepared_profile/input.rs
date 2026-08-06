use std::num::NonZeroU64;
use swallowtail_core::{ModelId, ModelRouteId, ModelRouteRevision, ReasoningMode};
use swallowtail_runtime::{Deadline, OperationContent, RequestId};

#[derive(Clone, Debug, Eq, PartialEq)]
/// Consumer input for a prepared Kimi Platform catalogue observation.
pub struct KimiPlatformCatalogueProfileInput {
    request_id: RequestId,
    deadline: Option<Deadline>,
}

impl KimiPlatformCatalogueProfileInput {
    #[must_use]
    /// Creates catalogue input without a deadline.
    pub const fn new(request_id: RequestId) -> Self {
        Self {
            request_id,
            deadline: None,
        }
    }

    #[must_use]
    /// Adds an exact host-monotonic deadline.
    pub const fn with_deadline(mut self, deadline: Deadline) -> Self {
        self.deadline = Some(deadline);
        self
    }

    pub(super) fn into_parts(self) -> (RequestId, Option<Deadline>) {
        (self.request_id, self.deadline)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
/// Exact Kimi Platform model-route selection supplied by the consumer.
pub struct KimiPlatformModelSelection {
    route_id: ModelRouteId,
    route_revision: ModelRouteRevision,
    model_id: ModelId,
}

impl KimiPlatformModelSelection {
    #[must_use]
    /// Creates an exact route, revision, and model selection.
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
/// Consumer input for one prepared K3 inference attempt.
pub struct KimiPlatformInferenceAttemptInput {
    request_id: RequestId,
    model: KimiPlatformModelSelection,
    content: OperationContent,
    reasoning: ReasoningMode,
    maximum_output_tokens: NonZeroU64,
    deadline: Option<Deadline>,
}

impl KimiPlatformInferenceAttemptInput {
    #[must_use]
    /// Creates input with explicit model, content, reasoning, and output bound.
    pub const fn new(
        request_id: RequestId,
        model: KimiPlatformModelSelection,
        content: OperationContent,
        reasoning: ReasoningMode,
        maximum_output_tokens: NonZeroU64,
    ) -> Self {
        Self {
            request_id,
            model,
            content,
            reasoning,
            maximum_output_tokens,
            deadline: None,
        }
    }

    #[must_use]
    /// Adds an exact host-monotonic deadline.
    pub const fn with_deadline(mut self, deadline: Deadline) -> Self {
        self.deadline = Some(deadline);
        self
    }

    pub(super) fn into_parts(
        self,
    ) -> (
        RequestId,
        KimiPlatformModelSelection,
        OperationContent,
        ReasoningMode,
        NonZeroU64,
        Option<Deadline>,
    ) {
        (
            self.request_id,
            self.model,
            self.content,
            self.reasoning,
            self.maximum_output_tokens,
            self.deadline,
        )
    }
}

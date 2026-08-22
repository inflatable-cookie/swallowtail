use std::num::NonZeroU64;
use swallowtail_core::{ModelId, ModelRouteId, ModelRouteRevision, ReasoningMode};
use swallowtail_runtime::{Deadline, OperationContent, RequestId};

#[derive(Clone, Debug, Eq, PartialEq)]
/// Exact xAI model-route selection supplied by the consumer.
pub struct XaiModelSelection {
    route_id: ModelRouteId,
    route_revision: ModelRouteRevision,
    model_id: ModelId,
}

#[derive(Clone, Debug, Eq, PartialEq)]
/// Consumer input for one xAI Responses structured run.
pub struct XaiRunProfileInput {
    request_id: RequestId,
    model: XaiModelSelection,
    content: OperationContent,
    deadline: Option<Deadline>,
    reasoning: Option<ReasoningMode>,
    maximum_output_tokens: Option<NonZeroU64>,
}

impl XaiRunProfileInput {
    #[must_use]
    /// Creates a structured-run input with an optional exact deadline.
    pub const fn new(
        request_id: RequestId,
        model: XaiModelSelection,
        content: OperationContent,
        deadline: Option<Deadline>,
    ) -> Self {
        Self {
            request_id,
            model,
            content,
            deadline,
            reasoning: None,
            maximum_output_tokens: None,
        }
    }

    #[must_use]
    /// Selects one exact qualified reasoning effort for the model route.
    pub fn with_reasoning_mode(mut self, reasoning: ReasoningMode) -> Self {
        self.reasoning = Some(reasoning);
        self
    }

    #[must_use]
    /// Selects one positive Responses maximum-output-token bound.
    pub const fn with_maximum_output_tokens(mut self, maximum: NonZeroU64) -> Self {
        self.maximum_output_tokens = Some(maximum);
        self
    }

    #[must_use]
    /// Returns the selected reasoning effort, when present.
    pub const fn reasoning_mode(&self) -> Option<&ReasoningMode> {
        self.reasoning.as_ref()
    }

    #[must_use]
    /// Returns the selected maximum-output-token bound, when present.
    pub const fn maximum_output_tokens(&self) -> Option<NonZeroU64> {
        self.maximum_output_tokens
    }

    pub(super) fn into_parts(
        self,
    ) -> (
        RequestId,
        XaiModelSelection,
        OperationContent,
        Option<Deadline>,
        Option<ReasoningMode>,
        Option<NonZeroU64>,
    ) {
        (
            self.request_id,
            self.model,
            self.content,
            self.deadline,
            self.reasoning,
            self.maximum_output_tokens,
        )
    }
}

impl XaiModelSelection {
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

    pub(super) const fn model_id(&self) -> &ModelId {
        &self.model_id
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
/// Consumer input for a serial xAI Responses session.
pub struct XaiSessionProfileInput {
    request_id: RequestId,
    model: XaiModelSelection,
    deadline: Option<Deadline>,
    reasoning: Option<ReasoningMode>,
    maximum_output_tokens: Option<NonZeroU64>,
}

impl XaiSessionProfileInput {
    #[must_use]
    /// Creates session input with an optional exact deadline.
    pub const fn new(
        request_id: RequestId,
        model: XaiModelSelection,
        deadline: Option<Deadline>,
    ) -> Self {
        Self {
            request_id,
            model,
            deadline,
            reasoning: None,
            maximum_output_tokens: None,
        }
    }

    #[must_use]
    /// Selects one exact qualified reasoning effort for the session.
    pub fn with_reasoning_mode(mut self, reasoning: ReasoningMode) -> Self {
        self.reasoning = Some(reasoning);
        self
    }

    #[must_use]
    /// Selects one positive Responses maximum-output-token bound per turn.
    pub const fn with_maximum_output_tokens(mut self, maximum: NonZeroU64) -> Self {
        self.maximum_output_tokens = Some(maximum);
        self
    }

    #[must_use]
    /// Returns the selected reasoning effort, when present.
    pub const fn reasoning_mode(&self) -> Option<&ReasoningMode> {
        self.reasoning.as_ref()
    }

    #[must_use]
    /// Returns the selected maximum-output-token bound, when present.
    pub const fn maximum_output_tokens(&self) -> Option<NonZeroU64> {
        self.maximum_output_tokens
    }

    pub(super) fn into_parts(
        self,
    ) -> (
        RequestId,
        XaiModelSelection,
        Option<Deadline>,
        Option<ReasoningMode>,
        Option<NonZeroU64>,
    ) {
        (
            self.request_id,
            self.model,
            self.deadline,
            self.reasoning,
            self.maximum_output_tokens,
        )
    }
}

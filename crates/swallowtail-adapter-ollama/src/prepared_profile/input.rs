use std::num::NonZeroU64;
use swallowtail_core::{ModelId, ModelRouteId, ModelRouteRevision, ReasoningMode};
use swallowtail_runtime::{Deadline, OperationContent, RequestId, StructuredOutputDescriptor};

#[derive(Clone, Debug, Eq, PartialEq)]
/// Exact local model route selected for the attached runtime.
pub struct OllamaModelSelection {
    route_id: ModelRouteId,
    route_revision: ModelRouteRevision,
    model_id: ModelId,
}

impl OllamaModelSelection {
    /// Creates a model selection without inferring a local tag or route.
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
/// Inputs for one fresh observation of attached model inventory.
pub struct OllamaInventoryProfileInput {
    request_id: RequestId,
    deadline: Option<Deadline>,
}

impl OllamaInventoryProfileInput {
    /// Creates an inventory input without a deadline.
    #[must_use]
    pub const fn new(request_id: RequestId) -> Self {
        Self {
            request_id,
            deadline: None,
        }
    }

    /// Adds a caller-owned inventory deadline.
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
/// Inputs for one bounded, resource-free native inference attempt.
pub struct OllamaInferenceAttemptInput {
    request_id: RequestId,
    content: OperationContent,
    maximum_output_tokens: NonZeroU64,
    reasoning: Option<ReasoningMode>,
    structured_output: Option<StructuredOutputDescriptor>,
    deadline: Option<Deadline>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
/// Inputs for one resource-free interactive Ollama session.
pub struct OllamaSessionProfileInput {
    request_id: RequestId,
    deadline: Option<Deadline>,
}

impl OllamaSessionProfileInput {
    /// Creates a session profile without a deadline.
    #[must_use]
    pub const fn new(request_id: RequestId) -> Self {
        Self {
            request_id,
            deadline: None,
        }
    }

    /// Adds a caller-owned session deadline.
    #[must_use]
    pub const fn with_deadline(mut self, deadline: Deadline) -> Self {
        self.deadline = Some(deadline);
        self
    }

    pub(super) fn into_parts(self) -> (RequestId, Option<Deadline>) {
        (self.request_id, self.deadline)
    }
}

impl OllamaInferenceAttemptInput {
    /// Creates an attempt with explicit content and output-token bound.
    #[must_use]
    pub const fn new(
        request_id: RequestId,
        content: OperationContent,
        maximum_output_tokens: NonZeroU64,
    ) -> Self {
        Self {
            request_id,
            content,
            maximum_output_tokens,
            reasoning: None,
            structured_output: None,
            deadline: None,
        }
    }

    /// Adds a caller-owned attempt deadline.
    #[must_use]
    pub const fn with_deadline(mut self, deadline: Deadline) -> Self {
        self.deadline = Some(deadline);
        self
    }

    /// Selects an explicit native reasoning mode.
    #[must_use]
    pub fn with_reasoning_mode(mut self, reasoning: ReasoningMode) -> Self {
        self.reasoning = Some(reasoning);
        self
    }

    /// Requests provider-native inline JSON Schema output.
    #[must_use]
    pub fn with_structured_output(mut self, output: StructuredOutputDescriptor) -> Self {
        self.structured_output = Some(output);
        self
    }

    pub(super) fn into_parts(
        self,
    ) -> (
        RequestId,
        OperationContent,
        NonZeroU64,
        Option<ReasoningMode>,
        Option<StructuredOutputDescriptor>,
        Option<Deadline>,
    ) {
        (
            self.request_id,
            self.content,
            self.maximum_output_tokens,
            self.reasoning,
            self.structured_output,
            self.deadline,
        )
    }
}

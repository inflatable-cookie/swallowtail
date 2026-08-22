use std::num::NonZeroU64;
use swallowtail_core::{ModelId, ModelRouteId, ModelRouteRevision, ReasoningMode};
use swallowtail_runtime::{
    AttachmentDescriptor, Deadline, OperationContent, RequestId, ToolDeclaration,
};

type InferenceAttemptParts = (
    RequestId,
    AnthropicModelSelection,
    OperationContent,
    NonZeroU64,
    Option<Deadline>,
    Vec<AttachmentDescriptor>,
    Option<AnthropicWebSearchInput>,
    Option<ReasoningMode>,
);

#[derive(Clone, Debug, Eq, PartialEq)]
/// Consumer input for an Anthropic Messages catalogue observation.
pub struct AnthropicCatalogueProfileInput {
    request_id: RequestId,
    deadline: Option<Deadline>,
}

impl AnthropicCatalogueProfileInput {
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
/// Exact Anthropic model-route selection supplied by the consumer.
pub struct AnthropicModelSelection {
    route_id: ModelRouteId,
    route_revision: ModelRouteRevision,
    model_id: ModelId,
}

impl AnthropicModelSelection {
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
/// Consumer input for one Anthropic Messages inference attempt.
pub struct AnthropicInferenceAttemptInput {
    request_id: RequestId,
    model: AnthropicModelSelection,
    content: OperationContent,
    maximum_output_tokens: NonZeroU64,
    deadline: Option<Deadline>,
    attachments: Vec<AttachmentDescriptor>,
    web_search: Option<AnthropicWebSearchInput>,
    reasoning_mode: Option<ReasoningMode>,
}

impl AnthropicInferenceAttemptInput {
    #[must_use]
    /// Creates input with exact model, content, and output-token bound.
    pub const fn new(
        request_id: RequestId,
        model: AnthropicModelSelection,
        content: OperationContent,
        maximum_output_tokens: NonZeroU64,
    ) -> Self {
        Self {
            request_id,
            model,
            content,
            maximum_output_tokens,
            deadline: None,
            attachments: Vec::new(),
            web_search: None,
            reasoning_mode: None,
        }
    }

    #[must_use]
    /// Adds an exact host-monotonic deadline.
    pub const fn with_deadline(mut self, deadline: Deadline) -> Self {
        self.deadline = Some(deadline);
        self
    }

    #[must_use]
    /// Adds the bounded portable attachment set.
    pub fn with_attachments(
        mut self,
        attachments: impl IntoIterator<Item = AttachmentDescriptor>,
    ) -> Self {
        self.attachments = attachments.into_iter().collect();
        self
    }

    #[must_use]
    /// Enables provider web search within an explicit domain allowlist.
    pub fn with_web_search(mut self, search: AnthropicWebSearchInput) -> Self {
        self.web_search = Some(search);
        self
    }

    #[must_use]
    /// Selects one exact Anthropic effort value for the qualified model.
    pub fn with_reasoning_mode(mut self, reasoning_mode: ReasoningMode) -> Self {
        self.reasoning_mode = Some(reasoning_mode);
        self
    }

    pub(super) fn into_parts(self) -> InferenceAttemptParts {
        (
            self.request_id,
            self.model,
            self.content,
            self.maximum_output_tokens,
            self.deadline,
            self.attachments,
            self.web_search,
            self.reasoning_mode,
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
/// Explicit provider web-search allowlist for one Messages attempt.
pub struct AnthropicWebSearchInput {
    allowed_domains: Vec<String>,
}

impl AnthropicWebSearchInput {
    #[must_use]
    /// Creates search input from bare allowed domain names.
    pub fn new(allowed_domains: impl IntoIterator<Item = impl Into<String>>) -> Self {
        Self {
            allowed_domains: allowed_domains.into_iter().map(Into::into).collect(),
        }
    }

    pub(super) fn into_allowed_domains(self) -> Vec<String> {
        self.allowed_domains
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
/// Consumer input for an Anthropic direct tool-continuation session.
pub struct AnthropicSessionProfileInput {
    request_id: RequestId,
    model: AnthropicModelSelection,
    tools: Vec<ToolDeclaration>,
    reasoning_mode: Option<ReasoningMode>,
}

impl AnthropicSessionProfileInput {
    #[must_use]
    /// Creates session input with an exact model and declared consumer tools.
    pub fn new(
        request_id: RequestId,
        model: AnthropicModelSelection,
        tools: impl IntoIterator<Item = ToolDeclaration>,
    ) -> Self {
        Self {
            request_id,
            model,
            tools: tools.into_iter().collect(),
            reasoning_mode: None,
        }
    }

    #[must_use]
    /// Selects one exact Anthropic effort value for the qualified model.
    pub fn with_reasoning_mode(mut self, reasoning_mode: ReasoningMode) -> Self {
        self.reasoning_mode = Some(reasoning_mode);
        self
    }

    pub(super) fn into_parts(
        self,
    ) -> (
        RequestId,
        AnthropicModelSelection,
        Vec<ToolDeclaration>,
        Option<ReasoningMode>,
    ) {
        (self.request_id, self.model, self.tools, self.reasoning_mode)
    }
}

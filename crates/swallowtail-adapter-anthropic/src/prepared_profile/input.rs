use std::num::NonZeroU64;
use swallowtail_core::{ModelId, ModelRouteId, ModelRouteRevision};
use swallowtail_runtime::{
    AttachmentDescriptor, Deadline, OperationContent, RequestId, ToolDeclaration,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AnthropicCatalogueProfileInput {
    request_id: RequestId,
    deadline: Option<Deadline>,
}

impl AnthropicCatalogueProfileInput {
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
pub struct AnthropicModelSelection {
    route_id: ModelRouteId,
    route_revision: ModelRouteRevision,
    model_id: ModelId,
}

impl AnthropicModelSelection {
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
pub struct AnthropicInferenceAttemptInput {
    request_id: RequestId,
    model: AnthropicModelSelection,
    content: OperationContent,
    maximum_output_tokens: NonZeroU64,
    deadline: Option<Deadline>,
    attachments: Vec<AttachmentDescriptor>,
    web_search: Option<AnthropicWebSearchInput>,
}

impl AnthropicInferenceAttemptInput {
    #[must_use]
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
        }
    }

    #[must_use]
    pub const fn with_deadline(mut self, deadline: Deadline) -> Self {
        self.deadline = Some(deadline);
        self
    }

    #[must_use]
    pub fn with_attachments(
        mut self,
        attachments: impl IntoIterator<Item = AttachmentDescriptor>,
    ) -> Self {
        self.attachments = attachments.into_iter().collect();
        self
    }

    #[must_use]
    pub fn with_web_search(mut self, search: AnthropicWebSearchInput) -> Self {
        self.web_search = Some(search);
        self
    }

    pub(super) fn into_parts(
        self,
    ) -> (
        RequestId,
        AnthropicModelSelection,
        OperationContent,
        NonZeroU64,
        Option<Deadline>,
        Vec<AttachmentDescriptor>,
        Option<AnthropicWebSearchInput>,
    ) {
        (
            self.request_id,
            self.model,
            self.content,
            self.maximum_output_tokens,
            self.deadline,
            self.attachments,
            self.web_search,
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AnthropicWebSearchInput {
    allowed_domains: Vec<String>,
}

impl AnthropicWebSearchInput {
    #[must_use]
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
pub struct AnthropicSessionProfileInput {
    request_id: RequestId,
    model: AnthropicModelSelection,
    tools: Vec<ToolDeclaration>,
}

impl AnthropicSessionProfileInput {
    #[must_use]
    pub fn new(
        request_id: RequestId,
        model: AnthropicModelSelection,
        tools: impl IntoIterator<Item = ToolDeclaration>,
    ) -> Self {
        Self {
            request_id,
            model,
            tools: tools.into_iter().collect(),
        }
    }

    pub(super) fn into_parts(self) -> (RequestId, AnthropicModelSelection, Vec<ToolDeclaration>) {
        (self.request_id, self.model, self.tools)
    }
}

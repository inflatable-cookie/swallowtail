use swallowtail_core::{
    ModelCatalogEntry, ModelId, ModelRouteId, ModelRouteRevision, ProviderId, ReasoningMode,
};
use swallowtail_runtime::{
    AttachmentDescriptor, Deadline, OperationContent, ProviderSessionManagementBinding, RequestId,
    StructuredOutputDescriptor, WorkingResourceRef,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OpenCodeCatalogueProfileInput {
    request_id: RequestId,
    deadline: Option<Deadline>,
}

impl OpenCodeCatalogueProfileInput {
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
pub struct OpenCodeModelSelection {
    route_id: ModelRouteId,
    route_revision: ModelRouteRevision,
    provider_id: ProviderId,
    model_id: ModelId,
    catalogue_entry: Option<ModelCatalogEntry>,
}

impl OpenCodeModelSelection {
    #[must_use]
    pub const fn new(
        route_id: ModelRouteId,
        route_revision: ModelRouteRevision,
        provider_id: ProviderId,
        model_id: ModelId,
    ) -> Self {
        Self {
            route_id,
            route_revision,
            provider_id,
            model_id,
            catalogue_entry: None,
        }
    }

    #[must_use]
    pub fn with_catalogue_entry(mut self, entry: ModelCatalogEntry) -> Self {
        self.catalogue_entry = Some(entry);
        self
    }

    pub(super) fn into_parts(
        self,
    ) -> (
        ModelRouteId,
        ModelRouteRevision,
        ProviderId,
        ModelId,
        Option<ModelCatalogEntry>,
    ) {
        (
            self.route_id,
            self.route_revision,
            self.provider_id,
            self.model_id,
            self.catalogue_entry,
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OpenCodeSessionProfileInput {
    request_id: RequestId,
    model: OpenCodeModelSelection,
    working_resource: WorkingResourceRef,
    deadline: Option<Deadline>,
    image_attachments: bool,
    provider_callbacks: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OpenCodeSessionManagementInput {
    request_id: RequestId,
    binding: ProviderSessionManagementBinding,
    deadline: Option<Deadline>,
    allow_unverified_newer: bool,
}

impl OpenCodeSessionManagementInput {
    #[must_use]
    pub const fn new(request_id: RequestId, binding: ProviderSessionManagementBinding) -> Self {
        Self {
            request_id,
            binding,
            deadline: None,
            allow_unverified_newer: false,
        }
    }

    #[must_use]
    pub const fn with_deadline(mut self, deadline: Deadline) -> Self {
        self.deadline = Some(deadline);
        self
    }

    #[must_use]
    pub const fn allow_unverified_newer(mut self) -> Self {
        self.allow_unverified_newer = true;
        self
    }

    pub(super) fn into_parts(
        self,
    ) -> (
        RequestId,
        ProviderSessionManagementBinding,
        Option<Deadline>,
        bool,
    ) {
        (
            self.request_id,
            self.binding,
            self.deadline,
            self.allow_unverified_newer,
        )
    }
}

impl OpenCodeSessionProfileInput {
    #[must_use]
    pub const fn new(
        request_id: RequestId,
        model: OpenCodeModelSelection,
        working_resource: WorkingResourceRef,
    ) -> Self {
        Self {
            request_id,
            model,
            working_resource,
            deadline: None,
            image_attachments: false,
            provider_callbacks: false,
        }
    }

    #[must_use]
    pub const fn with_image_attachments(mut self) -> Self {
        self.image_attachments = true;
        self
    }

    #[must_use]
    pub const fn with_provider_callbacks(mut self) -> Self {
        self.provider_callbacks = true;
        self
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
        OpenCodeModelSelection,
        WorkingResourceRef,
        Option<Deadline>,
        bool,
        bool,
    ) {
        (
            self.request_id,
            self.model,
            self.working_resource,
            self.deadline,
            self.image_attachments,
            self.provider_callbacks,
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OpenCodeRunProfileInput {
    request_id: RequestId,
    model: OpenCodeModelSelection,
    content: OperationContent,
    working_resource: WorkingResourceRef,
    reasoning: Option<ReasoningMode>,
    structured_output: Option<StructuredOutputDescriptor>,
    deadline: Option<Deadline>,
    attachments: Vec<AttachmentDescriptor>,
    provider_callbacks: bool,
}

pub(super) struct OpenCodeRunProfileParts {
    pub(super) request_id: RequestId,
    pub(super) model: OpenCodeModelSelection,
    pub(super) content: OperationContent,
    pub(super) working_resource: WorkingResourceRef,
    pub(super) reasoning: Option<ReasoningMode>,
    pub(super) structured_output: Option<StructuredOutputDescriptor>,
    pub(super) deadline: Option<Deadline>,
    pub(super) attachments: Vec<AttachmentDescriptor>,
    pub(super) provider_callbacks: bool,
}

impl OpenCodeRunProfileInput {
    #[must_use]
    pub const fn new(
        request_id: RequestId,
        model: OpenCodeModelSelection,
        content: OperationContent,
        working_resource: WorkingResourceRef,
    ) -> Self {
        Self {
            request_id,
            model,
            content,
            working_resource,
            reasoning: None,
            structured_output: None,
            deadline: None,
            attachments: Vec::new(),
            provider_callbacks: false,
        }
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
    pub const fn with_provider_callbacks(mut self) -> Self {
        self.provider_callbacks = true;
        self
    }

    #[must_use]
    pub const fn with_deadline(mut self, deadline: Deadline) -> Self {
        self.deadline = Some(deadline);
        self
    }

    #[must_use]
    pub fn with_reasoning_mode(mut self, reasoning: ReasoningMode) -> Self {
        self.reasoning = Some(reasoning);
        self
    }

    #[must_use]
    pub fn with_structured_output(mut self, output: StructuredOutputDescriptor) -> Self {
        self.structured_output = Some(output);
        self
    }

    pub(super) fn into_parts(self) -> OpenCodeRunProfileParts {
        OpenCodeRunProfileParts {
            request_id: self.request_id,
            model: self.model,
            content: self.content,
            working_resource: self.working_resource,
            reasoning: self.reasoning,
            structured_output: self.structured_output,
            deadline: self.deadline,
            attachments: self.attachments,
            provider_callbacks: self.provider_callbacks,
        }
    }
}

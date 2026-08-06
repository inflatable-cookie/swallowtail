use swallowtail_core::{ModelId, ModelRouteId, ModelRouteRevision, ProviderId};
use swallowtail_runtime::{
    AttachmentDescriptor, Deadline, OperationContent, RequestId, SessionOptions, WorkingResourceRef,
};

#[derive(Clone, Debug, Eq, PartialEq)]
/// Inputs for one Pi model-catalogue request.
pub struct PiCatalogueProfileInput {
    request_id: RequestId,
    deadline: Option<Deadline>,
}

impl PiCatalogueProfileInput {
    /// Creates a catalogue input without a deadline.
    #[must_use]
    pub const fn new(request_id: RequestId) -> Self {
        Self {
            request_id,
            deadline: None,
        }
    }

    /// Adds a caller-owned catalogue deadline.
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
/// Exact provider and model-route identity selected for RPC execution.
pub struct PiModelSelection {
    route_id: ModelRouteId,
    route_revision: ModelRouteRevision,
    provider_id: ProviderId,
    model_id: ModelId,
}

impl PiModelSelection {
    /// Creates a model selection without inferring provider or model defaults.
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
        }
    }

    pub(super) fn into_parts(self) -> (ModelRouteId, ModelRouteRevision, ProviderId, ModelId) {
        (
            self.route_id,
            self.route_revision,
            self.provider_id,
            self.model_id,
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
/// Inputs for one read-only Pi interactive session.
pub struct PiSessionProfileInput {
    request_id: RequestId,
    model: PiModelSelection,
    working_resource: WorkingResourceRef,
    deadline: Option<Deadline>,
    options: SessionOptions,
    image_attachments: bool,
}

impl PiSessionProfileInput {
    /// Creates a session with explicit model, workspace, and portable options.
    #[must_use]
    pub const fn new(
        request_id: RequestId,
        model: PiModelSelection,
        working_resource: WorkingResourceRef,
        options: SessionOptions,
    ) -> Self {
        Self {
            request_id,
            model,
            working_resource,
            deadline: None,
            options,
            image_attachments: false,
        }
    }

    /// Adds a caller-owned session deadline.
    #[must_use]
    pub const fn with_deadline(mut self, deadline: Deadline) -> Self {
        self.deadline = Some(deadline);
        self
    }

    /// Enables the route's bounded image-attachment capability.
    #[must_use]
    pub const fn with_image_attachments(mut self) -> Self {
        self.image_attachments = true;
        self
    }

    pub(super) fn into_parts(
        self,
    ) -> (
        RequestId,
        PiModelSelection,
        WorkingResourceRef,
        Option<Deadline>,
        SessionOptions,
        bool,
    ) {
        (
            self.request_id,
            self.model,
            self.working_resource,
            self.deadline,
            self.options,
            self.image_attachments,
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
/// Inputs for one bounded Pi structured run.
pub struct PiRunProfileInput {
    request_id: RequestId,
    model: PiModelSelection,
    content: OperationContent,
    working_resource: WorkingResourceRef,
    deadline: Deadline,
    attachments: Vec<AttachmentDescriptor>,
}

impl PiRunProfileInput {
    /// Creates a run with explicit model, content, workspace, and deadline.
    #[must_use]
    pub const fn new(
        request_id: RequestId,
        model: PiModelSelection,
        content: OperationContent,
        working_resource: WorkingResourceRef,
        deadline: Deadline,
    ) -> Self {
        Self {
            request_id,
            model,
            content,
            working_resource,
            deadline,
            attachments: Vec::new(),
        }
    }

    /// Replaces the run's bounded attachment set.
    #[must_use]
    pub fn with_attachments(
        mut self,
        attachments: impl IntoIterator<Item = AttachmentDescriptor>,
    ) -> Self {
        self.attachments = attachments.into_iter().collect();
        self
    }

    pub(super) fn into_parts(
        self,
    ) -> (
        RequestId,
        PiModelSelection,
        OperationContent,
        WorkingResourceRef,
        Deadline,
        Vec<AttachmentDescriptor>,
    ) {
        (
            self.request_id,
            self.model,
            self.content,
            self.working_resource,
            self.deadline,
            self.attachments,
        )
    }
}

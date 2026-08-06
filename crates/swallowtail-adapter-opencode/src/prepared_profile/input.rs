use swallowtail_core::ProviderSessionCatalogueBounds;
use swallowtail_core::{
    ModelCatalogEntry, ModelId, ModelRouteId, ModelRouteRevision, ProviderId, ReasoningMode,
    TurnRef,
};
use swallowtail_runtime::{
    AttachmentDescriptor, Deadline, OperationContent, ProviderSessionCatalogueId,
    ProviderSessionManagementBinding, ProviderSessionReconciliationBounds, RequestId,
    RuntimeTurnId, SessionResumeBinding, StructuredOutputDescriptor, WorkingResourceRef,
};

#[derive(Clone, Debug, Eq, PartialEq)]
/// Inputs for read-only reconciliation of one exact retained OpenCode session.
pub struct OpenCodeSessionReconciliationInput {
    request_id: RequestId,
    model: OpenCodeModelSelection,
    binding: SessionResumeBinding,
    interrupted_turn_id: RuntimeTurnId,
    provider_turn_ref: Option<TurnRef>,
    bounds: ProviderSessionReconciliationBounds,
    deadline: Option<Deadline>,
}

impl OpenCodeSessionReconciliationInput {
    /// Creates a bounded session-scoped reconciliation input.
    #[must_use]
    pub const fn new(
        request_id: RequestId,
        model: OpenCodeModelSelection,
        binding: SessionResumeBinding,
        interrupted_turn_id: RuntimeTurnId,
        bounds: ProviderSessionReconciliationBounds,
    ) -> Self {
        Self {
            request_id,
            model,
            binding,
            interrupted_turn_id,
            provider_turn_ref: None,
            bounds,
            deadline: None,
        }
    }

    /// Supplies an optional provider turn reference for capability validation.
    ///
    /// The current OpenCode route rejects turn-scoped reconciliation.
    #[must_use]
    pub fn with_provider_turn_ref(mut self, provider_turn_ref: TurnRef) -> Self {
        self.provider_turn_ref = Some(provider_turn_ref);
        self
    }

    /// Adds a caller-owned reconciliation deadline.
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
        SessionResumeBinding,
        RuntimeTurnId,
        Option<TurnRef>,
        ProviderSessionReconciliationBounds,
        Option<Deadline>,
    ) {
        (
            self.request_id,
            self.model,
            self.binding,
            self.interrupted_turn_id,
            self.provider_turn_ref,
            self.bounds,
            self.deadline,
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
/// Inputs for listing retained OpenCode sessions within one working resource.
pub struct OpenCodeSessionCatalogueInput {
    request_id: RequestId,
    catalogue_id: ProviderSessionCatalogueId,
    working_resource: WorkingResourceRef,
    bounds: ProviderSessionCatalogueBounds,
    deadline: Option<Deadline>,
}

impl OpenCodeSessionCatalogueInput {
    /// Creates a bounded, working-resource-scoped session catalogue input.
    #[must_use]
    pub const fn new(
        request_id: RequestId,
        catalogue_id: ProviderSessionCatalogueId,
        working_resource: WorkingResourceRef,
        bounds: ProviderSessionCatalogueBounds,
    ) -> Self {
        Self {
            request_id,
            catalogue_id,
            working_resource,
            bounds,
            deadline: None,
        }
    }

    /// Adds a caller-owned catalogue deadline.
    #[must_use]
    pub const fn with_deadline(mut self, deadline: Deadline) -> Self {
        self.deadline = Some(deadline);
        self
    }

    pub(super) fn into_parts(
        self,
    ) -> (
        RequestId,
        ProviderSessionCatalogueId,
        WorkingResourceRef,
        ProviderSessionCatalogueBounds,
        Option<Deadline>,
    ) {
        (
            self.request_id,
            self.catalogue_id,
            self.working_resource,
            self.bounds,
            self.deadline,
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
/// Inputs for one read-only OpenCode model-catalogue request.
pub struct OpenCodeCatalogueProfileInput {
    request_id: RequestId,
    deadline: Option<Deadline>,
}

impl OpenCodeCatalogueProfileInput {
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
/// Exact provider and model-route identity selected for OpenCode execution.
pub struct OpenCodeModelSelection {
    route_id: ModelRouteId,
    route_revision: ModelRouteRevision,
    provider_id: ProviderId,
    model_id: ModelId,
    catalogue_entry: Option<ModelCatalogEntry>,
}

impl OpenCodeModelSelection {
    /// Creates a model selection without inferring a default route.
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

    /// Attaches catalogue evidence for model-specific capabilities and limits.
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
/// Inputs for preparing one interactive OpenCode session.
pub struct OpenCodeSessionProfileInput {
    request_id: RequestId,
    model: OpenCodeModelSelection,
    working_resource: WorkingResourceRef,
    deadline: Option<Deadline>,
    image_attachments: bool,
    provider_callbacks: bool,
    active_turn_detachment: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
/// Inputs for a separately authorized inactive-session management action.
pub struct OpenCodeSessionManagementInput {
    request_id: RequestId,
    binding: ProviderSessionManagementBinding,
    deadline: Option<Deadline>,
    allow_unverified_newer: bool,
}

impl OpenCodeSessionManagementInput {
    /// Creates a management input bound to one exact provider session.
    #[must_use]
    pub const fn new(request_id: RequestId, binding: ProviderSessionManagementBinding) -> Self {
        Self {
            request_id,
            binding,
            deadline: None,
            allow_unverified_newer: false,
        }
    }

    /// Adds a caller-owned management deadline.
    #[must_use]
    pub const fn with_deadline(mut self, deadline: Deadline) -> Self {
        self.deadline = Some(deadline);
        self
    }

    /// Explicitly permits management against an unverified-newer server.
    ///
    /// This opt-in does not upgrade the server's compatibility assessment.
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
    /// Creates the default read-only interactive session profile.
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
            active_turn_detachment: false,
        }
    }

    /// Enables image attachment transport for the prepared session.
    #[must_use]
    pub const fn with_image_attachments(mut self) -> Self {
        self.image_attachments = true;
        self
    }

    /// Enables consumer-mediated permission and question callbacks.
    #[must_use]
    pub const fn with_provider_callbacks(mut self) -> Self {
        self.provider_callbacks = true;
        self
    }

    /// Requests durable provider state and active-turn detachment support.
    #[must_use]
    pub const fn with_active_turn_detachment(mut self) -> Self {
        self.active_turn_detachment = true;
        self
    }

    /// Adds a caller-owned session deadline.
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
        bool,
    ) {
        (
            self.request_id,
            self.model,
            self.working_resource,
            self.deadline,
            self.image_attachments,
            self.provider_callbacks,
            self.active_turn_detachment,
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
/// Inputs for preparing one OpenCode structured run.
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
    /// Creates a run input with explicit model, content, and working resource.
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

    /// Replaces the run's ordered attachment set.
    #[must_use]
    pub fn with_attachments(
        mut self,
        attachments: impl IntoIterator<Item = AttachmentDescriptor>,
    ) -> Self {
        self.attachments = attachments.into_iter().collect();
        self
    }

    /// Enables consumer-mediated permission and question callbacks.
    #[must_use]
    pub const fn with_provider_callbacks(mut self) -> Self {
        self.provider_callbacks = true;
        self
    }

    /// Adds a caller-owned run deadline.
    #[must_use]
    pub const fn with_deadline(mut self, deadline: Deadline) -> Self {
        self.deadline = Some(deadline);
        self
    }

    /// Selects an explicit portable reasoning mode.
    #[must_use]
    pub fn with_reasoning_mode(mut self, reasoning: ReasoningMode) -> Self {
        self.reasoning = Some(reasoning);
        self
    }

    /// Requests schema-constrained structured output.
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

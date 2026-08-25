use crate::CodexModelVerbosity;
use swallowtail_core::{
    ExternalNetworkPolicy, ExternalSearchPolicy, ModelId, ModelRouteId, ModelRouteRevision,
    ProviderSessionCatalogueBounds, ReasoningMode, TurnRef,
};
use swallowtail_runtime::{
    AttachmentDescriptor, Deadline, OperationContent, ProviderSessionCatalogueId,
    ProviderSessionHistoryBounds, ProviderSessionHistoryId, ProviderSessionManagementBinding,
    ProviderSessionReconciliationBounds, RequestId, RuntimeTurnId, SessionOptions,
    SessionResumeBinding, StructuredOutputDescriptor, ToolDeclaration, WorkingResourceRef,
};

#[derive(Clone, Debug, Eq, PartialEq)]
/// Inputs for one newest-first Codex provider-session history page plan.
pub struct CodexSessionHistoryInput {
    request_id: RequestId,
    history_id: ProviderSessionHistoryId,
    model: CodexModelSelection,
    binding: SessionResumeBinding,
    bounds: ProviderSessionHistoryBounds,
    deadline: Option<Deadline>,
}

impl CodexSessionHistoryInput {
    /// Creates bounded history-page input from an exact durable session binding.
    #[must_use]
    pub const fn new(
        request_id: RequestId,
        history_id: ProviderSessionHistoryId,
        model: CodexModelSelection,
        binding: SessionResumeBinding,
        bounds: ProviderSessionHistoryBounds,
    ) -> Self {
        Self {
            request_id,
            history_id,
            model,
            binding,
            bounds,
            deadline: None,
        }
    }

    /// Adds a history-page deadline.
    #[must_use]
    pub const fn with_deadline(mut self, deadline: Deadline) -> Self {
        self.deadline = Some(deadline);
        self
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        RequestId,
        ProviderSessionHistoryId,
        CodexModelSelection,
        SessionResumeBinding,
        ProviderSessionHistoryBounds,
        Option<Deadline>,
    ) {
        (
            self.request_id,
            self.history_id,
            self.model,
            self.binding,
            self.bounds,
            self.deadline,
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
/// Inputs for read-only reconciliation of one interrupted Codex thread turn.
pub struct CodexSessionReconciliationInput {
    request_id: RequestId,
    model: CodexModelSelection,
    binding: SessionResumeBinding,
    interrupted_turn_id: RuntimeTurnId,
    provider_turn_ref: Option<TurnRef>,
    bounds: ProviderSessionReconciliationBounds,
    deadline: Option<Deadline>,
}

impl CodexSessionReconciliationInput {
    /// Creates bounded reconciliation input from an exact durable session binding.
    #[must_use]
    pub const fn new(
        request_id: RequestId,
        model: CodexModelSelection,
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

    /// Adds the exact provider turn reference when the interrupted turn is known.
    #[must_use]
    pub fn with_provider_turn_ref(mut self, provider_turn_ref: TurnRef) -> Self {
        self.provider_turn_ref = Some(provider_turn_ref);
        self
    }

    /// Adds a reconciliation deadline.
    #[must_use]
    pub const fn with_deadline(mut self, deadline: Deadline) -> Self {
        self.deadline = Some(deadline);
        self
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        RequestId,
        CodexModelSelection,
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
/// Exact model route for a prepared Codex operation.
pub struct CodexModelSelection {
    route_id: ModelRouteId,
    route_revision: ModelRouteRevision,
    model_id: ModelId,
}

impl CodexModelSelection {
    /// Creates an exact Codex model selection.
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

    pub(crate) const fn route_id(&self) -> &ModelRouteId {
        &self.route_id
    }

    pub(crate) const fn route_revision(&self) -> &ModelRouteRevision {
        &self.route_revision
    }

    pub(crate) const fn model_id(&self) -> &ModelId {
        &self.model_id
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
/// Consumer inputs for one prepared Codex app-server session.
pub struct CodexSessionProfileInput {
    request_id: RequestId,
    model: CodexModelSelection,
    working_resource: WorkingResourceRef,
    deadline: Option<Deadline>,
    options: SessionOptions,
    user_input_exchange: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
/// Consumer inputs for one inactive Codex thread-management action.
pub struct CodexSessionManagementInput {
    request_id: RequestId,
    binding: ProviderSessionManagementBinding,
    deadline: Option<Deadline>,
    allow_unverified_newer: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
/// Consumer inputs for one bounded Codex thread-catalogue request.
pub struct CodexSessionCatalogueInput {
    request_id: RequestId,
    catalogue_id: ProviderSessionCatalogueId,
    working_resource: WorkingResourceRef,
    bounds: ProviderSessionCatalogueBounds,
    deadline: Option<Deadline>,
}

impl CodexSessionCatalogueInput {
    /// Creates a working-resource-scoped thread catalogue request.
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

    /// Adds a thread-catalogue deadline.
    #[must_use]
    pub const fn with_deadline(mut self, deadline: Deadline) -> Self {
        self.deadline = Some(deadline);
        self
    }

    pub(crate) fn into_parts(
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

impl CodexSessionManagementInput {
    /// Creates an inactive-thread management request.
    #[must_use]
    pub const fn new(request_id: RequestId, binding: ProviderSessionManagementBinding) -> Self {
        Self {
            request_id,
            binding,
            deadline: None,
            allow_unverified_newer: false,
        }
    }

    /// Adds a lifecycle-operation deadline.
    #[must_use]
    pub const fn with_deadline(mut self, deadline: Deadline) -> Self {
        self.deadline = Some(deadline);
        self
    }

    /// Explicitly admits an unverified-newer lifecycle implementation.
    #[must_use]
    pub const fn allow_unverified_newer(mut self) -> Self {
        self.allow_unverified_newer = true;
        self
    }

    pub(crate) fn into_parts(
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

impl CodexSessionProfileInput {
    /// Creates a session profile with explicit model, workspace, deadline, and options.
    #[must_use]
    pub const fn new(
        request_id: RequestId,
        model: CodexModelSelection,
        working_resource: WorkingResourceRef,
        deadline: Option<Deadline>,
        options: SessionOptions,
    ) -> Self {
        Self {
            request_id,
            model,
            working_resource,
            deadline,
            options,
            user_input_exchange: false,
        }
    }

    /// Enables typed provider-to-consumer user-input exchange for the session.
    #[must_use]
    pub const fn with_user_input_exchange(mut self) -> Self {
        self.user_input_exchange = true;
        self
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        RequestId,
        CodexModelSelection,
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
            self.user_input_exchange,
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
/// Consumer inputs for one prepared Codex exec run.
pub struct CodexExecProfileInput {
    request_id: RequestId,
    content: OperationContent,
    model: CodexModelSelection,
    working_resource: WorkingResourceRef,
    external_network: ExternalNetworkPolicy,
    external_search: ExternalSearchPolicy,
    reasoning_mode: Option<ReasoningMode>,
    model_verbosity: Option<CodexModelVerbosity>,
    deadline: Option<Deadline>,
    attachments: Vec<AttachmentDescriptor>,
    tools: Vec<ToolDeclaration>,
    structured_output: Option<StructuredOutputDescriptor>,
}

impl CodexExecProfileInput {
    /// Creates an exec profile with explicit model and network/search policy.
    #[must_use]
    pub const fn new(
        request_id: RequestId,
        content: OperationContent,
        model: CodexModelSelection,
        working_resource: WorkingResourceRef,
        external_network: ExternalNetworkPolicy,
        external_search: ExternalSearchPolicy,
    ) -> Self {
        Self {
            request_id,
            content,
            model,
            working_resource,
            external_network,
            external_search,
            reasoning_mode: None,
            model_verbosity: None,
            deadline: None,
            attachments: Vec::new(),
            tools: Vec::new(),
            structured_output: None,
        }
    }

    /// Selects the requested reasoning mode.
    #[must_use]
    pub fn with_reasoning_mode(mut self, mode: ReasoningMode) -> Self {
        self.reasoning_mode = Some(mode);
        self
    }

    /// Selects one closed adapter-local `model_verbosity` value.
    #[must_use]
    pub const fn with_model_verbosity(mut self, verbosity: CodexModelVerbosity) -> Self {
        self.model_verbosity = Some(verbosity);
        self
    }

    /// Adds a run deadline.
    #[must_use]
    pub const fn with_deadline(mut self, deadline: Deadline) -> Self {
        self.deadline = Some(deadline);
        self
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

    /// Replaces the run's dynamic tool declarations.
    #[must_use]
    pub fn with_tools(mut self, tools: impl IntoIterator<Item = ToolDeclaration>) -> Self {
        self.tools = tools.into_iter().collect();
        self
    }

    /// Requests output conforming to the supplied structured-output descriptor.
    #[must_use]
    pub fn with_structured_output(mut self, output: StructuredOutputDescriptor) -> Self {
        self.structured_output = Some(output);
        self
    }

    pub(crate) fn into_parts(self) -> CodexExecProfileParts {
        CodexExecProfileParts {
            request_id: self.request_id,
            content: self.content,
            model: self.model,
            working_resource: self.working_resource,
            external_network: self.external_network,
            external_search: self.external_search,
            reasoning_mode: self.reasoning_mode,
            model_verbosity: self.model_verbosity,
            deadline: self.deadline,
            attachments: self.attachments,
            tools: self.tools,
            structured_output: self.structured_output,
        }
    }
}

pub(crate) struct CodexExecProfileParts {
    pub request_id: RequestId,
    pub content: OperationContent,
    pub model: CodexModelSelection,
    pub working_resource: WorkingResourceRef,
    pub external_network: ExternalNetworkPolicy,
    pub external_search: ExternalSearchPolicy,
    pub reasoning_mode: Option<ReasoningMode>,
    pub model_verbosity: Option<CodexModelVerbosity>,
    pub deadline: Option<Deadline>,
    pub attachments: Vec<AttachmentDescriptor>,
    pub tools: Vec<ToolDeclaration>,
    pub structured_output: Option<StructuredOutputDescriptor>,
}

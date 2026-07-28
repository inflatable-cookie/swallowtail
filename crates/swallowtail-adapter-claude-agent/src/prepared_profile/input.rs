use swallowtail_core::{ModelId, ModelRouteId, ModelRouteRevision, ReasoningMode};
use swallowtail_runtime::{
    Deadline, OperationContent, ProviderSessionManagementBinding, RequestId, SessionOptions,
    WorkingResourceRef,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ClaudeAgentModelSelection {
    route_id: ModelRouteId,
    route_revision: ModelRouteRevision,
    model_id: ModelId,
}

impl ClaudeAgentModelSelection {
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
pub struct ClaudeAgentSessionProfileInput {
    request_id: RequestId,
    model: ClaudeAgentModelSelection,
    working_resource: WorkingResourceRef,
    options: SessionOptions,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ClaudeAgentRunProfileInput {
    request_id: RequestId,
    model: ClaudeAgentModelSelection,
    content: OperationContent,
    working_resource: WorkingResourceRef,
    deadline: Option<Deadline>,
    reasoning_mode: Option<ReasoningMode>,
}

impl ClaudeAgentRunProfileInput {
    #[must_use]
    pub const fn new(
        request_id: RequestId,
        model: ClaudeAgentModelSelection,
        content: OperationContent,
        working_resource: WorkingResourceRef,
        deadline: Option<Deadline>,
    ) -> Self {
        Self {
            request_id,
            model,
            content,
            working_resource,
            deadline,
            reasoning_mode: None,
        }
    }

    #[must_use]
    pub fn with_reasoning_mode(mut self, reasoning_mode: ReasoningMode) -> Self {
        self.reasoning_mode = Some(reasoning_mode);
        self
    }

    pub(super) fn into_parts(
        self,
    ) -> (
        RequestId,
        ClaudeAgentModelSelection,
        OperationContent,
        WorkingResourceRef,
        Option<Deadline>,
        Option<ReasoningMode>,
    ) {
        (
            self.request_id,
            self.model,
            self.content,
            self.working_resource,
            self.deadline,
            self.reasoning_mode,
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ClaudeAgentSessionManagementInput {
    request_id: RequestId,
    binding: ProviderSessionManagementBinding,
    deadline: Option<Deadline>,
    allow_unverified_newer: bool,
}

impl ClaudeAgentSessionManagementInput {
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

impl ClaudeAgentSessionProfileInput {
    #[must_use]
    pub const fn new(
        request_id: RequestId,
        model: ClaudeAgentModelSelection,
        working_resource: WorkingResourceRef,
        options: SessionOptions,
    ) -> Self {
        Self {
            request_id,
            model,
            working_resource,
            options,
        }
    }

    pub(super) fn into_parts(
        self,
    ) -> (
        RequestId,
        ClaudeAgentModelSelection,
        WorkingResourceRef,
        SessionOptions,
    ) {
        (
            self.request_id,
            self.model,
            self.working_resource,
            self.options,
        )
    }
}

use swallowtail_core::{ModelId, ModelRouteId, ModelRouteRevision, ReasoningMode};
use swallowtail_runtime::{
    Deadline, OperationContent, ProviderSessionManagementBinding, RequestId, SessionOptions,
    WorkingResourceRef,
};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
/// Policy for permission requests emitted by Claude Agent ACP.
pub enum ClaudeAgentPermissionHandling {
    /// Reject the request and terminate the operation safely.
    #[default]
    RejectAndStop,
    /// Expose the request through Swallowtail's exact callback exchange.
    ConsumerMediated,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
/// Provider-session retention policy for a Claude Agent structured run.
pub enum ClaudeAgentRunRetention {
    /// Preserve the provider-owned session after the run.
    #[default]
    Durable,
    /// Delete the run-owned temporary session during joined cleanup.
    TemporaryWithOwnedSessionCleanup,
}

#[derive(Clone, Debug, Eq, PartialEq)]
/// Exact model route for a Claude Agent ACP operation.
pub struct ClaudeAgentModelSelection {
    route_id: ModelRouteId,
    route_revision: ModelRouteRevision,
    model_id: ModelId,
}

impl ClaudeAgentModelSelection {
    /// Creates an exact Claude Agent model selection.
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
/// Consumer inputs for one prepared Claude Agent interactive session.
pub struct ClaudeAgentSessionProfileInput {
    request_id: RequestId,
    model: ClaudeAgentModelSelection,
    working_resource: WorkingResourceRef,
    options: SessionOptions,
    permission_handling: ClaudeAgentPermissionHandling,
}

#[derive(Clone, Debug, Eq, PartialEq)]
/// Consumer inputs for one prepared Claude Agent structured run.
pub struct ClaudeAgentRunProfileInput {
    request_id: RequestId,
    model: ClaudeAgentModelSelection,
    content: OperationContent,
    working_resource: WorkingResourceRef,
    deadline: Option<Deadline>,
    reasoning_mode: Option<ReasoningMode>,
    permission_handling: ClaudeAgentPermissionHandling,
    retention: ClaudeAgentRunRetention,
}

impl ClaudeAgentRunProfileInput {
    /// Creates a durable run profile that rejects unattended permission requests.
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
            permission_handling: ClaudeAgentPermissionHandling::RejectAndStop,
            retention: ClaudeAgentRunRetention::Durable,
        }
    }

    /// Selects the requested reasoning mode.
    #[must_use]
    pub fn with_reasoning_mode(mut self, reasoning_mode: ReasoningMode) -> Self {
        self.reasoning_mode = Some(reasoning_mode);
        self
    }

    /// Enables exact consumer-mediated permission callbacks.
    #[must_use]
    pub const fn with_consumer_mediated_permissions(mut self) -> Self {
        self.permission_handling = ClaudeAgentPermissionHandling::ConsumerMediated;
        self
    }

    /// Selects temporary provider-session retention with owned cleanup.
    #[must_use]
    pub const fn with_owned_session_cleanup(mut self) -> Self {
        self.retention = ClaudeAgentRunRetention::TemporaryWithOwnedSessionCleanup;
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
        ClaudeAgentPermissionHandling,
        ClaudeAgentRunRetention,
    ) {
        (
            self.request_id,
            self.model,
            self.content,
            self.working_resource,
            self.deadline,
            self.reasoning_mode,
            self.permission_handling,
            self.retention,
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
/// Consumer inputs for deleting one inactive Claude Agent session.
pub struct ClaudeAgentSessionManagementInput {
    request_id: RequestId,
    binding: ProviderSessionManagementBinding,
    deadline: Option<Deadline>,
    allow_unverified_newer: bool,
}

impl ClaudeAgentSessionManagementInput {
    /// Creates an inactive-session management request.
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
    /// Creates a session profile that rejects unattended permission requests.
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
            permission_handling: ClaudeAgentPermissionHandling::RejectAndStop,
        }
    }

    /// Enables exact consumer-mediated permission callbacks for the session.
    #[must_use]
    pub const fn with_consumer_mediated_permissions(mut self) -> Self {
        self.permission_handling = ClaudeAgentPermissionHandling::ConsumerMediated;
        self
    }

    pub(super) fn into_parts(
        self,
    ) -> (
        RequestId,
        ClaudeAgentModelSelection,
        WorkingResourceRef,
        SessionOptions,
        ClaudeAgentPermissionHandling,
    ) {
        (
            self.request_id,
            self.model,
            self.working_resource,
            self.options,
            self.permission_handling,
        )
    }
}

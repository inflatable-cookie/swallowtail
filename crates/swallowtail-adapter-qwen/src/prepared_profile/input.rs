use swallowtail_core::{ModelId, ModelRouteId, ModelRouteRevision, ProviderId};
use swallowtail_runtime::{Deadline, OperationContent, RequestId, WorkingResourceRef};

#[derive(Clone, Debug, Eq, PartialEq)]
/// Exact provider and model route for a Qwen operation.
pub struct QwenModelSelection {
    route_id: ModelRouteId,
    route_revision: ModelRouteRevision,
    provider_id: ProviderId,
    model_id: ModelId,
}

impl QwenModelSelection {
    /// Creates an exact Qwen model selection.
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
/// Consumer inputs for one prepared Qwen structured run.
pub struct QwenRunProfileInput {
    request_id: RequestId,
    model: QwenModelSelection,
    content: OperationContent,
    working_resource: WorkingResourceRef,
    deadline: Deadline,
}

#[derive(Clone, Debug, Eq, PartialEq)]
/// Consumer inputs for one prepared Qwen interactive session.
pub struct QwenSessionProfileInput {
    request_id: RequestId,
    model: QwenModelSelection,
    working_resource: WorkingResourceRef,
    deadline: Option<Deadline>,
}

impl QwenSessionProfileInput {
    /// Creates a Qwen session profile without a deadline.
    #[must_use]
    pub const fn new(
        request_id: RequestId,
        model: QwenModelSelection,
        working_resource: WorkingResourceRef,
    ) -> Self {
        Self {
            request_id,
            model,
            working_resource,
            deadline: None,
        }
    }

    /// Adds a deadline to the turn-scoped session.
    #[must_use]
    pub const fn with_deadline(mut self, deadline: Deadline) -> Self {
        self.deadline = Some(deadline);
        self
    }

    pub(super) fn into_parts(
        self,
    ) -> (
        RequestId,
        QwenModelSelection,
        WorkingResourceRef,
        Option<Deadline>,
    ) {
        (
            self.request_id,
            self.model,
            self.working_resource,
            self.deadline,
        )
    }
}

impl QwenRunProfileInput {
    /// Creates a bounded Qwen structured-run profile.
    #[must_use]
    pub const fn new(
        request_id: RequestId,
        model: QwenModelSelection,
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
        }
    }

    pub(super) fn into_parts(
        self,
    ) -> (
        RequestId,
        QwenModelSelection,
        OperationContent,
        WorkingResourceRef,
        Deadline,
    ) {
        (
            self.request_id,
            self.model,
            self.content,
            self.working_resource,
            self.deadline,
        )
    }
}

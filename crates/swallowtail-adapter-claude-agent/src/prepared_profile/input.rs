use swallowtail_core::{ModelId, ModelRouteId, ModelRouteRevision};
use swallowtail_runtime::{RequestId, SessionOptions, WorkingResourceRef};

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

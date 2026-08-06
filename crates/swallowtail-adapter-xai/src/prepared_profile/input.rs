use swallowtail_core::{ModelId, ModelRouteId, ModelRouteRevision};
use swallowtail_runtime::{Deadline, OperationContent, RequestId};

#[derive(Clone, Debug, Eq, PartialEq)]
/// Exact xAI model-route selection supplied by the consumer.
pub struct XaiModelSelection {
    route_id: ModelRouteId,
    route_revision: ModelRouteRevision,
    model_id: ModelId,
}

#[derive(Clone, Debug, Eq, PartialEq)]
/// Consumer input for one xAI Responses structured run.
pub struct XaiRunProfileInput {
    request_id: RequestId,
    model: XaiModelSelection,
    content: OperationContent,
    deadline: Option<Deadline>,
}

impl XaiRunProfileInput {
    #[must_use]
    /// Creates a structured-run input with an optional exact deadline.
    pub const fn new(
        request_id: RequestId,
        model: XaiModelSelection,
        content: OperationContent,
        deadline: Option<Deadline>,
    ) -> Self {
        Self {
            request_id,
            model,
            content,
            deadline,
        }
    }

    pub(super) fn into_parts(
        self,
    ) -> (
        RequestId,
        XaiModelSelection,
        OperationContent,
        Option<Deadline>,
    ) {
        (self.request_id, self.model, self.content, self.deadline)
    }
}

impl XaiModelSelection {
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
}

#[derive(Clone, Debug, Eq, PartialEq)]
/// Consumer input for a serial xAI Responses session.
pub struct XaiSessionProfileInput {
    request_id: RequestId,
    model: XaiModelSelection,
    deadline: Option<Deadline>,
}

impl XaiSessionProfileInput {
    #[must_use]
    /// Creates session input with an optional exact deadline.
    pub const fn new(
        request_id: RequestId,
        model: XaiModelSelection,
        deadline: Option<Deadline>,
    ) -> Self {
        Self {
            request_id,
            model,
            deadline,
        }
    }

    pub(super) fn into_parts(self) -> (RequestId, XaiModelSelection, Option<Deadline>) {
        (self.request_id, self.model, self.deadline)
    }
}

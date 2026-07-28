use swallowtail_core::{ModelId, ModelRouteId, ModelRouteRevision, SessionProviderStatePolicy};
use swallowtail_runtime::{Deadline, OperationContent, RequestId};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AlibabaConversationProfileInput {
    request_id: RequestId,
    route_id: ModelRouteId,
    route_revision: ModelRouteRevision,
    model_id: ModelId,
    provider_state: SessionProviderStatePolicy,
    deadline: Option<Deadline>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AlibabaRunProfileInput {
    request_id: RequestId,
    route_id: ModelRouteId,
    route_revision: ModelRouteRevision,
    model_id: ModelId,
    content: OperationContent,
    deadline: Option<Deadline>,
}

impl AlibabaRunProfileInput {
    #[must_use]
    pub const fn new(
        request_id: RequestId,
        route_id: ModelRouteId,
        route_revision: ModelRouteRevision,
        model_id: ModelId,
        content: OperationContent,
    ) -> Self {
        Self {
            request_id,
            route_id,
            route_revision,
            model_id,
            content,
            deadline: None,
        }
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
        ModelRouteId,
        ModelRouteRevision,
        ModelId,
        OperationContent,
        Option<Deadline>,
    ) {
        (
            self.request_id,
            self.route_id,
            self.route_revision,
            self.model_id,
            self.content,
            self.deadline,
        )
    }
}

impl AlibabaConversationProfileInput {
    #[must_use]
    pub const fn new(
        request_id: RequestId,
        route_id: ModelRouteId,
        route_revision: ModelRouteRevision,
        model_id: ModelId,
        provider_state: SessionProviderStatePolicy,
    ) -> Self {
        Self {
            request_id,
            route_id,
            route_revision,
            model_id,
            provider_state,
            deadline: None,
        }
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
        ModelRouteId,
        ModelRouteRevision,
        ModelId,
        SessionProviderStatePolicy,
        Option<Deadline>,
    ) {
        (
            self.request_id,
            self.route_id,
            self.route_revision,
            self.model_id,
            self.provider_state,
            self.deadline,
        )
    }
}

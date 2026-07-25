use swallowtail_core::{ModelId, ModelRouteId, ModelRouteRevision, ProviderId};
use swallowtail_runtime::{Deadline, RequestId, WorkingResourceRef};

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
pub struct OpenCodeSessionProfileInput {
    request_id: RequestId,
    model: OpenCodeModelSelection,
    working_resource: WorkingResourceRef,
    deadline: Option<Deadline>,
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
        OpenCodeModelSelection,
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

use std::num::NonZeroU64;
use swallowtail_core::{
    AccessProfile, ConfiguredInstanceId, ExecutionHostId, InstanceRevision, InstanceTargetRef,
    ModelId, ModelRouteId, ModelRouteRevision,
};
use swallowtail_runtime::{Deadline, OperationContent, PreparedAccessEvidence, RequestId};

#[derive(Clone)]
/// Inputs for admitting one externally managed llama.cpp endpoint.
pub struct LlamaCppAttachedPreparationInput {
    instance_id: ConfiguredInstanceId,
    instance_revision: InstanceRevision,
    execution_host: ExecutionHostId,
    endpoint_target: InstanceTargetRef,
    access: AccessProfile,
    evidence: PreparedAccessEvidence,
}

impl LlamaCppAttachedPreparationInput {
    /// Creates explicit endpoint and access inputs for attached preparation.
    #[must_use]
    pub const fn new(
        instance_id: ConfiguredInstanceId,
        instance_revision: InstanceRevision,
        execution_host: ExecutionHostId,
        endpoint_target: InstanceTargetRef,
        access: AccessProfile,
        evidence: PreparedAccessEvidence,
    ) -> Self {
        Self {
            instance_id,
            instance_revision,
            execution_host,
            endpoint_target,
            access,
            evidence,
        }
    }

    pub(super) fn into_parts(
        self,
    ) -> (
        ConfiguredInstanceId,
        InstanceRevision,
        ExecutionHostId,
        InstanceTargetRef,
        AccessProfile,
        PreparedAccessEvidence,
    ) {
        (
            self.instance_id,
            self.instance_revision,
            self.execution_host,
            self.endpoint_target,
            self.access,
            self.evidence,
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
/// Exact model route selected for catalogue-bound inference.
pub struct LlamaCppModelSelection {
    route_id: ModelRouteId,
    route_revision: ModelRouteRevision,
    model_id: ModelId,
}

impl LlamaCppModelSelection {
    /// Creates a model selection without inferring an alias or route.
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

    pub(crate) fn into_parts(self) -> (ModelRouteId, ModelRouteRevision, ModelId) {
        (self.route_id, self.route_revision, self.model_id)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
/// Inputs for one read-only attached-server model catalogue request.
pub struct LlamaCppCatalogueProfileInput {
    request_id: RequestId,
    deadline: Option<Deadline>,
}

impl LlamaCppCatalogueProfileInput {
    /// Creates a catalogue profile without a deadline.
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
/// Inputs for one bounded attached-server inference attempt.
pub struct LlamaCppInferenceProfileInput {
    request_id: RequestId,
    selection: LlamaCppModelSelection,
    content: OperationContent,
    maximum_output_tokens: NonZeroU64,
    deadline: Option<Deadline>,
}

impl LlamaCppInferenceProfileInput {
    /// Creates an attempt with explicit model, content, and output-token bound.
    #[must_use]
    pub const fn new(
        request_id: RequestId,
        selection: LlamaCppModelSelection,
        content: OperationContent,
        maximum_output_tokens: NonZeroU64,
    ) -> Self {
        Self {
            request_id,
            selection,
            content,
            maximum_output_tokens,
            deadline: None,
        }
    }

    /// Adds a caller-owned inference deadline.
    #[must_use]
    pub const fn with_deadline(mut self, deadline: Deadline) -> Self {
        self.deadline = Some(deadline);
        self
    }

    pub(super) fn into_parts(
        self,
    ) -> (
        RequestId,
        LlamaCppModelSelection,
        OperationContent,
        NonZeroU64,
        Option<Deadline>,
    ) {
        (
            self.request_id,
            self.selection,
            self.content,
            self.maximum_output_tokens,
            self.deadline,
        )
    }
}

use crate::BedrockCloudClientConfig;
use std::num::NonZeroU64;
use swallowtail_core::{
    AccessProfile, ConfiguredInstanceId, ExecutionHostId, InstanceRevision, InstanceTargetRef,
    ModelId, ModelRouteId, ModelRouteRevision, ProviderId,
};
use swallowtail_runtime::{Deadline, OperationContent, PreparedAccessEvidence, RequestId};

#[derive(Clone)]
pub struct BedrockRuntimePreparationInput {
    instance_id: ConfiguredInstanceId,
    instance_revision: InstanceRevision,
    execution_host: ExecutionHostId,
    endpoint_target: InstanceTargetRef,
    access: AccessProfile,
    evidence: PreparedAccessEvidence,
    cloud_client: BedrockCloudClientConfig,
}

impl BedrockRuntimePreparationInput {
    #[must_use]
    pub const fn new(
        instance_id: ConfiguredInstanceId,
        instance_revision: InstanceRevision,
        execution_host: ExecutionHostId,
        endpoint_target: InstanceTargetRef,
        access: AccessProfile,
        evidence: PreparedAccessEvidence,
        cloud_client: BedrockCloudClientConfig,
    ) -> Self {
        Self {
            instance_id,
            instance_revision,
            execution_host,
            endpoint_target,
            access,
            evidence,
            cloud_client,
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
        BedrockCloudClientConfig,
    ) {
        (
            self.instance_id,
            self.instance_revision,
            self.execution_host,
            self.endpoint_target,
            self.access,
            self.evidence,
            self.cloud_client,
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BedrockModelSelection {
    route_id: ModelRouteId,
    route_revision: ModelRouteRevision,
    model_id: ModelId,
    provider_id: ProviderId,
}

impl BedrockModelSelection {
    #[must_use]
    pub const fn new(
        route_id: ModelRouteId,
        route_revision: ModelRouteRevision,
        model_id: ModelId,
        provider_id: ProviderId,
    ) -> Self {
        Self {
            route_id,
            route_revision,
            model_id,
            provider_id,
        }
    }

    pub(super) fn into_parts(self) -> (ModelRouteId, ModelRouteRevision, ModelId, ProviderId) {
        (
            self.route_id,
            self.route_revision,
            self.model_id,
            self.provider_id,
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BedrockRuntimeProfileInput {
    request_id: RequestId,
    model: BedrockModelSelection,
    content: OperationContent,
    maximum_output_tokens: NonZeroU64,
    deadline: Option<Deadline>,
}

impl BedrockRuntimeProfileInput {
    #[must_use]
    pub const fn new(
        request_id: RequestId,
        model: BedrockModelSelection,
        content: OperationContent,
        maximum_output_tokens: NonZeroU64,
    ) -> Self {
        Self {
            request_id,
            model,
            content,
            maximum_output_tokens,
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
        BedrockModelSelection,
        OperationContent,
        NonZeroU64,
        Option<Deadline>,
    ) {
        (
            self.request_id,
            self.model,
            self.content,
            self.maximum_output_tokens,
            self.deadline,
        )
    }
}

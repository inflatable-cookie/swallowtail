use crate::BedrockCloudClientConfig;
use swallowtail_core::{
    AccessProfile, ConfiguredInstanceId, ExecutionHostId, InstanceRevision, InstanceTargetRef,
};
use swallowtail_runtime::{Deadline, PreparedAccessEvidence, RequestId};

#[derive(Clone)]
/// Instance, access, host, region, and delegated SDK inputs for catalogue preparation.
pub struct BedrockCataloguePreparationInput {
    instance_id: ConfiguredInstanceId,
    instance_revision: InstanceRevision,
    execution_host: ExecutionHostId,
    endpoint_target: InstanceTargetRef,
    access: AccessProfile,
    evidence: PreparedAccessEvidence,
    cloud_client: BedrockCloudClientConfig,
}

impl BedrockCataloguePreparationInput {
    #[must_use]
    /// Creates control-plane preparation input without provider effects.
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
/// Turn-scoped input for one bounded Bedrock catalogue request.
pub struct BedrockCatalogueProfileInput {
    request_id: RequestId,
    deadline: Option<Deadline>,
}

impl BedrockCatalogueProfileInput {
    #[must_use]
    /// Creates catalogue input with no deadline.
    pub const fn new(request_id: RequestId) -> Self {
        Self {
            request_id,
            deadline: None,
        }
    }

    #[must_use]
    /// Adds the operation deadline.
    pub const fn with_deadline(mut self, deadline: Deadline) -> Self {
        self.deadline = Some(deadline);
        self
    }

    pub(super) fn into_parts(self) -> (RequestId, Option<Deadline>) {
        (self.request_id, self.deadline)
    }
}

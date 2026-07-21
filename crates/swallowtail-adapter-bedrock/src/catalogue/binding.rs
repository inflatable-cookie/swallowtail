use crate::{BedrockCredentialProvider, BedrockRegion};
use swallowtail_core::{AccessProfileId, ConfiguredInstanceId, ExecutionHostId};
use swallowtail_runtime::CredentialRef;

/// Exact control-plane binding for one Bedrock catalogue instance.
#[derive(Clone, Debug)]
pub struct BedrockCatalogueBinding {
    instance: ConfiguredInstanceId,
    access_profile: AccessProfileId,
    credential: CredentialRef,
    execution_host: ExecutionHostId,
    region: BedrockRegion,
    provider: BedrockCredentialProvider,
}

impl BedrockCatalogueBinding {
    #[must_use]
    pub const fn new(
        instance: ConfiguredInstanceId,
        access_profile: AccessProfileId,
        credential: CredentialRef,
        execution_host: ExecutionHostId,
        region: BedrockRegion,
        provider: BedrockCredentialProvider,
    ) -> Self {
        Self {
            instance,
            access_profile,
            credential,
            execution_host,
            region,
            provider,
        }
    }

    pub(super) const fn instance(&self) -> &ConfiguredInstanceId {
        &self.instance
    }

    pub(super) const fn access_profile(&self) -> &AccessProfileId {
        &self.access_profile
    }

    pub(super) const fn credential(&self) -> &CredentialRef {
        &self.credential
    }

    pub(super) const fn execution_host(&self) -> &ExecutionHostId {
        &self.execution_host
    }

    pub(super) const fn region(&self) -> &BedrockRegion {
        &self.region
    }

    pub(super) const fn provider(&self) -> &BedrockCredentialProvider {
        &self.provider
    }
}

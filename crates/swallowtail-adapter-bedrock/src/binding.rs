use crate::failure::failure;
use aws_credential_types::provider::{ProvideCredentials, SharedCredentialsProvider};
use std::fmt;
use swallowtail_core::{AccessProfileId, ConfiguredInstanceId, ExecutionHostId};
use swallowtail_runtime::{CredentialRef, RuntimeFailure};

#[derive(Clone)]
pub struct BedrockCredentialProvider(SharedCredentialsProvider);

impl BedrockCredentialProvider {
    #[must_use]
    pub fn new(provider: impl ProvideCredentials + 'static) -> Self {
        Self(SharedCredentialsProvider::new(provider))
    }

    pub(crate) fn as_sdk_provider(&self) -> SharedCredentialsProvider {
        self.0.clone()
    }
}

impl fmt::Debug for BedrockCredentialProvider {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("BedrockCredentialProvider(<opaque>)")
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BedrockRegion(String);

impl BedrockRegion {
    pub fn new(value: impl Into<String>) -> Result<Self, RuntimeFailure> {
        let value = value.into();
        if value.is_empty()
            || value.len() > 64
            || !value
                .bytes()
                .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-')
        {
            return Err(failure(
                "swallowtail.bedrock.region_invalid",
                "Bedrock region was invalid",
            ));
        }
        Ok(Self(value))
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Debug)]
pub struct BedrockDriverBinding {
    instance: ConfiguredInstanceId,
    access_profile: AccessProfileId,
    credential: CredentialRef,
    execution_host: ExecutionHostId,
    region: BedrockRegion,
    provider: BedrockCredentialProvider,
}

impl BedrockDriverBinding {
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

    pub(crate) const fn instance(&self) -> &ConfiguredInstanceId {
        &self.instance
    }

    pub(crate) const fn access_profile(&self) -> &AccessProfileId {
        &self.access_profile
    }

    pub(crate) const fn credential(&self) -> &CredentialRef {
        &self.credential
    }

    pub(crate) const fn execution_host(&self) -> &ExecutionHostId {
        &self.execution_host
    }

    pub(crate) const fn region(&self) -> &BedrockRegion {
        &self.region
    }

    pub(crate) const fn provider(&self) -> &BedrockCredentialProvider {
        &self.provider
    }
}

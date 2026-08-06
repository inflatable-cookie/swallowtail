use crate::failure::failure;
use aws_credential_types::provider::{ProvideCredentials, SharedCredentialsProvider};
use std::fmt;
use swallowtail_core::{AccessProfileId, ConfiguredInstanceId, ExecutionHostId};
use swallowtail_runtime::{CredentialRef, RuntimeFailure};

#[derive(Clone)]
/// Opaque AWS credential provider delegated to the official SDK.
pub struct BedrockCredentialProvider(SharedCredentialsProvider);

impl BedrockCredentialProvider {
    #[must_use]
    /// Wraps a credential provider without extracting credential material.
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

/// Explicit SDK client inputs selected by the consumer or execution host.
///
/// This value does not consult the ambient AWS region or credential chains.
#[derive(Clone, Debug)]
pub struct BedrockCloudClientConfig {
    region: BedrockRegion,
    credential_provider: BedrockCredentialProvider,
}

impl BedrockCloudClientConfig {
    #[must_use]
    /// Creates explicit SDK client configuration for one region.
    pub const fn new(
        region: BedrockRegion,
        credential_provider: BedrockCredentialProvider,
    ) -> Self {
        Self {
            region,
            credential_provider,
        }
    }

    #[must_use]
    /// Returns the configured AWS region.
    pub const fn region(&self) -> &BedrockRegion {
        &self.region
    }

    pub(crate) fn into_parts(self) -> (BedrockRegion, BedrockCredentialProvider) {
        (self.region, self.credential_provider)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
/// Validated AWS region name used by a Bedrock SDK client.
pub struct BedrockRegion(String);

impl BedrockRegion {
    /// Validates and creates a bounded AWS region name.
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
    /// Returns the validated region string.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Debug)]
/// Exact Runtime driver binding across instance, access, host, region, and SDK credentials.
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
    /// Creates a Runtime driver binding without consulting ambient AWS state.
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

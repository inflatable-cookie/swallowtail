use crate::diagnostic::SafeDiagnostic;
use crate::{
    CatalogTimestamp, ConfiguredInstanceId, ExecutionHostId, InterfaceVersionBinding, ModelId,
};
use std::error::Error;
use std::fmt;

const MAX_MODEL_TAG_BYTES: usize = 256;
const SHA256_PREFIX: &str = "sha256:";
const SHA256_HEX_BYTES: usize = 64;

/// Source operation for one attached-runtime model observation.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum AttachedModelObservationScope {
    InstalledInventory,
    RunningInventory,
    SelectedModelDetail,
}

/// Runtime-native model tag. This is not a Swallowtail model or route identity.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct AttachedModelTag(String);

impl AttachedModelTag {
    pub fn new(value: impl Into<String>) -> Result<Self, InvalidAttachedRuntimeRecord> {
        let value = value.into();
        if value.trim().is_empty()
            || value.trim() != value
            || value.len() > MAX_MODEL_TAG_BYTES
            || value.chars().any(char::is_control)
        {
            return Err(InvalidAttachedRuntimeRecord::model_tag());
        }
        Ok(Self(value))
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Safe digest of one observed runtime manifest. The manifest stays private.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ModelManifestDigest(String);

impl ModelManifestDigest {
    pub fn new(value: impl Into<String>) -> Result<Self, InvalidAttachedRuntimeRecord> {
        let value = value.into();
        let Some(hex) = value.strip_prefix(SHA256_PREFIX) else {
            return Err(InvalidAttachedRuntimeRecord::manifest_digest());
        };
        if hex.len() != SHA256_HEX_BYTES || !hex.bytes().all(|byte| byte.is_ascii_hexdigit()) {
            return Err(InvalidAttachedRuntimeRecord::manifest_digest());
        }
        Ok(Self(value))
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Bounded evidence from one exact attached-runtime catalogue operation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AttachedModelObservation {
    scope: AttachedModelObservationScope,
    instance_id: ConfiguredInstanceId,
    execution_host_id: ExecutionHostId,
    runtime_version: InterfaceVersionBinding,
    observed_at: CatalogTimestamp,
    model_tag: AttachedModelTag,
    manifest_digest: Option<ModelManifestDigest>,
}

impl AttachedModelObservation {
    #[must_use]
    pub const fn new(
        scope: AttachedModelObservationScope,
        instance_id: ConfiguredInstanceId,
        execution_host_id: ExecutionHostId,
        runtime_version: InterfaceVersionBinding,
        observed_at: CatalogTimestamp,
        model_tag: AttachedModelTag,
    ) -> Self {
        Self {
            scope,
            instance_id,
            execution_host_id,
            runtime_version,
            observed_at,
            model_tag,
            manifest_digest: None,
        }
    }

    #[must_use]
    pub fn with_manifest_digest(mut self, digest: ModelManifestDigest) -> Self {
        self.manifest_digest = Some(digest);
        self
    }

    #[must_use]
    pub const fn scope(&self) -> AttachedModelObservationScope {
        self.scope
    }

    #[must_use]
    pub const fn instance_id(&self) -> &ConfiguredInstanceId {
        &self.instance_id
    }

    #[must_use]
    pub const fn execution_host_id(&self) -> &ExecutionHostId {
        &self.execution_host_id
    }

    #[must_use]
    pub const fn runtime_version(&self) -> &InterfaceVersionBinding {
        &self.runtime_version
    }

    #[must_use]
    pub const fn observed_at(&self) -> CatalogTimestamp {
        self.observed_at
    }

    #[must_use]
    pub const fn model_tag(&self) -> &AttachedModelTag {
        &self.model_tag
    }

    #[must_use]
    pub const fn manifest_digest(&self) -> Option<&ModelManifestDigest> {
        self.manifest_digest.as_ref()
    }
}

/// Accepted invocation side effect for an externally managed runtime.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum AttachedRuntimeResidency {
    /// The runtime may load, retain, or evict models. This grants no unload authority.
    RuntimeManaged,
}

/// Exact attached-runtime evidence required by one inference operation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AttachedRuntimeRequirements {
    runtime_version: InterfaceVersionBinding,
    model_id: ModelId,
    model_tag: AttachedModelTag,
    manifest_digest: ModelManifestDigest,
    residency: AttachedRuntimeResidency,
}

impl AttachedRuntimeRequirements {
    #[must_use]
    pub const fn new(
        runtime_version: InterfaceVersionBinding,
        model_id: ModelId,
        model_tag: AttachedModelTag,
        manifest_digest: ModelManifestDigest,
        residency: AttachedRuntimeResidency,
    ) -> Self {
        Self {
            runtime_version,
            model_id,
            model_tag,
            manifest_digest,
            residency,
        }
    }

    #[must_use]
    pub const fn runtime_version(&self) -> &InterfaceVersionBinding {
        &self.runtime_version
    }

    #[must_use]
    pub const fn model_id(&self) -> &ModelId {
        &self.model_id
    }

    #[must_use]
    pub const fn model_tag(&self) -> &AttachedModelTag {
        &self.model_tag
    }

    #[must_use]
    pub const fn manifest_digest(&self) -> &ModelManifestDigest {
        &self.manifest_digest
    }

    #[must_use]
    pub const fn residency(&self) -> AttachedRuntimeResidency {
        self.residency
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InvalidAttachedRuntimeRecord {
    diagnostic: SafeDiagnostic,
}

impl InvalidAttachedRuntimeRecord {
    fn model_tag() -> Self {
        Self {
            diagnostic: SafeDiagnostic::new(
                "swallowtail.attached_model_tag_rejected",
                "Attached model tag was empty, unsafe, or too long",
            ),
        }
    }

    fn manifest_digest() -> Self {
        Self {
            diagnostic: SafeDiagnostic::new(
                "swallowtail.model_manifest_digest_rejected",
                "Model manifest digest must be one SHA-256 value",
            ),
        }
    }

    #[must_use]
    pub const fn diagnostic(&self) -> &SafeDiagnostic {
        &self.diagnostic
    }
}

impl fmt::Display for InvalidAttachedRuntimeRecord {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.diagnostic.fmt(formatter)
    }
}

impl Error for InvalidAttachedRuntimeRecord {}

#[cfg(test)]
#[path = "attached_runtime/tests.rs"]
mod tests;

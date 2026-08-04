use crate::RuntimeRunId;
use sha2::{Digest, Sha256};
use std::error::Error;
use std::fmt;
use swallowtail_core::{CredentialMechanism, PreflightPlan, RunRef, SafeDiagnostic};

const MAGIC: &[u8; 16] = b"SWST-RUN-CHECKPT";
const VERSION: u16 = 1;
const DIGEST_BYTES: usize = 32;
const FINGERPRINT_BYTES: usize = 32;
const MAXIMUM_FIELD_BYTES: usize = 4 * 1024;
const MAXIMUM_RECORD_BYTES: usize = 16 * 1024;

/// Exact provider-owned run and adapter-private durable event position.
#[derive(Clone, Eq, PartialEq)]
pub struct ProviderRunCheckpoint {
    runtime_run_id: RuntimeRunId,
    provider_run_ref: RunRef,
    cursor: Vec<u8>,
    route_fingerprint: [u8; FINGERPRINT_BYTES],
}

impl ProviderRunCheckpoint {
    pub fn new(
        plan: &PreflightPlan,
        runtime_run_id: RuntimeRunId,
        provider_run_ref: RunRef,
        cursor: impl AsRef<[u8]>,
    ) -> Result<Self, ProviderRunCheckpointFailure> {
        let cursor = cursor.as_ref();
        if cursor.is_empty() || cursor.len() > MAXIMUM_FIELD_BYTES {
            return Err(invalid());
        }
        Ok(Self {
            runtime_run_id,
            provider_run_ref,
            cursor: cursor.to_vec(),
            route_fingerprint: route_fingerprint(plan).ok_or_else(attachment_mismatch)?,
        })
    }

    #[must_use]
    pub const fn runtime_run_id(&self) -> &RuntimeRunId {
        &self.runtime_run_id
    }

    #[must_use]
    pub const fn provider_run_ref(&self) -> &RunRef {
        &self.provider_run_ref
    }

    /// Passes the opaque cursor back to its owning provider adapter.
    #[must_use]
    pub fn cursor(&self) -> &[u8] {
        &self.cursor
    }

    pub fn export_persisted(
        &self,
        plan: &PreflightPlan,
    ) -> Result<PersistedProviderRunCheckpoint, ProviderRunCheckpointFailure> {
        let fingerprint = route_fingerprint(plan).ok_or_else(attachment_mismatch)?;
        if fingerprint != self.route_fingerprint {
            return Err(attachment_mismatch());
        }
        let fields = [
            self.runtime_run_id.as_str().as_bytes(),
            self.provider_run_ref.as_provider_value().as_bytes(),
            self.cursor.as_slice(),
        ];
        if fields
            .iter()
            .any(|field| field.is_empty() || field.len() > MAXIMUM_FIELD_BYTES)
        {
            return Err(oversized());
        }
        let mut payload = Vec::new();
        payload.extend_from_slice(MAGIC);
        payload.extend_from_slice(&VERSION.to_be_bytes());
        for field in fields {
            let length = u16::try_from(field.len()).map_err(|_| oversized())?;
            payload.extend_from_slice(&length.to_be_bytes());
            payload.extend_from_slice(field);
        }
        payload.extend_from_slice(&fingerprint);
        let digest = Sha256::digest(&payload);
        payload.extend_from_slice(&digest);
        PersistedProviderRunCheckpoint::from_bytes(payload)
    }

    pub fn restore_persisted(
        record: &PersistedProviderRunCheckpoint,
        plan: &PreflightPlan,
    ) -> Result<Self, ProviderRunCheckpointFailure> {
        let decoded = decode_record(record.as_bytes())?;
        if route_fingerprint(plan).ok_or_else(attachment_mismatch)? != decoded.fingerprint {
            return Err(attachment_mismatch());
        }
        Self::new(
            plan,
            RuntimeRunId::new(decoded.runtime_run_id).map_err(|_| invalid())?,
            RunRef::new(decoded.provider_run_ref).map_err(|_| invalid())?,
            decoded.cursor,
        )
    }

    #[must_use]
    pub(crate) fn matches_plan(&self, plan: &PreflightPlan) -> bool {
        route_fingerprint(plan) == Some(self.route_fingerprint)
    }
}

impl fmt::Debug for ProviderRunCheckpoint {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("ProviderRunCheckpoint(<opaque>)")
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct PersistedProviderRunCheckpoint(Vec<u8>);

impl PersistedProviderRunCheckpoint {
    pub fn from_bytes(bytes: impl AsRef<[u8]>) -> Result<Self, ProviderRunCheckpointFailure> {
        let bytes = bytes.as_ref();
        decode_record(bytes)?;
        Ok(Self(bytes.to_vec()))
    }

    #[must_use]
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}

impl fmt::Debug for PersistedProviderRunCheckpoint {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("PersistedProviderRunCheckpoint(<opaque>)")
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProviderRunCheckpointFailureKind {
    InvalidEncoding,
    UnsupportedVersion,
    Oversized,
    IntegrityMismatch,
    AttachmentMismatch,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProviderRunCheckpointFailure {
    kind: ProviderRunCheckpointFailureKind,
    diagnostic: SafeDiagnostic,
}

impl ProviderRunCheckpointFailure {
    fn new(
        kind: ProviderRunCheckpointFailureKind,
        code: &'static str,
        message: &'static str,
    ) -> Self {
        Self {
            kind,
            diagnostic: SafeDiagnostic::new(code, message),
        }
    }

    #[must_use]
    pub const fn kind(&self) -> ProviderRunCheckpointFailureKind {
        self.kind
    }

    #[must_use]
    pub const fn diagnostic(&self) -> &SafeDiagnostic {
        &self.diagnostic
    }
}

impl fmt::Display for ProviderRunCheckpointFailure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.diagnostic.fmt(formatter)
    }
}

impl Error for ProviderRunCheckpointFailure {}

struct DecodedRecord {
    runtime_run_id: String,
    provider_run_ref: String,
    cursor: Vec<u8>,
    fingerprint: [u8; FINGERPRINT_BYTES],
}

fn decode_record(bytes: &[u8]) -> Result<DecodedRecord, ProviderRunCheckpointFailure> {
    if bytes.len() > MAXIMUM_RECORD_BYTES {
        return Err(oversized());
    }
    if bytes.len() < MAGIC.len() + 2 + FINGERPRINT_BYTES + DIGEST_BYTES
        || &bytes[..MAGIC.len()] != MAGIC
    {
        return Err(invalid());
    }
    let payload_end = bytes.len() - DIGEST_BYTES;
    if Sha256::digest(&bytes[..payload_end]).as_slice() != &bytes[payload_end..] {
        return Err(integrity_mismatch());
    }
    let mut offset = MAGIC.len();
    let version = read_u16(bytes, &mut offset)?;
    if version != VERSION {
        return Err(unsupported_version());
    }
    let runtime_run_id = read_text(bytes, &mut offset, payload_end)?;
    let provider_run_ref = read_text(bytes, &mut offset, payload_end)?;
    let cursor = read_field(bytes, &mut offset, payload_end)?.to_vec();
    if offset + FINGERPRINT_BYTES != payload_end {
        return Err(invalid());
    }
    let mut fingerprint = [0_u8; FINGERPRINT_BYTES];
    fingerprint.copy_from_slice(&bytes[offset..payload_end]);
    Ok(DecodedRecord {
        runtime_run_id,
        provider_run_ref,
        cursor,
        fingerprint,
    })
}

fn read_u16(bytes: &[u8], offset: &mut usize) -> Result<u16, ProviderRunCheckpointFailure> {
    let end = offset.checked_add(2).ok_or_else(invalid)?;
    let value = bytes.get(*offset..end).ok_or_else(invalid)?;
    *offset = end;
    Ok(u16::from_be_bytes([value[0], value[1]]))
}

fn read_field<'a>(
    bytes: &'a [u8],
    offset: &mut usize,
    payload_end: usize,
) -> Result<&'a [u8], ProviderRunCheckpointFailure> {
    let length = usize::from(read_u16(bytes, offset)?);
    if length == 0 || length > MAXIMUM_FIELD_BYTES {
        return Err(invalid());
    }
    let end = offset.checked_add(length).ok_or_else(invalid)?;
    if end > payload_end {
        return Err(invalid());
    }
    let field = &bytes[*offset..end];
    *offset = end;
    Ok(field)
}

fn read_text(
    bytes: &[u8],
    offset: &mut usize,
    payload_end: usize,
) -> Result<String, ProviderRunCheckpointFailure> {
    std::str::from_utf8(read_field(bytes, offset, payload_end)?)
        .map(str::to_owned)
        .map_err(|_| invalid())
}

pub(crate) fn route_fingerprint(plan: &PreflightPlan) -> Option<[u8; FINGERPRINT_BYTES]> {
    let route = plan.model_route_id()?;
    let route_revision = plan.model_route_revision()?;
    let model = plan.model_id()?;
    let mut digest = Sha256::new();
    digest.update(b"swallowtail.provider-run-checkpoint.route.v1");
    for value in [
        plan.driver_identity().id().as_str(),
        plan.driver_identity().version().as_str(),
        plan.integration_family().as_str(),
        plan.transport_family().as_str(),
        plan.instance_id().as_str(),
        plan.instance_revision().as_str(),
        plan.instance_target_ref().as_host_value(),
        plan.execution_host_id().as_str(),
        plan.protocol_facade_id().as_str(),
        plan.instance_policy_id().as_str(),
        plan.access_profile_id().as_str(),
        plan.endpoint_audience().as_str(),
        route.as_str(),
        route_revision.as_str(),
        model.as_str(),
    ] {
        hash_text(&mut digest, value);
    }
    let (credential_tag, credential_extension) = match plan.credential_mechanism() {
        CredentialMechanism::InteractiveOauth => (0, None),
        CredentialMechanism::DeviceOauth => (1, None),
        CredentialMechanism::AutomationToken => (2, None),
        CredentialMechanism::ApiKey => (3, None),
        CredentialMechanism::WorkloadIdentity => (4, None),
        CredentialMechanism::CloudProviderIdentity => (5, None),
        CredentialMechanism::GatewayHelper => (6, None),
        CredentialMechanism::Unauthenticated => (7, None),
        CredentialMechanism::LocalUnauthenticated => (8, None),
        CredentialMechanism::ProviderSpecific(namespace) => (9, Some(namespace.as_str())),
    };
    digest.update([credential_tag]);
    if let Some(extension) = credential_extension {
        hash_text(&mut digest, extension);
    }
    if let Some(provider) = plan.provider_id() {
        hash_text(&mut digest, provider.as_str());
    }
    let mut versions = plan
        .interface_versions()
        .map(|version| (version.axis().as_str(), version.version().as_str()))
        .collect::<Vec<_>>();
    versions.sort_unstable();
    digest.update(
        u64::try_from(versions.len())
            .unwrap_or(u64::MAX)
            .to_be_bytes(),
    );
    for (axis, version) in versions {
        hash_text(&mut digest, axis);
        hash_text(&mut digest, version);
    }
    Some(digest.finalize().into())
}

fn hash_text(digest: &mut Sha256, value: &str) {
    digest.update(u64::try_from(value.len()).unwrap_or(u64::MAX).to_be_bytes());
    digest.update(value.as_bytes());
}

fn invalid() -> ProviderRunCheckpointFailure {
    ProviderRunCheckpointFailure::new(
        ProviderRunCheckpointFailureKind::InvalidEncoding,
        "swallowtail.provider_run_checkpoint.invalid",
        "Provider run checkpoint encoding is invalid",
    )
}

fn unsupported_version() -> ProviderRunCheckpointFailure {
    ProviderRunCheckpointFailure::new(
        ProviderRunCheckpointFailureKind::UnsupportedVersion,
        "swallowtail.provider_run_checkpoint.version_unsupported",
        "Provider run checkpoint version is unsupported",
    )
}

fn oversized() -> ProviderRunCheckpointFailure {
    ProviderRunCheckpointFailure::new(
        ProviderRunCheckpointFailureKind::Oversized,
        "swallowtail.provider_run_checkpoint.oversized",
        "Provider run checkpoint exceeds its bound",
    )
}

fn integrity_mismatch() -> ProviderRunCheckpointFailure {
    ProviderRunCheckpointFailure::new(
        ProviderRunCheckpointFailureKind::IntegrityMismatch,
        "swallowtail.provider_run_checkpoint.integrity_mismatch",
        "Provider run checkpoint integrity check failed",
    )
}

fn attachment_mismatch() -> ProviderRunCheckpointFailure {
    ProviderRunCheckpointFailure::new(
        ProviderRunCheckpointFailureKind::AttachmentMismatch,
        "swallowtail.provider_run_checkpoint.attachment_mismatch",
        "Provider run checkpoint does not match the prepared route",
    )
}

#[cfg(test)]
mod tests;

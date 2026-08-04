use crate::{RuntimeRunId, provider_run_checkpoint::route_fingerprint};
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;
use swallowtail_core::{OwnedRemoteResourceKind, PreflightPlan, RunRef, SafeDiagnostic};

const MAGIC: &[u8] = b"SWST-RSRC-CLEAN";
const VERSION: u16 = 1;
const DIGEST_BYTES: usize = 32;
const FINGERPRINT_BYTES: usize = 32;
const MAXIMUM_TEXT_BYTES: usize = 4 * 1024;
const MAXIMUM_BINDING_BYTES: usize = 16 * 1024;
const MAXIMUM_RECORD_BYTES: usize = 24 * 1024;
const MAXIMUM_RESOURCE_KINDS: usize = 8;

/// Exact runtime run, provider run, and adapter-private resources left by it.
#[derive(Clone, Eq, PartialEq)]
pub struct ProviderRecoveredResourceCleanupBinding {
    runtime_run_id: RuntimeRunId,
    provider_run_ref: RunRef,
    resource_kinds: BTreeSet<OwnedRemoteResourceKind>,
    provider_resource_binding: Vec<u8>,
    route_fingerprint: [u8; FINGERPRINT_BYTES],
}

impl ProviderRecoveredResourceCleanupBinding {
    pub fn new(
        plan: &PreflightPlan,
        runtime_run_id: RuntimeRunId,
        provider_run_ref: RunRef,
        resource_kinds: impl IntoIterator<Item = OwnedRemoteResourceKind>,
        provider_resource_binding: impl AsRef<[u8]>,
    ) -> Result<Self, ProviderRecoveredResourceCleanupBindingFailure> {
        let resource_kinds = resource_kinds.into_iter().collect::<BTreeSet<_>>();
        let provider_resource_binding = provider_resource_binding.as_ref();
        if resource_kinds.is_empty()
            || resource_kinds.len() > MAXIMUM_RESOURCE_KINDS
            || provider_resource_binding.is_empty()
            || provider_resource_binding.len() > MAXIMUM_BINDING_BYTES
        {
            return Err(invalid());
        }
        Ok(Self {
            runtime_run_id,
            provider_run_ref,
            resource_kinds,
            provider_resource_binding: provider_resource_binding.to_vec(),
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

    pub fn resource_kinds(&self) -> impl ExactSizeIterator<Item = OwnedRemoteResourceKind> + '_ {
        self.resource_kinds.iter().copied()
    }

    /// Passes the opaque binding back to its owning provider adapter.
    #[must_use]
    pub fn provider_resource_binding(&self) -> &[u8] {
        &self.provider_resource_binding
    }

    pub fn export_persisted(
        &self,
        plan: &PreflightPlan,
    ) -> Result<
        PersistedProviderRecoveredResourceCleanupBinding,
        ProviderRecoveredResourceCleanupBindingFailure,
    > {
        let fingerprint = route_fingerprint(plan).ok_or_else(attachment_mismatch)?;
        if fingerprint != self.route_fingerprint {
            return Err(attachment_mismatch());
        }
        let mut payload = Vec::new();
        payload.extend_from_slice(MAGIC);
        payload.extend_from_slice(&VERSION.to_be_bytes());
        write_text(&mut payload, self.runtime_run_id.as_str().as_bytes())?;
        write_text(
            &mut payload,
            self.provider_run_ref.as_provider_value().as_bytes(),
        )?;
        payload.push(u8::try_from(self.resource_kinds.len()).map_err(|_| oversized())?);
        for kind in &self.resource_kinds {
            payload.push(encode_kind(*kind));
        }
        write_field(&mut payload, &self.provider_resource_binding)?;
        payload.extend_from_slice(&fingerprint);
        let digest = Sha256::digest(&payload);
        payload.extend_from_slice(&digest);
        PersistedProviderRecoveredResourceCleanupBinding::from_bytes(payload)
    }

    pub fn restore_persisted(
        record: &PersistedProviderRecoveredResourceCleanupBinding,
        plan: &PreflightPlan,
    ) -> Result<Self, ProviderRecoveredResourceCleanupBindingFailure> {
        let decoded = decode_record(record.as_bytes())?;
        if route_fingerprint(plan).ok_or_else(attachment_mismatch)? != decoded.fingerprint {
            return Err(attachment_mismatch());
        }
        Self::new(
            plan,
            RuntimeRunId::new(decoded.runtime_run_id).map_err(|_| invalid())?,
            RunRef::new(decoded.provider_run_ref).map_err(|_| invalid())?,
            decoded.resource_kinds,
            decoded.provider_resource_binding,
        )
    }

    #[must_use]
    pub(crate) fn matches_plan(&self, plan: &PreflightPlan) -> bool {
        route_fingerprint(plan) == Some(self.route_fingerprint)
    }
}

impl fmt::Debug for ProviderRecoveredResourceCleanupBinding {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("ProviderRecoveredResourceCleanupBinding(<opaque>)")
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct PersistedProviderRecoveredResourceCleanupBinding(Vec<u8>);

impl PersistedProviderRecoveredResourceCleanupBinding {
    pub fn from_bytes(
        bytes: impl AsRef<[u8]>,
    ) -> Result<Self, ProviderRecoveredResourceCleanupBindingFailure> {
        let bytes = bytes.as_ref();
        decode_record(bytes)?;
        Ok(Self(bytes.to_vec()))
    }

    #[must_use]
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}

impl fmt::Debug for PersistedProviderRecoveredResourceCleanupBinding {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("PersistedProviderRecoveredResourceCleanupBinding(<opaque>)")
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProviderRecoveredResourceCleanupBindingFailureKind {
    InvalidEncoding,
    UnsupportedVersion,
    Oversized,
    IntegrityMismatch,
    AttachmentMismatch,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProviderRecoveredResourceCleanupBindingFailure {
    kind: ProviderRecoveredResourceCleanupBindingFailureKind,
    diagnostic: SafeDiagnostic,
}

impl ProviderRecoveredResourceCleanupBindingFailure {
    fn new(
        kind: ProviderRecoveredResourceCleanupBindingFailureKind,
        code: &'static str,
        message: &'static str,
    ) -> Self {
        Self {
            kind,
            diagnostic: SafeDiagnostic::new(code, message),
        }
    }

    #[must_use]
    pub const fn kind(&self) -> ProviderRecoveredResourceCleanupBindingFailureKind {
        self.kind
    }

    #[must_use]
    pub const fn diagnostic(&self) -> &SafeDiagnostic {
        &self.diagnostic
    }
}

impl fmt::Display for ProviderRecoveredResourceCleanupBindingFailure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.diagnostic.fmt(formatter)
    }
}

impl Error for ProviderRecoveredResourceCleanupBindingFailure {}

struct DecodedRecord {
    runtime_run_id: String,
    provider_run_ref: String,
    resource_kinds: BTreeSet<OwnedRemoteResourceKind>,
    provider_resource_binding: Vec<u8>,
    fingerprint: [u8; FINGERPRINT_BYTES],
}

fn decode_record(
    bytes: &[u8],
) -> Result<DecodedRecord, ProviderRecoveredResourceCleanupBindingFailure> {
    if bytes.len() > MAXIMUM_RECORD_BYTES {
        return Err(oversized());
    }
    if bytes.len() < MAGIC.len() + 2 + 1 + FINGERPRINT_BYTES + DIGEST_BYTES
        || &bytes[..MAGIC.len()] != MAGIC
    {
        return Err(invalid());
    }
    let payload_end = bytes.len() - DIGEST_BYTES;
    if Sha256::digest(&bytes[..payload_end]).as_slice() != &bytes[payload_end..] {
        return Err(integrity_mismatch());
    }
    let mut offset = MAGIC.len();
    if read_u16(bytes, &mut offset)? != VERSION {
        return Err(unsupported_version());
    }
    let runtime_run_id = read_text(bytes, &mut offset, payload_end)?;
    let provider_run_ref = read_text(bytes, &mut offset, payload_end)?;
    let count = usize::from(*bytes.get(offset).ok_or_else(invalid)?);
    offset += 1;
    if count == 0 || count > MAXIMUM_RESOURCE_KINDS {
        return Err(invalid());
    }
    let mut resource_kinds = BTreeSet::new();
    for tag in bytes.get(offset..offset + count).ok_or_else(invalid)? {
        if !resource_kinds.insert(decode_kind(*tag)?) {
            return Err(invalid());
        }
    }
    offset += count;
    let provider_resource_binding = read_field(bytes, &mut offset, payload_end)?.to_vec();
    if provider_resource_binding.len() > MAXIMUM_BINDING_BYTES
        || offset + FINGERPRINT_BYTES != payload_end
    {
        return Err(invalid());
    }
    let mut fingerprint = [0_u8; FINGERPRINT_BYTES];
    fingerprint.copy_from_slice(&bytes[offset..payload_end]);
    Ok(DecodedRecord {
        runtime_run_id,
        provider_run_ref,
        resource_kinds,
        provider_resource_binding,
        fingerprint,
    })
}

fn write_field(
    payload: &mut Vec<u8>,
    field: &[u8],
) -> Result<(), ProviderRecoveredResourceCleanupBindingFailure> {
    if field.is_empty() || field.len() > MAXIMUM_BINDING_BYTES {
        return Err(oversized());
    }
    let length = u16::try_from(field.len()).map_err(|_| oversized())?;
    payload.extend_from_slice(&length.to_be_bytes());
    payload.extend_from_slice(field);
    Ok(())
}

fn write_text(
    payload: &mut Vec<u8>,
    field: &[u8],
) -> Result<(), ProviderRecoveredResourceCleanupBindingFailure> {
    if field.len() > MAXIMUM_TEXT_BYTES {
        return Err(oversized());
    }
    write_field(payload, field)
}

fn read_u16(
    bytes: &[u8],
    offset: &mut usize,
) -> Result<u16, ProviderRecoveredResourceCleanupBindingFailure> {
    let end = offset.checked_add(2).ok_or_else(invalid)?;
    let value = bytes.get(*offset..end).ok_or_else(invalid)?;
    *offset = end;
    Ok(u16::from_be_bytes([value[0], value[1]]))
}

fn read_field<'a>(
    bytes: &'a [u8],
    offset: &mut usize,
    payload_end: usize,
) -> Result<&'a [u8], ProviderRecoveredResourceCleanupBindingFailure> {
    let length = usize::from(read_u16(bytes, offset)?);
    if length == 0 || length > MAXIMUM_BINDING_BYTES {
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
) -> Result<String, ProviderRecoveredResourceCleanupBindingFailure> {
    let field = read_field(bytes, offset, payload_end)?;
    if field.len() > MAXIMUM_TEXT_BYTES {
        return Err(invalid());
    }
    std::str::from_utf8(field)
        .map(str::to_owned)
        .map_err(|_| invalid())
}

const fn encode_kind(kind: OwnedRemoteResourceKind) -> u8 {
    match kind {
        OwnedRemoteResourceKind::Environment => 0,
        OwnedRemoteResourceKind::Session => 1,
        OwnedRemoteResourceKind::Response => 2,
        OwnedRemoteResourceKind::Conversation => 3,
        OwnedRemoteResourceKind::ConversationItems => 4,
    }
}

fn decode_kind(
    tag: u8,
) -> Result<OwnedRemoteResourceKind, ProviderRecoveredResourceCleanupBindingFailure> {
    match tag {
        0 => Ok(OwnedRemoteResourceKind::Environment),
        1 => Ok(OwnedRemoteResourceKind::Session),
        2 => Ok(OwnedRemoteResourceKind::Response),
        3 => Ok(OwnedRemoteResourceKind::Conversation),
        4 => Ok(OwnedRemoteResourceKind::ConversationItems),
        _ => Err(invalid()),
    }
}

fn invalid() -> ProviderRecoveredResourceCleanupBindingFailure {
    ProviderRecoveredResourceCleanupBindingFailure::new(
        ProviderRecoveredResourceCleanupBindingFailureKind::InvalidEncoding,
        "swallowtail.provider_recovered_resource_cleanup_binding.invalid",
        "Recovered-resource cleanup binding encoding is invalid",
    )
}

fn unsupported_version() -> ProviderRecoveredResourceCleanupBindingFailure {
    ProviderRecoveredResourceCleanupBindingFailure::new(
        ProviderRecoveredResourceCleanupBindingFailureKind::UnsupportedVersion,
        "swallowtail.provider_recovered_resource_cleanup_binding.version_unsupported",
        "Recovered-resource cleanup binding version is unsupported",
    )
}

fn oversized() -> ProviderRecoveredResourceCleanupBindingFailure {
    ProviderRecoveredResourceCleanupBindingFailure::new(
        ProviderRecoveredResourceCleanupBindingFailureKind::Oversized,
        "swallowtail.provider_recovered_resource_cleanup_binding.oversized",
        "Recovered-resource cleanup binding exceeds its bound",
    )
}

fn integrity_mismatch() -> ProviderRecoveredResourceCleanupBindingFailure {
    ProviderRecoveredResourceCleanupBindingFailure::new(
        ProviderRecoveredResourceCleanupBindingFailureKind::IntegrityMismatch,
        "swallowtail.provider_recovered_resource_cleanup_binding.integrity_mismatch",
        "Recovered-resource cleanup binding integrity check failed",
    )
}

fn attachment_mismatch() -> ProviderRecoveredResourceCleanupBindingFailure {
    ProviderRecoveredResourceCleanupBindingFailure::new(
        ProviderRecoveredResourceCleanupBindingFailureKind::AttachmentMismatch,
        "swallowtail.provider_recovered_resource_cleanup_binding.attachment_mismatch",
        "Recovered-resource cleanup binding does not match the prepared route",
    )
}

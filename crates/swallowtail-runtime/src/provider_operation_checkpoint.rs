use crate::{RuntimeTurnId, SessionResumeBinding};
use sha2::{Digest, Sha256};
use std::error::Error;
use std::fmt;
use swallowtail_core::{PreflightPlan, SafeDiagnostic, SessionRef, TurnRef};

const MAGIC: &[u8; 16] = b"SWST-OP-CHECKPT\0";
const VERSION: u16 = 1;
const DIGEST_BYTES: usize = 32;
const FINGERPRINT_BYTES: usize = 32;
const MAXIMUM_FIELD_BYTES: usize = 4 * 1024;
const MAXIMUM_RECORD_BYTES: usize = 20 * 1024;

/// Exact provider operation and durable event position observed by one route.
///
/// The cursor remains opaque outside its owning adapter. Default formatting
/// exposes none of the provider or consumer identities.
#[derive(Clone, Eq, PartialEq)]
pub struct ProviderOperationCheckpoint {
    provider_session_ref: SessionRef,
    runtime_turn_id: RuntimeTurnId,
    provider_turn_ref: TurnRef,
    cursor: Vec<u8>,
}

impl ProviderOperationCheckpoint {
    pub fn new(
        provider_session_ref: SessionRef,
        runtime_turn_id: RuntimeTurnId,
        provider_turn_ref: TurnRef,
        cursor: impl AsRef<[u8]>,
    ) -> Result<Self, ProviderOperationCheckpointFailure> {
        let cursor = cursor.as_ref();
        if cursor.is_empty() || cursor.len() > MAXIMUM_FIELD_BYTES {
            return Err(invalid());
        }
        Ok(Self {
            provider_session_ref,
            runtime_turn_id,
            provider_turn_ref,
            cursor: cursor.to_vec(),
        })
    }

    #[must_use]
    pub const fn provider_session_ref(&self) -> &SessionRef {
        &self.provider_session_ref
    }

    #[must_use]
    pub const fn runtime_turn_id(&self) -> &RuntimeTurnId {
        &self.runtime_turn_id
    }

    #[must_use]
    pub const fn provider_turn_ref(&self) -> &TurnRef {
        &self.provider_turn_ref
    }

    /// Passes the opaque cursor back to its owning provider adapter.
    #[must_use]
    pub fn cursor(&self) -> &[u8] {
        &self.cursor
    }

    pub fn export_persisted(
        &self,
        plan: &PreflightPlan,
        binding: &SessionResumeBinding,
    ) -> Result<PersistedProviderOperationCheckpoint, ProviderOperationCheckpointFailure> {
        if self.provider_session_ref != *binding.provider_session_ref()
            || !binding.matches_attachment(
                plan,
                binding.working_resource(),
                binding.access_policy(),
            )
        {
            return Err(attachment_mismatch());
        }
        let fingerprint = crate::session_binding::attachment_fingerprint_for_checkpoint(
            plan,
            binding.working_resource(),
            binding.access_policy(),
        )
        .ok_or_else(attachment_mismatch)?;
        let fields = [
            self.provider_session_ref.as_provider_value().as_bytes(),
            self.runtime_turn_id.as_str().as_bytes(),
            self.provider_turn_ref.as_provider_value().as_bytes(),
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
        PersistedProviderOperationCheckpoint::from_bytes(payload)
    }

    pub fn restore_persisted(
        record: &PersistedProviderOperationCheckpoint,
        plan: &PreflightPlan,
        binding: &SessionResumeBinding,
    ) -> Result<Self, ProviderOperationCheckpointFailure> {
        let decoded = decode_record(record.as_bytes())?;
        let current = crate::session_binding::attachment_fingerprint_for_checkpoint(
            plan,
            binding.working_resource(),
            binding.access_policy(),
        )
        .ok_or_else(attachment_mismatch)?;
        if decoded.fingerprint != current
            || decoded.provider_session_ref != binding.provider_session_ref().as_provider_value()
            || !binding.matches_attachment(
                plan,
                binding.working_resource(),
                binding.access_policy(),
            )
        {
            return Err(attachment_mismatch());
        }
        Self::new(
            binding.provider_session_ref().clone(),
            RuntimeTurnId::new(decoded.runtime_turn_id).map_err(|_| invalid())?,
            TurnRef::new(decoded.provider_turn_ref).map_err(|_| invalid())?,
            decoded.cursor,
        )
    }
}

impl fmt::Debug for ProviderOperationCheckpoint {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("ProviderOperationCheckpoint(<opaque>)")
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct PersistedProviderOperationCheckpoint(Vec<u8>);

impl PersistedProviderOperationCheckpoint {
    pub fn from_bytes(bytes: impl AsRef<[u8]>) -> Result<Self, ProviderOperationCheckpointFailure> {
        let bytes = bytes.as_ref();
        decode_record(bytes)?;
        Ok(Self(bytes.to_vec()))
    }

    #[must_use]
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}

impl fmt::Debug for PersistedProviderOperationCheckpoint {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("PersistedProviderOperationCheckpoint(<opaque>)")
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProviderOperationCheckpointFailureKind {
    InvalidEncoding,
    UnsupportedVersion,
    Oversized,
    IntegrityMismatch,
    AttachmentMismatch,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProviderOperationCheckpointFailure {
    kind: ProviderOperationCheckpointFailureKind,
    diagnostic: SafeDiagnostic,
}

impl ProviderOperationCheckpointFailure {
    fn new(
        kind: ProviderOperationCheckpointFailureKind,
        code: &'static str,
        message: &'static str,
    ) -> Self {
        Self {
            kind,
            diagnostic: SafeDiagnostic::new(code, message),
        }
    }

    #[must_use]
    pub const fn kind(&self) -> ProviderOperationCheckpointFailureKind {
        self.kind
    }

    #[must_use]
    pub const fn diagnostic(&self) -> &SafeDiagnostic {
        &self.diagnostic
    }
}

impl fmt::Display for ProviderOperationCheckpointFailure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.diagnostic.fmt(formatter)
    }
}

impl Error for ProviderOperationCheckpointFailure {}

struct DecodedRecord<'a> {
    provider_session_ref: &'a str,
    runtime_turn_id: &'a str,
    provider_turn_ref: &'a str,
    cursor: &'a [u8],
    fingerprint: [u8; FINGERPRINT_BYTES],
}

fn decode_record(bytes: &[u8]) -> Result<DecodedRecord<'_>, ProviderOperationCheckpointFailure> {
    if bytes.len() > MAXIMUM_RECORD_BYTES {
        return Err(oversized());
    }
    let minimum = MAGIC.len() + 2 + (2 * 4) + FINGERPRINT_BYTES + DIGEST_BYTES;
    if bytes.len() < minimum || bytes.get(..MAGIC.len()) != Some(MAGIC) {
        return Err(invalid());
    }
    let mut offset = MAGIC.len();
    let version = read_u16(bytes, &mut offset)?;
    if version != VERSION {
        return Err(ProviderOperationCheckpointFailure::new(
            ProviderOperationCheckpointFailureKind::UnsupportedVersion,
            "swallowtail.provider_operation_checkpoint.version_unsupported",
            "Persisted provider operation checkpoint uses an unsupported version",
        ));
    }
    let session = read_field(bytes, &mut offset)?;
    let runtime_turn = read_field(bytes, &mut offset)?;
    let provider_turn = read_field(bytes, &mut offset)?;
    let cursor = read_field(bytes, &mut offset)?;
    let fingerprint_end = offset.checked_add(FINGERPRINT_BYTES).ok_or_else(invalid)?;
    let digest_end = fingerprint_end
        .checked_add(DIGEST_BYTES)
        .ok_or_else(invalid)?;
    if digest_end != bytes.len() {
        return Err(invalid());
    }
    let expected = Sha256::digest(&bytes[..fingerprint_end]);
    if expected.as_slice() != &bytes[fingerprint_end..digest_end] {
        return Err(ProviderOperationCheckpointFailure::new(
            ProviderOperationCheckpointFailureKind::IntegrityMismatch,
            "swallowtail.provider_operation_checkpoint.integrity_mismatch",
            "Persisted provider operation checkpoint failed its integrity check",
        ));
    }
    let provider_session_ref = std::str::from_utf8(session).map_err(|_| invalid())?;
    let runtime_turn_id = std::str::from_utf8(runtime_turn).map_err(|_| invalid())?;
    let provider_turn_ref = std::str::from_utf8(provider_turn).map_err(|_| invalid())?;
    if [provider_session_ref, runtime_turn_id, provider_turn_ref]
        .iter()
        .any(|value| value.trim().is_empty())
    {
        return Err(invalid());
    }
    Ok(DecodedRecord {
        provider_session_ref,
        runtime_turn_id,
        provider_turn_ref,
        cursor,
        fingerprint: bytes[offset..fingerprint_end]
            .try_into()
            .map_err(|_| invalid())?,
    })
}

fn read_u16(bytes: &[u8], offset: &mut usize) -> Result<u16, ProviderOperationCheckpointFailure> {
    let end = offset.checked_add(2).ok_or_else(invalid)?;
    let value = u16::from_be_bytes(
        bytes
            .get(*offset..end)
            .ok_or_else(invalid)?
            .try_into()
            .map_err(|_| invalid())?,
    );
    *offset = end;
    Ok(value)
}

fn read_field<'a>(
    bytes: &'a [u8],
    offset: &mut usize,
) -> Result<&'a [u8], ProviderOperationCheckpointFailure> {
    let length = usize::from(read_u16(bytes, offset)?);
    if length == 0 || length > MAXIMUM_FIELD_BYTES {
        return Err(oversized());
    }
    let end = offset.checked_add(length).ok_or_else(invalid)?;
    let field = bytes.get(*offset..end).ok_or_else(invalid)?;
    *offset = end;
    Ok(field)
}

fn invalid() -> ProviderOperationCheckpointFailure {
    ProviderOperationCheckpointFailure::new(
        ProviderOperationCheckpointFailureKind::InvalidEncoding,
        "swallowtail.provider_operation_checkpoint.invalid",
        "Provider operation checkpoint is malformed",
    )
}

fn oversized() -> ProviderOperationCheckpointFailure {
    ProviderOperationCheckpointFailure::new(
        ProviderOperationCheckpointFailureKind::Oversized,
        "swallowtail.provider_operation_checkpoint.oversized",
        "Provider operation checkpoint exceeds its bound",
    )
}

fn attachment_mismatch() -> ProviderOperationCheckpointFailure {
    ProviderOperationCheckpointFailure::new(
        ProviderOperationCheckpointFailureKind::AttachmentMismatch,
        "swallowtail.provider_operation_checkpoint.attachment_mismatch",
        "Provider operation checkpoint does not match the requested attachment",
    )
}

#[cfg(test)]
mod tests;

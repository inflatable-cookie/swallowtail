use super::SessionResumeBinding;
use crate::WorkingResourceRef;
use sha2::{Digest, Sha256};
use std::error::Error;
use std::fmt;
use swallowtail_core::{
    PreflightPlan, ProviderSessionBindingOrigin, SafeDiagnostic, SessionAccessPolicy, SessionRef,
};

mod fingerprint;

use fingerprint::attachment_fingerprint;

pub(crate) fn attachment_fingerprint_for_checkpoint(
    plan: &PreflightPlan,
    working_resource: &WorkingResourceRef,
    access_policy: &SessionAccessPolicy,
) -> Option<[u8; 32]> {
    attachment_fingerprint(plan, working_resource, access_policy).ok()
}

const MAGIC: &[u8; 16] = b"SWST-RESUME-BIND";
const VERSION: u16 = 1;
const DIGEST_BYTES: usize = 32;
const FINGERPRINT_BYTES: usize = 32;
const MAXIMUM_RECORD_BYTES: usize = 8 * 1024;
const MAXIMUM_PROVIDER_REFERENCE_BYTES: usize = 4 * 1024;

/// Stable persisted form of one exact provider-session resume binding.
///
/// The bytes are deliberately available only through an explicit accessor;
/// default formatting never exposes the provider-session reference.
#[derive(Clone, Eq, PartialEq)]
pub struct PersistedSessionResumeBinding(Vec<u8>);

impl PersistedSessionResumeBinding {
    /// Validates and owns one previously persisted record.
    pub fn from_bytes(
        bytes: impl AsRef<[u8]>,
    ) -> Result<Self, SessionResumeBindingPersistenceFailure> {
        let bytes = bytes.as_ref();
        decode_record(bytes)?;
        Ok(Self(bytes.to_vec()))
    }

    /// Returns the versioned opaque bytes for consumer-owned storage.
    #[must_use]
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}

impl fmt::Debug for PersistedSessionResumeBinding {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("PersistedSessionResumeBinding(<opaque>)")
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SessionResumeBindingPersistenceFailureKind {
    InvalidEncoding,
    UnsupportedVersion,
    Oversized,
    IntegrityMismatch,
    AttachmentMismatch,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SessionResumeBindingPersistenceFailure {
    kind: SessionResumeBindingPersistenceFailureKind,
    diagnostic: SafeDiagnostic,
}

impl SessionResumeBindingPersistenceFailure {
    fn new(
        kind: SessionResumeBindingPersistenceFailureKind,
        code: &'static str,
        message: &'static str,
    ) -> Self {
        Self {
            kind,
            diagnostic: SafeDiagnostic::new(code, message),
        }
    }

    #[must_use]
    pub const fn kind(&self) -> SessionResumeBindingPersistenceFailureKind {
        self.kind
    }

    #[must_use]
    pub const fn diagnostic(&self) -> &SafeDiagnostic {
        &self.diagnostic
    }
}

impl fmt::Display for SessionResumeBindingPersistenceFailure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.diagnostic.fmt(formatter)
    }
}

impl Error for SessionResumeBindingPersistenceFailure {}

impl SessionResumeBinding {
    /// Exports this binding only under the exact plan which issued or accepted it.
    pub fn export_persisted(
        &self,
        plan: &PreflightPlan,
    ) -> Result<PersistedSessionResumeBinding, SessionResumeBindingPersistenceFailure> {
        if !self.matches_attachment(plan, &self.working_resource, &self.access_policy) {
            return Err(attachment_mismatch());
        }
        let provider = self.provider_session_ref.as_provider_value().as_bytes();
        if provider.is_empty() || provider.len() > MAXIMUM_PROVIDER_REFERENCE_BYTES {
            return Err(oversized());
        }
        let provider_len = u16::try_from(provider.len()).map_err(|_| oversized())?;
        let fingerprint =
            attachment_fingerprint(plan, &self.working_resource, &self.access_policy)?;
        let mut payload = Vec::with_capacity(
            MAGIC.len() + 2 + 2 + provider.len() + 1 + FINGERPRINT_BYTES + DIGEST_BYTES,
        );
        payload.extend_from_slice(MAGIC);
        payload.extend_from_slice(&VERSION.to_be_bytes());
        payload.extend_from_slice(&provider_len.to_be_bytes());
        payload.extend_from_slice(provider);
        payload.push(origin_tag(self.origin));
        payload.extend_from_slice(&fingerprint);
        let digest = Sha256::digest(&payload);
        payload.extend_from_slice(&digest);
        PersistedSessionResumeBinding::from_bytes(payload)
    }

    /// Restores a binding only under the exact current attachment dimensions.
    pub fn restore_persisted(
        record: &PersistedSessionResumeBinding,
        plan: &PreflightPlan,
        working_resource: &WorkingResourceRef,
        access_policy: &SessionAccessPolicy,
    ) -> Result<Self, SessionResumeBindingPersistenceFailure> {
        let decoded = decode_record(record.as_bytes())?;
        let current = attachment_fingerprint(plan, working_resource, access_policy)?;
        if decoded.fingerprint != current {
            return Err(attachment_mismatch());
        }
        let provider_session_ref =
            SessionRef::new(decoded.provider_session_ref).map_err(|_| invalid_encoding())?;
        let binding = match (plan.model_route_id(), plan.model_id()) {
            (Some(model_route_id), Some(model_id)) => Self::new(
                provider_session_ref,
                plan.instance_id().clone(),
                plan.execution_host_id().clone(),
                model_route_id.clone(),
                model_id.clone(),
                working_resource.clone(),
                access_policy.clone(),
            ),
            (None, None) => Self::without_model(
                provider_session_ref,
                plan.instance_id().clone(),
                plan.execution_host_id().clone(),
                working_resource.clone(),
                access_policy.clone(),
            ),
            _ => return Err(attachment_mismatch()),
        };
        Ok(binding.with_origin(decoded.origin))
    }
}

struct DecodedRecord<'a> {
    provider_session_ref: &'a str,
    origin: ProviderSessionBindingOrigin,
    fingerprint: [u8; FINGERPRINT_BYTES],
}

fn decode_record(
    bytes: &[u8],
) -> Result<DecodedRecord<'_>, SessionResumeBindingPersistenceFailure> {
    if bytes.len() > MAXIMUM_RECORD_BYTES {
        return Err(oversized());
    }
    let minimum = MAGIC.len() + 2 + 2 + 1 + FINGERPRINT_BYTES + DIGEST_BYTES;
    if bytes.len() < minimum || bytes.get(..MAGIC.len()) != Some(MAGIC) {
        return Err(invalid_encoding());
    }
    let version_offset = MAGIC.len();
    let version = u16::from_be_bytes(
        bytes[version_offset..version_offset + 2]
            .try_into()
            .map_err(|_| invalid_encoding())?,
    );
    if version != VERSION {
        return Err(SessionResumeBindingPersistenceFailure::new(
            SessionResumeBindingPersistenceFailureKind::UnsupportedVersion,
            "swallowtail.session_resume_binding.persistence_version_unsupported",
            "Persisted session resume binding uses an unsupported version",
        ));
    }
    let length_offset = version_offset + 2;
    let provider_len = usize::from(u16::from_be_bytes(
        bytes[length_offset..length_offset + 2]
            .try_into()
            .map_err(|_| invalid_encoding())?,
    ));
    if provider_len == 0 || provider_len > MAXIMUM_PROVIDER_REFERENCE_BYTES {
        return Err(oversized());
    }
    let provider_start = length_offset + 2;
    let origin_offset = provider_start
        .checked_add(provider_len)
        .ok_or_else(invalid_encoding)?;
    let fingerprint_start = origin_offset + 1;
    let digest_start = fingerprint_start + FINGERPRINT_BYTES;
    if digest_start + DIGEST_BYTES != bytes.len() {
        return Err(invalid_encoding());
    }
    let expected = Sha256::digest(&bytes[..digest_start]);
    if expected.as_slice() != &bytes[digest_start..] {
        return Err(SessionResumeBindingPersistenceFailure::new(
            SessionResumeBindingPersistenceFailureKind::IntegrityMismatch,
            "swallowtail.session_resume_binding.persistence_integrity_mismatch",
            "Persisted session resume binding failed its integrity check",
        ));
    }
    let provider_session_ref = std::str::from_utf8(&bytes[provider_start..origin_offset])
        .ok()
        .filter(|value| !value.trim().is_empty())
        .ok_or_else(invalid_encoding)?;
    let origin = decode_origin(bytes[origin_offset])?;
    let fingerprint = bytes[fingerprint_start..digest_start]
        .try_into()
        .map_err(|_| invalid_encoding())?;
    Ok(DecodedRecord {
        provider_session_ref,
        origin,
        fingerprint,
    })
}

const fn origin_tag(origin: ProviderSessionBindingOrigin) -> u8 {
    match origin {
        ProviderSessionBindingOrigin::Created => 0,
        ProviderSessionBindingOrigin::Loaded => 1,
        ProviderSessionBindingOrigin::Resumed => 2,
        ProviderSessionBindingOrigin::ExplicitlyImported => 3,
    }
}

fn decode_origin(
    tag: u8,
) -> Result<ProviderSessionBindingOrigin, SessionResumeBindingPersistenceFailure> {
    match tag {
        0 => Ok(ProviderSessionBindingOrigin::Created),
        1 => Ok(ProviderSessionBindingOrigin::Loaded),
        2 => Ok(ProviderSessionBindingOrigin::Resumed),
        3 => Ok(ProviderSessionBindingOrigin::ExplicitlyImported),
        _ => Err(invalid_encoding()),
    }
}

fn invalid_encoding() -> SessionResumeBindingPersistenceFailure {
    SessionResumeBindingPersistenceFailure::new(
        SessionResumeBindingPersistenceFailureKind::InvalidEncoding,
        "swallowtail.session_resume_binding.persistence_invalid",
        "Persisted session resume binding is malformed",
    )
}

fn oversized() -> SessionResumeBindingPersistenceFailure {
    SessionResumeBindingPersistenceFailure::new(
        SessionResumeBindingPersistenceFailureKind::Oversized,
        "swallowtail.session_resume_binding.persistence_oversized",
        "Persisted session resume binding exceeds its bound",
    )
}

fn attachment_mismatch() -> SessionResumeBindingPersistenceFailure {
    SessionResumeBindingPersistenceFailure::new(
        SessionResumeBindingPersistenceFailureKind::AttachmentMismatch,
        "swallowtail.session_resume_binding.persistence_attachment_mismatch",
        "Persisted session resume binding does not match the requested attachment",
    )
}

#[cfg(test)]
mod tests;

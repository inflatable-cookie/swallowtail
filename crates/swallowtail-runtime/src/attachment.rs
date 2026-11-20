use crate::input::required_text;
use crate::{AttachmentRef, InputValueRequired};
use std::io::Read;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
/// Host-supported representation used to transport an attachment.
pub enum AttachmentRepresentation {
    /// An operation-scoped byte stream.
    Stream,
    /// Bounded bytes held in memory.
    BoundedBytes,
    /// An operation-scoped host-materialized file.
    TemporaryFile,
    /// A provider-owned upload created for the operation.
    ProviderUpload,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
/// Authority responsible for cleaning up leased material.
pub enum LeaseCleanupAuthority {
    /// The consumer retains ownership; Swallowtail must not delete the material.
    Consumer,
    /// The execution host cleans up material created for this operation scope.
    OperationScope,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
/// Semantic role an attachment plays in an operation.
pub enum AttachmentRole {
    /// Primary input to the operation.
    Input,
    /// Additional context considered by the provider.
    Context,
    /// Reference material that is not primary input.
    Reference,
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
/// Nonempty content digest carried as safe attachment metadata.
pub struct AttachmentDigest(String);

impl AttachmentDigest {
    /// Creates a digest, rejecting an empty value.
    pub fn new(value: impl Into<String>) -> Result<Self, InputValueRequired> {
        required_text("attachment digest", value).map(Self)
    }

    #[must_use]
    /// Returns the digest text.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
/// Safe metadata and opaque host reference for one portable attachment.
pub struct AttachmentDescriptor {
    reference: AttachmentRef,
    media_type: String,
    display_name: Option<String>,
    role: AttachmentRole,
    known_length: Option<u64>,
    digest: Option<AttachmentDigest>,
}

impl AttachmentDescriptor {
    /// Creates a descriptor with required media type and semantic role.
    pub fn new(
        reference: AttachmentRef,
        media_type: impl Into<String>,
        role: AttachmentRole,
    ) -> Result<Self, InputValueRequired> {
        Ok(Self {
            reference,
            media_type: required_text("attachment media type", media_type)?,
            display_name: None,
            role,
            known_length: None,
            digest: None,
        })
    }

    /// Adds a nonempty consumer-facing display name.
    pub fn with_display_name(
        mut self,
        display_name: impl Into<String>,
    ) -> Result<Self, InputValueRequired> {
        self.display_name = Some(required_text("attachment display name", display_name)?);
        Ok(self)
    }

    #[must_use]
    /// Adds the known attachment length in bytes.
    pub const fn with_known_length(mut self, length: u64) -> Self {
        self.known_length = Some(length);
        self
    }

    #[must_use]
    /// Adds an optional content digest.
    pub fn with_digest(mut self, digest: AttachmentDigest) -> Self {
        self.digest = Some(digest);
        self
    }

    #[must_use]
    /// Returns the opaque host-approved attachment reference.
    pub const fn reference(&self) -> &AttachmentRef {
        &self.reference
    }

    #[must_use]
    /// Returns the declared media type.
    pub fn media_type(&self) -> &str {
        &self.media_type
    }

    #[must_use]
    /// Returns the optional display name.
    pub fn display_name(&self) -> Option<&str> {
        self.display_name.as_deref()
    }

    #[must_use]
    /// Returns the attachment's semantic role.
    pub const fn role(&self) -> AttachmentRole {
        self.role
    }

    #[must_use]
    /// Returns the known length in bytes, when supplied.
    pub const fn known_length(&self) -> Option<u64> {
        self.known_length
    }

    #[must_use]
    /// Returns the optional content digest.
    pub const fn digest(&self) -> Option<&AttachmentDigest> {
        self.digest.as_ref()
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
/// Reason a bounded attachment read produced no bytes.
///
/// The set is closed on purpose: every adapter matches it exhaustively, so a
/// new refusal has to be classified by each caller instead of defaulting.
pub enum AttachmentReadFailure {
    /// The materialized file could not be opened or read.
    Unreadable,
    /// The materialized file held more than the permitted number of bytes.
    TooLarge,
}

/// Reads one host-materialized attachment file without exceeding `maximum` bytes.
///
/// The read stops one byte past `maximum`, so an oversized file is rejected
/// from a buffer bounded by the caller's own limit instead of being drained
/// into memory first. That bound is what makes this safe on a stream that never
/// ends. A refusal carries only its reason, so each adapter keeps its own
/// provider-specific code and message.
pub fn read_bounded_attachment(path: &str, maximum: u64) -> Result<Vec<u8>, AttachmentReadFailure> {
    let file = std::fs::File::open(path).map_err(|_| AttachmentReadFailure::Unreadable)?;
    let mut bytes = Vec::new();
    file.take(maximum.saturating_add(1))
        .read_to_end(&mut bytes)
        .map_err(|_| AttachmentReadFailure::Unreadable)?;
    if bytes.len() as u64 > maximum {
        return Err(AttachmentReadFailure::TooLarge);
    }
    Ok(bytes)
}

#[cfg(test)]
#[path = "attachment/tests.rs"]
mod tests;

use crate::input::required_text;
use crate::{AttachmentRef, InputValueRequired};

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

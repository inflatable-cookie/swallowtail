use crate::input::required_text;
use crate::{AttachmentRef, InputValueRequired};

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum AttachmentRepresentation {
    Stream,
    BoundedBytes,
    TemporaryFile,
    ProviderUpload,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum LeaseCleanupAuthority {
    Consumer,
    OperationScope,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum AttachmentRole {
    Input,
    Context,
    Reference,
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct AttachmentDigest(String);

impl AttachmentDigest {
    pub fn new(value: impl Into<String>) -> Result<Self, InputValueRequired> {
        required_text("attachment digest", value).map(Self)
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AttachmentDescriptor {
    reference: AttachmentRef,
    media_type: String,
    display_name: Option<String>,
    role: AttachmentRole,
    known_length: Option<u64>,
    digest: Option<AttachmentDigest>,
}

impl AttachmentDescriptor {
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

    pub fn with_display_name(
        mut self,
        display_name: impl Into<String>,
    ) -> Result<Self, InputValueRequired> {
        self.display_name = Some(required_text("attachment display name", display_name)?);
        Ok(self)
    }

    #[must_use]
    pub const fn with_known_length(mut self, length: u64) -> Self {
        self.known_length = Some(length);
        self
    }

    #[must_use]
    pub fn with_digest(mut self, digest: AttachmentDigest) -> Self {
        self.digest = Some(digest);
        self
    }

    #[must_use]
    pub const fn reference(&self) -> &AttachmentRef {
        &self.reference
    }

    #[must_use]
    pub fn media_type(&self) -> &str {
        &self.media_type
    }

    #[must_use]
    pub fn display_name(&self) -> Option<&str> {
        self.display_name.as_deref()
    }

    #[must_use]
    pub const fn role(&self) -> AttachmentRole {
        self.role
    }

    #[must_use]
    pub const fn known_length(&self) -> Option<u64> {
        self.known_length
    }

    #[must_use]
    pub const fn digest(&self) -> Option<&AttachmentDigest> {
        self.digest.as_ref()
    }
}

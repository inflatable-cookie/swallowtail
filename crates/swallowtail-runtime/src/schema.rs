use crate::input::required_text;
use crate::{AttachmentDigest, InputLimitExceeded, InputValueRequired, SchemaRef};
use std::fmt;

#[derive(Clone, Eq, PartialEq)]
/// Bounded inline schema bytes or an opaque host-resolved schema reference.
pub enum SchemaDocument {
    /// Bounded inline schema bytes.
    Inline(Vec<u8>),
    /// Opaque schema reference resolved by the execution host.
    Reference(SchemaRef),
}

impl SchemaDocument {
    /// Creates an inline document when its bytes fit the admitted bound.
    pub fn inline(
        bytes: impl Into<Vec<u8>>,
        maximum_bytes: usize,
    ) -> Result<Self, InputLimitExceeded> {
        let bytes = bytes.into();
        if bytes.len() > maximum_bytes {
            Err(InputLimitExceeded::new(
                "inline schema document",
                maximum_bytes,
                bytes.len(),
            ))
        } else {
            Ok(Self::Inline(bytes))
        }
    }

    #[must_use]
    /// Creates a document backed by an opaque host schema reference.
    pub const fn reference(reference: SchemaRef) -> Self {
        Self::Reference(reference)
    }

    #[must_use]
    /// Returns inline bytes, or `None` for a referenced document.
    pub fn inline_bytes(&self) -> Option<&[u8]> {
        match self {
            Self::Inline(bytes) => Some(bytes),
            Self::Reference(_) => None,
        }
    }
}

impl fmt::Debug for SchemaDocument {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Inline(bytes) => formatter
                .debug_tuple("Inline")
                .field(&format_args!("<opaque:{} bytes>", bytes.len()))
                .finish(),
            Self::Reference(reference) => {
                formatter.debug_tuple("Reference").field(reference).finish()
            }
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
/// Schema transport descriptor for one structured-output request.
///
/// It carries compatibility metadata only; schema meaning, validation, repair,
/// and result acceptance remain consumer-owned.
pub struct StructuredOutputDescriptor {
    document: SchemaDocument,
    media_type: String,
    dialect: String,
    digest: Option<AttachmentDigest>,
}

impl StructuredOutputDescriptor {
    /// Creates a descriptor with required media type and dialect.
    pub fn new(
        document: SchemaDocument,
        media_type: impl Into<String>,
        dialect: impl Into<String>,
    ) -> Result<Self, InputValueRequired> {
        Ok(Self {
            document,
            media_type: required_text("schema media type", media_type)?,
            dialect: required_text("schema dialect", dialect)?,
            digest: None,
        })
    }

    #[must_use]
    /// Adds an optional digest for the schema document.
    pub fn with_digest(mut self, digest: AttachmentDigest) -> Self {
        self.digest = Some(digest);
        self
    }

    #[must_use]
    /// Returns the inline or referenced schema document.
    pub const fn document(&self) -> &SchemaDocument {
        &self.document
    }

    #[must_use]
    /// Returns the schema media type.
    pub fn media_type(&self) -> &str {
        &self.media_type
    }

    #[must_use]
    /// Returns the declared schema dialect.
    pub fn dialect(&self) -> &str {
        &self.dialect
    }
}

#[cfg(test)]
mod tests {
    use super::{SchemaDocument, StructuredOutputDescriptor};

    #[test]
    fn inline_schema_is_bounded_and_redacted() {
        let document = SchemaDocument::inline(b"secret schema body".to_vec(), 1024)
            .expect("schema is within the bound");
        let descriptor = StructuredOutputDescriptor::new(
            document,
            "application/schema+json",
            "json-schema-2020-12",
        )
        .expect("descriptor is valid");

        assert!(!format!("{descriptor:?}").contains("secret schema body"));
        assert!(SchemaDocument::inline(vec![0; 5], 4).is_err());
    }
}

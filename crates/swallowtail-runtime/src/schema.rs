use crate::input::required_text;
use crate::{AttachmentDigest, InputLimitExceeded, InputValueRequired, SchemaRef};
use std::fmt;

#[derive(Clone, Eq, PartialEq)]
pub enum SchemaDocument {
    Inline(Vec<u8>),
    Reference(SchemaRef),
}

impl SchemaDocument {
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
    pub const fn reference(reference: SchemaRef) -> Self {
        Self::Reference(reference)
    }

    #[must_use]
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
pub struct StructuredOutputDescriptor {
    document: SchemaDocument,
    media_type: String,
    dialect: String,
    digest: Option<AttachmentDigest>,
}

impl StructuredOutputDescriptor {
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
    pub fn with_digest(mut self, digest: AttachmentDigest) -> Self {
        self.digest = Some(digest);
        self
    }

    #[must_use]
    pub const fn document(&self) -> &SchemaDocument {
        &self.document
    }

    #[must_use]
    pub fn media_type(&self) -> &str {
        &self.media_type
    }

    #[must_use]
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

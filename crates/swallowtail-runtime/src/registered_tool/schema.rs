//! Bounded declared schema descriptors whose bodies never reach diagnostics.

use super::failure::{RegisteredToolFailure, RegisteredToolFailureKind, reject};
use super::identity::{RegisteredServerRevision, admit_identity};
use super::limits::MAX_REGISTERED_TOOL_SCHEMA_BYTES;
use std::fmt;

macro_rules! bounded_schema_label {
    ($name:ident, $doc:literal) => {
        #[doc = $doc]
        #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        pub struct $name(String);

        impl $name {
            /// Admits bounded, non-blank, control-free label text.
            pub fn new(value: impl Into<String>) -> Result<Self, RegisteredToolFailure> {
                admit_identity(value.into()).map(Self)
            }

            /// Returns the exact admitted label text.
            #[must_use]
            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str(&self.0)
            }
        }
    };
}

bounded_schema_label!(
    RegisteredToolSchemaNamespace,
    "Consumer-owned namespace of one declared schema."
);
bounded_schema_label!(
    RegisteredToolSchemaMediaType,
    "Exact media type of one declared schema document."
);
bounded_schema_label!(
    RegisteredToolSchemaDialect,
    "Exact schema dialect of one declared schema document."
);
bounded_schema_label!(
    RegisteredToolSchemaDigest,
    "Consumer-supplied content digest of one declared schema document."
);

/// Bounded declared schema body.
///
/// The body is available only through an explicit execution accessor. It never
/// appears in `Debug`, `Display`, diagnostics, events, or durable records.
#[derive(Clone, Eq, PartialEq)]
pub struct RegisteredToolSchemaDocument {
    body: String,
}

impl RegisteredToolSchemaDocument {
    /// Admits one nonempty schema body within the per-schema byte bound.
    pub fn new(body: impl Into<String>) -> Result<Self, RegisteredToolFailure> {
        let body = body.into();
        if body.trim().is_empty() {
            return Err(reject(RegisteredToolFailureKind::UnsupportedSchema));
        }
        if body.len() > MAX_REGISTERED_TOOL_SCHEMA_BYTES {
            return Err(reject(RegisteredToolFailureKind::LimitExceeded));
        }
        Ok(Self { body })
    }

    /// Returns the schema body for validation or dispatch execution only.
    #[must_use]
    pub fn expose_for_execution(&self) -> &str {
        &self.body
    }

    /// Returns the exact declared byte length.
    #[must_use]
    pub fn byte_len(&self) -> usize {
        self.body.len()
    }
}

impl fmt::Debug for RegisteredToolSchemaDocument {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("RegisteredToolSchemaDocument")
            .field("body", &"<redacted>")
            .field("bytes", &self.body.len())
            .finish()
    }
}

impl fmt::Display for RegisteredToolSchemaDocument {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("<redacted registered tool schema>")
    }
}

/// Immutable declared schema for one registered tool input or output.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RegisteredToolSchema {
    namespace: RegisteredToolSchemaNamespace,
    media_type: RegisteredToolSchemaMediaType,
    dialect: RegisteredToolSchemaDialect,
    revision: RegisteredServerRevision,
    digest: RegisteredToolSchemaDigest,
    document: RegisteredToolSchemaDocument,
}

impl RegisteredToolSchema {
    /// Binds one bounded schema body to its exact namespace and digest.
    #[must_use]
    pub const fn new(
        namespace: RegisteredToolSchemaNamespace,
        media_type: RegisteredToolSchemaMediaType,
        dialect: RegisteredToolSchemaDialect,
        revision: RegisteredServerRevision,
        digest: RegisteredToolSchemaDigest,
        document: RegisteredToolSchemaDocument,
    ) -> Self {
        Self {
            namespace,
            media_type,
            dialect,
            revision,
            digest,
            document,
        }
    }

    /// Returns the consumer-owned schema namespace.
    #[must_use]
    pub const fn namespace(&self) -> &RegisteredToolSchemaNamespace {
        &self.namespace
    }

    /// Returns the exact schema media type.
    #[must_use]
    pub const fn media_type(&self) -> &RegisteredToolSchemaMediaType {
        &self.media_type
    }

    /// Returns the exact schema dialect.
    #[must_use]
    pub const fn dialect(&self) -> &RegisteredToolSchemaDialect {
        &self.dialect
    }

    /// Returns the schema revision carried into the prepared plan.
    #[must_use]
    pub const fn revision(&self) -> &RegisteredServerRevision {
        &self.revision
    }

    /// Returns the consumer-supplied content digest.
    #[must_use]
    pub const fn digest(&self) -> &RegisteredToolSchemaDigest {
        &self.digest
    }

    /// Returns the bounded schema document wrapper.
    #[must_use]
    pub const fn document(&self) -> &RegisteredToolSchemaDocument {
        &self.document
    }

    /// Returns the declared schema byte length.
    #[must_use]
    pub fn byte_len(&self) -> usize {
        self.document.byte_len()
    }
}

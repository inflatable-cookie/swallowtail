use crate::diagnostic::{ValueRequired, required_text};
use std::fmt;

macro_rules! artifact_text {
    ($name:ident, $field:literal) => {
        #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        pub struct $name(String);

        impl $name {
            pub fn new(value: impl Into<String>) -> Result<Self, ValueRequired> {
                required_text($field, value).map(Self)
            }

            #[must_use]
            pub fn as_str(&self) -> &str {
                &self.0
            }
        }
    };
}

artifact_text!(ModelArtifactId, "model artifact id");
artifact_text!(ModelArtifactFormat, "model artifact format");
artifact_text!(ModelArtifactRevision, "model artifact revision");
artifact_text!(ModelArtifactDigest, "model artifact digest");

/// Host-owned reference to one pre-existing model artifact.
#[derive(Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ModelArtifactRef(String);

impl ModelArtifactRef {
    pub fn new(value: impl Into<String>) -> Result<Self, ValueRequired> {
        required_text("model artifact reference", value).map(Self)
    }

    /// Passes the opaque reference back to the execution host.
    #[must_use]
    pub fn as_host_value(&self) -> &str {
        &self.0
    }
}

impl fmt::Debug for ModelArtifactRef {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_tuple("ModelArtifactRef")
            .field(&"<opaque>")
            .finish()
    }
}

/// Safe expected identity for one model artifact. This does not authorize a host path.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ModelArtifactDescriptor {
    id: ModelArtifactId,
    format: ModelArtifactFormat,
    revision: ModelArtifactRevision,
    digest: ModelArtifactDigest,
    quantization: Option<String>,
}

impl ModelArtifactDescriptor {
    #[must_use]
    pub const fn new(
        id: ModelArtifactId,
        format: ModelArtifactFormat,
        revision: ModelArtifactRevision,
        digest: ModelArtifactDigest,
    ) -> Self {
        Self {
            id,
            format,
            revision,
            digest,
            quantization: None,
        }
    }

    pub fn with_quantization(
        mut self,
        quantization: impl Into<String>,
    ) -> Result<Self, ValueRequired> {
        self.quantization = Some(required_text("model artifact quantization", quantization)?);
        Ok(self)
    }

    #[must_use]
    pub const fn id(&self) -> &ModelArtifactId {
        &self.id
    }

    #[must_use]
    pub const fn format(&self) -> &ModelArtifactFormat {
        &self.format
    }

    #[must_use]
    pub const fn revision(&self) -> &ModelArtifactRevision {
        &self.revision
    }

    #[must_use]
    pub const fn digest(&self) -> &ModelArtifactDigest {
        &self.digest
    }

    #[must_use]
    pub fn quantization(&self) -> Option<&str> {
        self.quantization.as_deref()
    }
}

/// One exact artifact identity paired with its opaque host reference.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ModelArtifactBinding {
    reference: ModelArtifactRef,
    descriptor: ModelArtifactDescriptor,
}

impl ModelArtifactBinding {
    #[must_use]
    pub const fn new(reference: ModelArtifactRef, descriptor: ModelArtifactDescriptor) -> Self {
        Self {
            reference,
            descriptor,
        }
    }

    #[must_use]
    pub const fn reference(&self) -> &ModelArtifactRef {
        &self.reference
    }

    #[must_use]
    pub const fn descriptor(&self) -> &ModelArtifactDescriptor {
        &self.descriptor
    }
}

#[cfg(test)]
mod tests {
    use super::{
        ModelArtifactBinding, ModelArtifactDescriptor, ModelArtifactDigest, ModelArtifactFormat,
        ModelArtifactId, ModelArtifactRef, ModelArtifactRevision,
    };

    #[test]
    fn artifact_identity_and_host_reference_remain_separate() {
        let reference = ModelArtifactRef::new("/private/models/model.gguf")
            .expect("artifact reference is valid");
        let binding = ModelArtifactBinding::new(
            reference.clone(),
            ModelArtifactDescriptor::new(
                ModelArtifactId::new("stories-260k").expect("artifact id is valid"),
                ModelArtifactFormat::new("gguf").expect("format is valid"),
                ModelArtifactRevision::new("revision-1").expect("revision is valid"),
                ModelArtifactDigest::new("sha256:fixture").expect("digest is valid"),
            )
            .with_quantization("q8_0")
            .expect("quantization is valid"),
        );

        assert_eq!(binding.descriptor().id().as_str(), "stories-260k");
        assert_eq!(binding.descriptor().quantization(), Some("q8_0"));
        assert!(!format!("{binding:?}").contains(reference.as_host_value()));
    }
}

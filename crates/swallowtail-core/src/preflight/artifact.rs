use super::{PreflightContext, PreflightPlan};
use crate::{ModelArtifactBinding, ModelArtifactDescriptor, ModelArtifactRef};

impl<'a> PreflightContext<'a> {
    #[must_use]
    /// Binds the exact local model artifact selected for the operation.
    pub const fn with_model_artifact(mut self, artifact: &'a ModelArtifactBinding) -> Self {
        self.model_artifact = Some(artifact);
        self
    }
}

impl PreflightPlan {
    #[must_use]
    /// Returns the complete model-artifact binding frozen by preflight.
    pub fn model_artifact_binding(&self) -> Option<&ModelArtifactBinding> {
        self.binding.model_artifact.as_ref()
    }

    #[must_use]
    /// Returns the opaque host reference for the selected artifact.
    pub fn model_artifact_reference(&self) -> Option<&ModelArtifactRef> {
        self.model_artifact_binding()
            .map(ModelArtifactBinding::reference)
    }

    #[must_use]
    /// Returns the validated descriptor for the selected artifact.
    pub fn model_artifact_descriptor(&self) -> Option<&ModelArtifactDescriptor> {
        self.model_artifact_binding()
            .map(ModelArtifactBinding::descriptor)
    }
}

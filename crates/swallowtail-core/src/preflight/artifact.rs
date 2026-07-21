use super::{PreflightContext, PreflightPlan};
use crate::{ModelArtifactBinding, ModelArtifactDescriptor, ModelArtifactRef};

impl<'a> PreflightContext<'a> {
    #[must_use]
    pub const fn with_model_artifact(mut self, artifact: &'a ModelArtifactBinding) -> Self {
        self.model_artifact = Some(artifact);
        self
    }
}

impl PreflightPlan {
    #[must_use]
    pub fn model_artifact_binding(&self) -> Option<&ModelArtifactBinding> {
        self.binding.model_artifact.as_ref()
    }

    #[must_use]
    pub fn model_artifact_reference(&self) -> Option<&ModelArtifactRef> {
        self.model_artifact_binding()
            .map(ModelArtifactBinding::reference)
    }

    #[must_use]
    pub fn model_artifact_descriptor(&self) -> Option<&ModelArtifactDescriptor> {
        self.model_artifact_binding()
            .map(ModelArtifactBinding::descriptor)
    }
}

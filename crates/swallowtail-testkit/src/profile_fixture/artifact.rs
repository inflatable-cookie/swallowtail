use super::valid;
use swallowtail_core::{
    ModelArtifactBinding, ModelArtifactDescriptor, ModelArtifactDigest, ModelArtifactFormat,
    ModelArtifactId, ModelArtifactRef, ModelArtifactRevision,
};

pub(super) fn fixture_artifact() -> ModelArtifactBinding {
    ModelArtifactBinding::new(
        valid(ModelArtifactRef::new, "fixture.model-artifact"),
        ModelArtifactDescriptor::new(
            valid(ModelArtifactId::new, "fixture-artifact"),
            valid(ModelArtifactFormat::new, "gguf"),
            valid(ModelArtifactRevision::new, "fixture-artifact-revision-1"),
            valid(ModelArtifactDigest::new, "sha256:fixture-artifact"),
        ),
    )
}

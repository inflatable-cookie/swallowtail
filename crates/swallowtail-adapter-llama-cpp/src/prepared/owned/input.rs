use crate::LlamaCppModelSelection;
use swallowtail_core::{
    AccessProfile, ConfiguredInstanceId, ExecutionHostId, InstanceRevision, InstanceTargetRef,
    ModelArtifactBinding,
};
use swallowtail_runtime::PreparedAccessEvidence;

#[derive(Clone, Debug, Eq, PartialEq)]
/// Exact model-artifact and model-route selection for owned serving.
pub struct LlamaCppOwnedServingSelection {
    artifact: ModelArtifactBinding,
    model: LlamaCppModelSelection,
}

impl LlamaCppOwnedServingSelection {
    /// Creates one explicit artifact-backed serving selection.
    #[must_use]
    pub const fn new(artifact: ModelArtifactBinding, model: LlamaCppModelSelection) -> Self {
        Self { artifact, model }
    }

    pub(super) fn into_parts(self) -> (ModelArtifactBinding, LlamaCppModelSelection) {
        (self.artifact, self.model)
    }
}

#[derive(Clone)]
/// Inputs for admitting a host-owned llama.cpp executable and artifact.
pub struct LlamaCppOwnedPreparationInput {
    instance_id: ConfiguredInstanceId,
    instance_revision: InstanceRevision,
    execution_host: ExecutionHostId,
    executable_target: InstanceTargetRef,
    access: AccessProfile,
    evidence: PreparedAccessEvidence,
    serving: LlamaCppOwnedServingSelection,
}

impl LlamaCppOwnedPreparationInput {
    /// Creates explicit executable, access, artifact, and route inputs.
    #[must_use]
    pub const fn new(
        instance_id: ConfiguredInstanceId,
        instance_revision: InstanceRevision,
        execution_host: ExecutionHostId,
        executable_target: InstanceTargetRef,
        access: AccessProfile,
        evidence: PreparedAccessEvidence,
        serving: LlamaCppOwnedServingSelection,
    ) -> Self {
        Self {
            instance_id,
            instance_revision,
            execution_host,
            executable_target,
            access,
            evidence,
            serving,
        }
    }

    pub(super) fn into_parts(
        self,
    ) -> (
        ConfiguredInstanceId,
        InstanceRevision,
        ExecutionHostId,
        InstanceTargetRef,
        AccessProfile,
        PreparedAccessEvidence,
        LlamaCppOwnedServingSelection,
    ) {
        (
            self.instance_id,
            self.instance_revision,
            self.execution_host,
            self.executable_target,
            self.access,
            self.evidence,
            self.serving,
        )
    }
}

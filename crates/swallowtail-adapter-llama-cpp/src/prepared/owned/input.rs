use crate::LlamaCppModelSelection;
use swallowtail_core::{
    AccessProfile, ConfiguredInstanceId, ExecutionHostId, InstanceRevision, InstanceTargetRef,
    ModelArtifactBinding,
};
use swallowtail_runtime::PreparedAccessEvidence;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LlamaCppOwnedServingSelection {
    artifact: ModelArtifactBinding,
    model: LlamaCppModelSelection,
}

impl LlamaCppOwnedServingSelection {
    #[must_use]
    pub const fn new(artifact: ModelArtifactBinding, model: LlamaCppModelSelection) -> Self {
        Self { artifact, model }
    }

    pub(super) fn into_parts(self) -> (ModelArtifactBinding, LlamaCppModelSelection) {
        (self.artifact, self.model)
    }
}

#[derive(Clone)]
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

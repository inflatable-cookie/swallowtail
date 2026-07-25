use super::CodexPreparationInput;
use super::failure::{CompatibilityBehavior, preparation_failure};
use crate::selection::CODEX_EXEC_BEHAVIOR;
use crate::{CodexPreparedDriver, codex_app_server_descriptor, codex_exec_descriptor};
use swallowtail_core::{
    Capability, CapabilityProfile, CapabilityRequirement, ConfiguredInstance,
    HarnessConfigurationPosture, InstalledExecutableObservation, InstanceOwnership,
    InstancePolicyId, InstanceTargetRef, ProtocolFacadeId,
};
use swallowtail_runtime::{PreparationFailure, PreparationStage};

pub(super) fn configured_instance(
    input: &CodexPreparationInput,
    observation: &InstalledExecutableObservation,
) -> Result<ConfiguredInstance, PreparationFailure> {
    let target =
        InstanceTargetRef::new(input.target.executable().as_host_value()).map_err(|_| {
            preparation_failure(
                PreparationStage::TargetSelection,
                "swallowtail.codex.preparation.target_invalid",
                "Codex approved target could not be bound to the configured instance",
            )
        })?;
    let (descriptor, ownership, facade, policy, capabilities, posture) = match input.driver {
        CodexPreparedDriver::StructuredExec => {
            let behavior = observation
                .compatibility()
                .behavior_revision()
                .map(|revision| revision.as_str());
            let posture = if behavior == Some(CODEX_EXEC_BEHAVIOR) {
                HarnessConfigurationPosture::ProviderSuppressed
            } else {
                HarnessConfigurationPosture::Ambient
            };
            (
                codex_exec_descriptor(),
                InstanceOwnership::HostOwnedEphemeral,
                "codex-exec-jsonl",
                "codex-exec-prepared",
                CapabilityProfile::new([CapabilityRequirement::new(Capability::StructuredRun, [])]),
                posture,
            )
        }
        CodexPreparedDriver::AppServer => (
            codex_app_server_descriptor(),
            InstanceOwnership::HostOwnedPersistent,
            "codex-app-server-v2",
            "codex-app-server-prepared",
            CapabilityProfile::new([
                CapabilityRequirement::new(Capability::ModelCatalog, []),
                CapabilityRequirement::new(Capability::InteractiveSession, []),
            ]),
            HarnessConfigurationPosture::Ambient,
        ),
    };
    Ok(ConfiguredInstance::new(
        input.instance_id.clone(),
        input.instance_revision.clone(),
        descriptor.identity().id().clone(),
        input.execution_host_id.clone(),
        target,
        ownership,
        input.access_profile.id().clone(),
        input.access_profile.support_authority(),
        ProtocolFacadeId::new(facade).expect("static Codex facade is valid"),
        InstancePolicyId::new(policy).expect("static Codex policy is valid"),
        capabilities,
    )
    .with_interface_versions([observation.version().clone()])
    .with_harness_configuration_posture(posture))
}

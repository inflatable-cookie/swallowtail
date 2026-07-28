use super::{KimiHeadlessPreparationInput, preparation};
use swallowtail_core::{
    Capability, CapabilityConstraint, CapabilityProfile, CapabilityRequirement, ConfiguredInstance,
    HarnessConfigurationPosture, InstalledExecutableObservation, InstanceOwnership,
    InstancePolicyId, InstanceTargetRef, ProtocolFacadeId, ResourceAccess, ResourceRepresentation,
};
use swallowtail_runtime::{PreparationFailure, PreparationStage};

pub(super) fn configured_instance(
    input: &KimiHeadlessPreparationInput,
    observation: &InstalledExecutableObservation,
) -> Result<ConfiguredInstance, PreparationFailure> {
    let target =
        InstanceTargetRef::new(input.target.executable().as_host_value()).map_err(|_| {
            preparation::failure(
                PreparationStage::TargetSelection,
                "swallowtail.kimi.headless.preparation.target_invalid",
                "Kimi headless approved target could not be bound to the configured instance",
            )
        })?;
    Ok(ConfiguredInstance::new(
        input.instance_id.clone(),
        input.instance_revision.clone(),
        crate::kimi_headless_descriptor().identity().id().clone(),
        input.execution_host_id.clone(),
        target,
        InstanceOwnership::HostOwnedEphemeral,
        input.access_profile.id().clone(),
        input.access_profile.support_authority(),
        ProtocolFacadeId::new("kimi-headless-stream-json-v1")
            .expect("static Kimi headless facade is valid"),
        InstancePolicyId::new("kimi-headless-ambient-read-write")
            .expect("static Kimi headless policy is valid"),
        run_capabilities(),
    )
    .with_interface_versions([observation.version().clone()])
    .with_harness_configuration_posture(HarnessConfigurationPosture::Ambient))
}

pub(super) fn run_capabilities() -> CapabilityProfile {
    CapabilityProfile::new([
        CapabilityRequirement::new(Capability::StructuredRun, []),
        CapabilityRequirement::new(Capability::StreamingEvents, []),
        CapabilityRequirement::new(Capability::ProviderDurableRetention, []),
        CapabilityRequirement::new(
            Capability::Interruption,
            [CapabilityConstraint::CancellationScope(
                swallowtail_core::CancellationScope::StructuredRun,
            )],
        ),
        CapabilityRequirement::new(
            Capability::WorkingResource,
            [
                CapabilityConstraint::ResourceAccess(ResourceAccess::ReadWrite),
                CapabilityConstraint::ResourceRepresentation(ResourceRepresentation::Filesystem),
            ],
        ),
    ])
}

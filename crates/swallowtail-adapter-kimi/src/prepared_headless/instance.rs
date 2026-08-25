use super::{KimiHeadlessPreparationInput, preparation};
use crate::selection::{HEADLESS_BEHAVIOR, HEADLESS_BEHAVIOR_V2};
use swallowtail_core::{
    Capability, CapabilityConstraint, CapabilityProfile, CapabilityRequirement, ConfiguredInstance,
    HarnessConfigurationPosture, InstalledExecutableCompatibility, InstalledExecutableObservation,
    InstanceOwnership, InstancePolicyId, InstanceTargetRef, ProtocolFacadeId, ResourceAccess,
    ResourceRepresentation,
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
        headless_protocol_facade_id(observation)?,
        InstancePolicyId::new("kimi-headless-ambient-read-write")
            .expect("static Kimi headless policy is valid"),
        run_capabilities(),
    )
    .with_interface_versions([observation.version().clone()])
    .with_harness_configuration_posture(HarnessConfigurationPosture::Ambient))
}

fn headless_protocol_facade_id(
    observation: &InstalledExecutableObservation,
) -> Result<ProtocolFacadeId, PreparationFailure> {
    let revision = match observation.compatibility() {
        InstalledExecutableCompatibility::Qualified(matched) => {
            matched.behavior_revision().as_str()
        }
        InstalledExecutableCompatibility::UnverifiedNewer(newer) => {
            newer.behavior_revision().as_str()
        }
        InstalledExecutableCompatibility::Incompatible => {
            return Err(preparation::failure(
                PreparationStage::CompatibilityClassification,
                "swallowtail.kimi.headless.preparation.behavior_incompatible",
                "Kimi headless executable behavior is not mapped by this driver",
            ));
        }
    };
    let facade = match revision {
        HEADLESS_BEHAVIOR => "kimi-headless-stream-json-v1",
        HEADLESS_BEHAVIOR_V2 => "kimi-headless-stream-json-v2",
        _ => {
            return Err(preparation::failure(
                PreparationStage::CompatibilityClassification,
                "swallowtail.kimi.headless.preparation.behavior_incompatible",
                "Kimi headless executable behavior is not mapped by this driver",
            ));
        }
    };
    Ok(ProtocolFacadeId::new(facade).expect("static Kimi headless facade is valid"))
}

pub(super) fn run_capabilities() -> CapabilityProfile {
    CapabilityProfile::new([
        CapabilityRequirement::new(Capability::StructuredRun, []),
        CapabilityRequirement::new(Capability::StreamingEvents, []),
        CapabilityRequirement::new(Capability::ProviderDurableRetention, []),
        CapabilityRequirement::new(Capability::ProviderManagedRecovery, []),
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

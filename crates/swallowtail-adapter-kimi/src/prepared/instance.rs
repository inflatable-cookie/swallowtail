use super::KimiPreparationInput;
use super::failure::preparation_failure;
use crate::kimi_acp_descriptor;
use crate::selection::KimiAcpBehavior;
use swallowtail_core::{
    Capability, CapabilityConstraint, CapabilityProfile, CapabilityRequirement, ConfiguredInstance,
    HarnessConfigurationPosture, HarnessMode, InstalledExecutableCompatibility,
    InstalledExecutableObservation, InstanceOwnership, InstancePolicyId, InstanceTargetRef,
    ProtocolFacadeId, ReasoningMode, ResourceAccess, ResourceRepresentation,
};
use swallowtail_runtime::{PreparationFailure, PreparationStage};

pub(super) fn configured_instance(
    input: &KimiPreparationInput,
    observation: &InstalledExecutableObservation,
) -> Result<ConfiguredInstance, PreparationFailure> {
    let target =
        InstanceTargetRef::new(input.target.executable().as_host_value()).map_err(|_| {
            preparation_failure(
                PreparationStage::TargetSelection,
                "swallowtail.kimi.preparation.target_invalid",
                "Kimi approved target could not be bound to the configured instance",
            )
        })?;
    let behavior = acp_behavior(observation)?;
    Ok(ConfiguredInstance::new(
        input.instance_id.clone(),
        input.instance_revision.clone(),
        kimi_acp_descriptor().identity().id().clone(),
        input.execution_host_id.clone(),
        target,
        InstanceOwnership::HostOwnedEphemeral,
        input.access_profile.id().clone(),
        input.access_profile.support_authority(),
        ProtocolFacadeId::new("acp-v1").expect("static Kimi facade is valid"),
        InstancePolicyId::new("kimi-prepared-ambient").expect("static Kimi policy is valid"),
        session_capabilities(behavior),
    )
    .with_interface_versions([observation.version().clone()])
    .with_harness_configuration_posture(HarnessConfigurationPosture::Ambient))
}

pub(crate) fn acp_behavior(
    observation: &InstalledExecutableObservation,
) -> Result<KimiAcpBehavior, PreparationFailure> {
    let revision = match observation.compatibility() {
        InstalledExecutableCompatibility::Qualified(assessment) => {
            assessment.behavior_revision().as_str()
        }
        InstalledExecutableCompatibility::UnverifiedNewer(_)
        | InstalledExecutableCompatibility::Incompatible => {
            return Err(preparation_failure(
                PreparationStage::CompatibilityClassification,
                "swallowtail.kimi.preparation.behavior_incompatible",
                "Kimi ACP executable behavior is not mapped by this driver",
            ));
        }
    };
    KimiAcpBehavior::from_revision(revision).ok_or_else(|| {
        preparation_failure(
            PreparationStage::CompatibilityClassification,
            "swallowtail.kimi.preparation.behavior_incompatible",
            "Kimi ACP executable behavior is not mapped by this driver",
        )
    })
}

pub(crate) fn session_capabilities(behavior: KimiAcpBehavior) -> CapabilityProfile {
    CapabilityProfile::new([
        CapabilityRequirement::new(Capability::InteractiveSession, []),
        CapabilityRequirement::new(
            Capability::LoadSession,
            [
                CapabilityConstraint::ReplayMaximumItems(crate::MAXIMUM_REPLAY_ITEMS as u32),
                CapabilityConstraint::ReplayMaximumBytes(crate::MAXIMUM_REPLAY_BYTES as u64),
            ],
        ),
        CapabilityRequirement::new(Capability::Resume, []),
        CapabilityRequirement::new(Capability::StreamingEvents, []),
        CapabilityRequirement::new(
            Capability::Interruption,
            [CapabilityConstraint::CancellationScope(
                swallowtail_core::CancellationScope::ActiveTurn,
            )],
        ),
        CapabilityRequirement::new(
            Capability::WorkingResource,
            [
                CapabilityConstraint::ResourceAccess(ResourceAccess::ReadWrite),
                CapabilityConstraint::ResourceRepresentation(ResourceRepresentation::Filesystem),
            ],
        ),
        CapabilityRequirement::new(
            Capability::WorkingResourceTextWrite,
            [CapabilityConstraint::WorkingResourceMaximumBytes(
                crate::MAXIMUM_WRITE_BYTES as u64,
            )],
        ),
        CapabilityRequirement::new(
            Capability::ReasoningSelection,
            behavior.admitted_reasoning_modes().iter().map(|mode| {
                CapabilityConstraint::ReasoningMode(
                    ReasoningMode::new(*mode).expect("static Kimi reasoning mode is valid"),
                )
            }),
        ),
        CapabilityRequirement::new(
            Capability::HarnessModeSelection,
            [CapabilityConstraint::HarnessMode(HarnessMode::Plan)],
        ),
    ])
}

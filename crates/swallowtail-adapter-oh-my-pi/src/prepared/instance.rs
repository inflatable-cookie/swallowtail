use super::OhMyPiPreparationInput;
use super::failure::preparation_failure;
use crate::oh_my_pi_rpc_descriptor;
use std::num::NonZeroU32;
use swallowtail_core::{
    Capability, CapabilityConstraint, CapabilityProfile, CapabilityRequirement, ConfiguredInstance,
    HarnessConfigurationPosture, HarnessRpcPolicy, HarnessSchedulingBounds,
    InstalledExecutableObservation, InstanceOwnership, InstancePolicyId, InstanceTargetRef,
    ProtocolFacadeId, ResourceAccess, ResourceRepresentation,
};
use swallowtail_runtime::{PreparationFailure, PreparationStage};

pub(super) fn configured_instance(
    input: &OhMyPiPreparationInput,
    observation: &InstalledExecutableObservation,
) -> Result<ConfiguredInstance, PreparationFailure> {
    let target =
        InstanceTargetRef::new(input.target.executable().as_host_value()).map_err(|_| {
            preparation_failure(
                PreparationStage::TargetSelection,
                "swallowtail.oh_my_pi.preparation.target_invalid",
                "OhMyPi approved target could not be bound to the configured instance",
            )
        })?;
    Ok(ConfiguredInstance::new(
        input.instance_id.clone(),
        input.instance_revision.clone(),
        oh_my_pi_rpc_descriptor().identity().id().clone(),
        input.execution_host_id.clone(),
        target,
        InstanceOwnership::HostOwnedEphemeral,
        input.access_profile.id().clone(),
        input.access_profile.support_authority(),
        ProtocolFacadeId::new("oh-my-pi-rpc-v2").expect("static OhMyPi facade is valid"),
        InstancePolicyId::new("oh-my-pi-prepared-ambient-read")
            .expect("static OhMyPi policy is valid"),
        session_capabilities(false),
    )
    .with_interface_versions([observation.version().clone()])
    .with_harness_configuration_posture(HarnessConfigurationPosture::ProviderSuppressed)
    .with_harness_rpc_policy(rpc_policy()))
}

pub(crate) fn session_capabilities(image_attachments: bool) -> CapabilityProfile {
    let mut capabilities = vec![
        CapabilityRequirement::new(Capability::InteractiveSession, []),
        CapabilityRequirement::new(Capability::StreamingEvents, []),
        CapabilityRequirement::new(Capability::UsageReporting, []),
        CapabilityRequirement::new(
            Capability::Interruption,
            [CapabilityConstraint::CancellationScope(
                swallowtail_core::CancellationScope::ActiveTurn,
            )],
        ),
        CapabilityRequirement::new(
            Capability::WorkingResource,
            [
                CapabilityConstraint::ResourceAccess(ResourceAccess::Read),
                CapabilityConstraint::ResourceRepresentation(ResourceRepresentation::Filesystem),
            ],
        ),
    ];
    if image_attachments {
        capabilities.push(image_attachment_capability());
    }
    CapabilityProfile::new(capabilities)
}

pub(crate) fn run_capabilities(image_attachments: bool) -> CapabilityProfile {
    let mut capabilities = vec![
        CapabilityRequirement::new(Capability::StructuredRun, []),
        CapabilityRequirement::new(Capability::StreamingEvents, []),
        CapabilityRequirement::new(Capability::UsageReporting, []),
        CapabilityRequirement::new(
            Capability::Interruption,
            [CapabilityConstraint::CancellationScope(
                swallowtail_core::CancellationScope::StructuredRun,
            )],
        ),
        CapabilityRequirement::new(
            Capability::WorkingResource,
            [
                CapabilityConstraint::ResourceAccess(ResourceAccess::Read),
                CapabilityConstraint::ResourceRepresentation(ResourceRepresentation::Filesystem),
            ],
        ),
    ];
    if image_attachments {
        capabilities.push(image_attachment_capability());
    }
    CapabilityProfile::new(capabilities)
}

pub(crate) fn image_attachment_capability() -> CapabilityRequirement {
    CapabilityRequirement::new(
        Capability::Attachments,
        [
            CapabilityConstraint::attachment_media_type("image/png")
                .expect("static media type is valid"),
            CapabilityConstraint::AttachmentMaximumBytes(1024 * 1024),
            CapabilityConstraint::AttachmentMaximumCount(1),
        ],
    )
}

pub(crate) fn reasoning_capability(
    mode: &swallowtail_core::ReasoningMode,
) -> CapabilityRequirement {
    CapabilityRequirement::new(
        Capability::ReasoningSelection,
        [CapabilityConstraint::reasoning_mode(mode.clone())],
    )
}

pub(crate) fn rpc_policy() -> HarnessRpcPolicy {
    let one = NonZeroU32::new(1).expect("one is non-zero");
    HarnessRpcPolicy::restrictive(HarnessSchedulingBounds::new(
        one,
        NonZeroU32::new(2).expect("two is non-zero"),
        one,
        one,
    ))
}

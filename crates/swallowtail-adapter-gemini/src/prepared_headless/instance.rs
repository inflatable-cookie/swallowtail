use super::{GeminiHeadlessPreparationInput, preparation_failure};
use swallowtail_core::{
    Capability, CapabilityConstraint, CapabilityProfile, CapabilityRequirement, ConfiguredInstance,
    HarnessConfigurationPosture, InstalledExecutableObservation, InstanceOwnership,
    InstancePolicyId, InstanceTargetRef, ProtocolFacadeId, ResourceAccess, ResourceRepresentation,
};
use swallowtail_runtime::{PreparationFailure, PreparationStage};

pub(super) fn configured_instance(
    input: &GeminiHeadlessPreparationInput,
    observation: &InstalledExecutableObservation,
) -> Result<ConfiguredInstance, PreparationFailure> {
    let target =
        InstanceTargetRef::new(input.target.executable().as_host_value()).map_err(|_| {
            preparation_failure(
                PreparationStage::TargetSelection,
                "swallowtail.gemini.headless.preparation.target_invalid",
                "Gemini headless approved target could not be bound to the configured instance",
            )
        })?;
    Ok(ConfiguredInstance::new(
        input.instance_id.clone(),
        input.instance_revision.clone(),
        crate::gemini_headless_descriptor().identity().id().clone(),
        input.execution_host_id.clone(),
        target,
        InstanceOwnership::HostOwnedEphemeral,
        input.access_profile.id().clone(),
        input.access_profile.support_authority(),
        ProtocolFacadeId::new("gemini-headless-stream-json-v1")
            .expect("static Gemini headless facade is valid"),
        InstancePolicyId::new("gemini-headless-ambient-plan")
            .expect("static Gemini headless policy is valid"),
        run_capabilities(),
    )
    .with_interface_versions([observation.version().clone()])
    .with_harness_configuration_posture(HarnessConfigurationPosture::Ambient))
}

pub(super) fn run_capabilities() -> CapabilityProfile {
    CapabilityProfile::new([
        CapabilityRequirement::new(Capability::StructuredRun, []),
        CapabilityRequirement::new(Capability::StreamingEvents, []),
        CapabilityRequirement::new(Capability::UsageReporting, []),
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
                CapabilityConstraint::ResourceAccess(ResourceAccess::Read),
                CapabilityConstraint::ResourceRepresentation(ResourceRepresentation::Filesystem),
            ],
        ),
    ])
}

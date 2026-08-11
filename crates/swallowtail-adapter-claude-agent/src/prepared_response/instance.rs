use super::{ClaudeCodeResponsePreparationInput, preparation};
use swallowtail_core::{
    Capability, CapabilityConstraint, CapabilityProfile, CapabilityRequirement, ConfiguredInstance,
    HarnessConfigurationPosture, InstalledExecutableObservation, InstanceOwnership,
    InstancePolicyId, InstanceTargetRef, ProtocolFacadeId,
};
use swallowtail_runtime::{PreparationFailure, PreparationStage};

pub(crate) const REASONING_MODES: [&str; 6] = ["default", "low", "medium", "high", "xhigh", "max"];

pub(super) fn configured_instance(
    input: &ClaudeCodeResponsePreparationInput,
    observation: &InstalledExecutableObservation,
) -> Result<ConfiguredInstance, PreparationFailure> {
    let target =
        InstanceTargetRef::new(input.target.executable().as_host_value()).map_err(|_| {
            preparation::preparation_failure(
                PreparationStage::TargetSelection,
                "swallowtail.claude_code.response_only.preparation.target_invalid",
                "Claude Code response-only target could not be bound to the configured instance",
            )
        })?;
    Ok(ConfiguredInstance::new(
        input.instance_id.clone(),
        input.instance_revision.clone(),
        crate::claude_code_response_only_descriptor()
            .identity()
            .id()
            .clone(),
        input.execution_host_id.clone(),
        target,
        InstanceOwnership::HostOwnedEphemeral,
        input.access_profile.id().clone(),
        input.access_profile.support_authority(),
        ProtocolFacadeId::new("claude-code-response-only-stream-json-v1")
            .expect("static Claude Code response-only facade is valid"),
        InstancePolicyId::new("claude-code-safe-tool-free-text")
            .expect("static Claude Code response-only policy is valid"),
        run_capabilities(),
    )
    .with_interface_versions([observation.version().clone()])
    .with_harness_configuration_posture(HarnessConfigurationPosture::ProviderSuppressed))
}

pub(super) fn run_capabilities() -> CapabilityProfile {
    CapabilityProfile::new([
        CapabilityRequirement::new(Capability::StructuredRun, []),
        CapabilityRequirement::new(Capability::StreamingEvents, []),
        CapabilityRequirement::new(Capability::UsageReporting, []),
        CapabilityRequirement::new(
            Capability::ReasoningSelection,
            REASONING_MODES.into_iter().map(|mode| {
                CapabilityConstraint::ReasoningMode(
                    swallowtail_core::ReasoningMode::new(mode)
                        .expect("static Claude Code reasoning mode is valid"),
                )
            }),
        ),
        CapabilityRequirement::new(
            Capability::Interruption,
            [CapabilityConstraint::CancellationScope(
                swallowtail_core::CancellationScope::StructuredRun,
            )],
        ),
    ])
}

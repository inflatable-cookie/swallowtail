use super::ClaudeAgentPreparationInput;
use super::failure::preparation_failure;
use crate::claude_agent_acp_descriptor;
use swallowtail_core::{
    Capability, CapabilityConstraint, CapabilityProfile, CapabilityRequirement, ConfiguredInstance,
    HarnessConfigurationPosture, InstalledExecutableObservation, InstanceOwnership,
    InstancePolicyId, InstanceTargetRef, ProtocolFacadeId, ResourceAccess, ResourceRepresentation,
};
use swallowtail_runtime::{PreparationFailure, PreparationStage};

pub(crate) const REASONING_MODES: [&str; 6] = ["default", "low", "medium", "high", "xhigh", "max"];

pub(super) fn configured_instance(
    input: &ClaudeAgentPreparationInput,
    observation: &InstalledExecutableObservation,
) -> Result<ConfiguredInstance, PreparationFailure> {
    let target =
        InstanceTargetRef::new(input.target.executable().as_host_value()).map_err(|_| {
            preparation_failure(
                PreparationStage::TargetSelection,
                "swallowtail.claude_agent.preparation.target_invalid",
                "Claude Agent approved target could not be bound to the configured instance",
            )
        })?;
    Ok(ConfiguredInstance::new(
        input.instance_id.clone(),
        input.instance_revision.clone(),
        claude_agent_acp_descriptor().identity().id().clone(),
        input.execution_host_id.clone(),
        target,
        InstanceOwnership::HostOwnedEphemeral,
        input.access_profile.id().clone(),
        input.access_profile.support_authority(),
        ProtocolFacadeId::new("acp-v1").expect("static Claude Agent facade is valid"),
        InstancePolicyId::new("claude-agent-prepared-ambient")
            .expect("static Claude Agent policy is valid"),
        session_capabilities(crate::selection::version_supports_config_options(
            observation.version().version(),
        )),
    )
    .with_interface_versions([observation.version().clone()])
    .with_harness_configuration_posture(HarnessConfigurationPosture::Ambient))
}

pub(crate) fn session_capabilities(reasoning: bool) -> CapabilityProfile {
    let mut capabilities = vec![
        CapabilityRequirement::new(Capability::InteractiveSession, []),
        CapabilityRequirement::new(Capability::StreamingEvents, []),
        CapabilityRequirement::new(Capability::UsageReporting, []),
        CapabilityRequirement::new(
            Capability::LoadSession,
            [
                CapabilityConstraint::ReplayMaximumItems(crate::MAXIMUM_REPLAY_ITEMS as u32),
                CapabilityConstraint::ReplayMaximumBytes(crate::MAXIMUM_REPLAY_BYTES as u64),
            ],
        ),
        CapabilityRequirement::new(Capability::Resume, []),
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
    add_reasoning_capability(&mut capabilities, reasoning);
    if reasoning {
        capabilities.push(CapabilityRequirement::new(
            Capability::HarnessModeSelection,
            [CapabilityConstraint::HarnessMode(
                swallowtail_core::HarnessMode::Plan,
            )],
        ));
    }
    CapabilityProfile::new(capabilities)
}

pub(crate) fn run_capabilities(reasoning: bool) -> CapabilityProfile {
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
                CapabilityConstraint::ResourceAccess(ResourceAccess::ReadWrite),
                CapabilityConstraint::ResourceRepresentation(ResourceRepresentation::Filesystem),
            ],
        ),
        CapabilityRequirement::new(Capability::ProviderDurableRetention, []),
    ];
    add_reasoning_capability(&mut capabilities, reasoning);
    CapabilityProfile::new(capabilities)
}

fn add_reasoning_capability(capabilities: &mut Vec<CapabilityRequirement>, supported: bool) {
    if supported {
        capabilities.push(CapabilityRequirement::new(
            Capability::ReasoningSelection,
            REASONING_MODES.into_iter().map(|mode| {
                CapabilityConstraint::ReasoningMode(
                    swallowtail_core::ReasoningMode::new(mode)
                        .expect("static Claude Agent reasoning mode is valid"),
                )
            }),
        ));
    }
}

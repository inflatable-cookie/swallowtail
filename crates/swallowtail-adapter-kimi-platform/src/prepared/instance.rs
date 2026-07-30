use super::KimiPlatformPreparationInput;
use swallowtail_core::{
    Capability, CapabilityConstraint, CapabilityProfile, CapabilityRequirement, ConfiguredInstance,
    InstanceOwnership, InstancePolicyId, ProtocolFacadeId, ReasoningMode,
};

pub(super) fn configured_instance(input: &KimiPlatformPreparationInput) -> ConfiguredInstance {
    ConfiguredInstance::new(
        input.instance_id.clone(),
        input.instance_revision.clone(),
        crate::kimi_platform_direct_descriptor()
            .identity()
            .id()
            .clone(),
        input.execution_host_id.clone(),
        input.endpoint_target.clone(),
        InstanceOwnership::ExternalAttached,
        input.access_profile.id().clone(),
        input.access_profile.support_authority(),
        ProtocolFacadeId::new(crate::KIMI_PLATFORM_FACADE_REVISION)
            .expect("static Kimi Platform facade is valid"),
        InstancePolicyId::new("public-platform-api-key")
            .expect("static Kimi Platform policy is valid"),
        all_capabilities(),
    )
    .with_interface_versions([crate::kimi_platform_facade_binding()])
}

pub(crate) fn all_capabilities() -> CapabilityProfile {
    let mut requirements = vec![
        CapabilityRequirement::new(Capability::ModelCatalog, []),
        CapabilityRequirement::new(Capability::StructuredRun, []),
        CapabilityRequirement::new(Capability::StreamingEvents, []),
        CapabilityRequirement::new(Capability::UsageReporting, []),
        CapabilityRequirement::new(Capability::OutputTokenLimit, []),
    ];
    requirements.extend(["low", "high", "max"].into_iter().map(|mode| {
        CapabilityRequirement::new(
            Capability::ReasoningSelection,
            [CapabilityConstraint::ReasoningMode(
                ReasoningMode::new(mode).expect("static Kimi reasoning mode is valid"),
            )],
        )
    }));
    CapabilityProfile::new(requirements)
}

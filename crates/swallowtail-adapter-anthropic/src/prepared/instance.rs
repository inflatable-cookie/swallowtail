use super::AnthropicPreparationInput;
use swallowtail_core::{
    Capability, CapabilityProfile, CapabilityRequirement, ConfiguredInstance, InstanceOwnership,
    InstancePolicyId, ProtocolFacadeId,
};

pub(super) fn configured_instance(input: &AnthropicPreparationInput) -> ConfiguredInstance {
    ConfiguredInstance::new(
        input.instance_id.clone(),
        input.instance_revision.clone(),
        crate::anthropic_direct_descriptor().identity().id().clone(),
        input.execution_host_id.clone(),
        input.endpoint_target.clone(),
        InstanceOwnership::ExternalAttached,
        input.access_profile.id().clone(),
        input.access_profile.support_authority(),
        ProtocolFacadeId::new(crate::ANTHROPIC_MESSAGES_FACADE_REVISION)
            .expect("static Anthropic facade is valid"),
        InstancePolicyId::new("public-api-key").expect("static Anthropic policy is valid"),
        all_capabilities(),
    )
    .with_interface_versions([crate::anthropic_messages_facade_binding()])
}

pub(crate) fn all_capabilities() -> CapabilityProfile {
    CapabilityProfile::new([
        CapabilityRequirement::new(Capability::ModelCatalog, []),
        CapabilityRequirement::new(Capability::StructuredRun, []),
        CapabilityRequirement::new(Capability::StreamingEvents, []),
        CapabilityRequirement::new(Capability::UsageReporting, []),
        CapabilityRequirement::new(Capability::OutputTokenLimit, []),
    ])
}

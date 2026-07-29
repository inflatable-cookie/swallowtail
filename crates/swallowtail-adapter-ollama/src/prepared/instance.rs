use super::OllamaPreparationInput;
use swallowtail_core::{
    Capability, CapabilityProfile, CapabilityRequirement, ConfiguredInstance, InstanceOwnership,
    InstancePolicyId, InterfaceVersionBinding, ProtocolFacadeId,
};

pub(super) fn configured_instance(
    input: &OllamaPreparationInput,
    version: &InterfaceVersionBinding,
) -> ConfiguredInstance {
    ConfiguredInstance::new(
        input.instance_id.clone(),
        input.instance_revision.clone(),
        crate::ollama_native_descriptor().identity().id().clone(),
        input.execution_host_id.clone(),
        input.endpoint_target.clone(),
        InstanceOwnership::ExternalAttached,
        input.access_profile.id().clone(),
        input.access_profile.support_authority(),
        ProtocolFacadeId::new(crate::OLLAMA_NATIVE_FACADE).expect("static Ollama facade is valid"),
        InstancePolicyId::new("attached-text-only").expect("static Ollama policy is valid"),
        all_capabilities(),
    )
    .with_interface_versions([version.clone()])
}

pub(crate) fn all_capabilities() -> CapabilityProfile {
    CapabilityProfile::new([
        CapabilityRequirement::new(Capability::ModelCatalog, []),
        CapabilityRequirement::new(Capability::StructuredRun, []),
        CapabilityRequirement::new(Capability::InteractiveSession, []),
        CapabilityRequirement::new(Capability::StreamingEvents, []),
        CapabilityRequirement::new(Capability::UsageReporting, []),
        CapabilityRequirement::new(Capability::OutputTokenLimit, []),
    ])
}

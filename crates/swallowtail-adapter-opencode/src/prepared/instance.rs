use super::OpenCodePreparationInput;
use swallowtail_core::{
    Capability, CapabilityConstraint, CapabilityProfile, CapabilityRequirement, ConfiguredInstance,
    HarnessConfigurationPosture, InstanceOwnership, InstancePolicyId, InterfaceVersionBinding,
    ProtocolFacadeId, ResourceAccess, ResourceRepresentation,
};

pub(super) fn configured_instance(
    input: &OpenCodePreparationInput,
    version: &InterfaceVersionBinding,
) -> ConfiguredInstance {
    ConfiguredInstance::new(
        input.instance_id.clone(),
        input.instance_revision.clone(),
        crate::opencode_http_descriptor().identity().id().clone(),
        input.execution_host_id.clone(),
        input.endpoint_target.clone(),
        InstanceOwnership::ExternalAttached,
        input.access_profile.id().clone(),
        input.access_profile.support_authority(),
        ProtocolFacadeId::new("opencode-http-sse-v1").expect("static OpenCode facade is valid"),
        InstancePolicyId::new("read-only-deny-first").expect("static OpenCode policy is valid"),
        all_capabilities(),
    )
    .with_interface_versions([version.clone()])
    .with_harness_configuration_posture(HarnessConfigurationPosture::Ambient)
}

pub(crate) fn all_capabilities() -> CapabilityProfile {
    CapabilityProfile::new([
        CapabilityRequirement::new(Capability::ModelCatalog, []),
        CapabilityRequirement::new(Capability::InteractiveSession, []),
        CapabilityRequirement::new(Capability::StreamingEvents, []),
        CapabilityRequirement::new(Capability::ProviderSessionDelete, []),
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
    ])
}

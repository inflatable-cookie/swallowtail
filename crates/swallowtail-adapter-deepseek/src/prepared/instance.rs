use super::DeepSeekPreparationInput;
use swallowtail_core::{
    Capability, CapabilityProfile, CapabilityRequirement, ConfiguredInstance, InstanceOwnership,
    InstancePolicyId, ProtocolFacadeId,
};

pub(super) fn configured_instance(input: &DeepSeekPreparationInput) -> ConfiguredInstance {
    ConfiguredInstance::new(
        input.instance_id.clone(),
        input.instance_revision.clone(),
        crate::deepseek_direct_descriptor().identity().id().clone(),
        input.execution_host_id.clone(),
        input.endpoint_target.clone(),
        InstanceOwnership::ExternalAttached,
        input.access_profile.id().clone(),
        input.access_profile.support_authority(),
        ProtocolFacadeId::new(crate::DEEPSEEK_FACADE_REVISION)
            .expect("static DeepSeek facade is valid"),
        InstancePolicyId::new("deepseek-public-api-key").expect("static DeepSeek policy is valid"),
        all_capabilities(input),
    )
    .with_interface_versions([crate::deepseek_facade_binding()])
}

pub(crate) fn all_capabilities(input: &DeepSeekPreparationInput) -> CapabilityProfile {
    let requirements = crate::deepseek_v4_requirements(
        input.execution_host_id.clone(),
        input.access_profile.id().clone(),
    );
    let mut capabilities: Vec<_> = requirements.capabilities().cloned().collect();
    capabilities.extend(
        crate::deepseek_v4_run_requirements(
            input.execution_host_id.clone(),
            input.access_profile.id().clone(),
        )
        .capabilities()
        .cloned(),
    );
    capabilities.push(CapabilityRequirement::new(Capability::ModelCatalog, []));
    CapabilityProfile::new(capabilities)
}

use super::AlibabaModelStudioPreparationInput;
use swallowtail_core::ConfiguredInstance;

pub(super) fn configured_instance(
    input: &AlibabaModelStudioPreparationInput,
) -> ConfiguredInstance {
    let base = crate::alibaba_model_studio_instance(input.execution_host_id.clone());
    ConfiguredInstance::new(
        base.id().clone(),
        input.instance_revision.clone(),
        base.driver_id().clone(),
        base.execution_host_id().clone(),
        base.target_reference().clone(),
        base.ownership(),
        input.access_profile.id().clone(),
        input.access_profile.support_authority(),
        base.protocol_facade_id().clone(),
        base.policy_id().clone(),
        base.capabilities().clone(),
    )
}

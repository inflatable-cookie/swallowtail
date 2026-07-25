use super::OpenAiBackgroundPreparationInput;
use swallowtail_core::ConfiguredInstance;

pub(super) fn configured_instance(input: &OpenAiBackgroundPreparationInput) -> ConfiguredInstance {
    crate::openai_background_instance(
        input.instance_revision.clone(),
        input.execution_host_id.clone(),
        input.endpoint_target.clone(),
        input.access_profile.id().clone(),
    )
}

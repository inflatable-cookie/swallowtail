#![allow(dead_code)]

use swallowtail_adapter_alibaba_model_studio::{
    AlibabaConversationProfileInput, AlibabaModelStudioPreparationInput,
    AlibabaModelStudioPreparedConversation, AlibabaModelStudioPreparedIntegration,
    prepare_alibaba_model_studio,
};
use swallowtail_runtime::{HostServices, PreparationFailure};

fn prepare_integration(
    input: AlibabaModelStudioPreparationInput,
    services: &HostServices,
) -> Result<AlibabaModelStudioPreparedIntegration, PreparationFailure> {
    prepare_alibaba_model_studio(input, services)
}

fn prepare_conversation(
    integration: &AlibabaModelStudioPreparedIntegration,
    input: AlibabaConversationProfileInput,
) -> Result<AlibabaModelStudioPreparedConversation, PreparationFailure> {
    integration.prepare_conversation(input)
}

fn main() {}

#![allow(dead_code)]

use swallowtail_adapter_bedrock::{
    BedrockPreparedInferenceAttempt, BedrockRuntimePreparationInput,
    BedrockRuntimePreparedIntegration, BedrockRuntimeProfileInput, prepare_bedrock_runtime,
};
use swallowtail_runtime::{HostServices, PreparationFailure};

fn prepare_integration(
    input: BedrockRuntimePreparationInput,
    services: &HostServices,
) -> Result<BedrockRuntimePreparedIntegration, PreparationFailure> {
    prepare_bedrock_runtime(input, services)
}

fn prepare_attempt(
    integration: &BedrockRuntimePreparedIntegration,
    input: BedrockRuntimeProfileInput,
) -> Result<BedrockPreparedInferenceAttempt, PreparationFailure> {
    integration.prepare_inference_attempt(input)
}

fn main() {}

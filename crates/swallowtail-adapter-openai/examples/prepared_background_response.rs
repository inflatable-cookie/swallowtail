#![allow(dead_code)]

use swallowtail_adapter_openai::{
    OpenAiBackgroundPreparationInput, OpenAiBackgroundPreparedIntegration,
    OpenAiBackgroundRunProfileInput, OpenAiPreparedBackgroundRun, prepare_openai_background,
};
use swallowtail_runtime::{HostServices, PreparationFailure};

fn prepare_integration(
    input: OpenAiBackgroundPreparationInput,
    services: &HostServices,
) -> Result<OpenAiBackgroundPreparedIntegration, PreparationFailure> {
    prepare_openai_background(input, services)
}

fn prepare_background_run(
    integration: &OpenAiBackgroundPreparedIntegration,
    input: OpenAiBackgroundRunProfileInput,
) -> Result<OpenAiPreparedBackgroundRun, PreparationFailure> {
    integration.prepare_background_run(input)
}

fn main() {}

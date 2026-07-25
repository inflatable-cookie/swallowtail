#![allow(dead_code)]

use swallowtail_adapter_deepseek::{
    DeepSeekCatalogueProfileInput, DeepSeekPreparationInput, DeepSeekPreparedCatalogue,
    DeepSeekPreparedIntegration, DeepSeekPreparedSession, DeepSeekSessionProfileInput,
    prepare_deepseek_direct,
};
use swallowtail_runtime::{HostServices, PreparationFailure};

fn prepare_integration(
    input: DeepSeekPreparationInput,
    services: &HostServices,
) -> Result<DeepSeekPreparedIntegration, PreparationFailure> {
    prepare_deepseek_direct(input, services)
}

fn prepare_catalogue(
    integration: &DeepSeekPreparedIntegration,
    input: DeepSeekCatalogueProfileInput,
) -> Result<DeepSeekPreparedCatalogue, PreparationFailure> {
    integration.prepare_catalogue(input)
}

fn prepare_session(
    integration: &DeepSeekPreparedIntegration,
    input: DeepSeekSessionProfileInput,
) -> Result<DeepSeekPreparedSession, PreparationFailure> {
    integration.prepare_session(input)
}

fn main() {}

#![allow(dead_code)]

use swallowtail_adapter_llama_cpp::{
    LlamaCppAttachedPreparationInput, LlamaCppAttachedPreparedIntegration,
    LlamaCppCatalogueProfileInput, LlamaCppInferenceProfileInput, LlamaCppPreparedCatalogue,
    LlamaCppPreparedInferenceAttempt, prepare_llama_cpp_attached,
};
use swallowtail_runtime::{HostServices, PreparationFailure};

fn prepare_integration(
    input: LlamaCppAttachedPreparationInput,
    services: &HostServices,
) -> Result<LlamaCppAttachedPreparedIntegration, PreparationFailure> {
    prepare_llama_cpp_attached(input, services)
}

fn prepare_catalogue(
    integration: &LlamaCppAttachedPreparedIntegration,
    input: LlamaCppCatalogueProfileInput,
) -> Result<LlamaCppPreparedCatalogue, PreparationFailure> {
    integration.prepare_catalogue(input)
}

fn prepare_inference(
    integration: &LlamaCppAttachedPreparedIntegration,
    input: LlamaCppInferenceProfileInput,
) -> Result<LlamaCppPreparedInferenceAttempt, PreparationFailure> {
    integration.prepare_inference_attempt(input)
}

fn main() {}

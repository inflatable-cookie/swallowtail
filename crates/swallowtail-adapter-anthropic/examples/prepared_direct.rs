#![allow(dead_code)]

use futures_util::StreamExt;
use swallowtail_adapter_anthropic::{
    AnthropicCatalogueProfileInput, AnthropicInferenceAttemptInput, AnthropicPreparationInput,
    AnthropicPreparedCatalogue, AnthropicPreparedInferenceAttempt, AnthropicPreparedIntegration,
    prepare_anthropic_direct,
};
use swallowtail_core::ModelCatalogEntry;
use swallowtail_runtime::{
    CleanupOutcome, HostServices, PreparationFailure, RuntimeFailure, TerminalOutcome,
};

fn prepare_integration(
    input: AnthropicPreparationInput,
    services: &HostServices,
) -> Result<AnthropicPreparedIntegration, PreparationFailure> {
    prepare_anthropic_direct(input, services)
}

fn prepare_catalogue(
    integration: &AnthropicPreparedIntegration,
    input: AnthropicCatalogueProfileInput,
) -> Result<AnthropicPreparedCatalogue, PreparationFailure> {
    integration.prepare_catalogue(input)
}

async fn observe_models(
    catalogue: &AnthropicPreparedCatalogue,
    services: HostServices,
) -> Result<Vec<ModelCatalogEntry>, RuntimeFailure> {
    catalogue.list_models(services).await
}

fn prepare_attempt(
    integration: &AnthropicPreparedIntegration,
    input: AnthropicInferenceAttemptInput,
) -> Result<AnthropicPreparedInferenceAttempt, PreparationFailure> {
    integration.prepare_inference_attempt(input)
}

async fn execute_one_attempt(
    attempt: &AnthropicPreparedInferenceAttempt,
    services: HostServices,
) -> Result<(TerminalOutcome, CleanupOutcome), RuntimeFailure> {
    let mut run = attempt.start_run(services).await?;
    let mut events = run
        .take_events()
        .expect("Anthropic runs expose one event stream");
    let terminal = run
        .take_terminal_outcome()
        .expect("Anthropic runs expose one terminal outcome");
    while let Some(event) = events.next().await {
        let _ = event?;
    }
    let outcome = terminal.await;
    Ok((outcome, run.close().await))
}

fn main() {}

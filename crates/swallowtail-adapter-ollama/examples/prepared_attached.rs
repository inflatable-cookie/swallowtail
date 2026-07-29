#![allow(dead_code)]

use futures_util::StreamExt;
use swallowtail_adapter_ollama::{
    OllamaInferenceAttemptInput, OllamaInventoryProfileInput, OllamaInventorySnapshot,
    OllamaPreparationInput, OllamaPreparationProbe, OllamaPreparedInferenceAttempt,
    OllamaPreparedIntegration, OllamaPreparedInventory, OllamaPreparedSession,
    OllamaSessionProfileInput, prepare_ollama_attached,
};
use swallowtail_runtime::{
    CleanupOutcome, HostServices, InteractiveSessionHandle, PreparationFailure, RuntimeFailure,
    TerminalOutcome,
};

async fn prepare_runtime(
    input: OllamaPreparationInput,
    probe: OllamaPreparationProbe,
    services: HostServices,
) -> Result<OllamaPreparedIntegration, PreparationFailure> {
    prepare_ollama_attached(input, probe, services).await
}

fn prepare_inventory(
    runtime: &OllamaPreparedIntegration,
    input: OllamaInventoryProfileInput,
) -> Result<OllamaPreparedInventory, PreparationFailure> {
    runtime.prepare_inventory(input)
}

async fn observe_inventory(
    inventory: &OllamaPreparedInventory,
    services: HostServices,
) -> Result<OllamaInventorySnapshot, RuntimeFailure> {
    inventory.observe_inventory(services).await
}

fn prepare_attempt(
    runtime: &OllamaPreparedIntegration,
    input: OllamaInferenceAttemptInput,
) -> Result<OllamaPreparedInferenceAttempt, PreparationFailure> {
    runtime.prepare_inference_attempt(input)
}

fn prepare_session(
    runtime: &OllamaPreparedIntegration,
    input: OllamaSessionProfileInput,
) -> Result<OllamaPreparedSession, PreparationFailure> {
    runtime.prepare_session(input)
}

async fn open_session(
    prepared: &OllamaPreparedSession,
    services: HostServices,
) -> Result<Box<dyn InteractiveSessionHandle>, RuntimeFailure> {
    prepared.open_session(services).await
}

async fn execute_one_attempt(
    attempt: &OllamaPreparedInferenceAttempt,
    services: HostServices,
) -> Result<(TerminalOutcome, CleanupOutcome), RuntimeFailure> {
    let mut run = attempt.start_run(services).await?;
    let mut events = run
        .take_events()
        .expect("Ollama runs expose one event stream");
    let terminal = run
        .take_terminal_outcome()
        .expect("Ollama runs expose one terminal outcome");
    while let Some(event) = events.next().await {
        let _ = event?;
    }
    let outcome = terminal.await;
    Ok((outcome, run.close().await))
}

fn main() {}

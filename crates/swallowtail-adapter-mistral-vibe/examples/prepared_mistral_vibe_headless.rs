#![allow(dead_code)]

use swallowtail_adapter_mistral_vibe::{
    MistralVibeHeadlessPreparationInput, MistralVibeHeadlessPreparationProbe,
    MistralVibeHeadlessPreparedIntegration, MistralVibeHeadlessPreparedRun,
    MistralVibeHeadlessRunProfileInput, MistralVibeMaxTurns, prepare_mistral_vibe_headless,
};
use swallowtail_runtime::{
    CleanupOutcome, HostServices, PreparationFailure, RuntimeFailure, TerminalOutcome,
};

async fn prepare_installation(
    input: MistralVibeHeadlessPreparationInput,
    probe: MistralVibeHeadlessPreparationProbe,
    services: HostServices,
) -> Result<MistralVibeHeadlessPreparedIntegration, PreparationFailure> {
    prepare_mistral_vibe_headless(input, probe, services).await
}

fn prepare_run(
    integration: &MistralVibeHeadlessPreparedIntegration,
    input: MistralVibeHeadlessRunProfileInput,
) -> Result<MistralVibeHeadlessPreparedRun, PreparationFailure> {
    integration.prepare_run(input)
}

fn prepare_run_with_max_turns(
    integration: &MistralVibeHeadlessPreparedIntegration,
    input: MistralVibeHeadlessRunProfileInput,
    turns: MistralVibeMaxTurns,
) -> Result<MistralVibeHeadlessPreparedRun, PreparationFailure> {
    integration.prepare_run(input.with_max_turns(turns))
}

async fn execute_run(
    prepared: &MistralVibeHeadlessPreparedRun,
    services: HostServices,
) -> Result<(TerminalOutcome, CleanupOutcome), RuntimeFailure> {
    let mut run = prepared.start_run(services).await?;
    let outcome = run
        .take_terminal_outcome()
        .expect("Mistral Vibe headless runs expose one terminal outcome")
        .await;
    Ok((outcome, run.close().await))
}

fn main() {}

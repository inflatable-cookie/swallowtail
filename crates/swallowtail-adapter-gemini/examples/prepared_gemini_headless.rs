#![allow(dead_code)]

use swallowtail_adapter_gemini::{
    GeminiCliPreparationInput, GeminiCliPreparationProbe, GeminiCliPreparedIntegration,
    GeminiHeadlessPreparedIntegration, GeminiHeadlessPreparedRun, GeminiHeadlessRunProfileInput,
    prepare_gemini_cli,
};
use swallowtail_runtime::{
    CleanupOutcome, HostServices, PreparationFailure, RuntimeFailure, TerminalOutcome,
};

async fn prepare_headless(
    input: GeminiCliPreparationInput,
    probe: GeminiCliPreparationProbe,
    services: HostServices,
) -> Result<GeminiHeadlessPreparedIntegration, PreparationFailure> {
    match prepare_gemini_cli(input, probe, services).await? {
        GeminiCliPreparedIntegration::Headless(prepared) => Ok(prepared),
        GeminiCliPreparedIntegration::Acp(_) => {
            unreachable!("the caller explicitly selected the headless route")
        }
    }
}

fn prepare_run(
    integration: &GeminiHeadlessPreparedIntegration,
    input: GeminiHeadlessRunProfileInput,
) -> Result<GeminiHeadlessPreparedRun, PreparationFailure> {
    integration.prepare_run(input)
}

async fn execute(
    prepared: &GeminiHeadlessPreparedRun,
    services: HostServices,
) -> Result<(TerminalOutcome, CleanupOutcome), RuntimeFailure> {
    let mut run = prepared.start_run(services).await?;
    let terminal = run
        .take_terminal_outcome()
        .expect("Gemini headless runs expose one terminal outcome")
        .await;
    Ok((terminal, run.close().await))
}

fn main() {}

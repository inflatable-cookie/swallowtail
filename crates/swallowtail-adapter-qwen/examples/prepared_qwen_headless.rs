#![allow(dead_code)]

use swallowtail_adapter_qwen::{
    QwenPreparationInput, QwenPreparationProbe, QwenPreparedIntegration, QwenPreparedRun,
    QwenRunProfileInput, prepare_qwen_headless,
};
use swallowtail_runtime::{
    CleanupOutcome, HostServices, PreparationFailure, RuntimeFailure, TerminalOutcome,
};

async fn prepare_installation(
    input: QwenPreparationInput,
    probe: QwenPreparationProbe,
    services: HostServices,
) -> Result<QwenPreparedIntegration, PreparationFailure> {
    prepare_qwen_headless(input, probe, services).await
}

fn prepare_run(
    integration: &QwenPreparedIntegration,
    input: QwenRunProfileInput,
) -> Result<QwenPreparedRun, PreparationFailure> {
    integration.prepare_run(input)
}

async fn execute(
    prepared: &QwenPreparedRun,
    services: HostServices,
) -> Result<(TerminalOutcome, CleanupOutcome), RuntimeFailure> {
    let mut run = prepared.start_run(services).await?;
    let outcome = run
        .take_terminal_outcome()
        .expect("Qwen runs expose one terminal outcome")
        .await;
    Ok((outcome, run.close().await))
}

fn main() {}

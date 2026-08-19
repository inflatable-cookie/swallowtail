#![allow(dead_code)]

use swallowtail_adapter_qoder::{
    QoderHeadlessPreparationInput, QoderHeadlessPreparationProbe, QoderHeadlessPreparedIntegration,
    QoderHeadlessPreparedRun, QoderHeadlessRunProfileInput, prepare_qoder_headless,
};
use swallowtail_runtime::{
    CleanupOutcome, HostServices, PreparationFailure, RuntimeFailure, TerminalOutcome,
};

async fn prepare_installation(
    input: QoderHeadlessPreparationInput,
    probe: QoderHeadlessPreparationProbe,
    services: HostServices,
) -> Result<QoderHeadlessPreparedIntegration, PreparationFailure> {
    prepare_qoder_headless(input, probe, services).await
}

fn prepare_run(
    integration: &QoderHeadlessPreparedIntegration,
    input: QoderHeadlessRunProfileInput,
) -> Result<QoderHeadlessPreparedRun, PreparationFailure> {
    integration.prepare_run(input)
}

async fn execute_run(
    prepared: &QoderHeadlessPreparedRun,
    services: HostServices,
) -> Result<(TerminalOutcome, CleanupOutcome), RuntimeFailure> {
    let mut run = prepared.start_run(services).await?;
    let outcome = run
        .take_terminal_outcome()
        .expect("Qoder headless runs expose one terminal outcome")
        .await;
    Ok((outcome, run.close().await))
}

fn main() {}

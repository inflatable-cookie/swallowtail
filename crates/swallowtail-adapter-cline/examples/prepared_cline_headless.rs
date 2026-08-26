#![allow(dead_code)]

use swallowtail_adapter_cline::{
    ClineHeadlessPreparationInput, ClineHeadlessPreparationProbe, ClineHeadlessPreparedIntegration,
    ClineHeadlessPreparedRun, ClineHeadlessRunProfileInput, prepare_cline_headless,
};
use swallowtail_core::HarnessMode;
use swallowtail_runtime::{
    CleanupOutcome, HostServices, PreparationFailure, RuntimeFailure, TerminalOutcome,
};

async fn prepare_installation(
    input: ClineHeadlessPreparationInput,
    probe: ClineHeadlessPreparationProbe,
    services: HostServices,
) -> Result<ClineHeadlessPreparedIntegration, PreparationFailure> {
    prepare_cline_headless(input, probe, services).await
}

fn prepare_run(
    integration: &ClineHeadlessPreparedIntegration,
    input: ClineHeadlessRunProfileInput,
) -> Result<ClineHeadlessPreparedRun, PreparationFailure> {
    integration.prepare_run(input)
}

fn prepare_run_with_plan(
    integration: &ClineHeadlessPreparedIntegration,
    input: ClineHeadlessRunProfileInput,
) -> Result<ClineHeadlessPreparedRun, PreparationFailure> {
    integration.prepare_run(input.with_harness_mode(HarnessMode::Plan))
}

async fn execute_run(
    prepared: &ClineHeadlessPreparedRun,
    services: HostServices,
) -> Result<(TerminalOutcome, CleanupOutcome), RuntimeFailure> {
    let mut run = prepared.start_run(services).await?;
    let outcome = run
        .take_terminal_outcome()
        .expect("Cline headless runs expose one terminal outcome")
        .await;
    Ok((outcome, run.close().await))
}

fn main() {}

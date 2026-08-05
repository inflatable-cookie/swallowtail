#![allow(dead_code)]

use swallowtail_adapter_cursor::{
    CursorHeadlessRunProfileInput, CursorPreparationInput, CursorPreparationProbe,
    CursorPreparedHeadlessIntegration, CursorPreparedHeadlessRun, CursorPreparedIntegration,
    prepare_cursor,
};
use swallowtail_runtime::{
    CleanupOutcome, HostServices, PreparationFailure, RuntimeFailure, TerminalOutcome,
};

async fn prepare_installation(
    input: CursorPreparationInput,
    probe: CursorPreparationProbe,
    services: HostServices,
) -> Result<CursorPreparedIntegration, PreparationFailure> {
    prepare_cursor(input, probe, services).await
}

fn prepare_run(
    integration: &CursorPreparedHeadlessIntegration,
    input: CursorHeadlessRunProfileInput,
) -> Result<CursorPreparedHeadlessRun, PreparationFailure> {
    integration.prepare_run(input)
}

async fn execute(
    prepared: &CursorPreparedHeadlessRun,
    services: HostServices,
) -> Result<(TerminalOutcome, CleanupOutcome), RuntimeFailure> {
    let mut run = prepared.start_run(services).await?;
    let outcome = run
        .take_terminal_outcome()
        .expect("Cursor headless runs expose one terminal outcome")
        .await;
    Ok((outcome, run.close().await))
}

fn main() {}

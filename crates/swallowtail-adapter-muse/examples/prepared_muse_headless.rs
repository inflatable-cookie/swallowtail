#![allow(dead_code)]

use swallowtail_adapter_muse::{
    MusePreparationInput, MusePreparationProbe, MusePreparedIntegration, MusePreparedRun,
    MuseRunProfileInput, prepare_muse_headless,
};
use swallowtail_runtime::{
    CleanupOutcome, HostServices, PreparationFailure, RuntimeFailure, TerminalOutcome,
};

async fn prepare_installation(
    input: MusePreparationInput,
    probe: MusePreparationProbe,
    services: HostServices,
) -> Result<MusePreparedIntegration, PreparationFailure> {
    prepare_muse_headless(input, probe, services).await
}

fn prepare_run(
    integration: &MusePreparedIntegration,
    input: MuseRunProfileInput,
) -> Result<MusePreparedRun, PreparationFailure> {
    integration.prepare_run(input)
}

async fn execute_run(
    prepared: &MusePreparedRun,
    services: HostServices,
) -> Result<(TerminalOutcome, CleanupOutcome), RuntimeFailure> {
    let mut run = prepared.start_run(services).await?;
    let outcome = run
        .take_terminal_outcome()
        .expect("Muse Code runs expose one terminal outcome")
        .await;
    Ok((outcome, run.close().await))
}

fn main() {}

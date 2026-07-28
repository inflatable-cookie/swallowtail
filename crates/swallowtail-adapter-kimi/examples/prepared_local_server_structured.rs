#![allow(dead_code)]

use swallowtail_adapter_kimi::{
    KimiLocalServerPreparedIntegration, KimiLocalServerPreparedRun, KimiLocalServerRunInput,
};
use swallowtail_runtime::{
    CleanupOutcome, HostServices, PreparationFailure, RuntimeFailure, TerminalOutcome,
};

fn prepare_run(
    integration: &KimiLocalServerPreparedIntegration,
    input: KimiLocalServerRunInput,
) -> Result<KimiLocalServerPreparedRun, PreparationFailure> {
    integration.prepare_run(input)
}

async fn execute(
    prepared: &KimiLocalServerPreparedRun,
    services: HostServices,
) -> Result<(TerminalOutcome, CleanupOutcome), RuntimeFailure> {
    let mut run = prepared.start_run(services).await?;
    let terminal = run
        .take_terminal_outcome()
        .expect("Kimi local-server runs expose one terminal outcome")
        .await;
    Ok((terminal, run.close().await))
}

fn main() {}

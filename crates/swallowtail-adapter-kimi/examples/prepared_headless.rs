#![allow(dead_code)]

use swallowtail_adapter_kimi::{
    KimiCodePreparationInput, KimiCodePreparationProbe, KimiCodePreparedIntegration,
    KimiHeadlessPreparedIntegration, KimiHeadlessPreparedRun, KimiHeadlessRunInput,
    prepare_kimi_code,
};
use swallowtail_runtime::{
    CleanupOutcome, HostServices, PreparationFailure, RuntimeFailure, TerminalOutcome,
};

async fn prepare_headless(
    input: KimiCodePreparationInput,
    probe: KimiCodePreparationProbe,
    services: HostServices,
) -> Result<KimiHeadlessPreparedIntegration, PreparationFailure> {
    match prepare_kimi_code(input, probe, services).await? {
        KimiCodePreparedIntegration::Headless(prepared) => Ok(prepared),
        KimiCodePreparedIntegration::Acp(_) => {
            unreachable!("the caller explicitly selected the headless route")
        }
    }
}

fn prepare_run(
    integration: &KimiHeadlessPreparedIntegration,
    input: KimiHeadlessRunInput,
) -> Result<KimiHeadlessPreparedRun, PreparationFailure> {
    integration.prepare_run(input)
}

async fn execute(
    prepared: &KimiHeadlessPreparedRun,
    services: HostServices,
) -> Result<(TerminalOutcome, CleanupOutcome), RuntimeFailure> {
    let mut run = prepared.start_run(services).await?;
    let terminal = run
        .take_terminal_outcome()
        .expect("Kimi headless runs expose one terminal outcome")
        .await;
    Ok((terminal, run.close().await))
}

fn main() {}

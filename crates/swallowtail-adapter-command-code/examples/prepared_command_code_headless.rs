#![allow(dead_code)]

use swallowtail_adapter_command_code::{
    CommandCodePreparationInput, CommandCodePreparationProbe, CommandCodePreparedIntegration,
    CommandCodePreparedRun, CommandCodePreparedSession, CommandCodeRunProfileInput,
    CommandCodeSessionProfileInput, prepare_command_code_headless,
};
use swallowtail_runtime::{
    CleanupOutcome, HostServices, InteractiveSessionHandle, PreparationFailure, RuntimeFailure,
    TerminalOutcome,
};

async fn prepare_installation(
    input: CommandCodePreparationInput,
    probe: CommandCodePreparationProbe,
    services: HostServices,
) -> Result<CommandCodePreparedIntegration, PreparationFailure> {
    prepare_command_code_headless(input, probe, services).await
}

fn prepare_run(
    integration: &CommandCodePreparedIntegration,
    input: CommandCodeRunProfileInput,
) -> Result<CommandCodePreparedRun, PreparationFailure> {
    integration.prepare_run(input)
}

async fn execute_run(
    prepared: &CommandCodePreparedRun,
    services: HostServices,
) -> Result<(TerminalOutcome, CleanupOutcome), RuntimeFailure> {
    let mut run = prepared.start_run(services).await?;
    let outcome = run
        .take_terminal_outcome()
        .expect("Command Code runs expose one terminal outcome")
        .await;
    Ok((outcome, run.close().await))
}

fn prepare_session(
    integration: &CommandCodePreparedIntegration,
    input: CommandCodeSessionProfileInput,
) -> Result<CommandCodePreparedSession, PreparationFailure> {
    integration.prepare_session(input)
}

async fn open_session(
    prepared: &CommandCodePreparedSession,
    services: HostServices,
) -> Result<Box<dyn InteractiveSessionHandle>, RuntimeFailure> {
    prepared.open_session(services).await
}

fn main() {}

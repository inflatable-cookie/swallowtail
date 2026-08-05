#![allow(dead_code)]

use swallowtail_adapter_antigravity::{
    AntigravityContinuationProfileInput, AntigravityHeadlessRunProfileInput,
    AntigravityPreparationInput, AntigravityPreparationProbe, AntigravityPreparedContinuation,
    AntigravityPreparedContinuationIntegration, AntigravityPreparedHeadlessIntegration,
    AntigravityPreparedHeadlessRun, AntigravityPreparedIntegration, prepare_antigravity,
};
use swallowtail_runtime::{
    CleanupOutcome, HostServices, InteractiveSessionHandle, PreparationFailure,
    PreparedWorkingStateRestoration, RuntimeFailure, RuntimeTurnId, TerminalOutcome,
};

async fn prepare_installation(
    input: AntigravityPreparationInput,
    probe: AntigravityPreparationProbe,
    services: HostServices,
) -> Result<AntigravityPreparedIntegration, PreparationFailure> {
    prepare_antigravity(input, probe, services).await
}

fn prepare_run(
    integration: &AntigravityPreparedHeadlessIntegration,
    input: AntigravityHeadlessRunProfileInput,
) -> Result<AntigravityPreparedHeadlessRun, PreparationFailure> {
    integration.prepare_run(input)
}

async fn execute_run(
    prepared: &AntigravityPreparedHeadlessRun,
    services: HostServices,
) -> Result<(TerminalOutcome, CleanupOutcome), RuntimeFailure> {
    let mut run = prepared.start_run(services).await?;
    let outcome = run
        .take_terminal_outcome()
        .expect("Antigravity runs expose one terminal outcome")
        .await;
    Ok((outcome, run.close().await))
}

fn prepare_continuation(
    integration: &AntigravityPreparedContinuationIntegration,
    input: AntigravityContinuationProfileInput,
) -> Result<AntigravityPreparedContinuation, PreparationFailure> {
    integration.prepare_session(input)
}

async fn open_continuation(
    prepared: &AntigravityPreparedContinuation,
    services: HostServices,
) -> Result<Box<dyn InteractiveSessionHandle>, RuntimeFailure> {
    prepared.open_session(services).await
}

fn prepare_fresh_replacement(
    prepared: &AntigravityPreparedContinuation,
    interrupted_turn_id: RuntimeTurnId,
) -> PreparedWorkingStateRestoration {
    prepared.prepare_working_state_restoration(interrupted_turn_id)
}

fn main() {}

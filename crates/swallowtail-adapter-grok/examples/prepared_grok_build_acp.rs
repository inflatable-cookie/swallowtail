#![allow(dead_code)]

use swallowtail_adapter_grok::{
    GrokPreparationInput, GrokPreparationProbe, GrokPreparedIntegration, GrokPreparedRun,
    GrokPreparedSession, GrokRunProfileInput, GrokSessionProfileInput, prepare_grok_build,
};
use swallowtail_runtime::{
    CleanupOutcome, HostServices, InteractiveSessionHandle, PreparationFailure,
    PreparedWorkingStateRestoration, RequestId, RuntimeFailure, RuntimeTurnId,
    SessionResumeBinding, TerminalOutcome,
};

async fn prepare_installation(
    input: GrokPreparationInput,
    probe: GrokPreparationProbe,
    services: HostServices,
) -> Result<GrokPreparedIntegration, PreparationFailure> {
    prepare_grok_build(input, probe, services).await
}

fn prepare_run(
    integration: &GrokPreparedIntegration,
    input: GrokRunProfileInput,
) -> Result<GrokPreparedRun, PreparationFailure> {
    integration.prepare_run(input)
}

async fn execute_run(
    prepared: &GrokPreparedRun,
    services: HostServices,
) -> Result<(TerminalOutcome, CleanupOutcome), RuntimeFailure> {
    let mut run = prepared.start_run(services).await?;
    let outcome = run
        .take_terminal_outcome()
        .expect("Grok runs expose one terminal outcome")
        .await;
    Ok((outcome, run.close().await))
}

fn prepare_session(
    integration: &GrokPreparedIntegration,
    input: GrokSessionProfileInput,
) -> Result<GrokPreparedSession, PreparationFailure> {
    integration.prepare_session(input)
}

async fn open_session(
    prepared: &GrokPreparedSession,
    services: HostServices,
) -> Result<Box<dyn InteractiveSessionHandle>, RuntimeFailure> {
    prepared.open_session(services).await
}

fn prepare_attachment_recovery(
    prepared: &GrokPreparedSession,
    request_id: RequestId,
    binding: SessionResumeBinding,
    interrupted_turn_id: RuntimeTurnId,
) -> Result<PreparedWorkingStateRestoration, PreparationFailure> {
    prepared.prepare_working_state_restoration(request_id, binding, interrupted_turn_id)
}

fn main() {}

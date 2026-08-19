#![allow(dead_code)]

use swallowtail_adapter_kiro::{
    KiroPreparationInput, KiroPreparationProbe, KiroPreparedIntegration, KiroPreparedSession,
    KiroSessionProfileInput, prepare_kiro_acp,
};
use swallowtail_runtime::{
    CleanupOutcome, HostServices, InteractiveSessionHandle, OperationContent, PreparationFailure,
    PreparedWorkingStateRestoration, RuntimeFailure, RuntimeTurnId, TerminalOutcome, TurnRequest,
};

async fn prepare_installation(
    input: KiroPreparationInput,
    probe: KiroPreparationProbe,
    services: HostServices,
) -> Result<KiroPreparedIntegration, PreparationFailure> {
    prepare_kiro_acp(input, probe, services).await
}

fn prepare_session(
    integration: &KiroPreparedIntegration,
    input: KiroSessionProfileInput,
) -> Result<KiroPreparedSession, PreparationFailure> {
    integration.prepare_session(input)
}

async fn open_and_prompt(
    prepared: &KiroPreparedSession,
    services: HostServices,
    turn_id: RuntimeTurnId,
    content: OperationContent,
) -> Result<(TerminalOutcome, CleanupOutcome), RuntimeFailure> {
    let mut session = prepared.open_session(services.clone()).await?;
    let mut turn = session
        .start_turn(TurnRequest::new(turn_id, content), services)
        .await?;
    let outcome = turn
        .take_terminal_outcome()
        .expect("Kiro ACP turns expose one terminal outcome")
        .await;
    let _ = turn.close().await;
    Ok((outcome, session.close().await))
}

fn prepare_attachment_recovery(
    prepared: &KiroPreparedSession,
    interrupted_turn_id: RuntimeTurnId,
) -> PreparedWorkingStateRestoration {
    prepared.prepare_working_state_restoration(interrupted_turn_id)
}

fn _session_handle(_: Box<dyn InteractiveSessionHandle>) {}

fn main() {}

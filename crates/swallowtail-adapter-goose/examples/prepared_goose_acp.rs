#![allow(dead_code)]

use swallowtail_adapter_goose::{
    GoosePreparationInput, GoosePreparationProbe, GoosePreparedIntegration, GoosePreparedSession,
    GooseSessionProfileInput, prepare_goose_acp,
};
use swallowtail_runtime::{
    CleanupOutcome, HostServices, InteractiveSessionHandle, OperationContent, PreparationFailure,
    PreparedWorkingStateRestoration, RuntimeFailure, RuntimeTurnId, TerminalOutcome, TurnRequest,
};

async fn prepare_installation(
    input: GoosePreparationInput,
    probe: GoosePreparationProbe,
    services: HostServices,
) -> Result<GoosePreparedIntegration, PreparationFailure> {
    prepare_goose_acp(input, probe, services).await
}

fn prepare_session(
    integration: &GoosePreparedIntegration,
    input: GooseSessionProfileInput,
) -> Result<GoosePreparedSession, PreparationFailure> {
    integration.prepare_session(input)
}

async fn open_and_prompt(
    prepared: &GoosePreparedSession,
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
        .expect("Goose ACP turns expose one terminal outcome")
        .await;
    let _ = turn.close().await;
    Ok((outcome, session.close().await))
}

fn prepare_attachment_recovery(
    prepared: &GoosePreparedSession,
    interrupted_turn_id: RuntimeTurnId,
) -> PreparedWorkingStateRestoration {
    prepared.prepare_working_state_restoration(interrupted_turn_id)
}

fn _session_handle(_: Box<dyn InteractiveSessionHandle>) {}

fn main() {}

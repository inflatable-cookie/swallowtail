#![allow(dead_code)]

use swallowtail_adapter_oh_my_pi::{
    OhMyPiPreparationInput, OhMyPiPreparationProbe, OhMyPiPreparedIntegration,
    OhMyPiPreparedSession, OhMyPiSessionProfileInput, prepare_oh_my_pi_rpc,
};
use swallowtail_runtime::{
    CleanupOutcome, HostServices, OperationContent, PreparationFailure, RuntimeFailure,
    RuntimeTurnId, TerminalOutcome, TurnRequest,
};

async fn prepare_installation(
    input: OhMyPiPreparationInput,
    probe: OhMyPiPreparationProbe,
    services: HostServices,
) -> Result<OhMyPiPreparedIntegration, PreparationFailure> {
    prepare_oh_my_pi_rpc(input, probe, services).await
}

fn prepare_session(
    integration: &OhMyPiPreparedIntegration,
    input: OhMyPiSessionProfileInput,
) -> Result<OhMyPiPreparedSession, PreparationFailure> {
    integration.prepare_session(input)
}

async fn open_and_prompt(
    prepared: &OhMyPiPreparedSession,
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
        .expect("OhMyPi turns expose one terminal outcome")
        .await;
    let _ = turn.close().await;
    Ok((outcome, session.close().await))
}

fn main() {}

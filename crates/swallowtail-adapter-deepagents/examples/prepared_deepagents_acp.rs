#![allow(dead_code)]

use swallowtail_adapter_deepagents::{
    DeepAgentsPreparationInput, DeepAgentsPreparationProbe, DeepAgentsPreparedIntegration,
    DeepAgentsPreparedSession, DeepAgentsSessionProfileInput, prepare_deepagents_acp,
};
use swallowtail_runtime::{
    CleanupOutcome, HostServices, InteractiveSessionHandle, OperationContent, PreparationFailure,
    PreparedWorkingStateRestoration, RuntimeFailure, RuntimeTurnId, SessionCleanupRequest,
    TerminalOutcome, TurnRequest,
};

async fn prepare_installation(
    input: DeepAgentsPreparationInput,
    probe: DeepAgentsPreparationProbe,
    services: HostServices,
) -> Result<DeepAgentsPreparedIntegration, PreparationFailure> {
    prepare_deepagents_acp(input, probe, services).await
}

fn prepare_session(
    integration: &DeepAgentsPreparedIntegration,
    input: DeepAgentsSessionProfileInput,
) -> Result<DeepAgentsPreparedSession, PreparationFailure> {
    integration.prepare_session(input)
}

async fn open_and_prompt(
    prepared: &DeepAgentsPreparedSession,
    services: HostServices,
    cleanup: SessionCleanupRequest,
    turn_id: RuntimeTurnId,
    content: OperationContent,
) -> Result<(TerminalOutcome, CleanupOutcome), RuntimeFailure> {
    let mut session = prepared.open_session(services.clone()).await?;
    let mut turn = session
        .start_turn(TurnRequest::new(turn_id, content), services.clone())
        .await?;
    let outcome = turn
        .take_terminal_outcome()
        .expect("Deep Agents ACP turns expose one terminal outcome")
        .await;
    let _ = turn.close().await;
    Ok((outcome, session.close(cleanup, services).await))
}

fn prepare_attachment_recovery(
    prepared: &DeepAgentsPreparedSession,
    interrupted_turn_id: RuntimeTurnId,
) -> PreparedWorkingStateRestoration {
    prepared.prepare_working_state_restoration(interrupted_turn_id)
}

fn _session_handle(_: Box<dyn InteractiveSessionHandle>) {}

fn main() {}

#![allow(dead_code)]

use swallowtail_adapter_gemini::{
    GeminiPreparationInput, GeminiPreparationProbe, GeminiPreparedIntegration,
    GeminiPreparedSession, GeminiSessionProfileInput, prepare_gemini_acp,
};
use swallowtail_runtime::{
    CleanupOutcome, HostServices, OperationContent, PreparationFailure, RuntimeFailure,
    RuntimeTurnId, SessionCleanupRequest, TerminalOutcome, TurnRequest,
};

async fn prepare_installation(
    input: GeminiPreparationInput,
    probe: GeminiPreparationProbe,
    services: HostServices,
) -> Result<GeminiPreparedIntegration, PreparationFailure> {
    prepare_gemini_acp(input, probe, services).await
}

fn prepare_session(
    integration: &GeminiPreparedIntegration,
    input: GeminiSessionProfileInput,
) -> Result<GeminiPreparedSession, PreparationFailure> {
    integration.prepare_session(input)
}

async fn open_and_prompt(
    prepared: &GeminiPreparedSession,
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
        .expect("Gemini ACP turns expose one terminal outcome")
        .await;
    let _ = turn.close().await;
    Ok((outcome, session.close(cleanup, services).await))
}

fn main() {}

#![allow(dead_code)]

use swallowtail_adapter_gemini::{
    GeminiPreparationInput, GeminiPreparationProbe, GeminiPreparedIntegration,
    GeminiPreparedSession, GeminiSessionProfileInput, prepare_gemini_acp,
};
use swallowtail_runtime::{
    CleanupOutcome, HostServices, OperationContent, PreparationFailure, RuntimeFailure,
    RuntimeTurnId, TerminalOutcome, TurnRequest,
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
    turn_id: RuntimeTurnId,
    content: OperationContent,
) -> Result<(TerminalOutcome, CleanupOutcome), RuntimeFailure> {
    let mut session = prepared.open_session(services.clone()).await?;
    let mut turn = session
        .start_turn(TurnRequest::new(turn_id, content), services)
        .await?;
    let outcome = turn
        .take_terminal_outcome()
        .expect("Gemini ACP turns expose one terminal outcome")
        .await;
    let _ = turn.close().await;
    Ok((outcome, session.close().await))
}

fn main() {}

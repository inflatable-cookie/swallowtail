#![allow(dead_code)]

use swallowtail_adapter_opencode::{
    OpenCodeAcpPreparationInput, OpenCodeAcpPreparationProbe, OpenCodeAcpPreparedIntegration,
    OpenCodeAcpPreparedSession, OpenCodeAcpSessionProfileInput, prepare_opencode_acp,
};
use swallowtail_runtime::{
    CleanupOutcome, HostServices, InteractiveSessionHandle, OperationContent, PreparationFailure,
    PreparedWorkingStateRestoration, RuntimeFailure, RuntimeTurnId, SessionCleanupRequest,
    TerminalOutcome, TurnRequest,
};

async fn prepare_installation(
    input: OpenCodeAcpPreparationInput,
    probe: OpenCodeAcpPreparationProbe,
    services: HostServices,
) -> Result<OpenCodeAcpPreparedIntegration, PreparationFailure> {
    prepare_opencode_acp(input, probe, services).await
}

fn prepare_session(
    integration: &OpenCodeAcpPreparedIntegration,
    input: OpenCodeAcpSessionProfileInput,
) -> Result<OpenCodeAcpPreparedSession, PreparationFailure> {
    integration.prepare_session(input)
}

async fn open_and_prompt(
    prepared: &OpenCodeAcpPreparedSession,
    cleanup: SessionCleanupRequest,
    services: HostServices,
    turn_id: RuntimeTurnId,
    content: OperationContent,
) -> Result<(TerminalOutcome, CleanupOutcome), RuntimeFailure> {
    let mut session = prepared.open_session(services.clone()).await?;
    let mut turn = session
        .start_turn(TurnRequest::new(turn_id, content), services.clone())
        .await?;
    let outcome = turn
        .take_terminal_outcome()
        .expect("OpenCode ACP turns expose one terminal outcome")
        .await;
    let _ = turn.close().await;
    Ok((outcome, session.close(cleanup, services).await))
}

fn prepare_attachment_recovery(
    prepared: &OpenCodeAcpPreparedSession,
    interrupted_turn_id: RuntimeTurnId,
) -> PreparedWorkingStateRestoration {
    prepared.prepare_working_state_restoration(interrupted_turn_id)
}

fn bind_http_mcp(
    input: OpenCodeAcpSessionProfileInput,
    server: swallowtail_adapter_opencode::OpenCodeAcpRemoteMcpPlacement,
) -> OpenCodeAcpSessionProfileInput {
    input.with_http_mcp_placement(server)
}

fn _session_handle(_: Box<dyn InteractiveSessionHandle>) {}

fn main() {}

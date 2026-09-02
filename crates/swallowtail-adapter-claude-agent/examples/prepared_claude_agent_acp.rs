#![allow(dead_code)]

use swallowtail_adapter_claude_agent::{
    ClaudeAgentPreparationInput, ClaudeAgentPreparationProbe, ClaudeAgentPreparedDelete,
    ClaudeAgentPreparedIntegration, ClaudeAgentPreparedSession, ClaudeAgentSessionManagementInput,
    ClaudeAgentSessionProfileInput, prepare_claude_agent,
};
use swallowtail_runtime::{
    CleanupOutcome, HostServices, OperationContent, PreparationFailure,
    PreparedWorkingStateRestoration, ProviderSessionManagementBinding,
    ProviderSessionManagementOutcome, RequestId, RuntimeFailure, RuntimeTurnId,
    SessionCleanupRequest, SessionResumeBinding, TerminalOutcome, TurnRequest,
    WorkingStateRestorationOutcome,
};

async fn prepare_installation(
    input: ClaudeAgentPreparationInput,
    probe: ClaudeAgentPreparationProbe,
    services: HostServices,
) -> Result<ClaudeAgentPreparedIntegration, PreparationFailure> {
    prepare_claude_agent(input, probe, services).await
}

fn prepare_session(
    integration: &ClaudeAgentPreparedIntegration,
    input: ClaudeAgentSessionProfileInput,
) -> Result<ClaudeAgentPreparedSession, PreparationFailure> {
    integration.prepare_session(input)
}

fn prepare_working_state_restoration(
    prepared: &ClaudeAgentPreparedSession,
    request_id: RequestId,
    binding: SessionResumeBinding,
    interrupted_turn_id: RuntimeTurnId,
) -> Result<PreparedWorkingStateRestoration, PreparationFailure> {
    prepared.prepare_working_state_restoration(request_id, binding, interrupted_turn_id)
}

async fn restore_working_state(
    prepared: PreparedWorkingStateRestoration,
    services: HostServices,
) -> Result<WorkingStateRestorationOutcome, RuntimeFailure> {
    prepared.restore(services).await
}

async fn open_and_prompt(
    prepared: &ClaudeAgentPreparedSession,
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
        .expect("Claude Agent turns expose one terminal outcome")
        .await;
    let _ = turn.close().await;
    Ok((outcome, session.close(cleanup, services).await))
}

fn prepare_delete(
    prepared: &ClaudeAgentPreparedIntegration,
    request_id: RequestId,
    binding: ProviderSessionManagementBinding,
) -> Result<ClaudeAgentPreparedDelete, PreparationFailure> {
    prepared.prepare_delete_session(ClaudeAgentSessionManagementInput::new(request_id, binding))
}

async fn execute_delete(
    prepared: &ClaudeAgentPreparedDelete,
    services: HostServices,
) -> Result<ProviderSessionManagementOutcome, RuntimeFailure> {
    prepared.execute(services).await
}

fn main() {}

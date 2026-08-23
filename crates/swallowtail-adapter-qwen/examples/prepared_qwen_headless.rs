#![allow(dead_code)]

use swallowtail_adapter_qwen::{
    QwenPreparationInput, QwenPreparationProbe, QwenPreparedIntegration, QwenPreparedRun,
    QwenPreparedSession, QwenRunProfileInput, QwenSessionProfileInput, QwenSessionTurnBudget,
    QwenToolCallBudget, prepare_qwen_headless,
};
use swallowtail_runtime::{
    CleanupOutcome, HostServices, InteractiveSessionHandle, PreparationFailure, RuntimeFailure,
    TerminalOutcome,
};

async fn prepare_installation(
    input: QwenPreparationInput,
    probe: QwenPreparationProbe,
    services: HostServices,
) -> Result<QwenPreparedIntegration, PreparationFailure> {
    prepare_qwen_headless(input, probe, services).await
}

fn prepare_run(
    integration: &QwenPreparedIntegration,
    input: QwenRunProfileInput,
) -> Result<QwenPreparedRun, PreparationFailure> {
    integration.prepare_run(input)
}

fn prepare_session(
    integration: &QwenPreparedIntegration,
    input: QwenSessionProfileInput,
) -> Result<QwenPreparedSession, PreparationFailure> {
    integration.prepare_session(input)
}

fn prepare_run_with_budgets(
    integration: &QwenPreparedIntegration,
    input: QwenRunProfileInput,
    turns: QwenSessionTurnBudget,
    tools: QwenToolCallBudget,
) -> Result<QwenPreparedRun, PreparationFailure> {
    integration.prepare_run(
        input
            .with_session_turn_budget(turns)
            .with_tool_call_budget(tools),
    )
}

async fn open_session(
    prepared: &QwenPreparedSession,
    services: HostServices,
) -> Result<Box<dyn InteractiveSessionHandle>, RuntimeFailure> {
    prepared.open_session(services).await
}

async fn execute(
    prepared: &QwenPreparedRun,
    services: HostServices,
) -> Result<(TerminalOutcome, CleanupOutcome), RuntimeFailure> {
    let mut run = prepared.start_run(services).await?;
    let outcome = run
        .take_terminal_outcome()
        .expect("Qwen runs expose one terminal outcome")
        .await;
    Ok((outcome, run.close().await))
}

fn main() {}

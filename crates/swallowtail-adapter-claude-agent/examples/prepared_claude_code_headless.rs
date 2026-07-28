#![allow(dead_code)]

use swallowtail_adapter_claude_agent::{
    ClaudeCodePreparationInput, ClaudeCodePreparationProbe, ClaudeCodePreparedIntegration,
    ClaudeCodePreparedRun, ClaudeCodeRunProfileInput, prepare_claude_code_headless,
};
use swallowtail_runtime::{
    CleanupOutcome, HostServices, PreparationFailure, RuntimeFailure, TerminalOutcome,
};

async fn prepare_installation(
    input: ClaudeCodePreparationInput,
    probe: ClaudeCodePreparationProbe,
    services: HostServices,
) -> Result<ClaudeCodePreparedIntegration, PreparationFailure> {
    prepare_claude_code_headless(input, probe, services).await
}

fn prepare_run(
    integration: &ClaudeCodePreparedIntegration,
    input: ClaudeCodeRunProfileInput,
) -> Result<ClaudeCodePreparedRun, PreparationFailure> {
    integration.prepare_run(input)
}

async fn execute(
    prepared: &ClaudeCodePreparedRun,
    services: HostServices,
) -> Result<(TerminalOutcome, CleanupOutcome), RuntimeFailure> {
    let mut run = prepared.start_run(services).await?;
    let outcome = run
        .take_terminal_outcome()
        .expect("Claude Code runs expose one terminal outcome")
        .await;
    Ok((outcome, run.close().await))
}

fn main() {}

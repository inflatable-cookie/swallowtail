#![allow(dead_code)]

use swallowtail_adapter_claude_agent::{
    ClaudeCodeMaximumTurns, ClaudeCodePreparationInput, ClaudeCodePreparationProbe,
    ClaudeCodePreparedIntegration, ClaudeCodePreparedRun, ClaudeCodeRunProfileInput,
    prepare_claude_code_headless,
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

/// Caps agentic tool-use turns on a qualified Claude Code version.
///
/// Omitting `with_maximum_turns` keeps the exact command with no `--max-turns`
/// argument and passes the approved environment through unchanged. That is not
/// a claim of unlimited execution: an ambient `CLAUDE_CODE_MAX_TURNS` still
/// applies on the host when the flag is absent.
fn prepare_bounded_run(
    integration: &ClaudeCodePreparedIntegration,
    input: ClaudeCodeRunProfileInput,
    maximum_turns: u64,
) -> Result<ClaudeCodePreparedRun, PreparationFailure> {
    let maximum_turns = ClaudeCodeMaximumTurns::from_u64(maximum_turns).map_err(|error| {
        PreparationFailure::new(
            swallowtail_runtime::PreparationStage::Preflight,
            swallowtail_core::Diagnostic::new(error.diagnostic().clone()),
        )
    })?;
    integration.prepare_run(input.with_maximum_turns(maximum_turns))
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

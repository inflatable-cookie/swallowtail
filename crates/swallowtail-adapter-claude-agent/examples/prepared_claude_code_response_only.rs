#![allow(dead_code)]

use futures_util::StreamExt;
use swallowtail_adapter_claude_agent::{
    ClaudeCodeResponsePreparationInput, ClaudeCodeResponsePreparationProbe,
    ClaudeCodeResponsePreparedIntegration, ClaudeCodeResponsePreparedRun,
    ClaudeCodeResponseProfileInput, prepare_claude_code_response_only,
};
use swallowtail_runtime::{
    CleanupOutcome, HostServices, OperationContent, PreparationFailure, RuntimeFailure,
    TerminalOutcome,
};

async fn prepare_installation(
    input: ClaudeCodeResponsePreparationInput,
    probe: ClaudeCodeResponsePreparationProbe,
    services: HostServices,
) -> Result<ClaudeCodeResponsePreparedIntegration, PreparationFailure> {
    prepare_claude_code_response_only(input, probe, services).await
}

fn prepare_text_response(
    integration: &ClaudeCodeResponsePreparedIntegration,
    input: ClaudeCodeResponseProfileInput,
) -> Result<ClaudeCodeResponsePreparedRun, PreparationFailure> {
    integration.prepare_run(input)
}

async fn execute(
    prepared: &ClaudeCodeResponsePreparedRun,
    services: HostServices,
) -> Result<(TerminalOutcome, CleanupOutcome), RuntimeFailure> {
    let mut run = prepared.start_run(services).await?;
    let mut events = run.take_events().expect("prepared run exposes events");
    let terminal = run
        .take_terminal_outcome()
        .expect("prepared run exposes one terminal outcome");
    let outcome = async {
        while let Some(event) = events.next().await {
            event?;
        }
        Ok::<_, RuntimeFailure>(terminal.await)
    }
    .await?;
    let cleanup = run.close().await;
    Ok((outcome, cleanup))
}

fn untrusted_text(outcome: &TerminalOutcome) -> Option<&str> {
    outcome.output().map(OperationContent::as_str)
}

fn main() {}

#![allow(dead_code)]

use futures_util::StreamExt;
use swallowtail_adapter_kimi::{KimiLocalServerPreparedIntegration, KimiLocalServerSessionInput};
use swallowtail_runtime::{
    CleanupOutcome, HostServices, OperationContent, ProviderSessionManagementBinding, RuntimeEvent,
    RuntimeFailure, RuntimeTurnId, SessionCleanupRequest, TerminalOutcome, TurnRequest,
};

async fn run_interactive_turn(
    prepared: &KimiLocalServerPreparedIntegration,
    input: KimiLocalServerSessionInput,
    turn_id: RuntimeTurnId,
    content: OperationContent,
    services: HostServices,
    cleanup: SessionCleanupRequest,
) -> Result<
    (
        Vec<RuntimeEvent>,
        TerminalOutcome,
        CleanupOutcome,
        Option<ProviderSessionManagementBinding>,
    ),
    RuntimeFailure,
> {
    let profile = prepared
        .prepare_session(input)
        .map_err(|error| RuntimeFailure::new(error.diagnostic().safe().clone()))?;
    let mut session = profile.open_session(services.clone()).await?;
    let management = session.management_binding().cloned();
    let mut turn = session
        .start_turn(TurnRequest::new(turn_id, content), services.clone())
        .await?;
    let mut events = turn
        .take_events()
        .expect("Kimi local-server turns expose one event stream");
    let mut collected = Vec::new();
    while let Some(event) = events.next().await {
        collected.push(event?);
    }
    let outcome = turn
        .take_terminal_outcome()
        .expect("Kimi local-server turns expose one terminal outcome")
        .await;
    let _ = turn.close().await;
    let outcome_cleanup = session.close(cleanup, services).await;
    Ok((collected, outcome, outcome_cleanup, management))
}

fn main() {}

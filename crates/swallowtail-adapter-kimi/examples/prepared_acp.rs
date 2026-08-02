#![allow(dead_code)]

use swallowtail_adapter_kimi::{
    KimiPreparationInput, KimiPreparationProbe, KimiPreparedIntegration, KimiPreparedSession,
    KimiPreparedSessionCatalogue, KimiSessionCatalogueInput, KimiSessionProfileInput, prepare_kimi,
};
use swallowtail_runtime::{
    CleanupOutcome, HostServices, OperationContent, ProviderSessionCandidate, RuntimeFailure,
    RuntimeTurnId, SessionResumeBinding, TerminalOutcome, TurnRequest,
};

async fn prepare_installation(
    input: KimiPreparationInput,
    probe: KimiPreparationProbe,
    discovery_services: HostServices,
) -> Result<KimiPreparedIntegration, swallowtail_runtime::PreparationFailure> {
    prepare_kimi(input, probe, discovery_services).await
}

fn prepare_session(
    installation: &KimiPreparedIntegration,
    input: KimiSessionProfileInput,
) -> Result<KimiPreparedSession, swallowtail_runtime::PreparationFailure> {
    installation.prepare_session(input)
}

fn prepare_catalogue(
    installation: &KimiPreparedIntegration,
    input: KimiSessionCatalogueInput,
) -> Result<KimiPreparedSessionCatalogue, swallowtail_runtime::PreparationFailure> {
    installation.prepare_session_catalogue(input)
}

async fn import_selected_session(
    installation: &KimiPreparedIntegration,
    catalogue: &KimiPreparedSessionCatalogue,
    candidate: ProviderSessionCandidate,
    session_input: KimiSessionProfileInput,
    services: HostServices,
) -> Result<SessionResumeBinding, String> {
    installation
        .prepare_session_import(catalogue, candidate, session_input)
        .map_err(|error| error.to_string())?
        .import_session(services)
        .await
        .map(|outcome| outcome.binding().clone())
        .map_err(|error| error.to_string())
}

async fn open_prompt_and_interrupt(
    prepared: &KimiPreparedSession,
    services: HostServices,
    turn_id: RuntimeTurnId,
    content: OperationContent,
) -> Result<(TerminalOutcome, CleanupOutcome), RuntimeFailure> {
    let mut session = prepared.open_session(services.clone()).await?;
    let mut turn = session
        .start_turn(TurnRequest::new(turn_id, content), services)
        .await?;
    let _ = turn.cancellation().request().await?;
    let outcome = turn
        .take_terminal_outcome()
        .expect("Kimi turns expose one terminal outcome")
        .await;
    let _ = turn.close().await;
    Ok((outcome, session.close().await))
}

async fn load_with_replay(
    prepared: &KimiPreparedSession,
    request_id: swallowtail_runtime::RequestId,
    binding: SessionResumeBinding,
    services: HostServices,
) -> Result<usize, String> {
    let loaded = prepared
        .load_session(request_id, binding, services)
        .map_err(|error| error.to_string())?
        .await
        .map_err(|error| error.to_string())?;
    let (replay, session) = loaded.into_parts();
    let _ = session.close().await;
    Ok(replay.len())
}

async fn resume_without_replay(
    prepared: &KimiPreparedSession,
    request_id: swallowtail_runtime::RequestId,
    binding: SessionResumeBinding,
    services: HostServices,
) -> Result<CleanupOutcome, String> {
    let session = prepared
        .resume_session(request_id, binding, services)
        .map_err(|error| error.to_string())?
        .await
        .map_err(|error| error.to_string())?;
    Ok(session.close().await)
}

fn main() {}

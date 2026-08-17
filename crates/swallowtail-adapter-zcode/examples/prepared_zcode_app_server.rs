#![allow(dead_code)]

use swallowtail_adapter_zcode::{
    ZcodeAppServerMode, ZcodeModelSelection, ZcodePreparationInput, ZcodePreparationProbe,
    ZcodePreparedIntegration, ZcodePreparedRun, ZcodeRunProfileInput, prepare_zcode_app_server,
};
use swallowtail_runtime::{
    CleanupOutcome, HostServices, PreparationFailure, RuntimeFailure, TerminalOutcome,
};

async fn prepare_installation(
    input: ZcodePreparationInput,
    probe: ZcodePreparationProbe,
    services: HostServices,
) -> Result<ZcodePreparedIntegration, PreparationFailure> {
    prepare_zcode_app_server(input, probe, services).await
}

fn prepare_run(
    integration: &ZcodePreparedIntegration,
    input: ZcodeRunProfileInput,
) -> Result<ZcodePreparedRun, PreparationFailure> {
    integration.prepare_run(input)
}

async fn execute_run(
    prepared: &ZcodePreparedRun,
    services: HostServices,
) -> Result<(TerminalOutcome, CleanupOutcome), RuntimeFailure> {
    let mut run = prepared.start_run(services).await?;
    let outcome = run
        .take_terminal_outcome()
        .expect("ZCode app-server runs expose one terminal outcome")
        .await;
    Ok((outcome, run.close().await))
}

fn select_model(
    route_id: swallowtail_core::ModelRouteId,
    route_revision: swallowtail_core::ModelRouteRevision,
    provider_id: swallowtail_core::ProviderId,
    model_id: swallowtail_core::ModelId,
) -> ZcodeModelSelection {
    ZcodeModelSelection::new(route_id, route_revision, provider_id, model_id)
}

fn select_mode(value: &str) -> Option<ZcodeAppServerMode> {
    ZcodeAppServerMode::new(value)
}

fn main() {}

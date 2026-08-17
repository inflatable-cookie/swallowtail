#![allow(dead_code)]

use swallowtail_adapter_deepseek_harness::{
    DeepSeekHarnessModelSelection, DeepSeekHarnessPreparationInput,
    DeepSeekHarnessPreparationProbe, DeepSeekHarnessPreparedIntegration,
    DeepSeekHarnessPreparedRun, DeepSeekHarnessRunProfileInput, prepare_deepseek_harness_jsonrpc,
};
use swallowtail_runtime::{
    CleanupOutcome, HostServices, PreparationFailure, RuntimeFailure, TerminalOutcome,
};

async fn prepare_installation(
    input: DeepSeekHarnessPreparationInput,
    probe: DeepSeekHarnessPreparationProbe,
    services: HostServices,
) -> Result<DeepSeekHarnessPreparedIntegration, PreparationFailure> {
    prepare_deepseek_harness_jsonrpc(input, probe, services).await
}

fn prepare_run(
    integration: &DeepSeekHarnessPreparedIntegration,
    input: DeepSeekHarnessRunProfileInput,
) -> Result<DeepSeekHarnessPreparedRun, PreparationFailure> {
    integration.prepare_run(input)
}

async fn execute_run(
    prepared: &DeepSeekHarnessPreparedRun,
    services: HostServices,
) -> Result<(TerminalOutcome, CleanupOutcome), RuntimeFailure> {
    let mut run = prepared.start_run(services).await?;
    let outcome = run
        .take_terminal_outcome()
        .expect("DeepSeek Harness runs expose one terminal outcome")
        .await;
    Ok((outcome, run.close().await))
}

fn select_model(
    route_id: swallowtail_core::ModelRouteId,
    route_revision: swallowtail_core::ModelRouteRevision,
    provider_id: swallowtail_core::ProviderId,
    model_id: swallowtail_core::ModelId,
) -> DeepSeekHarnessModelSelection {
    DeepSeekHarnessModelSelection::new(route_id, route_revision, provider_id, model_id)
}

fn main() {}

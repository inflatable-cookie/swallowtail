#![allow(dead_code)]

use swallowtail_adapter_antigravity::{
    AntigravityCatalogueProfileInput, AntigravityPreparationInput, AntigravityPreparationProbe,
    AntigravityPreparedCatalogue, AntigravityPreparedCatalogueIntegration,
    AntigravityPreparedIntegration, prepare_antigravity,
};
use swallowtail_core::ModelCatalogEntry;
use swallowtail_runtime::{HostServices, PreparationFailure, RuntimeFailure};

async fn prepare_installation(
    input: AntigravityPreparationInput,
    probe: AntigravityPreparationProbe,
    services: HostServices,
) -> Result<AntigravityPreparedIntegration, PreparationFailure> {
    prepare_antigravity(input, probe, services).await
}

fn prepare_catalogue(
    integration: &AntigravityPreparedCatalogueIntegration,
    input: AntigravityCatalogueProfileInput,
) -> Result<AntigravityPreparedCatalogue, PreparationFailure> {
    integration.prepare_catalogue(input)
}

async fn list_models(
    prepared: &AntigravityPreparedCatalogue,
    services: HostServices,
) -> Result<Vec<ModelCatalogEntry>, RuntimeFailure> {
    prepared.list_models(services).await
}

fn main() {}

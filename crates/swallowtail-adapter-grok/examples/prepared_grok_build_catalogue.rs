#![allow(dead_code)]

use swallowtail_adapter_grok::{
    GrokCatalogueProfileInput, GrokPreparationInput, GrokPreparationProbe, GrokPreparedCatalogue,
    GrokPreparedIntegration, prepare_grok_build,
};
use swallowtail_core::ModelCatalogEntry;
use swallowtail_runtime::{HostServices, PreparationFailure, RuntimeFailure};

async fn prepare_installation(
    input: GrokPreparationInput,
    probe: GrokPreparationProbe,
    services: HostServices,
) -> Result<GrokPreparedIntegration, PreparationFailure> {
    prepare_grok_build(input, probe, services).await
}

fn prepare_catalogue(
    integration: &GrokPreparedIntegration,
    input: GrokCatalogueProfileInput,
) -> Result<GrokPreparedCatalogue, PreparationFailure> {
    integration.prepare_catalogue(input)
}

async fn list_models(
    prepared: &GrokPreparedCatalogue,
    services: HostServices,
) -> Result<Vec<ModelCatalogEntry>, RuntimeFailure> {
    prepared.list_models(services).await
}

fn main() {}

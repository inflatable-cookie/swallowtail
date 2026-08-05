#![allow(dead_code)]

use swallowtail_adapter_cursor::{
    CursorCatalogueProfileInput, CursorPreparationInput, CursorPreparationProbe,
    CursorPreparedCatalogue, CursorPreparedCatalogueIntegration, CursorPreparedIntegration,
    prepare_cursor,
};
use swallowtail_core::ModelCatalogEntry;
use swallowtail_runtime::{HostServices, PreparationFailure, RuntimeFailure};

async fn prepare_installation(
    input: CursorPreparationInput,
    probe: CursorPreparationProbe,
    services: HostServices,
) -> Result<CursorPreparedIntegration, PreparationFailure> {
    prepare_cursor(input, probe, services).await
}

fn prepare_catalogue(
    integration: &CursorPreparedCatalogueIntegration,
    input: CursorCatalogueProfileInput,
) -> Result<CursorPreparedCatalogue, PreparationFailure> {
    integration.prepare_catalogue(input)
}

async fn list_models(
    prepared: &CursorPreparedCatalogue,
    services: HostServices,
) -> Result<Vec<ModelCatalogEntry>, RuntimeFailure> {
    prepared.list_models(services).await
}

fn main() {}

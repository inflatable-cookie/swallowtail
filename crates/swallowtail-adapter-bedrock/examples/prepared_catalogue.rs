#![allow(dead_code)]

use swallowtail_adapter_bedrock::{
    BedrockCataloguePreparationInput, BedrockCataloguePreparedIntegration,
    BedrockCatalogueProfileInput, BedrockPreparedCatalogue, prepare_bedrock_catalogue,
};
use swallowtail_runtime::{HostServices, PreparationFailure};

fn prepare_integration(
    input: BedrockCataloguePreparationInput,
    services: &HostServices,
) -> Result<BedrockCataloguePreparedIntegration, PreparationFailure> {
    prepare_bedrock_catalogue(input, services)
}

fn prepare_catalogue(
    integration: &BedrockCataloguePreparedIntegration,
    input: BedrockCatalogueProfileInput,
) -> Result<BedrockPreparedCatalogue, PreparationFailure> {
    integration.prepare_catalogue(input)
}

fn main() {}

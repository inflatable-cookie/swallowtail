#![allow(dead_code)]

use swallowtail_adapter_bedrock::{
    BedrockCataloguePreparedIntegration, BedrockCatalogueRouteInput, BedrockFacade,
    BedrockFacadePreparationInput, BedrockRuntimePreparedIntegration, BedrockRuntimeRouteInput,
    prepare_bedrock,
};
use swallowtail_runtime::{HostServices, PreparationFailure};

fn prepare_provider(
    input: BedrockFacadePreparationInput,
    services: &HostServices,
) -> Result<BedrockFacade, PreparationFailure> {
    prepare_bedrock(input, services)
}

fn prepare_catalogue_route(
    bedrock: &BedrockFacade,
    input: BedrockCatalogueRouteInput,
    services: &HostServices,
) -> Result<BedrockCataloguePreparedIntegration, PreparationFailure> {
    bedrock.catalogue(input, services)
}

fn prepare_runtime_route(
    bedrock: &BedrockFacade,
    input: BedrockRuntimeRouteInput,
    services: &HostServices,
) -> Result<BedrockRuntimePreparedIntegration, PreparationFailure> {
    bedrock.runtime(input, services)
}

fn main() {}

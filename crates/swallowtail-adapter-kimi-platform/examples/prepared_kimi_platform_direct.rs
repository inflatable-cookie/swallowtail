#![allow(dead_code)]

use swallowtail_adapter_kimi_platform::{
    KimiPlatformCatalogueProfileInput, KimiPlatformInferenceAttemptInput,
    KimiPlatformPreparationInput, KimiPlatformPreparedCatalogue,
    KimiPlatformPreparedInferenceAttempt, KimiPlatformPreparedIntegration,
    prepare_kimi_platform_direct,
};
use swallowtail_runtime::{HostServices, PreparationFailure};

fn prepare_integration(
    input: KimiPlatformPreparationInput,
    services: &HostServices,
) -> Result<KimiPlatformPreparedIntegration, PreparationFailure> {
    prepare_kimi_platform_direct(input, services)
}

fn prepare_catalogue(
    integration: &KimiPlatformPreparedIntegration,
    input: KimiPlatformCatalogueProfileInput,
) -> Result<KimiPlatformPreparedCatalogue, PreparationFailure> {
    integration.prepare_catalogue(input)
}

fn prepare_one_attempt(
    integration: &KimiPlatformPreparedIntegration,
    input: KimiPlatformInferenceAttemptInput,
) -> Result<KimiPlatformPreparedInferenceAttempt, PreparationFailure> {
    integration.prepare_inference_attempt(input)
}

fn main() {}

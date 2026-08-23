#![allow(dead_code)]

use std::num::NonZeroU64;
use swallowtail_adapter_deepseek::{
    DeepSeekCatalogueProfileInput, DeepSeekModelSelection, DeepSeekPreparationInput,
    DeepSeekPreparedCatalogue, DeepSeekPreparedIntegration, DeepSeekPreparedRun,
    DeepSeekPreparedSession, DeepSeekRunProfileInput, DeepSeekSessionProfileInput,
    DeepSeekThinkingMode, prepare_deepseek_direct,
};
use swallowtail_core::ProviderInferenceCachePolicy;
use swallowtail_runtime::{HostServices, OperationContent, PreparationFailure, RequestId};

fn prepare_integration(
    input: DeepSeekPreparationInput,
    services: &HostServices,
) -> Result<DeepSeekPreparedIntegration, PreparationFailure> {
    prepare_deepseek_direct(input, services)
}

fn prepare_catalogue(
    integration: &DeepSeekPreparedIntegration,
    input: DeepSeekCatalogueProfileInput,
) -> Result<DeepSeekPreparedCatalogue, PreparationFailure> {
    integration.prepare_catalogue(input)
}

fn prepare_session(
    integration: &DeepSeekPreparedIntegration,
    input: DeepSeekSessionProfileInput,
) -> Result<DeepSeekPreparedSession, PreparationFailure> {
    integration.prepare_session(input)
}

fn prepare_disabled_run(
    integration: &DeepSeekPreparedIntegration,
    request_id: RequestId,
    model: DeepSeekModelSelection,
    content: OperationContent,
    maximum_output_tokens: NonZeroU64,
    cache_policy: ProviderInferenceCachePolicy,
) -> Result<DeepSeekPreparedRun, PreparationFailure> {
    integration.prepare_run(DeepSeekRunProfileInput::new_with_thinking_mode(
        request_id,
        model,
        content,
        DeepSeekThinkingMode::disabled(),
        maximum_output_tokens,
        cache_policy,
    ))
}

fn main() {}

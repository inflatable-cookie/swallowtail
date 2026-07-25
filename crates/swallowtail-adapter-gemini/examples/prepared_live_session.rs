#![allow(dead_code)]

use swallowtail_adapter_gemini::{
    GeminiLivePreparationInput, GeminiLivePreparedIntegration, GeminiLiveSessionProfileInput,
    GeminiPreparedLiveSession, prepare_gemini_live,
};
use swallowtail_runtime::{HostServices, PreparationFailure};

fn prepare_integration(
    input: GeminiLivePreparationInput,
    services: &HostServices,
) -> Result<GeminiLivePreparedIntegration, PreparationFailure> {
    prepare_gemini_live(input, services)
}

fn prepare_session(
    integration: &GeminiLivePreparedIntegration,
    input: GeminiLiveSessionProfileInput,
) -> Result<GeminiPreparedLiveSession, PreparationFailure> {
    integration.prepare_live_session(input)
}

fn main() {}

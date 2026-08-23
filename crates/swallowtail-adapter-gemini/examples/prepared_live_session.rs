#![allow(dead_code)]

use std::num::NonZeroU64;
use swallowtail_adapter_gemini::{
    GeminiLivePreparationInput, GeminiLivePreparedIntegration, GeminiLiveSessionProfileInput,
    GeminiPreparedLiveSession, prepare_gemini_live,
};
use swallowtail_core::ReasoningMode;
use swallowtail_runtime::{Deadline, HostServices, PreparationFailure, RequestId};

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

fn select_thinking_level(
    request_id: RequestId,
    deadline: Option<Deadline>,
    mode: ReasoningMode,
) -> GeminiLiveSessionProfileInput {
    GeminiLiveSessionProfileInput::manual_pcm_with_one_rollover(request_id, deadline)
        .with_reasoning_mode(mode)
}

fn select_output_maximum(
    request_id: RequestId,
    deadline: Option<Deadline>,
    maximum: NonZeroU64,
) -> GeminiLiveSessionProfileInput {
    GeminiLiveSessionProfileInput::manual_pcm_with_one_rollover(request_id, deadline)
        .with_maximum_output_tokens(maximum)
}

fn main() {}

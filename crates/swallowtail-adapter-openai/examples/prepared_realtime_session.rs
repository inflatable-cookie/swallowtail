#![allow(dead_code)]

use swallowtail_adapter_openai::{
    OpenAiPreparedRealtimeSession, OpenAiRealtimePreparationInput,
    OpenAiRealtimePreparedIntegration, OpenAiRealtimeSessionProfileInput, prepare_openai_realtime,
};
use swallowtail_core::ReasoningMode;
use swallowtail_runtime::{HostServices, PreparationFailure};

fn prepare_integration(
    input: OpenAiRealtimePreparationInput,
    services: &HostServices,
) -> Result<OpenAiRealtimePreparedIntegration, PreparationFailure> {
    prepare_openai_realtime(input, services)
}

fn prepare_session(
    integration: &OpenAiRealtimePreparedIntegration,
    input: OpenAiRealtimeSessionProfileInput,
) -> Result<OpenAiPreparedRealtimeSession, PreparationFailure> {
    integration.prepare_realtime_session(input)
}

fn prepare_session_with_reasoning(
    integration: &OpenAiRealtimePreparedIntegration,
    input: OpenAiRealtimeSessionProfileInput,
    effort: &str,
) -> Result<OpenAiPreparedRealtimeSession, PreparationFailure> {
    prepare_session(
        integration,
        input
            .with_reasoning_mode(ReasoningMode::new(effort).expect("compile-time effort is valid")),
    )
}

fn main() {}

#![allow(dead_code)]

use swallowtail_adapter_openai::{
    OpenAiPreparedRealtimeSession, OpenAiRealtimePreparationInput,
    OpenAiRealtimePreparedIntegration, OpenAiRealtimeSessionProfileInput, prepare_openai_realtime,
};
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

fn main() {}

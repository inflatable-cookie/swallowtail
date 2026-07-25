#![allow(dead_code)]

use swallowtail_adapter_xai::{
    XaiPreparationInput, XaiPreparedIntegration, XaiPreparedResponsesSession,
    XaiSessionProfileInput, prepare_xai_responses_websocket,
};
use swallowtail_runtime::{HostServices, PreparationFailure};

fn prepare_integration(
    input: XaiPreparationInput,
    services: &HostServices,
) -> Result<XaiPreparedIntegration, PreparationFailure> {
    prepare_xai_responses_websocket(input, services)
}

fn prepare_session(
    integration: &XaiPreparedIntegration,
    input: XaiSessionProfileInput,
) -> Result<XaiPreparedResponsesSession, PreparationFailure> {
    integration.prepare_responses_session(input)
}

fn main() {}

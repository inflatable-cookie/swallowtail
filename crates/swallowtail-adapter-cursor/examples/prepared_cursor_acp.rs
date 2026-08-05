#![allow(dead_code)]

use swallowtail_adapter_cursor::{
    CursorAcpSessionProfileInput, CursorPreparationInput, CursorPreparationProbe,
    CursorPreparedAcpIntegration, CursorPreparedAcpSession, CursorPreparedIntegration,
    prepare_cursor,
};
use swallowtail_runtime::{
    HostServices, InteractiveSessionHandle, PreparationFailure, PreparedWorkingStateRestoration,
    RequestId, RuntimeFailure, RuntimeTurnId, SessionResumeBinding,
};

async fn prepare_installation(
    input: CursorPreparationInput,
    probe: CursorPreparationProbe,
    services: HostServices,
) -> Result<CursorPreparedIntegration, PreparationFailure> {
    prepare_cursor(input, probe, services).await
}

fn prepare_session(
    integration: &CursorPreparedAcpIntegration,
    input: CursorAcpSessionProfileInput,
) -> Result<CursorPreparedAcpSession, PreparationFailure> {
    integration.prepare_session(input)
}

async fn open_session(
    prepared: &CursorPreparedAcpSession,
    services: HostServices,
) -> Result<Box<dyn InteractiveSessionHandle>, RuntimeFailure> {
    prepared.open_session(services).await
}

fn prepare_attachment_recovery(
    prepared: &CursorPreparedAcpSession,
    request_id: RequestId,
    binding: SessionResumeBinding,
    interrupted_turn_id: RuntimeTurnId,
) -> Result<PreparedWorkingStateRestoration, PreparationFailure> {
    prepared.prepare_working_state_restoration(request_id, binding, interrupted_turn_id)
}

fn main() {}

#![allow(dead_code)]

//! Pi SDK sidecar route: the application provisions the exact approved Node
//! runtime, the source-tagged sidecar entry point, and the exact
//! `@earendil-works/pi-coding-agent` SDK package through a host-approved
//! interpreted-script launch recipe, and carries the SDK module, agent
//! directory, and session directory in the approved environment
//! (`PI_SDK_SIDECAR_SDK_MODULE`, `PI_SDK_SIDECAR_AGENT_DIR`,
//! `PI_SDK_SIDECAR_SESSION_DIR`). Swallowtail never installs, discovers, or
//! repairs any of them.

use swallowtail_adapter_pi::{
    PiSdkSidecarPreparedSession, PiSdkSidecarSessionPreparation, prepare_pi_sdk_sidecar_session,
};
use swallowtail_runtime::{
    CleanupOutcome, HostServices, LoadedSession, OperationContent, PreparationFailure, RequestId,
    RuntimeFailure, RuntimeTurnId, SessionCleanupRequest, SessionOptions, SessionResumeBinding,
    TerminalOutcome, TurnRequest,
};

fn prepare_session(
    input: PiSdkSidecarSessionPreparation,
) -> Result<PiSdkSidecarPreparedSession, PreparationFailure> {
    prepare_pi_sdk_sidecar_session(input, SessionOptions::default())
}

async fn open_and_prompt(
    prepared: &PiSdkSidecarPreparedSession,
    services: HostServices,
    cleanup: SessionCleanupRequest,
    turn_id: RuntimeTurnId,
    content: OperationContent,
) -> Result<(TerminalOutcome, CleanupOutcome), RuntimeFailure> {
    let mut session = prepared.open_session(services.clone()).await?;
    let mut turn = session
        .start_turn(TurnRequest::new(turn_id, content), services.clone())
        .await?;
    let outcome = turn
        .take_terminal_outcome()
        .expect("sidecar turns expose one terminal outcome")
        .await;
    let _ = turn.close().await;
    Ok((outcome, session.close(cleanup, services).await))
}

async fn load_with_replay(
    prepared: &PiSdkSidecarPreparedSession,
    request_id: RequestId,
    binding: SessionResumeBinding,
    services: HostServices,
) -> Result<LoadedSession, RuntimeFailure> {
    prepared
        .load_session(request_id, binding, services)
        .expect("a binding from this plan builds a load request")
        .await
}

fn main() {}

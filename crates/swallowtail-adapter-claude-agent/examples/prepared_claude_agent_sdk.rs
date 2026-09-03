#![allow(dead_code)]

//! Claude Agent SDK sidecar route: the application provisions the exact
//! approved Node runtime, the source-tagged sidecar entry point, the exact
//! `@anthropic-ai/claude-agent-sdk` package with its peer dependencies, and
//! the exact platform package that carries the native `claude` binary, all
//! through a host-approved interpreted-script launch recipe. The approved
//! environment carries the SDK module, the native binary, and the shipped
//! manifest (`CLAUDE_AGENT_SDK_SIDECAR_SDK_MODULE`,
//! `CLAUDE_AGENT_SDK_SIDECAR_NATIVE_BINARY`,
//! `CLAUDE_AGENT_SDK_SIDECAR_MANIFEST`). Swallowtail never installs, vendors,
//! updates, repairs, or redistributes any of them.
//!
//! The user runs the official Claude login out of band. Swallowtail never
//! holds the subscription credential: it leases a delegated reference, and
//! the native binary authenticates itself.
//!
//! Close is the interesting part. The upstream SDK offers no joined stop, so
//! the sidecar joins its own retained native handle to a declared bound and
//! the host makes the declared descendant termination attempt. Every stage sits
//! inside the caller's single `SessionCleanupRequest` deadline. Close reports
//! `Clean` only where the host attests `OwnedTreeEmpty`; on ordinary macOS a
//! confirmed root-only completion is the accepted `Degraded` posture, and an
//! observed surviving descendant or unconfirmed root exit is `Failed`.

use swallowtail_adapter_claude_agent::sdk::{
    ClaudeAgentSdkPreparedSession, ClaudeAgentSdkSessionPreparation,
    prepare_claude_agent_sdk_session,
};
use swallowtail_runtime::{
    CallbackResponse, CleanupOutcome, HostServices, OperationContent, PreparationFailure,
    RuntimeFailure, RuntimeTurnId, SessionCleanupRequest, SessionOptions, TerminalOutcome,
    TurnRequest,
};

fn prepare_session(
    input: ClaudeAgentSdkSessionPreparation,
) -> Result<ClaudeAgentSdkPreparedSession, PreparationFailure> {
    // This layer admits no session options: model, effort, thinking, resume,
    // and fork are later layers.
    prepare_claude_agent_sdk_session(input, SessionOptions::default())
}

async fn open_and_prompt(
    prepared: &ClaudeAgentSdkPreparedSession,
    services: HostServices,
    turn_id: RuntimeTurnId,
    content: OperationContent,
    cleanup: SessionCleanupRequest,
) -> Result<(TerminalOutcome, CleanupOutcome), RuntimeFailure> {
    let services_for_cleanup = services.clone();
    let mut session = prepared.open_session(services.clone()).await?;
    let mut turn = session
        .start_turn(TurnRequest::new(turn_id, content), services)
        .await?;
    // Read-only tool admission is the consumer's decision, delivered through
    // the turn's bounded callback exchange.
    let _admission = turn.take_callbacks();
    let outcome = turn
        .take_terminal_outcome()
        .expect("sidecar turns expose one terminal outcome")
        .await;
    let _ = turn.close().await;
    Ok((outcome, session.close(cleanup, services_for_cleanup).await))
}

async fn admit_tool_use(
    exchange: &swallowtail_runtime::CallbackExchange,
    response: CallbackResponse,
) -> Result<(), RuntimeFailure> {
    exchange.responder().respond(response).await
}

fn main() {}

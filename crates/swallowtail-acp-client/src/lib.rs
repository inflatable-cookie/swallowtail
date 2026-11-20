//! Shared Agent Client Protocol v1 client engine, activity projection, and HTTP MCP placement.

#![forbid(unsafe_code)]
#![deny(missing_docs)]

mod activity;
mod connection;
mod diagnostics;
mod mcp;
mod session;
mod transport;
mod turn;

#[cfg(test)]
mod tests;

pub use activity::{AcpActivityProjection, text_content};
pub use connection::{
    AcpConnection, AcpConnectionHooks, PendingResponse, RequestDisposition,
    SessionUpdateDisposition, StandardAcpHooks,
};
pub use diagnostics::{
    AcpActivityConfig, AcpDiagnostics, MessageClassification, PlanProjection, StopReasonPolicy,
    UsageProgressPolicy, default_permission_namespace,
};
pub use mcp::{
    AcpHttpAdvertisement, AcpMcpDiagnostics, AcpRemoteMcpPlacement, production_mcp_servers,
    redacted_mcp_servers_debug,
};
pub use session::{
    AcpSessionConfig, AcpSessionHandle, AcpSessionOpenContext, SessionCancellation,
    TurnValidationPolicy, open_acp_session, open_acp_session_with_hooks,
};
pub use transport::AcpTransport;
pub use turn::{AcpTurnHandle, AcpTurnSink, ActiveTurn, ActiveTurnConfig, TurnCancellation};

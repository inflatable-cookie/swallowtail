//! Gate-only streamable-HTTP MCP infrastructure for `gemini-cli.acp`.
//!
//! The listener is test-only. Production `gemini-cli.acp` never starts it.

#![allow(dead_code, unused_imports)]

mod record;
mod server;

pub use record::{
    HttpMcpLiveRecord, HttpMcpLiveStop, HttpMcpLiveStopDiagnostic, HttpMcpTranscript,
};
pub use server::{DisposableHttpMcpServer, HTTP_MCP_LIVE_TOOL, HTTP_MCP_LIVE_TOOL_RESULT};

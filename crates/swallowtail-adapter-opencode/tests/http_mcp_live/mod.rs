//! Gate-only streamable-HTTP MCP infrastructure for `opencode.acp`.
//!
//! The listener is test-only. Production `opencode.acp` never starts it.

#![allow(dead_code, unused_imports)]

mod record;
mod server;

pub use record::{HttpMcpLiveRecord, HttpMcpLiveStop, HttpMcpTranscript};
pub use server::{DisposableHttpMcpServer, HTTP_MCP_LIVE_TOOL, HTTP_MCP_LIVE_TOOL_RESULT};

//! OpenCode ACP stdio route, kept unflattened from the attached HTTP family.

mod access;
mod activity;
mod command;
mod connection;
mod consumer_route_projection;
mod discovery;
mod driver;
mod failure;
mod mcp;
mod prepared;
mod selection;
mod turn;

pub use access::{OPENCODE_ACP_HOST_ACCOUNT_AUDIENCE, opencode_acp_host_account_access_profile};
pub use driver::{OpenCodeAcpDriver, opencode_acp_descriptor};
pub use mcp::{
    OPENCODE_ACP_MCP_SERVER_NAME, OpenCodeAcpRemoteMcpPlacement, OpenCodeAcpStdioMcpServer,
};
pub use prepared::{
    OpenCodeAcpPreparationInput, OpenCodeAcpPreparationProbe, OpenCodeAcpPreparedIntegration,
    OpenCodeAcpPreparedSession, OpenCodeAcpSessionProfileInput, prepare_opencode_acp,
};
pub use selection::{
    OPENCODE_ACP_AXIS, OPENCODE_ACP_BASELINE_VERSION, OPENCODE_ACP_EXECUTABLE_NAME,
    OPENCODE_ACP_LATEST_QUALIFIED_VERSION, opencode_acp_binding, opencode_acp_claim,
};

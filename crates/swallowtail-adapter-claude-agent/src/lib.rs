//! Claude Agent ACP integration for Swallowtail.

#![forbid(unsafe_code)]

mod connection;
mod discovery;
mod driver;
mod failure;
mod prepared;
mod prepared_profile;
mod selection;
mod turn;

pub use driver::{ClaudeAgentAcpDriver, claude_agent_acp_descriptor};
pub use prepared::{
    ClaudeAgentPreparationInput, ClaudeAgentPreparationProbe, ClaudeAgentPreparedIntegration,
    prepare_claude_agent,
};
pub use prepared_profile::{
    ClaudeAgentModelSelection, ClaudeAgentPreparedEvidence, ClaudeAgentPreparedSession,
    ClaudeAgentPreparedSessionFuture, ClaudeAgentSessionProfileInput,
};
pub use selection::{
    CLAUDE_AGENT_ACP_AXIS, CLAUDE_AGENT_ACP_BASELINE_VERSION,
    CLAUDE_AGENT_ACP_LATEST_QUALIFIED_VERSION, claude_agent_acp_binding, claude_agent_acp_claim,
};

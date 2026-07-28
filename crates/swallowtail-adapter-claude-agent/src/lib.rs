//! Claude Agent ACP integration for Swallowtail.

#![forbid(unsafe_code)]

mod claude_code;
mod claude_code_command;
mod claude_code_discovery;
mod claude_code_events;
mod claude_code_handle;
mod claude_code_pump;
mod claude_code_selection;
mod claude_code_validation;
mod connection;
mod discovery;
mod driver;
mod failure;
mod prepared;
mod prepared_code;
mod prepared_profile;
mod selection;
mod turn;

pub use claude_code::{ClaudeCodeHeadlessDriver, claude_code_headless_descriptor};
pub use claude_code_selection::{
    CLAUDE_CODE_HEADLESS_AXIS, CLAUDE_CODE_HEADLESS_BASELINE_VERSION,
    CLAUDE_CODE_HEADLESS_LATEST_QUALIFIED_VERSION, claude_code_headless_binding,
    claude_code_headless_claim,
};
pub use driver::{ClaudeAgentAcpDriver, claude_agent_acp_descriptor};
pub use prepared::{
    ClaudeAgentPreparationInput, ClaudeAgentPreparationProbe, ClaudeAgentPreparedIntegration,
    prepare_claude_agent,
};
pub use prepared_code::{
    ClaudeCodeModelSelection, ClaudeCodePreparationInput, ClaudeCodePreparationProbe,
    ClaudeCodePreparedEvidence, ClaudeCodePreparedIntegration, ClaudeCodePreparedRun,
    ClaudeCodeRunProfileInput, prepare_claude_code_headless,
};
pub use prepared_profile::{
    ClaudeAgentModelSelection, ClaudeAgentPreparedDelete, ClaudeAgentPreparedEvidence,
    ClaudeAgentPreparedRun, ClaudeAgentPreparedSession, ClaudeAgentPreparedSessionFuture,
    ClaudeAgentRunProfileInput, ClaudeAgentSessionManagementInput, ClaudeAgentSessionProfileInput,
};
pub use selection::{
    CLAUDE_AGENT_ACP_AXIS, CLAUDE_AGENT_ACP_BASELINE_VERSION,
    CLAUDE_AGENT_ACP_LATEST_QUALIFIED_VERSION, claude_agent_acp_binding, claude_agent_acp_claim,
};

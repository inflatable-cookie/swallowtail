//! Claude integrations for Swallowtail.
//!
//! The third-party Claude Agent ACP route and the native one-shot
//! `claude -p` stream-JSON route remain separate prepared integrations.

#![forbid(unsafe_code)]
#![deny(missing_docs)]

mod access;
mod acp_activity;
mod addable;
mod claude_code;
mod claude_code_activity;
mod claude_code_command;
mod claude_code_discovery;
mod claude_code_events;
mod claude_code_handle;
mod claude_code_maximum_turns;
mod claude_code_pump;
mod claude_code_response;
mod claude_code_response_activity;
mod claude_code_response_command;
mod claude_code_response_discovery;
mod claude_code_response_events;
mod claude_code_response_pump;
mod claude_code_response_selection;
mod claude_code_response_validation;
mod claude_code_selection;
mod claude_code_validation;
mod claude_code_watcher;
mod connection;
mod consumer_route_projection;
mod discovery;
mod driver;
mod elicitation;
mod failure;
mod permission;
mod prepared;
mod prepared_code;
mod prepared_profile;
mod prepared_response;
/// Claude Agent SDK sidecar route: asset, identity, private wire, and driver.
pub mod sdk;
mod selection;
mod turn;

pub use access::{
    CLAUDE_AGENT_ACP_SUBSCRIPTION_AUDIENCE, claude_agent_acp_subscription_access_profile,
};
pub use addable::{
    CLAUDE_AGENT_ACP_ADDABLE_ROUTE_ID, CLAUDE_AGENT_ACP_BINARY_PATH_FIELD_ID,
    CLAUDE_AGENT_ACP_ENVIRONMENT_FIELD_ID, claude_agent_acp_addable_route_descriptor,
};
pub use claude_code::{ClaudeCodeHeadlessDriver, claude_code_headless_descriptor};
pub use claude_code_maximum_turns::ClaudeCodeMaximumTurns;
pub use claude_code_response::{
    ClaudeCodeResponseOnlyDriver, claude_code_response_only_descriptor,
};
pub use claude_code_response_selection::{
    CLAUDE_CODE_RESPONSE_ONLY_AXIS, CLAUDE_CODE_RESPONSE_ONLY_BASELINE_VERSION,
    CLAUDE_CODE_RESPONSE_ONLY_DENIED_VERSIONS, CLAUDE_CODE_RESPONSE_ONLY_LATEST_QUALIFIED_VERSION,
    CLAUDE_CODE_RESPONSE_ONLY_VERSION, claude_code_response_only_binding,
    claude_code_response_only_claim,
};
pub use claude_code_selection::{
    CLAUDE_CODE_HEADLESS_AXIS, CLAUDE_CODE_HEADLESS_BASELINE_VERSION,
    CLAUDE_CODE_HEADLESS_LATEST_QUALIFIED_VERSION, claude_code_headless_binding,
    claude_code_headless_claim,
};
pub use consumer_route_projection::{
    ClaudeAgentProjectionOpenFailure, ClaudeAgentProjectionOpenFuture,
    ClaudeAgentProjectionOpenOutcome,
};
pub use driver::{ClaudeAgentAcpDriver, claude_agent_acp_descriptor};
pub use permission::claude_agent_permission_namespace;
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
    ClaudeAgentModelSelection, ClaudeAgentPermissionHandling, ClaudeAgentPreparedDelete,
    ClaudeAgentPreparedEvidence, ClaudeAgentPreparedRun, ClaudeAgentPreparedSession,
    ClaudeAgentPreparedSessionFuture, ClaudeAgentPreparedSessionLoadFuture,
    ClaudeAgentRunProfileInput, ClaudeAgentRunRetention, ClaudeAgentSessionManagementInput,
    ClaudeAgentSessionProfileInput,
};
pub use prepared_response::{
    ClaudeCodeResponseModelSelection, ClaudeCodeResponsePreparationInput,
    ClaudeCodeResponsePreparationProbe, ClaudeCodeResponsePreparedEvidence,
    ClaudeCodeResponsePreparedIntegration, ClaudeCodeResponsePreparedRun,
    ClaudeCodeResponseProfileInput, prepare_claude_code_response_only,
};

const MAXIMUM_REPLAY_ITEMS: usize = 64;
const MAXIMUM_REPLAY_BYTES: usize = 256 * 1024;
pub use selection::{
    CLAUDE_AGENT_ACP_AXIS, CLAUDE_AGENT_ACP_BASELINE_VERSION,
    CLAUDE_AGENT_ACP_LATEST_QUALIFIED_VERSION, claude_agent_acp_binding, claude_agent_acp_claim,
};

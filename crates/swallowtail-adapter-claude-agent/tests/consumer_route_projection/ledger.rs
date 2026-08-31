#[path = "ledger/agent.rs"]
mod agent;
#[path = "ledger/code.rs"]
mod code;
#[path = "ledger/response.rs"]
mod response;

pub(super) use agent::AGENT_TRANCHE;
pub(super) use code::CODE_TRANCHE;
pub(super) use response::RESPONSE_TRANCHE;

pub(super) const AGENT_ROUTE: &str = "claude-agent.acp";
pub(super) const CODE_ROUTE: &str = "claude-code.headless";
pub(super) const RESPONSE_ROUTE: &str = "claude-code.response-only";

pub(super) const AGENT_RUN: &str = "ClaudeAgentPreparedRun[maximal]";
pub(super) const AGENT_SESSION: &str = "ClaudeAgentPreparedSession[maximal]";
pub(super) const AGENT_DELETE: &str = "ClaudeAgentPreparedDelete";
pub(super) const AGENT_OBSERVED: &str = "ClaudeAgentProjectionOpenOutcome[effective]";
pub(super) const CODE_RUN: &str = "ClaudeCodePreparedRun[maximal]";
pub(super) const RESPONSE_RUN: &str = "ClaudeCodeResponsePreparedRun[maximal]";

pub(super) const PROFILES: [&str; 6] = [
    AGENT_RUN,
    AGENT_SESSION,
    AGENT_DELETE,
    AGENT_OBSERVED,
    CODE_RUN,
    RESPONSE_RUN,
];

pub(super) struct LedgerEntry {
    pub(super) route_id: &'static str,
    pub(super) operation_shape: &'static str,
    pub(super) semantic_id: &'static str,
    pub(super) emitted_by: &'static [&'static str],
    pub(super) withheld_because: &'static str,
}

pub(super) fn entries() -> impl Iterator<Item = &'static LedgerEntry> {
    AGENT_TRANCHE
        .iter()
        .chain(CODE_TRANCHE.iter())
        .chain(RESPONSE_TRANCHE.iter())
}

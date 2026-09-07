//! Contract 063 registered-tool adoption for the `claude-agent.sdk` route.
//!
//! The pinned `0.3.259` surface exposes MCP server configuration, a permission
//! callback, and bounded status. It exposes no public callback equivalent to
//! `RegisteredToolDispatcher::dispatch(call, context)`. This route therefore
//! adopts the registered-tool boundary through one Swallowtail-owned MCP
//! carrier that it mediates itself, rather than through a portable dispatch
//! surface that does not exist here.
//!
//! Exact bounds of this adoption:
//!
//! - the carrier is Swallowtail-owned and derived only from an immutable
//!   Contract 063 selection; it is a separate path from card 084's
//!   consumer-declared stdio servers, which are unchanged, and neither may
//!   present the other's reserved server name;
//! - every admitted call is issued through the Contract 063 kernel, so live
//!   consumer admission, correlation, deadlines, exactly-once settlement, and
//!   the honest execution disposition stay where the contract puts them;
//! - `canUseTool` stays permission admission only, and a call the admission
//!   seam did not allow never reaches the linked dispatcher;
//! - tool identities stay namespaced, and the provider spelling is reversible
//!   so one wire name resolves to exactly one identity;
//! - Contract 061 publishes this as route-local stdio MCP mediation, never as
//!   common dispatch, and no other route inherits anything from it; and
//! - SSE, streamable HTTP, in-process SDK, and managed/ambient MCP stay
//!   withheld: no pinned transport corpus qualifies them.
//!
//! What this module does not claim: the registered capability is projected
//! `Unqualified`. The disposable real-route gate is separately authorized and
//! has not run, so no support disposition is published from deterministic
//! evidence alone.

mod binding;
mod carrier;
mod mediation;
mod projection;
mod version;
mod wire;

pub use binding::ClaudeAgentSdkRegisteredToolBinding;
pub use carrier::{
    CLAUDE_AGENT_SDK_REGISTERED_TOOL_MEDIATION, CLAUDE_AGENT_SDK_REGISTERED_TOOL_SERVER,
    ClaudeAgentSdkRegisteredTool, ClaudeAgentSdkRegisteredToolCarrier,
};
pub use mediation::{
    ClaudeAgentSdkMcpReply, ClaudeAgentSdkRegisteredToolDecision,
    ClaudeAgentSdkRegisteredToolMediator,
};
pub use projection::{
    CLAUDE_AGENT_SDK_MEDIATION_KIND_SEMANTIC_ID, CLAUDE_AGENT_SDK_REAL_ROUTE_GATE_PENDING_CODE,
    CLAUDE_AGENT_SDK_REGISTERED_TOOL_SOURCE, claude_agent_sdk_registered_tool_qualification,
    project_claude_agent_sdk_registered_tool,
};
pub use version::{
    CLAUDE_AGENT_SDK_MCP_PROTOCOL_VERSION, CLAUDE_AGENT_SDK_MCP_SUPPORTED_PROTOCOL_VERSIONS,
    CLAUDE_AGENT_SDK_REGISTERED_TOOL_CARRIER_AXIS,
    CLAUDE_AGENT_SDK_REGISTERED_TOOL_CARRIER_REVISION,
    CLAUDE_AGENT_SDK_REGISTERED_TOOL_NATIVE_VERSION, CLAUDE_AGENT_SDK_REGISTERED_TOOL_SDK_VERSION,
    claude_agent_sdk_mcp_protocol_version_admitted, claude_agent_sdk_mcp_protocol_version_known,
    claude_agent_sdk_registered_tool_carrier_binding,
    claude_agent_sdk_registered_tool_carrier_claim,
};
pub use wire::{
    ClaudeAgentSdkMcpMethod, MAXIMUM_MCP_RECORD_BYTES, MCP_ERROR_INTERNAL,
    MCP_ERROR_INVALID_PARAMS, MCP_ERROR_INVALID_REQUEST, MCP_ERROR_METHOD_NOT_FOUND,
    MCP_JSONRPC_VERSION,
};

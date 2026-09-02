//! Claude Agent SDK sidecar route: asset, identity, private wire, and driver.
//!
//! The `claude-agent.sdk` route drives the official TypeScript Claude Agent
//! SDK inside a host-owned Node sidecar over a private wire. It is a distinct
//! route: different package, wire, and version axes from `claude-agent.acp`,
//! and a different axis from the Claude Code routes, which drive the same
//! native binary directly. Nothing here inherits an ACP or Claude Code claim.
//!
//! Swallowtail never possesses the user's subscription credential. The
//! application provisions the Node runtime, the `.` SDK entry point, its peer
//! dependencies, and the platform package; the native binary authenticates
//! itself. This route observes typed readiness only.

mod activity;
mod addable;
mod asset;
mod bounded;
mod close;
mod connection;
mod driver;
mod failure;
mod permission;
mod prepared;
/// Bounded public decoder for qualified sidecar wire record shapes.
pub mod protocol;
mod selection;
mod turn;
pub(crate) mod wire;

pub use addable::{
    CLAUDE_AGENT_SDK_ADDABLE_ROUTE_ID, CLAUDE_AGENT_SDK_CREDENTIAL_FIELD_ID,
    CLAUDE_AGENT_SDK_ENVIRONMENT_FIELD_ID, CLAUDE_AGENT_SDK_LAUNCH_RECIPE_FIELD_ID,
    claude_agent_sdk_addable_route_descriptor,
};
pub use asset::{
    CLAUDE_AGENT_SDK_SIDECAR_ENTRY_FILE, CLAUDE_AGENT_SDK_SIDECAR_SOURCE,
    CLAUDE_AGENT_SDK_SIDECAR_SOURCE_TAG,
};
pub use driver::{ClaudeAgentSdkDriver, claude_agent_sdk_descriptor};
pub use permission::claude_agent_sdk_tool_admission_namespace;
pub use prepared::{
    ClaudeAgentSdkPreparedSession, ClaudeAgentSdkSessionPreparation,
    prepare_claude_agent_sdk_session,
};
pub use selection::{
    CLAUDE_AGENT_SDK_NATIVE_AXIS, CLAUDE_AGENT_SDK_NODE_AXIS, CLAUDE_AGENT_SDK_PACKAGE_AXIS,
    CLAUDE_AGENT_SDK_SIDECAR_AXIS, CLAUDE_AGENT_SDK_WIRE_AXIS, claude_agent_sdk_native_binding,
    claude_agent_sdk_native_claim, claude_agent_sdk_node_binding, claude_agent_sdk_node_claim,
    claude_agent_sdk_package_binding, claude_agent_sdk_package_claim,
    claude_agent_sdk_sidecar_binding, claude_agent_sdk_sidecar_claim,
    claude_agent_sdk_wire_binding, claude_agent_sdk_wire_claim,
};

/// Private strict LF-JSON wire between the driver and the sidecar.
pub const CLAUDE_AGENT_SDK_WIRE: &str = "swallowtail-claude-agent-sdk-jsonl-v1";
/// Frozen sidecar construction and projection behavior revision.
pub const CLAUDE_AGENT_SDK_BEHAVIOR: &str = "claude-agent.sdk-v1";
/// Exact upstream SDK package the sidecar loads through its `.` entry point.
pub const CLAUDE_AGENT_SDK_PACKAGE: &str = "@anthropic-ai/claude-agent-sdk";
/// Exact qualified SDK wrapper package version.
pub const CLAUDE_AGENT_SDK_VERSION: &str = "0.3.258";
/// Exact native binary version the shipped SDK manifest declares. The wrapper
/// and native axes are coupled but never equal, and neither transfers a
/// Claude Code qualification to this route.
pub const CLAUDE_AGENT_SDK_NATIVE_VERSION: &str = "2.1.258";
/// Exact approved Node runtime version satisfying the upstream `>=18.0.0`
/// requirement.
pub const CLAUDE_AGENT_SDK_NODE_RUNTIME: &str = "22.23.2";

/// Reports whether this route is supported on the running execution host
/// platform.
///
/// Contract 019 requires the launch recipe to prove descendant enrollment or
/// containment on every supported platform, and makes a platform where that
/// cannot be proved unsupported rather than best-effort. The host's retained
/// process-group owner exists on Unix only; on Windows the host terminates a
/// tree by request without retaining ownership of it, so a native descendant
/// that outlives the Node root cannot be proved gone. This route therefore
/// declares Windows unsupported instead of shipping an unprovable lifecycle.
#[must_use]
pub const fn claude_agent_sdk_platform_supported() -> bool {
    !cfg!(windows)
}

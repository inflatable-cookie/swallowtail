//! Source-tagged Node sidecar asset owned by this adapter crate.
//!
//! The consuming application provisions the entry point through a
//! host-approved launch recipe. Swallowtail ships the source but never
//! installs, vendors, mirrors, updates, repairs, or redistributes the Node
//! runtime, the SDK package, its peer dependencies, or the platform package
//! that carries the native binary.

/// Sidecar entry point file name used by the application launch recipe.
pub const CLAUDE_AGENT_SDK_SIDECAR_ENTRY_FILE: &str = "claude-agent-sdk-sidecar.mjs";

/// Complete sidecar source packaged with this adapter crate.
pub const CLAUDE_AGENT_SDK_SIDECAR_SOURCE: &str =
    include_str!("../../sidecar/claude-agent-sdk-sidecar.mjs");

/// Source tag identifying the adapter source revision that ships the sidecar.
pub const CLAUDE_AGENT_SDK_SIDECAR_SOURCE_TAG: &str = concat!(
    "swallowtail-claude-agent-sdk-sidecar@",
    env!("CARGO_PKG_VERSION")
);

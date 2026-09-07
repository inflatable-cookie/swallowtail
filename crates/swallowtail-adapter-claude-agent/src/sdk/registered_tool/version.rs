//! Carrier, SDK, and native version qualification for the registered profile.
//!
//! The registered-tool carrier adds one axis to this route's existing five. It
//! does not replace them: a carrier claim is admissible only alongside the
//! already qualified package, native, node, wire, and sidecar claims, so the
//! registered profile can never be qualified against an unqualified SDK
//! wrapper or native binary.
//!
//! The MCP protocol version is a separate axis from the Contract 063
//! registered-tool protocol version. Contract 063's version names the
//! Swallowtail selection segment; this one names the MCP handshake the pinned
//! bundle carries. Both are exact, explicitly enumerated subsets, and neither
//! is negotiated from `latest`.

use super::super::{
    CLAUDE_AGENT_SDK_BEHAVIOR, CLAUDE_AGENT_SDK_NATIVE_VERSION, CLAUDE_AGENT_SDK_VERSION,
};
use swallowtail_core::{
    InterfaceBehaviorRevision, InterfaceCompatibilityClaim, InterfaceCompatibilityClaimId,
    InterfaceNewerVersionPosture, InterfaceSupportStatus, InterfaceVersion, InterfaceVersionAxis,
    InterfaceVersionBinding, InterfaceVersionScheme,
};

/// Opaque axis for the Swallowtail-owned registered-tool MCP carrier.
pub const CLAUDE_AGENT_SDK_REGISTERED_TOOL_CARRIER_AXIS: &str =
    "claude-agent.sdk.registered-tool-carrier";

/// Exact opaque revision of this route's registered-tool carrier profile.
pub const CLAUDE_AGENT_SDK_REGISTERED_TOOL_CARRIER_REVISION: &str =
    "swallowtail-claude-agent-sdk-registered-tool-mcp-v1";

/// Exact MCP protocol version the carrier answers `initialize` with.
///
/// Recovered from the pinned `0.3.259` bundle's own MCP constants and frozen in
/// `tests/fixtures/claude-agent-sdk-0.3.259/mcp-protocol.json`.
pub const CLAUDE_AGENT_SDK_MCP_PROTOCOL_VERSION: &str = "2025-11-25";

/// Exact MCP protocol versions the pinned bundle accepts from a server.
///
/// The carrier answers with [`CLAUDE_AGENT_SDK_MCP_PROTOCOL_VERSION`] only. This
/// list is the client-side acceptance set the pinned artifact carries; it is
/// evidence about the client, never a second value the carrier may substitute.
pub const CLAUDE_AGENT_SDK_MCP_SUPPORTED_PROTOCOL_VERSIONS: &[&str] = &[
    "2025-11-25",
    "2025-06-18",
    "2025-03-26",
    "2024-11-05",
    "2024-10-07",
];

/// Exact SDK wrapper version this carrier profile was qualified against.
pub const CLAUDE_AGENT_SDK_REGISTERED_TOOL_SDK_VERSION: &str = CLAUDE_AGENT_SDK_VERSION;

/// Exact native binary version this carrier profile was qualified against.
pub const CLAUDE_AGENT_SDK_REGISTERED_TOOL_NATIVE_VERSION: &str = CLAUDE_AGENT_SDK_NATIVE_VERSION;

/// Binds the exact opaque registered-tool carrier revision.
#[must_use]
pub fn claude_agent_sdk_registered_tool_carrier_binding(
    value: &str,
) -> Option<InterfaceVersionBinding> {
    if value != CLAUDE_AGENT_SDK_REGISTERED_TOOL_CARRIER_REVISION {
        return None;
    }
    Some(InterfaceVersionBinding::new(
        InterfaceVersionAxis::new(CLAUDE_AGENT_SDK_REGISTERED_TOOL_CARRIER_AXIS)
            .expect("static SDK registered-tool axis is valid"),
        InterfaceVersion::new(CLAUDE_AGENT_SDK_REGISTERED_TOOL_CARRIER_REVISION).ok()?,
    ))
}

/// Returns the qualified-only one-point registered-tool carrier claim.
///
/// `QualifiedOnly` is deliberate: an unverified newer carrier revision is not
/// admissible, exactly as on this route's other five axes.
#[must_use]
pub fn claude_agent_sdk_registered_tool_carrier_claim() -> InterfaceCompatibilityClaim {
    InterfaceCompatibilityClaim::new(
        InterfaceCompatibilityClaimId::new("claude-agent.sdk.registered-tool-carrier-v1")
            .expect("static SDK registered-tool claim id is valid"),
        InterfaceVersionAxis::new(CLAUDE_AGENT_SDK_REGISTERED_TOOL_CARRIER_AXIS)
            .expect("static SDK registered-tool axis is valid"),
        InterfaceVersionScheme::Opaque,
        InterfaceNewerVersionPosture::QualifiedOnly,
        [swallowtail_core::InterfaceVersionSegment::exact(
            InterfaceVersion::new(CLAUDE_AGENT_SDK_REGISTERED_TOOL_CARRIER_REVISION)
                .expect("static SDK registered-tool revision is valid"),
            InterfaceBehaviorRevision::new(CLAUDE_AGENT_SDK_BEHAVIOR)
                .expect("static SDK sidecar behavior revision is valid"),
            InterfaceSupportStatus::Maintained,
        )],
        [],
    )
    .expect("static SDK registered-tool compatibility claim is valid")
}

/// Reports whether one negotiated MCP protocol version is admissible here.
///
/// The carrier admits exactly one value. A client offering another supported
/// version is answered with the exact carrier version, and a negotiated value
/// that is not the carrier version fails before any provider work.
#[must_use]
pub fn claude_agent_sdk_mcp_protocol_version_admitted(value: &str) -> bool {
    value == CLAUDE_AGENT_SDK_MCP_PROTOCOL_VERSION
}

/// Reports whether the pinned client bundle would accept this version at all.
#[must_use]
pub fn claude_agent_sdk_mcp_protocol_version_known(value: &str) -> bool {
    CLAUDE_AGENT_SDK_MCP_SUPPORTED_PROTOCOL_VERSIONS.contains(&value)
}

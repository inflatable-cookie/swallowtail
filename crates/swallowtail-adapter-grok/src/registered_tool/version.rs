//! Carrier version qualification for the registered profile.
//!
//! The registered-tool courier adds one axis to this route's existing
//! executable axis. It does not replace it: a carrier claim is admissible only
//! alongside the already qualified `grok-build.executable` claim, so the
//! registered profile can never be qualified against an unqualified Grok
//! build.
//!
//! The MCP protocol version below is the Contract 063 courier's own exact
//! value, recorded here so a consumer reads it beside the carrier revision. It
//! is not a Grok acceptance set: no frozen exact-route corpus states which MCP
//! protocol versions Grok's client offers or admits, so none is claimed.

use crate::selection::GROK_BUILD_ACP_BEHAVIOR;
use swallowtail_core::{
    InterfaceBehaviorRevision, InterfaceCompatibilityClaim, InterfaceCompatibilityClaimId,
    InterfaceNewerVersionPosture, InterfaceSupportStatus, InterfaceVersion, InterfaceVersionAxis,
    InterfaceVersionBinding, InterfaceVersionScheme, InterfaceVersionSegment,
};

/// Opaque axis for the Swallowtail-owned registered-tool courier.
pub const GROK_ACP_REGISTERED_TOOL_CARRIER_AXIS: &str = "grok-build.acp.registered-tool-carrier";

/// Exact opaque revision of this route's registered-tool carrier profile.
pub const GROK_ACP_REGISTERED_TOOL_CARRIER_REVISION: &str =
    "swallowtail-grok-build-acp-registered-tool-courier-v1";

/// Exact MCP protocol version the Contract 063 courier answers `initialize`
/// with on this route.
pub const GROK_ACP_REGISTERED_TOOL_MCP_PROTOCOL_VERSION: &str =
    swallowtail_host_local::wire::REGISTERED_TOOL_PROXY_MCP_PROTOCOL_VERSION;

/// Binds the exact opaque registered-tool carrier revision.
#[must_use]
pub fn grok_acp_registered_tool_carrier_binding(value: &str) -> Option<InterfaceVersionBinding> {
    if value != GROK_ACP_REGISTERED_TOOL_CARRIER_REVISION {
        return None;
    }
    Some(InterfaceVersionBinding::new(
        InterfaceVersionAxis::new(GROK_ACP_REGISTERED_TOOL_CARRIER_AXIS)
            .expect("static Grok registered-tool axis is valid"),
        InterfaceVersion::new(GROK_ACP_REGISTERED_TOOL_CARRIER_REVISION).ok()?,
    ))
}

/// Returns the qualified-only one-point registered-tool carrier claim.
///
/// `QualifiedOnly` is deliberate: an unverified newer carrier revision is not
/// admissible, exactly as on this route's executable axis.
#[must_use]
pub fn grok_acp_registered_tool_carrier_claim() -> InterfaceCompatibilityClaim {
    InterfaceCompatibilityClaim::new(
        InterfaceCompatibilityClaimId::new("grok-build.acp.registered-tool-carrier-v1")
            .expect("static Grok registered-tool claim id is valid"),
        InterfaceVersionAxis::new(GROK_ACP_REGISTERED_TOOL_CARRIER_AXIS)
            .expect("static Grok registered-tool axis is valid"),
        InterfaceVersionScheme::Opaque,
        InterfaceNewerVersionPosture::QualifiedOnly,
        [InterfaceVersionSegment::exact(
            InterfaceVersion::new(GROK_ACP_REGISTERED_TOOL_CARRIER_REVISION)
                .expect("static Grok registered-tool revision is valid"),
            InterfaceBehaviorRevision::new(GROK_BUILD_ACP_BEHAVIOR)
                .expect("static Grok ACP behavior revision is valid"),
            InterfaceSupportStatus::Maintained,
        )],
        [],
    )
    .expect("static Grok registered-tool compatibility claim is valid")
}

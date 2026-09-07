//! Contract 063 registered-tool seam for the `grok-build.acp` route.
//!
//! ACP v1 lets a client describe MCP servers at session setup, and the exact
//! Grok route was observed on 2026-09-07 to admit a non-empty `mcpServers`
//! declaration, spawn the described server, connect to it, and enumerate its
//! tools. That admission evidence is enough to build one callable route-local
//! seam; it is not enough to claim support, and nothing here publishes one.
//!
//! Exact bounds of this seam:
//!
//! - the declared server is Swallowtail-owned. It is the Contract 063
//!   mediated-stdio courier derived from an immutable selection, reserved
//!   under its own server name, and it is never a consumer-declared server;
//! - every admitted call is issued through the Contract 063 kernel by the
//!   courier, so consumer admission, correlation, deadlines, exactly-once
//!   settlement, and the honest execution disposition stay where the contract
//!   puts them. This route adds no second registry, lease, or dispatcher;
//! - the provider-owned ACP `session/update` tool activity and the one-shot
//!   `session/request_permission` exchange are separate channels. Neither is
//!   relabelled as a consumer registration, result, or progress record;
//! - tool identities stay namespaced, and the courier spelling is reversible,
//!   so one wire name resolves to exactly one identity or fails closed; and
//! - a route that opens without a registered preparation is byte-identical to
//!   the merged route: `session/new` still carries `mcpServers: []`.
//!
//! What this module does not claim: the registered capability is projected
//! `Unqualified` with `real_route_gate_pending`. The disposable real-route
//! gate is separately authorized and has not run, so no support disposition
//! and no matrix cell follows from deterministic evidence alone.

mod binding;
mod carrier;
mod declaration;
mod open;
mod projection;
mod version;

pub use binding::GrokRegisteredToolBinding;
pub use carrier::{
    GROK_ACP_REGISTERED_TOOL_MEDIATION, GROK_ACP_REGISTERED_TOOL_SERVER, GrokRegisteredTool,
    GrokRegisteredToolCarrier,
};
pub use declaration::GrokAcpMcpServerDeclaration;
pub(crate) use open::{GrokRegisteredToolSession, PendingRegisteredOpen, prepare_registered};
pub use projection::{
    GROK_ACP_MEDIATION_KIND_SEMANTIC_ID, GROK_ACP_REAL_ROUTE_GATE_PENDING_CODE,
    GROK_ACP_REGISTERED_TOOL_SOURCE, grok_build_acp_registered_tool_qualification,
    project_grok_build_acp_registered_tool,
};
pub use version::{
    GROK_ACP_REGISTERED_TOOL_CARRIER_AXIS, GROK_ACP_REGISTERED_TOOL_CARRIER_REVISION,
    GROK_ACP_REGISTERED_TOOL_MCP_PROTOCOL_VERSION, grok_acp_registered_tool_carrier_binding,
    grok_acp_registered_tool_carrier_claim,
};

use swallowtail_core::{Diagnostic, SafeDiagnostic};
use swallowtail_runtime::{PreparationFailure, PreparationStage};

fn registered_preparation_failure(code: &'static str, message: &'static str) -> PreparationFailure {
    PreparationFailure::new(
        PreparationStage::Preflight,
        Diagnostic::new(SafeDiagnostic::new(code, message)),
    )
}

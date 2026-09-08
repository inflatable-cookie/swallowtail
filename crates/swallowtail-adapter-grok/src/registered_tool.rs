//! Contract 063 registered-tool seam for the `grok-build.acp` route.
//!
//! ACP v1 lets a client describe MCP servers at session setup, and the exact
//! Grok route was observed on 2026-09-07 to admit a non-empty `mcpServers`
//! declaration, spawn the described server, connect to it, and enumerate its
//! tools. The accepted Card 128 live gate then ran one real registered call
//! through this exact courier on each maintained segment — Grok Build `1.0.4`
//! and `1.0.5` (Research 295) — so the route projects
//! [`RegisteredToolRouteQualification::Qualified`] with the dimensions the
//! capsules proved.
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
//!   relabelled as a consumer registration, result, or progress record. A
//!   consumer Deny is not represented to the provider, and the route delivers
//!   no consumer tool progress;
//! - tool identities stay namespaced, and the courier spelling is reversible,
//!   so one wire name resolves to exactly one identity or fails closed; and
//! - a route that opens without a registered preparation is byte-identical to
//!   the merged route: `session/new` still carries `mcpServers: []`.
//!
//! What this module does not claim: selected-skill delivery. Frozen ACP v1
//! session setup has no session-scoped, distinctly labelled skill input, the
//! frozen Grok artifacts name none, and the accepted capsules carried none, so
//! the capability projects `NotCarried` and the matrix cell records the exact
//! route limitation.

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
    GROK_ACP_MEDIATION_KIND_SEMANTIC_ID, GROK_ACP_REGISTERED_TOOL_ROUTE,
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

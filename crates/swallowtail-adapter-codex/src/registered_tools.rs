//! Contract 063 registered-tool binding for Codex dynamic native tools.
//!
//! Codex app-server's qualified execution seam for consumer tools is its
//! dynamic native tool declaration plus the `item/tool/call` server request.
//! This module binds exactly that seam to the common registration, admission,
//! dispatch, and result lifecycle. It adds no provider-direct MCP attachment:
//! a registered MCP tool is refused before any provider work, because the
//! app-server surface and corpus that would qualify it do not exist yet.
//!
//! Nothing here is enabled by default. A prepared Codex session without a
//! registered-tool preparation keeps every previous behavior, declares the same
//! parameters, and never opens a lease.

#[path = "registered_tools/binding.rs"]
mod binding;
#[path = "registered_tools/projection.rs"]
mod projection;
#[path = "registered_tools/result.rs"]
mod result;
#[path = "registered_tools/runtime.rs"]
mod runtime;
#[path = "registered_tools/turn.rs"]
mod turn;

pub use binding::{
    CODEX_REGISTERED_TOOL_NAME_SEPARATOR, CODEX_REGISTERED_TOOL_ROUTE, CodexRegisteredToolBinding,
};
pub(crate) use runtime::CodexRegisteredToolRuntime;
pub(crate) use turn::CodexRegisteredTurn;

pub(crate) use projection::registered_capability_contribution;

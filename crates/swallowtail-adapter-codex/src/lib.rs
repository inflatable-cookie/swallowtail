//! Codex harness drivers for Swallowtail.

#![forbid(unsafe_code)]

mod app_server;
mod callback_exchange;
mod exec;
mod exec_events;
mod exec_handle;
mod exec_input;
mod exec_pump;
mod exec_validation;
mod rpc;
mod session;
mod session_access;
mod session_input;
mod session_open;
mod turn_state;

pub use app_server::{CodexAppServerDriver, codex_app_server_descriptor};
pub use exec::{CodexExecDriver, codex_exec_descriptor};
pub use session_access::{
    codex_approval_request_extension, codex_bounded_workspace_access_policy,
    codex_bounded_workspace_capability, codex_provider_request_extensions,
    codex_user_input_request_extension,
};

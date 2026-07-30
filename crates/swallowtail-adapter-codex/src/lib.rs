//! Codex harness drivers for Swallowtail.

#![forbid(unsafe_code)]

mod app_server;
mod app_server_activity;
mod callback_exchange;
mod discovery;
mod exec;
mod exec_activity;
mod exec_events;
mod exec_handle;
mod exec_input;
mod exec_pump;
mod exec_validation;
mod prepared;
mod prepared_profile;
mod rpc;
mod selection;
mod session;
mod session_access;
mod session_input;
mod session_management;
mod session_open;
mod session_replay;
mod turn_state;
mod user_input;

pub use app_server::{CodexAppServerDriver, codex_app_server_descriptor};
pub use exec::{CodexExecDriver, codex_exec_descriptor};
pub use prepared::{
    CodexPreparationInput, CodexPreparationProbe, CodexPreparedDriver, CodexPreparedIntegration,
    prepare_codex,
};
pub use prepared_profile::{
    CodexExecProfileInput, CodexModelSelection, CodexPreparedArchive, CodexPreparedCatalogue,
    CodexPreparedDelete, CodexPreparedEvidence, CodexPreparedExec, CodexPreparedRestore,
    CodexPreparedSession, CodexPreparedSessionFuture, CodexPreparedSessionKind,
    CodexPreparedSessionLoadFuture, CodexSessionManagementInput, CodexSessionProfileInput,
};
pub use selection::{
    CODEX_APP_SERVER_BASELINE_VERSION, CODEX_CLI_AXIS, CODEX_EXEC_BASELINE_VERSION,
    CODEX_LATEST_QUALIFIED_VERSION, codex_app_server_claim, codex_app_server_lifecycle_claim,
    codex_cli_binding, codex_exec_claim,
};
pub use session_access::{
    codex_approval_request_extension, codex_bounded_workspace_access_policy,
    codex_bounded_workspace_capability, codex_provider_request_extensions,
    codex_user_input_request_extension,
};

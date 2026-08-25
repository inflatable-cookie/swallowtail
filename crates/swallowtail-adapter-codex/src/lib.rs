//! Codex harness drivers for Swallowtail.

#![forbid(unsafe_code)]
#![deny(missing_docs)]

mod access;
mod addable;
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
mod model_verbosity;
mod prepared;
mod prepared_profile;
mod rpc;
mod safe_excerpt;
mod selection;
mod session;
mod session_access;
mod session_input;
mod session_management;
mod session_open;
mod session_replay;
mod thread_catalogue;
mod turn_state;
mod user_input;

pub use access::{CODEX_CHATGPT_SUBSCRIPTION_AUDIENCE, codex_chatgpt_subscription_access_profile};
pub use addable::{
    CODEX_APP_SERVER_ADDABLE_ROUTE_ID, CODEX_APP_SERVER_BINARY_PATH_FIELD_ID,
    CODEX_APP_SERVER_ENVIRONMENT_FIELD_ID, codex_app_server_addable_route_descriptor,
};
pub use app_server::{CodexAppServerDriver, codex_app_server_descriptor};
pub use exec::{CodexExecDriver, codex_exec_descriptor};
pub use model_verbosity::CodexModelVerbosity;
pub use prepared::{
    CodexPreparationInput, CodexPreparationProbe, CodexPreparedDriver, CodexPreparedIntegration,
    prepare_codex,
};
pub use prepared_profile::{
    CodexExecProfileInput, CodexModelSelection, CodexPreparedArchive, CodexPreparedCatalogue,
    CodexPreparedDelete, CodexPreparedEvidence, CodexPreparedExec, CodexPreparedRestore,
    CodexPreparedSession, CodexPreparedSessionCatalogue, CodexPreparedSessionFuture,
    CodexPreparedSessionHistory, CodexPreparedSessionImport, CodexPreparedSessionKind,
    CodexPreparedSessionLoadFuture, CodexPreparedSessionReconciliation, CodexSessionCatalogueInput,
    CodexSessionHistoryInput, CodexSessionManagementInput, CodexSessionProfileInput,
    CodexSessionReconciliationInput,
};
pub use selection::{
    CODEX_APP_SERVER_BASELINE_VERSION, CODEX_APP_SERVER_THREAD_CATALOGUE_BASELINE_VERSION,
    CODEX_CLI_AXIS, CODEX_EXEC_BASELINE_VERSION, CODEX_LATEST_QUALIFIED_VERSION,
    codex_app_server_claim, codex_app_server_lifecycle_claim, codex_cli_binding, codex_exec_claim,
};
pub use session_access::{
    codex_approval_request_extension, codex_bounded_workspace_access_policy,
    codex_bounded_workspace_capability, codex_provider_request_extensions,
    codex_user_input_request_extension,
};

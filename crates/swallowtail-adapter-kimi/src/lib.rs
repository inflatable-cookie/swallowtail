//! Kimi Code ACP integration for Swallowtail.

#![forbid(unsafe_code)]
#![deny(missing_docs)]

mod acp_activity;
mod connection;
mod consumer_route_projection;
mod discovery;
mod driver;
mod failure;
mod headless;
mod headless_activity;
mod headless_command;
mod headless_events;
mod headless_handle;
mod headless_pump;
mod headless_validation;
mod local_server;
mod prepared;
mod prepared_code;
mod prepared_headless;
mod prepared_profile;
mod selection;
mod turn;

pub use consumer_route_projection::{
    KimiProjectionOpenFailure, KimiProjectionOpenFuture, KimiProjectionOpenOutcome,
    KimiProviderValue,
};
pub use driver::{KimiAcpDriver, kimi_acp_descriptor};
pub use headless::{KimiHeadlessDriver, kimi_headless_descriptor};
pub use local_server::{
    KIMI_LOCAL_SERVER_BASELINE_VERSION, KIMI_LOCAL_SERVER_LATEST_QUALIFIED_VERSION,
    KimiLocalServerAttachedInput, KimiLocalServerBindingImportInput,
    KimiLocalServerBindingImportTarget, KimiLocalServerCatalogueInput, KimiLocalServerDriver,
    KimiLocalServerObservation, KimiLocalServerOwnedHandle, KimiLocalServerOwnedInput,
    KimiLocalServerPermissionMode, KimiLocalServerPreparationProbe, KimiLocalServerPreparedArchive,
    KimiLocalServerPreparedBindingImport, KimiLocalServerPreparedCatalogue,
    KimiLocalServerPreparedIntegration, KimiLocalServerPreparedReconciliation,
    KimiLocalServerPreparedRestore, KimiLocalServerPreparedRun, KimiLocalServerPreparedSession,
    KimiLocalServerPreparedSessionFuture, KimiLocalServerReconciliationInput,
    KimiLocalServerRunInput, KimiLocalServerSessionConfiguration, KimiLocalServerSessionInput,
    KimiLocalServerSessionManagementInput, kimi_local_server_claim, kimi_local_server_descriptor,
    prepare_kimi_local_server_attached, start_kimi_local_server_owned,
};
pub use prepared::{
    KimiPreparationInput, KimiPreparationProbe, KimiPreparedIntegration, prepare_kimi,
};
pub use prepared_code::{
    KimiCodePreparationInput, KimiCodePreparationProbe, KimiCodePreparedDriver,
    KimiCodePreparedIntegration, prepare_kimi_code,
};
pub use prepared_headless::{
    KimiHeadlessPreparationInput, KimiHeadlessPreparationProbe, KimiHeadlessPreparedEvidence,
    KimiHeadlessPreparedIntegration, KimiHeadlessPreparedRun, KimiHeadlessRunInput,
    prepare_kimi_headless,
};
pub use prepared_profile::{
    KimiAcpSessionImportAuthority, KimiModelSelection, KimiPreparedEvidence, KimiPreparedSession,
    KimiPreparedSessionCatalogue, KimiPreparedSessionFuture, KimiPreparedSessionImport,
    KimiPreparedSessionLoadFuture, KimiSessionCatalogueInput, KimiSessionProfileInput,
};
pub use selection::{
    KIMI_CODE_AXIS, KIMI_CODE_BASELINE_VERSION, KIMI_CODE_LATEST_QUALIFIED_VERSION,
    KIMI_HEADLESS_BASELINE_VERSION, KIMI_HEADLESS_LATEST_QUALIFIED_VERSION, kimi_acp_claim,
    kimi_code_binding, kimi_headless_claim,
};

const MAXIMUM_REPLAY_ITEMS: usize = 512;
const MAXIMUM_REPLAY_BYTES: usize = 4 * 1024 * 1024;
const MAXIMUM_WRITE_BYTES: usize = 1024 * 1024;

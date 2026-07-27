//! Kimi Code ACP integration for Swallowtail.

#![forbid(unsafe_code)]

mod connection;
mod discovery;
mod driver;
mod failure;
mod local_server;
mod prepared;
mod prepared_profile;
mod selection;
mod turn;

pub use driver::{KimiAcpDriver, kimi_acp_descriptor};
pub use local_server::{
    KIMI_LOCAL_SERVER_BASELINE_VERSION, KIMI_LOCAL_SERVER_LATEST_QUALIFIED_VERSION,
    KimiLocalServerAttachedInput, KimiLocalServerBindingImportInput,
    KimiLocalServerBindingImportTarget, KimiLocalServerCatalogueInput, KimiLocalServerDriver,
    KimiLocalServerObservation, KimiLocalServerOwnedHandle, KimiLocalServerOwnedInput,
    KimiLocalServerPermissionMode, KimiLocalServerPreparationProbe, KimiLocalServerPreparedArchive,
    KimiLocalServerPreparedBindingImport, KimiLocalServerPreparedCatalogue,
    KimiLocalServerPreparedIntegration, KimiLocalServerPreparedRestore,
    KimiLocalServerPreparedSession, KimiLocalServerPreparedSessionFuture,
    KimiLocalServerSessionConfiguration, KimiLocalServerSessionInput,
    KimiLocalServerSessionManagementInput, kimi_local_server_claim, kimi_local_server_descriptor,
    prepare_kimi_local_server_attached, start_kimi_local_server_owned,
};
pub use prepared::{
    KimiPreparationInput, KimiPreparationProbe, KimiPreparedIntegration, prepare_kimi,
};
pub use prepared_profile::{
    KimiAcpSessionImportAuthority, KimiModelSelection, KimiPreparedEvidence, KimiPreparedSession,
    KimiPreparedSessionFuture, KimiPreparedSessionLoadFuture, KimiSessionProfileInput,
};
pub use selection::{
    KIMI_CODE_AXIS, KIMI_CODE_BASELINE_VERSION, KIMI_CODE_LATEST_QUALIFIED_VERSION, kimi_acp_claim,
    kimi_code_binding,
};

const MAXIMUM_REPLAY_ITEMS: usize = 512;
const MAXIMUM_REPLAY_BYTES: usize = 4 * 1024 * 1024;
const MAXIMUM_WRITE_BYTES: usize = 1024 * 1024;

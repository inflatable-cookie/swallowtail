//! Exact installed ZCode app-server integration for Swallowtail.
//!
//! The surface owns one `node` + `zcode.cjs app-server` child and projects one
//! bounded structured run. It does not wrap the TUI, desktop GUI, `--print`,
//! community ACP, or OpenCode.

#![forbid(unsafe_code)]
#![deny(missing_docs)]

mod access;
mod activity;
mod command;
mod discovery;
mod driver;
mod failure;
mod handle;
mod mode;
mod prepared;
mod protocol;
mod pump;
mod selection;
mod validation;

pub use access::{ZCODE_CONFIG_AUDIENCE, zcode_access_profile};
pub use driver::{ZcodeAppServerDriver, zcode_app_server_descriptor};
pub use mode::ZcodeAppServerMode;
pub use prepared::{
    ZcodeModelSelection, ZcodePreparationInput, ZcodePreparationProbe, ZcodePreparedEvidence,
    ZcodePreparedIntegration, ZcodePreparedRun, ZcodeRunProfileInput, prepare_zcode_app_server,
};
pub use selection::{
    ZCODE_EXECUTABLE_BASENAME, ZCODE_LAUNCHER_SHA256, ZCODE_PAYLOAD_SHA256, ZCODE_RELEASE_AXIS,
    ZCODE_RELEASE_VERSION, zcode_app_server_claim, zcode_release_binding,
};

pub(crate) const DRIVER_ID: &str = "swallowtail.zcode.app-server";

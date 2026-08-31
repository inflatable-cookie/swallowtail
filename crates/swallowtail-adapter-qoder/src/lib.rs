//! Qoder headless harness driver.
//!
//! `qoder.headless` binds host-approved `qodercli --print --output-format stream-json`
//! for one bounded print run through `prepare_qoder_headless`. `--acp`, SDK
//! stdio, TUI, and `--yolo` / `bypass_permissions` stay out.

#![forbid(unsafe_code)]
#![deny(missing_docs)]

mod access;
mod command;
mod consumer_route_projection;
mod discovery;
mod driver;
mod failure;
mod prepared;
mod selection;

pub use access::{QODER_LOCAL_ACCOUNT_AUDIENCE, qoder_local_config_access_profile};
pub use driver::{QoderHeadlessDriver, qoder_headless_descriptor};
pub use prepared::{
    QoderHeadlessPreparationInput, QoderHeadlessPreparationProbe, QoderHeadlessPreparedIntegration,
    QoderHeadlessPreparedRun, QoderHeadlessRunProfileInput, prepare_qoder_headless,
};
pub use selection::{
    QODER_EXECUTABLE_NAME, QODER_PACKAGE_AXIS, QODER_PACKAGE_VERSION, qoder_headless_claim,
    qoder_package_binding,
};

//! Exact installed DeepSeek Harness integrations for Swallowtail.
//!
//! The JSON-RPC surface owns one bundled stdio process and projects one
//! bounded structured run. The separate Web surface owns one loopback `dsh
//! web` child and exposes its prepared structured-run and provider-session
//! operations. Neither surface wraps the Python SDK or retains sessions; the
//! JSON-RPC surface also has no native cancellation method.

#![forbid(unsafe_code)]
#![deny(missing_docs)]

mod access;
mod activity;
mod discovery;
mod driver;
mod failure;
mod handle;
mod prepared;
mod protocol;
mod pump;
mod selection;
mod validation;
mod web;
mod web_prepared;

pub use access::{DEEPSEEK_HARNESS_CONFIG_AUDIENCE, deepseek_harness_access_profile};
pub use driver::{DeepSeekHarnessJsonRpcDriver, deepseek_harness_jsonrpc_descriptor};
pub use prepared::{
    DeepSeekHarnessModelSelection, DeepSeekHarnessPreparationInput,
    DeepSeekHarnessPreparationProbe, DeepSeekHarnessPreparedEvidence,
    DeepSeekHarnessPreparedIntegration, DeepSeekHarnessPreparedRun, DeepSeekHarnessRunProfileInput,
    prepare_deepseek_harness_jsonrpc,
};
pub use selection::{
    DEEPSEEK_HARNESS_EXECUTABLE_BASENAME, DEEPSEEK_HARNESS_PAYLOAD_SHA256,
    DEEPSEEK_HARNESS_RELEASE_AXIS, DEEPSEEK_HARNESS_RELEASE_VERSION,
    DEEPSEEK_HARNESS_SPAWN_HELPER_SHA256, deepseek_harness_jsonrpc_claim,
    deepseek_harness_release_binding,
};
pub use web::{
    DEEPSEEK_HARNESS_WEB_EXECUTABLE_BASENAME, DEEPSEEK_HARNESS_WEB_RELEASE_AXIS,
    DEEPSEEK_HARNESS_WEB_RELEASE_VERSION, DeepSeekHarnessWebDriver, DeepSeekHarnessWebModel,
    deepseek_harness_web_claim, deepseek_harness_web_descriptor,
};
pub use web_prepared::{
    DeepSeekHarnessWebForkInput, DeepSeekHarnessWebModelSelection,
    DeepSeekHarnessWebPreparationInput, DeepSeekHarnessWebPreparationProbe,
    DeepSeekHarnessWebPreparedArchive, DeepSeekHarnessWebPreparedEvidence,
    DeepSeekHarnessWebPreparedFork, DeepSeekHarnessWebPreparedIntegration,
    DeepSeekHarnessWebPreparedRun, DeepSeekHarnessWebPreparedSessionCatalogue,
    DeepSeekHarnessWebPreparedSessionHistory, DeepSeekHarnessWebRunProfileInput,
    DeepSeekHarnessWebSessionCatalogueInput, DeepSeekHarnessWebSessionHistoryInput,
    DeepSeekHarnessWebSessionManagementInput, prepare_deepseek_harness_web,
};

pub(crate) const DRIVER_ID: &str = "swallowtail.deepseek-harness.jsonrpc";

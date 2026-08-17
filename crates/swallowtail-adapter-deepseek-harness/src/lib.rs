//! Exact installed DeepSeek Harness JSON-RPC integration for Swallowtail.
//!
//! The first surface owns one bundled JSON-RPC stdio process and projects one
//! bounded structured run. It does not wrap the Python SDK, expose ACP/Web
//! routes, retain sessions, or claim a native cancellation method.

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

pub(crate) const DRIVER_ID: &str = "swallowtail.deepseek-harness.jsonrpc";

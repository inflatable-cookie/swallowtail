//! Qwen Code harness integration for Swallowtail.
//!
//! The production driver implements the maintained read-only headless subset.

#![forbid(unsafe_code)]

mod activity;
mod catalogue;
mod command;
mod discovery;
mod driver;
mod events;
mod handle;
mod prepared;
mod prepared_catalogue;
mod prepared_profile;
mod pump;
mod selection;
mod session;
mod validation;

pub use driver::{QwenHeadlessDriver, qwen_headless_descriptor};
pub use prepared::{
    QwenPreparationInput, QwenPreparationProbe, QwenPreparedIntegration, prepare_qwen_headless,
};
pub use prepared_catalogue::{
    QwenCatalogueProfileInput, QwenPreparedCatalogue, prepare_qwen_catalogue,
};
pub use prepared_profile::{
    QwenModelSelection, QwenPreparedEvidence, QwenPreparedRun, QwenPreparedSession,
    QwenRunProfileInput, QwenSessionProfileInput,
};
pub use selection::{
    QWEN_CODE_AXIS, QWEN_CODE_BASELINE_VERSION, QWEN_CODE_LATEST_QUALIFIED_VERSION,
    qwen_code_binding, qwen_headless_claim,
};

pub const PINNED_QWEN_CODE_VERSION: &str = "0.19.11";
pub const PINNED_QWEN_CODE_COMMIT: &str = "f22cf5009ee3eb26b5c5de2eca6e1f1d0ffee0ad";

const DRIVER_ID: &str = "swallowtail.qwen.headless";

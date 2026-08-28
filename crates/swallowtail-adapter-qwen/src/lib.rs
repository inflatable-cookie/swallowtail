//! Qwen Code harness integration for Swallowtail.
//!
//! Installed discovery, model catalogue, one-shot structured runs, and
//! turn-scoped interactive sessions share one explicit headless CLI route.
//! Exact `0.21.15`, `0.22.0`, `0.22.1`, `0.22.2`, and `0.22.3` admit optional portable
//! `HarnessMode::Plan` as `--approval-mode plan`.

#![forbid(unsafe_code)]
#![deny(missing_docs)]

mod activity;
mod budgets;
mod catalogue;
mod command;
mod control;
mod discovery;
mod driver;
mod events;
mod handle;
mod plan_mode;
mod prepared;
mod prepared_catalogue;
mod prepared_profile;
mod pump;
mod reasoning;
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
    QwenHeadlessBudgets, QwenModelSelection, QwenPreparedEvidence, QwenPreparedRun,
    QwenPreparedSession, QwenRunProfileInput, QwenSessionProfileInput, QwenSessionTurnBudget,
    QwenToolCallBudget,
};
pub use selection::{
    QWEN_CODE_AXIS, QWEN_CODE_BASELINE_VERSION, QWEN_CODE_LATEST_QUALIFIED_VERSION,
    qwen_code_binding, qwen_headless_claim,
};

/// Historical baseline Qwen Code version used by the original fixture corpus.
pub const PINNED_QWEN_CODE_VERSION: &str = "0.19.11";
/// Source commit corresponding to [`PINNED_QWEN_CODE_VERSION`].
pub const PINNED_QWEN_CODE_COMMIT: &str = "f22cf5009ee3eb26b5c5de2eca6e1f1d0ffee0ad";

const DRIVER_ID: &str = "swallowtail.qwen.headless";

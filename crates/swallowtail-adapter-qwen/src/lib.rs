//! Qwen Code harness integration for Swallowtail.
//!
//! The production driver implements the pinned read-only headless subset.

#![forbid(unsafe_code)]

mod command;
mod driver;
mod events;
mod handle;
mod pump;
mod validation;

pub use driver::{QwenHeadlessDriver, qwen_headless_descriptor};

pub const PINNED_QWEN_CODE_VERSION: &str = "0.19.11";
pub const PINNED_QWEN_CODE_COMMIT: &str = "f22cf5009ee3eb26b5c5de2eca6e1f1d0ffee0ad";

const DRIVER_ID: &str = "swallowtail.qwen.headless";

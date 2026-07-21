//! Attached OpenCode HTTP harness integration for Swallowtail.

#![forbid(unsafe_code)]

mod driver;
mod failure;
mod protocol;
mod transport;

pub use driver::{OpenCodeHttpDriver, opencode_http_descriptor};

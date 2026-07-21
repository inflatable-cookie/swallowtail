//! Gemini CLI ACP integration for Swallowtail.

#![forbid(unsafe_code)]

mod connection;
mod driver;
mod failure;
mod turn;

pub use driver::{GeminiAcpDriver, gemini_acp_descriptor};

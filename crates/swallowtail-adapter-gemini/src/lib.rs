//! Gemini CLI ACP integration for Swallowtail.

#![forbid(unsafe_code)]

mod connection;
mod driver;
mod failure;
mod live;
mod live_protocol;
mod turn;

pub use driver::{GeminiAcpDriver, gemini_acp_descriptor};
pub use live::{GeminiLiveDriver, gemini_live_descriptor};

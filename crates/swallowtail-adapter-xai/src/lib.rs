//! xAI direct-inference drivers for Swallowtail.
//!
//! The adapter owns a serial, resource-free Responses WebSocket session. It
//! does not enable provider storage, reconnect, retry, or durable resume.

#![forbid(unsafe_code)]

mod driver;
mod failure;
mod protocol;
mod transport;

pub use driver::{XaiWebSocketDriver, xai_websocket_descriptor};

/// Provider-supported Responses WebSocket route frozen by the first fixture.
pub const RESPONSES_WEBSOCKET_PATH: &str = "/v1/responses";

/// Exact scale used by xAI's provider-authored billed-cost observation.
pub const USD_TICKS_PER_USD: u64 = 10_000_000_000;

//! Kimi Code ACP integration for Swallowtail.

#![forbid(unsafe_code)]

mod connection;
mod driver;
mod failure;
mod turn;

pub use driver::{KimiAcpDriver, kimi_acp_descriptor};

const MAXIMUM_REPLAY_ITEMS: usize = 512;
const MAXIMUM_REPLAY_BYTES: usize = 4 * 1024 * 1024;
const MAXIMUM_WRITE_BYTES: usize = 1024 * 1024;

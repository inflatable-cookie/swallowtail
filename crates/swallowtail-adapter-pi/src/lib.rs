//! Pi RPC harness integration for Swallowtail.

#![forbid(unsafe_code)]

mod callback;
mod connection;
mod driver;
mod failure;
pub mod protocol;
mod turn;

pub use driver::{PiRpcDriver, pi_rpc_descriptor};

pub const PINNED_PI_VERSION: &str = "0.80.10";

const DRIVER_ID: &str = "swallowtail.pi.rpc";

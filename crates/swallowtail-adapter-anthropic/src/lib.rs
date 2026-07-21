//! Anthropic direct-inference drivers for Swallowtail.
//!
#![forbid(unsafe_code)]

mod driver;
mod failure;
mod protocol;
mod transport;

pub use driver::{AnthropicDirectDriver, anthropic_direct_descriptor};

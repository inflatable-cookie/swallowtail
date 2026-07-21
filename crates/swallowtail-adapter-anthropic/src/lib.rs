//! Anthropic integration drivers for Swallowtail.
//!
#![forbid(unsafe_code)]

mod driver;
mod failure;
mod managed;
mod managed_driver;
mod managed_transport;
mod protocol;
mod transport;

pub use driver::{AnthropicDirectDriver, anthropic_direct_descriptor};
pub use managed_driver::{AnthropicManagedAgentDriver, anthropic_managed_agent_descriptor};

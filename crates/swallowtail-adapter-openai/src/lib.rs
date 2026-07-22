//! OpenAI public-API drivers for Swallowtail.
//!
//! The first fixture boundary covers provider-managed Responses background
//! runs. Production network behavior is added only after this corpus passes.

#![forbid(unsafe_code)]

mod driver;
mod failure;
mod protocol;
#[cfg(test)]
mod realtime_protocol;
mod transport;

pub use driver::{OpenAiBackgroundDriver, openai_background_descriptor};

pub(crate) const ENDPOINT_AUDIENCE: &str = "api.openai.com";
pub(crate) const INTEGRATION_FAMILY: &str = "openai";
#[cfg(test)]
pub(crate) const SUPPORT_AUTHORITY: &str = "provider-supported-public-api";

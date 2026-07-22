//! Kimi Platform direct-inference driver for Swallowtail.

#![forbid(unsafe_code)]

mod driver;
mod failure;
mod protocol;
mod transport;

pub use driver::{KimiPlatformDirectDriver, kimi_platform_direct_descriptor};

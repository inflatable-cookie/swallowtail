//! Source-tagged Node sidecar asset owned by this adapter crate.
//!
//! The consuming application provisions the entry point through a
//! host-approved launch recipe. Swallowtail ships the source but never
//! installs, discovers, repairs, or mutates the application's Node runtime or
//! SDK dependency state.

/// Sidecar entry point file name used by the application launch recipe.
pub const PI_SDK_SIDECAR_ENTRY_FILE: &str = "pi-sdk-sidecar.mjs";

/// Complete sidecar source packaged with this adapter crate.
pub const PI_SDK_SIDECAR_SOURCE: &str = include_str!("../../sidecar/pi-sdk-sidecar.mjs");

/// Source tag identifying the adapter source revision that ships the sidecar.
pub const PI_SDK_SIDECAR_SOURCE_TAG: &str =
    concat!("swallowtail-pi-sdk-sidecar@", env!("CARGO_PKG_VERSION"));

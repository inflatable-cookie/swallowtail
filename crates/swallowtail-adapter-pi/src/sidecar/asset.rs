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

/// Source tag identifying the exact sidecar asset content.
pub const PI_SDK_SIDECAR_SOURCE_TAG: &str =
    "swallowtail-pi-sdk-sidecar@sha256-b4a87c838d8884813d90eb4dac7306f3c571aef7892ee112a4f448ebfd463415";

/// Previously qualified source tag, which remains paired only with SDK 0.84.2.
pub(crate) const PI_SDK_SIDECAR_PREVIOUS_SOURCE_TAG: &str = "swallowtail-pi-sdk-sidecar@0.3.3";

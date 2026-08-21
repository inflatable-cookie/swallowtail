//! Pi SDK sidecar asset and private wire identity.
//!
//! The `pi.sdk-sidecar` route runs Pi's official TypeScript SDK inside a
//! host-owned Node sidecar. This module keeps the sidecar source asset, the
//! exact SDK package and Node runtime points, and the private wire identity
//! separate from one another and from the RPC route.

mod activity;
mod asset;
mod catalogue;
mod connection;
mod driver;
mod failure;
/// Bounded public decoder for qualified sidecar wire record shapes.
pub mod protocol;
mod selection;
mod turn;
pub(crate) mod wire;

pub use asset::{PI_SDK_SIDECAR_ENTRY_FILE, PI_SDK_SIDECAR_SOURCE, PI_SDK_SIDECAR_SOURCE_TAG};
pub use driver::{PiSdkSidecarDriver, pi_sdk_sidecar_descriptor};
pub use selection::{
    PI_SDK_SIDECAR_NODE_AXIS, PI_SDK_SIDECAR_PACKAGE_AXIS, PI_SDK_SIDECAR_SIDECAR_AXIS,
    PI_SDK_SIDECAR_WIRE_AXIS, pi_sdk_sidecar_node_binding, pi_sdk_sidecar_node_claim,
    pi_sdk_sidecar_package_binding, pi_sdk_sidecar_package_claim, pi_sdk_sidecar_sidecar_binding,
    pi_sdk_sidecar_sidecar_claim, pi_sdk_sidecar_wire_binding, pi_sdk_sidecar_wire_claim,
};

/// Private strict LF-JSON wire between the driver and the sidecar.
pub const PI_SDK_SIDECAR_WIRE: &str = "swallowtail-pi-sdk-jsonl-v1";
/// Frozen sidecar construction and projection behavior revision.
pub const PI_SDK_SIDECAR_BEHAVIOR: &str = "pi.sdk-sidecar-v1";
/// Exact upstream SDK package the sidecar loads.
pub const PI_SDK_SIDECAR_SDK_PACKAGE: &str = "@earendil-works/pi-coding-agent";
/// Exact qualified SDK package version.
pub const PI_SDK_SIDECAR_SDK_VERSION: &str = "0.84.2";
/// Exact approved Node runtime version satisfying the upstream `>=22.19.0`
/// requirement.
pub const PI_SDK_SIDECAR_NODE_RUNTIME: &str = "22.23.2";

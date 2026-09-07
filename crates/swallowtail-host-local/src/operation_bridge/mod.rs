//! One private-operation bridge kernel shared by both closed profiles.
//!
//! Contract 060's watcher profile and Contract 063's registered-tool profile
//! share one [`OperationBridgeRegistry`]: one monotonic generation space, one
//! per-turn ownership map, one live-lease registry, and one joined teardown
//! sequence. Neither profile owns a second lease manager, generation counter,
//! secret source, or lifecycle authority.

mod join;
mod listener;
mod registry;
mod secret;

pub(crate) use join::join_within;
pub(crate) use listener::{
    OperationBridgeFrame, OperationBridgeListener, OperationBridgeResponse, OperationBridgeRoute,
    OperationBridgeRouteSpec, namespace_registered_tool,
};
pub use registry::OperationBridgeCleanupCause;
pub(crate) use registry::{BridgeLease, BridgeLeaseOwner, BridgeProfile, OperationBridgeRegistry};
pub(crate) use secret::generate_operation_secret;

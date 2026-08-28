//! Provider-neutral operation-scoped watcher lifecycle core.
//!
//! This module owns identity correlation, pure registry transitions, distinct
//! model and operator control roles, optional host-service registration, and
//! turn activity projection. It does not start processes or select a route.

mod activity;
mod control;
mod host_service;
mod registry;

pub use activity::project_watcher_activity;
pub use control::{
    ModelWatcherControl, ModelWatcherRole, OperatorWatcherControl, OperatorWatcherRole,
    SharedWatcherRegistry, WatcherControlSurface,
};
pub use host_service::WatcherHostService;
pub use registry::{
    WatcherFailure, WatcherFailureKind, WatcherRegistry, WatcherSnapshot,
    WatcherStopAcknowledgement, WatcherWaitRepresentation,
};

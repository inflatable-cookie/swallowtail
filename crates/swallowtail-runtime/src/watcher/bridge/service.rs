//! Object-safe host port for one operation-scoped watcher HTTP bridge.

use super::{WatcherBridgeCompletionState, WatcherBridgeLease, WatcherBridgeOpenRequest};
use crate::{BoxFuture, CleanupOutcome, RuntimeFailure};
use swallowtail_core::WatcherCleanupCause;

/// Host boundary for opening and joining one Contract 060 bridge lease.
///
/// Registration of this port binds no listener. Opening binds one exact host,
/// operation, turn, and registered watcher service.
pub trait WatcherBridgeHostService: Send + Sync {
    /// Binds a ready loopback lease before any provider process is spawned.
    fn open(
        &self,
        request: WatcherBridgeOpenRequest,
    ) -> BoxFuture<'_, Result<WatcherBridgeLease, RuntimeFailure>>;

    /// Observes remaining work and freezes admission when the turn is idle.
    ///
    /// The query does not wait, stop work, or convert a provider-terminal
    /// response into success.
    fn completion_gate(
        &self,
        lease: &WatcherBridgeLease,
    ) -> BoxFuture<'_, Result<WatcherBridgeCompletionState, RuntimeFailure>>;

    /// Freezes admission, joins bridge and watcher work, and releases private material.
    fn close(
        &self,
        lease: WatcherBridgeLease,
        cause: WatcherCleanupCause,
    ) -> BoxFuture<'_, Result<CleanupOutcome, RuntimeFailure>>;
}

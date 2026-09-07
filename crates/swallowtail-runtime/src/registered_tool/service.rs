//! Optional Contract 063 registered-tool bridge host port.
//!
//! Registration of this port binds no transport and starts no work. Opening a
//! lease binds one exact host, configured instance, operation scope, turn
//! attempt, selection, admission binding, and lease generation.

use super::lease::{
    RegisteredToolBridgeLease, RegisteredToolCleanupCause, RegisteredToolCompletionState,
    RegisteredToolOpenRequest,
};
use crate::{BoxFuture, CleanupOutcome, RuntimeFailure};

/// Host boundary for opening, observing, and joining one registered-tool lease.
pub trait RegisteredToolBridgeHostService: Send + Sync {
    /// Binds a ready lease before any provider dispatch.
    ///
    /// Any opening failure joins the partial resources it created.
    fn open(
        &self,
        request: RegisteredToolOpenRequest,
    ) -> BoxFuture<'_, Result<RegisteredToolBridgeLease, RuntimeFailure>>;

    /// Observes outstanding work and freezes admission when the lease is clear.
    ///
    /// The query never silently waits and never turns a provider-terminal
    /// response into success.
    fn completion_gate(
        &self,
        lease: &RegisteredToolBridgeLease,
    ) -> BoxFuture<'_, Result<RegisteredToolCompletionState, RuntimeFailure>>;

    /// Freezes admission, joins issued work and readers, and releases material.
    ///
    /// A cleanup timeout reports failed cleanup; it never reports a clean close
    /// and never releases the lease for reuse.
    fn close(
        &self,
        lease: RegisteredToolBridgeLease,
        cause: RegisteredToolCleanupCause,
    ) -> BoxFuture<'_, Result<CleanupOutcome, RuntimeFailure>>;
}

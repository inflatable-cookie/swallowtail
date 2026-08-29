//! Host process-containment seam for Contract 059 watchers.
//!
//! The watcher registry coordinates lifecycle. Process-backed work is admitted
//! only through an exact injected backend whose lease contains descendants by
//! construction. Process groups, root handles, and process-table observation
//! do not implement this capability.

use std::sync::Arc;
use swallowtail_runtime::{BoxFuture, ProcessHandle, ProcessRequest, RuntimeFailure, ScopeId};

/// Bound start result for one process-backed watcher.
///
/// The lease must already contain the workload before this value returns. The
/// watcher host publishes a public watcher identity only after binding both
/// handles.
pub struct ContainedProcessStart {
    process: Arc<dyn ProcessHandle>,
    lease: Arc<dyn ProcessContainmentLease>,
}

impl ContainedProcessStart {
    /// Creates one contained start from an already-bound process and lease.
    #[must_use]
    pub fn new(process: Arc<dyn ProcessHandle>, lease: Arc<dyn ProcessContainmentLease>) -> Self {
        Self { process, lease }
    }

    /// Returns the contained process handle.
    #[must_use]
    pub fn process(&self) -> &Arc<dyn ProcessHandle> {
        &self.process
    }

    /// Returns the containment lease that owns stop and empty-scope join.
    #[must_use]
    pub fn lease(&self) -> &Arc<dyn ProcessContainmentLease> {
        &self.lease
    }

    /// Splits the start into independently owned handles.
    #[must_use]
    pub fn into_parts(self) -> (Arc<dyn ProcessHandle>, Arc<dyn ProcessContainmentLease>) {
        (self.process, self.lease)
    }
}

/// Owned containment lease for one process-backed watcher.
///
/// Stop and force-stop target the lease, never a caller-supplied PID. Joined
/// cleanup requires an empty containment scope and joined supervision work.
///
/// `prove_empty_and_join` may be observed more than once for one lease. Host
/// callers must treat a successful proof as durable for that lease: either the
/// implementation is idempotent, or the host records and reuses the first
/// result rather than requiring a second independent supervisor join.
pub trait ProcessContainmentLease: Send + Sync {
    /// Requests graceful stop through the containment lease.
    fn request_stop(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>>;

    /// Requests force-stop through the containment lease.
    fn force_stop(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>>;

    /// Proves the containment scope empty and joins supervision before release.
    ///
    /// A successful result is durable for the lease. Repeated calls after
    /// success must remain successful; a failed result remains failed.
    fn prove_empty_and_join(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>>;
}

/// Exact host containment backend required before process-backed watcher work.
///
/// Implementations own descendant inheritance, stop targeting, empty-scope
/// proof, and supervision join. Ordinary local process groups do not qualify.
pub trait ProcessContainmentBackend: Send + Sync {
    /// Starts contained work and returns the bound lease before watcher identity.
    fn start_contained(
        &self,
        scope: ScopeId,
        request: ProcessRequest,
    ) -> BoxFuture<'_, Result<ContainedProcessStart, RuntimeFailure>>;
}

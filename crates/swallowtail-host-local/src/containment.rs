//! Host-private process containment seam for Contract 059 watchers.
//!
//! The watcher registry coordinates lifecycle. This module admits process-backed
//! work only through an exact injected backend whose lease contains descendants
//! by construction. Process groups, root handles, and process-table observation
//! do not implement this capability.

use std::sync::Arc;
use swallowtail_runtime::{
    BoxFuture, ProcessHandle, ProcessRequest, ProcessService, RuntimeFailure, ScopeId,
};

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
pub trait ProcessContainmentLease: Send + Sync {
    /// Requests graceful stop through the containment lease.
    fn request_stop(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>>;

    /// Requests force-stop through the containment lease.
    fn force_stop(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>>;

    /// Proves the containment scope empty and joins supervision before release.
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

/// Deterministic contained-backend probe for registry and lifecycle proofs.
///
/// This probe binds stop and join to the ordinary local process handle. It is
/// not a platform containment implementation and must not be used to claim
/// macOS, Windows, or Linux ownership guarantees.
pub struct LocalProcessContainmentProbe {
    process_host: Arc<crate::LocalProcessHost>,
}

impl LocalProcessContainmentProbe {
    /// Creates a probe that starts work through the supplied local process host.
    #[must_use]
    pub fn new(process_host: Arc<crate::LocalProcessHost>) -> Self {
        Self { process_host }
    }
}

impl ProcessContainmentBackend for LocalProcessContainmentProbe {
    fn start_contained(
        &self,
        scope: ScopeId,
        request: ProcessRequest,
    ) -> BoxFuture<'_, Result<ContainedProcessStart, RuntimeFailure>> {
        let process_host = Arc::clone(&self.process_host);
        Box::pin(async move {
            let process = Arc::from(process_host.start(scope, request).await?);
            let lease: Arc<dyn ProcessContainmentLease> = Arc::new(ProbeContainmentLease {
                process: Arc::clone(&process),
            });
            Ok(ContainedProcessStart::new(process, lease))
        })
    }
}

struct ProbeContainmentLease {
    process: Arc<dyn ProcessHandle>,
}

impl ProcessContainmentLease for ProbeContainmentLease {
    fn request_stop(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
        let process = Arc::clone(&self.process);
        Box::pin(async move { process.request_stop().await })
    }

    fn force_stop(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
        let process = Arc::clone(&self.process);
        Box::pin(async move { process.force_stop().await })
    }

    fn prove_empty_and_join(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
        let process = Arc::clone(&self.process);
        Box::pin(async move {
            process.wait().await?;
            Ok(())
        })
    }
}

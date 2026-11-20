use crate::{
    BoxFuture, CancellationAcknowledgement, CancellationControl, Deadline, ExecutableRef,
    HostServices, ImmediateCancellation, RequestId, RuntimeFailure, ScopeId,
};
use std::fmt;
use std::sync::Arc;
use swallowtail_core::{CancellationScope, ExecutionHostId, InterfaceVersionAxis, SafeDiagnostic};

/// One opaque executable candidate selected and approved by the execution host.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InstalledExecutableTarget {
    executable: ExecutableRef,
    version_axis: InterfaceVersionAxis,
}

impl InstalledExecutableTarget {
    #[must_use]
    /// Creates an exact target from an opaque host-approved executable reference.
    pub const fn new(executable: ExecutableRef, version_axis: InterfaceVersionAxis) -> Self {
        Self {
            executable,
            version_axis,
        }
    }

    #[must_use]
    /// Returns the opaque executable reference selected by the host.
    pub const fn executable(&self) -> &ExecutableRef {
        &self.executable
    }

    #[must_use]
    /// Returns the exact interface-version axis to observe.
    pub const fn version_axis(&self) -> &InterfaceVersionAxis {
        &self.version_axis
    }
}

#[derive(Clone)]
/// Shared operation-scoped cancellation signal for an executable probe.
pub struct DiscoveryCancellation {
    inner: Arc<ImmediateCancellation>,
}

impl DiscoveryCancellation {
    #[must_use]
    /// Creates a cancellation signal in the not-requested state.
    pub fn new() -> Self {
        Self {
            inner: Arc::new(ImmediateCancellation::new(
                CancellationScope::DiscoveryProbe,
            )),
        }
    }

    #[must_use]
    /// Returns whether cancellation has been requested.
    pub fn is_requested(&self) -> bool {
        self.inner.is_requested()
    }

    /// Resolves once cancellation is requested.
    ///
    /// Concurrent waiters are all notified; each registered waker is woken
    /// exactly once outside the waiter lock when the request is recorded.
    pub fn wait_requested(&self) -> BoxFuture<'static, ()> {
        let inner = Arc::clone(&self.inner);
        Box::pin(async move {
            inner.wait_requested().await;
        })
    }

    #[cfg(test)]
    pub(crate) fn inner(&self) -> &ImmediateCancellation {
        &self.inner
    }
}

impl Default for DiscoveryCancellation {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Debug for DiscoveryCancellation {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("DiscoveryCancellation")
            .field("requested", &self.is_requested())
            .finish()
    }
}

impl CancellationControl for DiscoveryCancellation {
    fn scope(&self) -> CancellationScope {
        self.inner.scope()
    }

    fn request(&self) -> BoxFuture<'_, Result<CancellationAcknowledgement, RuntimeFailure>> {
        self.inner.request()
    }
}

/// Bounded request for one installed-executable version observation.
#[derive(Clone, Debug)]
pub struct InstalledExecutableDiscoveryRequest {
    request_id: RequestId,
    scope_id: ScopeId,
    execution_host_id: ExecutionHostId,
    target: InstalledExecutableTarget,
    deadline: Deadline,
    cancellation: DiscoveryCancellation,
}

impl InstalledExecutableDiscoveryRequest {
    #[must_use]
    /// Creates a request bound to one scope, host, target, deadline, and signal.
    pub const fn new(
        request_id: RequestId,
        scope_id: ScopeId,
        execution_host_id: ExecutionHostId,
        target: InstalledExecutableTarget,
        deadline: Deadline,
        cancellation: DiscoveryCancellation,
    ) -> Self {
        Self {
            request_id,
            scope_id,
            execution_host_id,
            target,
            deadline,
            cancellation,
        }
    }

    #[must_use]
    /// Returns the consumer request identity.
    pub const fn request_id(&self) -> &RequestId {
        &self.request_id
    }

    #[must_use]
    /// Returns the operation scope that owns probe work and cleanup.
    pub const fn scope_id(&self) -> &ScopeId {
        &self.scope_id
    }

    #[must_use]
    /// Returns the authoritative execution host for the probe.
    pub const fn execution_host_id(&self) -> &ExecutionHostId {
        &self.execution_host_id
    }

    #[must_use]
    /// Returns the single host-approved executable target.
    pub const fn target(&self) -> &InstalledExecutableTarget {
        &self.target
    }

    #[must_use]
    /// Returns the exact monotonic probe deadline.
    pub const fn deadline(&self) -> Deadline {
        self.deadline
    }

    #[must_use]
    /// Returns the shared operation-scoped cancellation signal.
    pub const fn cancellation(&self) -> &DiscoveryCancellation {
        &self.cancellation
    }
}

/// Validates host identity and required services before an executable probe.
///
/// This performs no executable resolution, process start, or provider work.
pub fn validate_installed_executable_discovery_services(
    request: &InstalledExecutableDiscoveryRequest,
    services: &HostServices,
) -> Result<(), RuntimeFailure> {
    services.require_execution_host(request.execution_host_id())?;
    if services.task().is_none() || services.time().is_none() || services.process().is_none() {
        return Err(RuntimeFailure::new(SafeDiagnostic::new(
            "swallowtail.installed_executable.host_services_missing",
            "Installed executable discovery requires task, time, and process host services",
        )));
    }
    Ok(())
}

#[cfg(test)]
#[path = "installed_executable_tests.rs"]
mod tests;

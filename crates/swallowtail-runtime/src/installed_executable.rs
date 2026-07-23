use crate::{
    BoxFuture, CancellationAcknowledgement, CancellationControl, Deadline, ExecutableRef,
    HostServices, RequestId, RuntimeFailure, ScopeId,
};
use std::fmt;
use std::future::poll_fn;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::task::Poll;
use swallowtail_core::{CancellationScope, ExecutionHostId, InterfaceVersionAxis, SafeDiagnostic};

/// One opaque executable candidate selected and approved by the execution host.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InstalledExecutableTarget {
    executable: ExecutableRef,
    version_axis: InterfaceVersionAxis,
}

impl InstalledExecutableTarget {
    #[must_use]
    pub const fn new(executable: ExecutableRef, version_axis: InterfaceVersionAxis) -> Self {
        Self {
            executable,
            version_axis,
        }
    }

    #[must_use]
    pub const fn executable(&self) -> &ExecutableRef {
        &self.executable
    }

    #[must_use]
    pub const fn version_axis(&self) -> &InterfaceVersionAxis {
        &self.version_axis
    }
}

#[derive(Clone)]
pub struct DiscoveryCancellation {
    state: Arc<DiscoveryCancellationState>,
}

struct DiscoveryCancellationState {
    requested: AtomicBool,
    waiter: Mutex<Option<std::task::Waker>>,
}

impl DiscoveryCancellation {
    #[must_use]
    pub fn new() -> Self {
        Self {
            state: Arc::new(DiscoveryCancellationState {
                requested: AtomicBool::new(false),
                waiter: Mutex::new(None),
            }),
        }
    }

    #[must_use]
    pub fn is_requested(&self) -> bool {
        self.state.requested.load(Ordering::SeqCst)
    }

    pub fn wait_requested(&self) -> BoxFuture<'static, ()> {
        let state = Arc::clone(&self.state);
        Box::pin(poll_fn(move |context| {
            if state.requested.load(Ordering::SeqCst) {
                Poll::Ready(())
            } else {
                *state
                    .waiter
                    .lock()
                    .expect("discovery cancellation waiter lock poisoned") =
                    Some(context.waker().clone());
                if state.requested.load(Ordering::SeqCst) {
                    Poll::Ready(())
                } else {
                    Poll::Pending
                }
            }
        }))
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
        CancellationScope::DiscoveryProbe
    }

    fn request(&self) -> BoxFuture<'_, Result<CancellationAcknowledgement, RuntimeFailure>> {
        let acknowledgement = if self.state.requested.swap(true, Ordering::SeqCst) {
            CancellationAcknowledgement::AlreadyRequested
        } else {
            if let Some(waiter) = self
                .state
                .waiter
                .lock()
                .expect("discovery cancellation waiter lock poisoned")
                .take()
            {
                waiter.wake();
            }
            CancellationAcknowledgement::Requested
        };
        Box::pin(async move { Ok(acknowledgement) })
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
    pub const fn request_id(&self) -> &RequestId {
        &self.request_id
    }

    #[must_use]
    pub const fn scope_id(&self) -> &ScopeId {
        &self.scope_id
    }

    #[must_use]
    pub const fn execution_host_id(&self) -> &ExecutionHostId {
        &self.execution_host_id
    }

    #[must_use]
    pub const fn target(&self) -> &InstalledExecutableTarget {
        &self.target
    }

    #[must_use]
    pub const fn deadline(&self) -> Deadline {
        self.deadline
    }

    #[must_use]
    pub const fn cancellation(&self) -> &DiscoveryCancellation {
        &self.cancellation
    }
}

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

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use swallowtail_core::SafeDiagnostic;
use swallowtail_runtime::{
    BoxFuture, ProcessHandle, ProcessRequest, ProcessService, RuntimeFailure, ScopeId,
};

use crate::containment::{
    ContainedProcessStart, ProcessContainmentBackend, ProcessContainmentLease,
};
use crate::host::LocalProcessHost;

/// Crate-local recording backend for watcher unit tests.
pub(super) struct TestContainmentBackend {
    process_host: Arc<LocalProcessHost>,
    state: Arc<TestContainmentState>,
}

struct TestContainmentState {
    calls: Mutex<Vec<&'static str>>,
    force_stop_fails: AtomicBool,
}

impl TestContainmentBackend {
    pub(super) fn new(process_host: Arc<LocalProcessHost>) -> Self {
        Self {
            process_host,
            state: Arc::new(TestContainmentState {
                calls: Mutex::new(Vec::new()),
                force_stop_fails: AtomicBool::new(false),
            }),
        }
    }

    pub(super) fn calls(&self) -> Vec<&'static str> {
        self.state
            .calls
            .lock()
            .expect("test containment lock")
            .clone()
    }

    pub(super) fn fail_force_stop(&self) {
        self.state.force_stop_fails.store(true, Ordering::SeqCst);
    }
}

impl ProcessContainmentBackend for TestContainmentBackend {
    fn start_contained(
        &self,
        scope: ScopeId,
        request: ProcessRequest,
    ) -> BoxFuture<'_, Result<ContainedProcessStart, RuntimeFailure>> {
        let process_host = Arc::clone(&self.process_host);
        let state = Arc::clone(&self.state);
        Box::pin(async move {
            let process = Arc::from(process_host.start(scope, request).await?);
            state
                .calls
                .lock()
                .expect("test containment lock")
                .push("backend.start_contained");
            let lease: Arc<dyn ProcessContainmentLease> = Arc::new(TestContainmentLease {
                process: Arc::clone(&process),
                state,
            });
            Ok(ContainedProcessStart::new(process, lease))
        })
    }
}

struct TestContainmentLease {
    process: Arc<dyn ProcessHandle>,
    state: Arc<TestContainmentState>,
}

impl ProcessContainmentLease for TestContainmentLease {
    fn request_stop(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
        self.state
            .calls
            .lock()
            .expect("test containment lock")
            .push("lease.request_stop");
        let process = Arc::clone(&self.process);
        Box::pin(async move { process.request_stop().await })
    }

    fn force_stop(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
        self.state
            .calls
            .lock()
            .expect("test containment lock")
            .push("lease.force_stop");
        if self.state.force_stop_fails.load(Ordering::SeqCst) {
            return Box::pin(async {
                Err(RuntimeFailure::new(SafeDiagnostic::new(
                    "fixture.containment.force_stop_failed",
                    "Fixture containment force-stop failed",
                )))
            });
        }
        let process = Arc::clone(&self.process);
        Box::pin(async move { process.force_stop().await })
    }

    fn prove_empty_and_join(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
        self.state
            .calls
            .lock()
            .expect("test containment lock")
            .push("lease.prove_empty_and_join");
        let process = Arc::clone(&self.process);
        Box::pin(async move {
            process.wait().await?;
            Ok(())
        })
    }
}

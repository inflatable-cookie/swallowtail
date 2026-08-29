use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use swallowtail_core::SafeDiagnostic;
use swallowtail_host_local::{
    ContainedProcessStart, LocalProcessHost, ProcessContainmentBackend, ProcessContainmentLease,
};
use swallowtail_runtime::{
    BoxFuture, ProcessHandle, ProcessRequest, ProcessService, RuntimeFailure, ScopeId,
};

/// Deterministic recording containment backend for watcher lifecycle proofs.
///
/// Lease calls are recorded separately from root process-handle calls so tests
/// can prove Contract 059 routing. This is test support only.
pub(super) struct RecordingContainmentBackend {
    process_host: Arc<LocalProcessHost>,
    state: Arc<RecordingState>,
}

struct RecordingState {
    calls: Mutex<Vec<&'static str>>,
    force_stop_fails: AtomicBool,
    prove_empty_fails: AtomicBool,
    prove_empty_count: AtomicUsize,
}

impl RecordingContainmentBackend {
    pub(super) fn new(process_host: Arc<LocalProcessHost>) -> Self {
        Self {
            process_host,
            state: Arc::new(RecordingState {
                calls: Mutex::new(Vec::new()),
                force_stop_fails: AtomicBool::new(false),
                prove_empty_fails: AtomicBool::new(false),
                prove_empty_count: AtomicUsize::new(0),
            }),
        }
    }

    pub(super) fn calls(&self) -> Vec<&'static str> {
        self.state
            .calls
            .lock()
            .expect("recording containment lock poisoned")
            .clone()
    }

    pub(super) fn fail_prove_empty(&self) {
        self.state.prove_empty_fails.store(true, Ordering::SeqCst);
    }

    #[allow(dead_code)]
    pub(super) fn fail_force_stop(&self) {
        self.state.force_stop_fails.store(true, Ordering::SeqCst);
    }

    pub(super) fn prove_empty_count(&self) -> usize {
        self.state.prove_empty_count.load(Ordering::SeqCst)
    }
}

impl ProcessContainmentBackend for RecordingContainmentBackend {
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
                .expect("recording containment lock poisoned")
                .push("backend.start_contained");
            let lease: Arc<dyn ProcessContainmentLease> = Arc::new(RecordingContainmentLease {
                process: Arc::clone(&process),
                state,
            });
            Ok(ContainedProcessStart::new(process, lease))
        })
    }
}

struct RecordingContainmentLease {
    process: Arc<dyn ProcessHandle>,
    state: Arc<RecordingState>,
}

impl RecordingContainmentLease {
    fn record(&self, call: &'static str) {
        self.state
            .calls
            .lock()
            .expect("recording containment lock poisoned")
            .push(call);
    }
}

impl ProcessContainmentLease for RecordingContainmentLease {
    fn request_stop(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
        self.record("lease.request_stop");
        let process = Arc::clone(&self.process);
        Box::pin(async move { process.request_stop().await })
    }

    fn force_stop(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
        self.record("lease.force_stop");
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
        self.record("lease.prove_empty_and_join");
        self.state.prove_empty_count.fetch_add(1, Ordering::SeqCst);
        if self.state.prove_empty_fails.load(Ordering::SeqCst) {
            return Box::pin(async {
                Err(RuntimeFailure::new(SafeDiagnostic::new(
                    "fixture.containment.prove_empty_failed",
                    "Fixture containment empty-scope join failed",
                )))
            });
        }
        let process = Arc::clone(&self.process);
        Box::pin(async move {
            process.wait().await?;
            Ok(())
        })
    }
}

use futures_channel::oneshot;
use futures_executor::block_on;
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};
use swallowtail_core::EndpointAudience;
use swallowtail_host_local::LocalProcessHost;
use swallowtail_runtime::{
    BlockingJob, BlockingWorkService, BoxFuture, CleanupOutcome, CredentialLease, CredentialRef,
    CredentialService, Deadline, DeadlineObservation, EndpointRef, JoinedTask, MonotonicInstant,
    NetworkGrant, NetworkPolicyService, RuntimeFailure, ScopeId, ScopedTaskService, TimeService,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DriverCall {
    TaskSpawn,
    TaskJoin,
    BlockingWork,
    NetworkAuthorize,
    CredentialAcquire,
    CredentialRelease,
}

#[derive(Clone, Default)]
pub struct CallLog(Arc<Mutex<Vec<DriverCall>>>);

impl CallLog {
    fn record(&self, call: DriverCall) {
        self.0.lock().expect("driver call lock poisoned").push(call);
    }

    pub fn calls(&self) -> Vec<DriverCall> {
        self.0.lock().expect("driver call lock poisoned").clone()
    }

    pub fn count(&self, call: DriverCall) -> usize {
        self.calls().iter().filter(|seen| **seen == call).count()
    }
}

#[derive(Clone)]
pub struct ThreadServices {
    origin: Instant,
    calls: CallLog,
}

impl ThreadServices {
    pub fn new(calls: CallLog) -> Self {
        Self {
            origin: Instant::now(),
            calls,
        }
    }
}

struct ThreadTask {
    handle: JoinHandle<()>,
    calls: CallLog,
}

impl JoinedTask for ThreadTask {
    fn join(self: Box<Self>) -> BoxFuture<'static, Result<(), RuntimeFailure>> {
        Box::pin(async move {
            self.handle.join().map_err(|_| {
                RuntimeFailure::new(swallowtail_core::SafeDiagnostic::new(
                    "fixture.task_panicked",
                    "Fixture task panicked",
                ))
            })?;
            self.calls.record(DriverCall::TaskJoin);
            Ok(())
        })
    }
}

impl ScopedTaskService for ThreadServices {
    fn spawn(
        &self,
        _scope: ScopeId,
        task: BoxFuture<'static, ()>,
    ) -> Result<Box<dyn JoinedTask>, RuntimeFailure> {
        self.calls.record(DriverCall::TaskSpawn);
        Ok(Box::new(ThreadTask {
            handle: thread::spawn(move || block_on(task)),
            calls: self.calls.clone(),
        }))
    }
}

impl BlockingWorkService for ThreadServices {
    fn run(
        &self,
        _scope: ScopeId,
        job: BlockingJob,
    ) -> BoxFuture<'static, Result<(), RuntimeFailure>> {
        self.calls.record(DriverCall::BlockingWork);
        let (sender, receiver) = oneshot::channel();
        thread::spawn(move || {
            let _ = sender.send(job());
        });
        Box::pin(async move {
            receiver.await.map_err(|_| {
                RuntimeFailure::new(swallowtail_core::SafeDiagnostic::new(
                    "fixture.blocking_panicked",
                    "Fixture blocking work panicked",
                ))
            })?
        })
    }
}

impl TimeService for ThreadServices {
    fn now(&self) -> MonotonicInstant {
        MonotonicInstant::from_ticks(self.origin.elapsed().as_millis() as u64)
    }

    fn wait_until(&self, deadline: Deadline) -> BoxFuture<'static, DeadlineObservation> {
        Box::pin(async move {
            thread::sleep(Duration::from_millis(10));
            DeadlineObservation::new(deadline, deadline.instant())
        })
    }
}

pub struct TrackingNetwork {
    pub inner: LocalProcessHost,
    pub calls: CallLog,
}

impl NetworkPolicyService for TrackingNetwork {
    fn authorize(
        &self,
        scope: ScopeId,
        endpoint: EndpointRef,
        audience: EndpointAudience,
    ) -> BoxFuture<'static, Result<NetworkGrant, RuntimeFailure>> {
        self.calls.record(DriverCall::NetworkAuthorize);
        self.inner.authorize(scope, endpoint, audience)
    }
}

pub struct TrackingCredential {
    pub inner: LocalProcessHost,
    pub calls: CallLog,
}

impl CredentialService for TrackingCredential {
    fn acquire(
        &self,
        scope: ScopeId,
        reference: CredentialRef,
        audience: EndpointAudience,
    ) -> BoxFuture<'static, Result<CredentialLease, RuntimeFailure>> {
        self.calls.record(DriverCall::CredentialAcquire);
        self.inner.acquire(scope, reference, audience)
    }

    fn release(&self, lease: CredentialLease) -> BoxFuture<'static, CleanupOutcome> {
        let release = self.inner.release(lease);
        let calls = self.calls.clone();
        Box::pin(async move {
            let outcome = release.await;
            calls.record(DriverCall::CredentialRelease);
            outcome
        })
    }
}

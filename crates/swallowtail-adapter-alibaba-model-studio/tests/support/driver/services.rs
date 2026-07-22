use futures_channel::oneshot;
use futures_executor::block_on;
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};
use swallowtail_core::EndpointAudience;
use swallowtail_runtime::{
    BlockingJob, BlockingWorkService, BoxFuture, CleanupOutcome, CredentialLease, CredentialRef,
    CredentialService, Deadline, DeadlineObservation, JoinedTask, MonotonicInstant, RuntimeFailure,
    ScopeId, ScopedTaskService, TimeService,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DriverCall {
    TaskSpawn,
    TaskJoin,
    BlockingWork,
    CredentialAcquire,
    CredentialRelease,
}

#[derive(Clone, Default)]
pub struct CallLog(Arc<Mutex<Vec<DriverCall>>>);

impl CallLog {
    pub fn record(&self, call: DriverCall) {
        self.0.lock().expect("call lock").push(call);
    }
    pub fn calls(&self) -> Vec<DriverCall> {
        self.0.lock().expect("call lock").clone()
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
    pub fn completed_blocking(&self) -> usize {
        self.calls.count(DriverCall::BlockingWork)
    }
}

struct ThreadTask {
    thread: JoinHandle<()>,
    calls: CallLog,
}

impl JoinedTask for ThreadTask {
    fn join(self: Box<Self>) -> BoxFuture<'static, Result<(), RuntimeFailure>> {
        Box::pin(async move {
            self.thread.join().map_err(|_| {
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
            thread: thread::spawn(move || block_on(task)),
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
        let (sender, receiver) = oneshot::channel();
        let calls = self.calls.clone();
        thread::spawn(move || {
            let result = job();
            calls.record(DriverCall::BlockingWork);
            let _ = sender.send(result);
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
        let remaining = deadline
            .instant()
            .ticks()
            .saturating_sub(self.now().ticks());
        let (sender, receiver) = oneshot::channel();
        thread::spawn(move || {
            thread::sleep(Duration::from_millis(remaining));
            let _ = sender.send(DeadlineObservation::new(deadline, deadline.instant()));
        });
        Box::pin(async move { receiver.await.expect("deadline observation") })
    }
}

pub struct TrackingCredential<T> {
    pub inner: T,
    pub calls: CallLog,
    pub blocking: ThreadServices,
    pub releases_after_blocking: Arc<Mutex<Vec<usize>>>,
}

impl<T: CredentialService> CredentialService for TrackingCredential<T> {
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
        let blocking = self.blocking.clone();
        let order = Arc::clone(&self.releases_after_blocking);
        Box::pin(async move {
            order
                .lock()
                .expect("release-order lock")
                .push(blocking.completed_blocking());
            let outcome = release.await;
            calls.record(DriverCall::CredentialRelease);
            outcome
        })
    }
}
